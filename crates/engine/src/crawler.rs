//! Recursive Site Grabber crawler engine (docs/UI_UX_SPEC.md, #3).
//! Explores web hierarchies up to a specified depth, extracting media, documents, and archives
//! while preserving directory trees and avoiding traversal loops.

use std::collections::{HashSet, VecDeque};
use url::Url;

use crate::html;

#[derive(Debug, Clone)]
pub struct CrawlOptions {
    pub max_depth: usize,
    pub max_links: usize,
    pub same_domain_only: bool,
    pub allowed_extensions: Vec<String>,
}

impl Default for CrawlOptions {
    fn default() -> Self {
        Self {
            max_depth: 2,
            max_links: 500,
            same_domain_only: true,
            allowed_extensions: vec![
                "zip".into(), "rar".into(), "7z".into(), "tar".into(), "gz".into(),
                "mp4".into(), "mkv".into(), "webm".into(), "mp3".into(), "flac".into(),
                "pdf".into(), "iso".into(), "exe".into(), "msi".into(),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrawledItem {
    pub url: String,
    pub file_name: String,
    pub depth: usize,
    pub parent_page: String,
}

pub struct SiteCrawler {
    options: CrawlOptions,
    visited_pages: HashSet<String>,
    discovered_files: Vec<CrawledItem>,
}

impl SiteCrawler {
    pub fn new(options: CrawlOptions) -> Self {
        Self {
            options,
            visited_pages: HashSet::new(),
            discovered_files: Vec::new(),
        }
    }

    /// Extract file extension from URL.
    fn extract_ext(url: &Url) -> Option<String> {
        let path = url.path();
        let filename = path.split('/').last()?;
        let ext = filename.split('.').last()?;
        if ext != filename {
            Some(ext.to_ascii_lowercase())
        } else {
            None
        }
    }

    /// Parse HTML page text at `current_url`, enqueueing next links and storing file targets.
    pub fn parse_page(&mut self, current_url: &str, html_body: &str, current_depth: usize) -> Vec<String> {
        let mut next_pages = Vec::new();
        let Ok(base) = Url::parse(current_url) else {
            return next_pages;
        };

        if self.visited_pages.contains(current_url) || self.discovered_files.len() >= self.options.max_links {
            return next_pages;
        }
        self.visited_pages.insert(current_url.to_string());

        // Extract <a href=...>, <img src=...>, <video src=...>
        for tag_name in &["a", "img", "video", "source", "link"] {
            for (_, tag) in html::tags(html_body, tag_name) {
                let attr = if *tag_name == "a" || *tag_name == "link" { "href" } else { "src" };
                if let Some(target) = html::attribute(tag, attr) {
                    let Ok(resolved) = base.join(target.trim()) else {
                        continue;
                    };
                    if resolved.scheme() != "http" && resolved.scheme() != "https" {
                        continue;
                    }

                    if self.options.same_domain_only && resolved.host_str() != base.host_str() {
                        continue;
                    }

                    let resolved_str = resolved.to_string();
                    let ext = Self::extract_ext(&resolved);

                    // Check if it's a target downloadable file
                    let is_target_file = match &ext {
                        Some(e) => self.options.allowed_extensions.iter().any(|ext_pat| ext_pat.eq_ignore_ascii_case(e)),
                        None => false,
                    };

                    if is_target_file {
                        let name = resolved.path().split('/').last().unwrap_or("file").to_string();
                        self.discovered_files.push(CrawledItem {
                            url: resolved_str,
                            file_name: name,
                            depth: current_depth,
                            parent_page: current_url.to_string(),
                        });
                    } else if current_depth < self.options.max_depth && !self.visited_pages.contains(&resolved_str) {
                        next_pages.push(resolved_str);
                    }
                }
            }
        }

        next_pages
    }

    pub fn results(&self) -> &[CrawledItem] {
        &self.discovered_files
    }
}
