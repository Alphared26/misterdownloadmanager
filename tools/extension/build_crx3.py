"""Build a CRX3 package for the fork's unpacked extension.

Chromium browsers deepseek-v4-pro-install an extension named in
HKCU\\Software\\Google\\Chrome\\Extensions\\<ID> when its `update_url` serves a
valid CRX3 (the same protocol the Chrome Web Store uses). The CRX is signed
with the extension's manifest key, so its ID stays stable.
"""
import base64
import hashlib
import io
import json
import os
import zipfile
from pathlib import Path

from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import padding as apadding
from cryptography.hazmat.primitives.asymmetric import rsa

ROOT = Path(__file__).resolve().parents[2]
EXT_DIR = ROOT / "extension"
OUT = ROOT / "target" / "extension" / "extension.crx"
PRIV_KEY = ROOT / "mdm_ext_private.pem"


def asn1(tag: int, data: bytes) -> bytes:
    if len(data) < 128:
        length = bytes([len(data)])
    else:
        lb = len(data).to_bytes((len(data).bit_length() + 7) // 8, "big")
        length = bytes([0x80 | len(lb)]) + lb
    return bytes([tag]) + length + data


def crx3_header(pubkey_der: bytes) -> bytes:
    crx_id = hashlib.sha256(pubkey_der).digest()[:16]
    signed = asn1(0x30, asn1(0x80 | 0, crx_id) + asn1(0x80 | 1, pubkey_der))
    sig = priv.sign(signed, apadding.PKCS1v15(), hashes.SHA256())
    proof = asn1(0x30, asn1(0x80 | 0, pubkey_der) + asn1(0x80 | 1, sig))
    # signature_algorithm 1 = RSA PKCS#1 v1.5 with SHA-256
    sig_entry = asn1(0x30, asn1(0x80 | 0, b"\x02\x01\x01") + asn1(0x80 | 1, sig))
    return asn1(0x30, signed + asn1(0x80 | 0, proof) + asn1(0x80 | 2, sig_entry))


priv = serialization.load_pem_private_key(PRIV_KEY.read_bytes(), password=None)
pub_der = priv.public_key().public_bytes(
    serialization.Encoding.DER, serialization.PublicFormat.SubjectPublicKeyInfo
)

buf = io.BytesIO()
with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as z:
    for path in sorted(EXT_DIR.rglob("*")):
        if path.is_file():
            z.write(path, path.relative_to(EXT_DIR).as_posix())
zip_bytes = buf.getvalue()

header = crx3_header(pub_der)
OUT.parent.mkdir(parents=True, exist_ok=True)
OUT.write_bytes(
    b"Cr24" + (3).to_bytes(4, "little") + len(header).to_bytes(4, "little") + header + zip_bytes
)

m = json.loads((EXT_DIR / "manifest.json").read_text())
pub_b64 = base64.b64decode(m["key"])
ext_id = "".join(chr(ord("a") + b % 16) for b in hashlib.sha256(pub_b64).digest()[:16])
print(f"CRX3 written: {OUT} ({OUT.stat().st_size} bytes)")
print(f"Extension ID: {ext_id}")
print(f"update_url: http://127.0.0.1:8080/extension/extension.crx")
