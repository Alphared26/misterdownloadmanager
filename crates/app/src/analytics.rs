//! Download Analytics & Traffic Heatmap Dashboard (docs/UI_UX_SPEC.md, #20).
//! Aggregates bandwidth statistics, domain throughput rankings, and hourly transfer metrics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DomainStat {
    pub total_bytes: u64,
    pub total_files: usize,
    pub avg_speed_bps: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AnalyticsRecord {
    pub total_bytes_downloaded: u64,
    pub total_bytes_uploaded: u64,
    pub completed_count: usize,
    pub failed_count: usize,
    pub domain_breakdown: HashMap<String, DomainStat>,
    pub hourly_traffic_bytes: [u64; 24],
}

impl AnalyticsRecord {
    pub fn record_completed(
        &mut self,
        bytes: u64,
        domain: &str,
        avg_speed: u64,
        hour_of_day: usize,
    ) {
        self.total_bytes_downloaded = self.total_bytes_downloaded.saturating_add(bytes);
        self.completed_count += 1;

        if hour_of_day < 24 {
            self.hourly_traffic_bytes[hour_of_day] =
                self.hourly_traffic_bytes[hour_of_day].saturating_add(bytes);
        }

        let entry = self.domain_breakdown.entry(domain.to_lowercase()).or_default();
        entry.total_bytes = entry.total_bytes.saturating_add(bytes);
        entry.total_files += 1;
        entry.avg_speed_bps = (entry.avg_speed_bps + avg_speed) / 2;
    }

    pub fn record_failure(&mut self) {
        self.failed_count += 1;
    }

    /// Top N domains by transferred bandwidth.
    pub fn top_domains(&self, limit: usize) -> Vec<(String, u64)> {
        let mut list: Vec<(String, u64)> = self
            .domain_breakdown
            .iter()
            .map(|(d, s)| (d.clone(), s.total_bytes))
            .collect();
        list.sort_by(|a, b| b.1.cmp(&a.1));
        list.truncate(limit);
        list
    }
}
