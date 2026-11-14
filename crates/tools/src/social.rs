//! Social media scraping and media extraction via yt-dlp and gallery-dl helpers.
//! Provides unified parsing for Instagram, TikTok, Twitter/X, and Pinterest.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Platform {
    Instagram,
    TikTok,
    Twitter,
    Pinterest,
    Generic,
}

impl Platform {
    /// Detect platform from URL.
    pub fn from_url(url: &str) -> Self {
        let lower = url.to_ascii_lowercase();
        if lower.contains("instagram.com") || lower.contains("instagr.am") {
            Self::Instagram
        } else if lower.contains("tiktok.com") {
            Self::TikTok
        } else if lower.contains("twitter.com") || lower.contains("x.com") {
            Self::Twitter
        } else if lower.contains("pinterest.com") || lower.contains("pin.it") {
            Self::Pinterest
        } else {
            Self::Generic
        }
    }

    /// Whether the target is best suited for gallery-dl (photo carousels/albums).
    pub fn prefers_gallery_dl(&self, url: &str) -> bool {
        match self {
            Self::Pinterest => true,
            Self::Instagram => !url.contains("/reel/") && !url.contains("/tv/"),
            _ => false,
        }
    }
}

/// Extra extraction arguments to inject for social media downloads.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SocialMediaOptions {
    pub cookies_text: Option<String>,
    pub no_watermark: bool,
    pub extract_audio: bool,
}

impl SocialMediaOptions {
    /// Recommended command arguments for yt-dlp based on platform.
    pub fn ytdlp_args(&self, platform: Platform) -> Vec<String> {
        let mut args = Vec::new();
        match platform {
            Platform::TikTok => {
                // Prefer clean, direct video streams without TikTok server-side overlays
                args.push("--format".to_string());
                args.push("bestvideo+bestaudio/best".to_string());
            }
            Platform::Instagram => {
                // Ensure best progressive MP4 or audio muxing
                args.push("--format".to_string());
                args.push("best".to_string());
            }
            Platform::Twitter => {
                args.push("--format".to_string());
                args.push("best".to_string());
            }
            _ => {}
        }
        if self.extract_audio {
            args.push("-x".to_string());
            args.push("--audio-format".to_string());
            args.push("mp3".to_string());
        }
        args
    }
}
