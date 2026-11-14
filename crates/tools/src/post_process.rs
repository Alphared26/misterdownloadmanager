//! Post-processing pipeline: auto-extraction, transcoding, and checksum verification (docs/UI_UX_SPEC.md, #9, #19).

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostAction {
    ExtractArchive { password: Option<String> },
    ConvertToMp3,
    ConvertToMp4,
    VerifyChecksum { expected: String, is_sha256: bool },
}

pub struct ChecksumResult {
    pub sha256: String,
    pub matches: bool,
}

/// Compute SHA-256 hash of a completed downloaded file.
pub fn verify_file_hashes(path: &Path, expected_hash: Option<&str>) -> std::io::Result<ChecksumResult> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut sha256_hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        sha256_hasher.update(&buffer[..bytes_read]);
    }

    let result = sha256_hasher.finalize();
    let sha256_str = hex::encode(result);

    let matches = match expected_hash {
        Some(expected) => {
            let exp_clean = expected.trim().to_ascii_lowercase();
            exp_clean == sha256_str
        }
        None => true,
    };

    Ok(ChecksumResult {
        sha256: sha256_str,
        matches,
    })
}
