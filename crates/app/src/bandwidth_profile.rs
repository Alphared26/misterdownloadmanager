//! Smart Bandwidth Scheduler & Profiles (docs/UI_UX_SPEC.md, #18).
//! Manages throughput limits and connection worker caps tailored for gaming,
//! office work, or unmetered nighttime transfers.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfileKind {
    Unlimited,
    Gaming,
    WorkHours,
    NightBoost,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthProfile {
    pub kind: ProfileKind,
    pub name: String,
    pub max_download_kbs: Option<u64>,
    pub max_upload_kbs: Option<u64>,
    pub max_connections_per_download: usize,
    pub prioritize_interactive: bool,
}

impl BandwidthProfile {
    pub fn gaming() -> Self {
        Self {
            kind: ProfileKind::Gaming,
            name: "Gaming Mode".to_string(),
            max_download_kbs: Some(250), // Low throttle to avoid ping jitter
            max_upload_kbs: Some(50),
            max_connections_per_download: 2,
            prioritize_interactive: true,
        }
    }

    pub fn work_hours() -> Self {
        Self {
            kind: ProfileKind::WorkHours,
            name: "Work Hours".to_string(),
            max_download_kbs: Some(2048),
            max_upload_kbs: Some(512),
            max_connections_per_download: 4,
            prioritize_interactive: true,
        }
    }

    pub fn night_boost() -> Self {
        Self {
            kind: ProfileKind::NightBoost,
            name: "Night Boost".to_string(),
            max_download_kbs: None, // Unlimited
            max_upload_kbs: None,
            max_connections_per_download: 32,
            prioritize_interactive: false,
        }
    }

    pub fn unlimited() -> Self {
        Self {
            kind: ProfileKind::Unlimited,
            name: "Default (Unlimited)".to_string(),
            max_download_kbs: None,
            max_upload_kbs: None,
            max_connections_per_download: 8,
            prioritize_interactive: false,
        }
    }
}

pub struct ProfileManager {
    pub active_profile: BandwidthProfile,
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self {
            active_profile: BandwidthProfile::unlimited(),
        }
    }
}

impl ProfileManager {
    pub fn set_profile(&mut self, kind: ProfileKind) {
        self.active_profile = match kind {
            ProfileKind::Gaming => BandwidthProfile::gaming(),
            ProfileKind::WorkHours => BandwidthProfile::work_hours(),
            ProfileKind::NightBoost => BandwidthProfile::night_boost(),
            _ => BandwidthProfile::unlimited(),
        };
    }
}
