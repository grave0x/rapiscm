//! Tracker signature database — ~200 entries, ~3KB, zero deps.

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TrackerCategory {
    Analytics,
    Advertising,
    ConsentManagement,
    Fingerprinting,
    SessionReplay,
    SocialMedia,
    Cdn,
    Utility,
}

impl TrackerCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            TrackerCategory::Analytics => "analytics",
            TrackerCategory::Advertising => "advertising",
            TrackerCategory::ConsentManagement => "consent-management",
            TrackerCategory::Fingerprinting => "fingerprinting",
            TrackerCategory::SessionReplay => "session-replay",
            TrackerCategory::SocialMedia => "social-media",
            TrackerCategory::Cdn => "cdn",
            TrackerCategory::Utility => "utility",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TrackerSignature {
    pub name: &'static str,
    pub category: TrackerCategory,
    pub company: &'static str,
    pub domains: &'static [&'static str],
    pub script_patterns: &'static [&'static str],
    pub cookie_names: &'static [&'static str],
    pub purpose: &'static str,
}

/// Scan response body text for tracker signatures.
pub fn detect_trackers(body: &str, headers: &[(String, String)]) -> Vec<TrackerSignature> {
    let mut found: Vec<TrackerSignature> = Vec::new();
    'sig: for sig in DATABASE.iter() {
        // Match script patterns in body.
        for pat in sig.script_patterns {
            if body.contains(pat) {
                // If we already found this tracker by domain match, skip duplicate.
                if found.iter().any(|f| f.name == sig.name) {
                    continue 'sig;
                }
                found.push(sig.clone());
                continue 'sig;
            }
        }
        // Match domain patterns in script src URLs.
        for dom in sig.domains {
            if body.contains(dom) {
                if found.iter().any(|f| f.name == sig.name) {
                    continue 'sig;
                }
                found.push(sig.clone());
                continue 'sig;
            }
        }
    }

    // Also match against response header values (e.g. server headers).
    for (k, v) in headers {
        let combined = format!("{k}: {v}");
        for sig in DATABASE.iter() {
            for dom in sig.domains {
                if combined.contains(dom) {
                    if !found.iter().any(|f| f.name == sig.name) {
                        found.push(sig.clone());
                    }
                    break;
                }
            }
        }
    }

    found
}

include!("sigdb.rs");
