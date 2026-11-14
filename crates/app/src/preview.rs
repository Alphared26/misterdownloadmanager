//! In-Flight Media Previewer (docs/UI_UX_SPEC.md, #8).
//! Probes in-progress `.mdm.part` files to inspect video/audio headers and provide
//! early playback or stream validation while transfers are still under way.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaContainer {
    Mp4,
    MatroskaWebM,
    Mp3,
    Unknown,
}

pub struct PartPreview {
    pub container: MediaContainer,
    pub header_bytes: Vec<u8>,
    pub is_playable: bool,
}

/// Inspect the initial chunk of an `.mdm.part` file to verify stream integrity and format.
pub fn inspect_part_file(part_path: &Path) -> std::io::Result<PartPreview> {
    let mut file = File::open(part_path)?;
    file.seek(SeekFrom::Start(0))?;

    // Read up to 16KB of header bytes
    let mut buffer = vec![0u8; 16384];
    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);

    let mut container = MediaContainer::Unknown;
    let mut is_playable = false;

    if buffer.len() >= 8 {
        // MP4 signature: bytes 4..8 usually 'ftyp'
        if &buffer[4..8] == b"ftyp" {
            container = MediaContainer::Mp4;
            is_playable = buffer.len() >= 4096;
        }
        // Matroska / WebM EBML ID: 0x1A, 0x45, 0xDF, 0xA3
        else if buffer.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
            container = MediaContainer::MatroskaWebM;
            is_playable = buffer.len() >= 4096;
        }
        // MP3 ID3v2 tag or MPEG audio frame sync
        else if buffer.starts_with(b"ID3") || (buffer[0] == 0xFF && (buffer[1] & 0xE0) == 0xE0) {
            container = MediaContainer::Mp3;
            is_playable = buffer.len() >= 2048;
        }
    }

    Ok(PartPreview {
        container,
        header_bytes: buffer,
        is_playable,
    })
}
