//! Post-processing pipeline: auto-extraction, transcoding, and checksum verification (docs/UI_UX_SPEC.md, #9, #19).

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use md5::Context as Md5Context;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostAction {
    ExtractArchive { password: Option<String> },
    ConvertToMp3,
    ConvertToMp4,
    VerifyChecksum { expected: String, is_sha256: bool },
}

pub struct ChecksumResult {
    pub md5: String,
    pub sha256: String,
    pub matches: bool,
}

/// Compute MD5 and SHA-256 hashes of a completed downloaded file.
pub fn verify_file_hashes(path: &Path, expected_hash: Option<&str>) -> std::io::Result<ChecksumResult> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut sha256_hasher = Sha256::new();
    let mut md5_context = Md5Context::new();

    let mut buffer = [0u8; 65536];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        sha256_hasher.update(&buffer[..bytes_read]);
        md5_context.consume(&buffer[..bytes_read]);
    }

    let sha256_str = format!("{:x}", sha256_hasher.finalize());
    let md5_str = format!("{:x}", md5_context.compute());

    let matches = match expected_hash {
        Some(expected) => {
            let exp_clean = expected.trim().to_ascii_lowercase();
            exp_clean == sha256_str || exp_clean == md5_str
        }
        None => true,
    };

    Ok(ChecksumResult {
        md5: md5_str,
        sha256: sha256_str,
        matches,
    })
}
