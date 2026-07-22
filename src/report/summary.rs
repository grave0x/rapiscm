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
            *s.trackers_by_category.entry(t.category.as_str()).or_insert(0) += 1;
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
        s.total, s.successful, s.errors, s.checks_passed, s.checks_failed, s.checks_warn, s.trackers_total,
    );
    if !s.trackers_by_category.is_empty() {
        out.push_str("  **Tracker categories:**\n");
        for (cat, count) in &s.trackers_by_category {
            out.push_str(&format!("    - {cat}: {count}\n"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Check, Severity};

    fn result() -> ResponseResult {
        ResponseResult {
            endpoint_method: "GET".into(),
            endpoint_url: "https://api.example.com/test".into(),
            status_code: 200,
            response_time_ms: 100,
            response_size: 64,
            response_headers: vec![],
            response_body: vec![],
            expected_status: None,
            timestamp: None,
            checks: vec![],
            error: None,
            tags: vec![],
            trackers: vec![],
        }
    }

    #[test]
    fn test_compute_summary_empty() {
        let s = compute_summary(&[]);
        assert_eq!(s.total, 0);
        assert_eq!(s.successful, 0);
        assert_eq!(s.errors, 0);
    }

    #[test]
    fn test_compute_summary_counts() {
        let mut r = result();
        r.status_code = 200;
        let mut r2 = result();
        r2.status_code = 0; // error
        let mut r3 = result();
        r3.status_code = 500;
        r3.checks = vec![
            Check {
                name: "CSP".into(),
                passed: true,
                severity: Severity::Info,
                message: "".into(),
            },
            Check {
                name: "HSTS".into(),
                passed: false,
                severity: Severity::Warn,
                message: "".into(),
            },
        ];
        let results = vec![r, r2, r3];
        let s = compute_summary(&results);
        assert_eq!(s.total, 3);
        assert_eq!(s.successful, 1);
        assert_eq!(s.errors, 1);
        assert_eq!(s.checks_passed, 1);
        assert_eq!(s.checks_failed, 1);
    }

    #[test]
    fn test_format_summary_contains_headings() {
        let out = format_summary(&[result()]);
        assert!(out.contains("Scan Summary"));
        assert!(out.contains("Total endpoints"));
    }

    #[test]
    fn test_format_summary_numbers() {
        let results = vec![result(), result()];
        let out = format_summary(&results);
        assert!(out.contains("2"));
    }

    #[test]
    fn test_compute_summary_trackers() {
        let results = vec![result()];
        let s = compute_summary(&results);
        assert_eq!(s.trackers_total, 0);
    }
}
