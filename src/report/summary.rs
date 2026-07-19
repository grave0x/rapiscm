//! Summary statistics and text-formatted scan reports.

use std::collections::BTreeMap;

use crate::types::{ResponseResult, Severity};

/// Aggregated scan statistics.
pub struct SummaryStats {
    pub total: usize,
    pub successful: usize,
    pub errors: usize,
    pub checks_passed: usize,
    pub checks_failed: usize,
    pub checks_warn: usize,
    pub trackers_total: usize,
    pub trackers_by_category: BTreeMap<&'static str, usize>,
}

/// Compute summary statistics from scan results.
pub fn compute_summary(results: &[ResponseResult]) -> SummaryStats {
    let mut s = SummaryStats {
        total: results.len(),
        successful: 0,
        errors: 0,
        checks_passed: 0,
        checks_failed: 0,
        checks_warn: 0,
        trackers_total: 0,
        trackers_by_category: BTreeMap::new(),
    };
    for r in results {
        if r.status_code == 0 {
            s.errors += 1;
        } else if (200..=399).contains(&r.status_code) {
            s.successful += 1;
        }
        for c in &r.checks {
            if c.passed {
                s.checks_passed += 1;
            } else {
                match c.severity {
                    Severity::Critical | Severity::Warn => s.checks_failed += 1,
                    Severity::Info => s.checks_warn += 1,
                }
            }
        }
        s.trackers_total += r.trackers.len();
        for t in &r.trackers {
            *s.trackers_by_category
                .entry(t.category.as_str())
                .or_insert(0) += 1;
        }
    }
    s
}

/// Format a human-readable summary.
pub fn format_summary(results: &[ResponseResult]) -> String {
    let s = compute_summary(results);
    let mut out = format!(
        "# Scan Summary\n\n\
         - **Total endpoints:** {}\n\
         - **Successful (2xx/3xx):** {}\n\
         - **Errors:** {}\n\
         - **Checks passed:** {}\n\
         - **Checks failed:** {}\n\
         - **Warnings:** {}\n\
         - **Trackers found:** {}\n",
        s.total,
        s.successful,
        s.errors,
        s.checks_passed,
        s.checks_failed,
        s.checks_warn,
        s.trackers_total,
    );
    if !s.trackers_by_category.is_empty() {
        out.push_str("  **Tracker categories:**\n");
        for (cat, count) in &s.trackers_by_category {
            out.push_str(&format!("    - {cat}: {count}\n"));
        }
    }
    out
}
