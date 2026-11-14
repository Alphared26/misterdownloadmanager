//! Pre-flight link health probe and anti-duplicate validator (docs/UI_UX_SPEC.md, #4).
//! Checks URL availability, content length, and server range support prior to queue insertion.

use std::collections::HashSet;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy {
        content_length: Option<u64>,
        accepts_ranges: bool,
        content_type: Option<String>,
        suggested_name: Option<String>,
    },
    ExpiredOrUnauthorized, // 401, 403, 410 (needs token refresh)
    NotFound,              // 404
    ServerError(u16),      // 5xx
    Unreachable(String),   // DNS / Connection timeout
    DuplicateInQueue(u64), // Existing item ID
}

pub struct HealthProbe {
    known_hashes: HashSet<String>,
}

impl HealthProbe {
    pub fn new() -> Self {
        Self {
            known_hashes: HashSet::new(),
        }
    }

    /// Canonical key for duplicate checking.
    pub fn canonical_key(url: &str) -> String {
        let Ok(parsed) = Url::parse(url) else {
            return url.to_string();
        };
        let host = parsed.host_str().unwrap_or("").to_lowercase();
        let path = parsed.path().trim_end_matches('/');
        format!("{}://{}{}", parsed.scheme(), host, path)
    }

    /// Check if URL is already queued.
    pub fn is_duplicate(&self, url: &str) -> bool {
        let key = Self::canonical_key(url);
        self.known_hashes.contains(&key)
    }

    /// Register a newly added URL.
    pub fn register(&mut self, url: &str) {
        let key = Self::canonical_key(url);
        self.known_hashes.insert(key);
    }
}
