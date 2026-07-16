use crate::types::{ResponseResult, Severity};

/// Aggregated scan statistics.
pub struct SummaryStats {
    pub total: usize,
    pub successful: usize,
    pub errors: usize,
    pub checks_passed: usize,
    pub checks_failed: usize,
    pub checks_warn: usize,
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
    }
    s
}

/// Format a human-readable summary.
pub fn format_summary(results: &[ResponseResult]) -> String {
    let s = compute_summary(results);
    format!(
        "# Scan Summary\n\n\
         - **Total endpoints:** {}\n\
         - **Successful (2xx/3xx):** {}\n\
         - **Errors:** {}\n\
         - **Checks passed:** {}\n\
         - **Checks failed:** {}\n\
         - **Warnings:** {}\n",
        s.total, s.successful, s.errors, s.checks_passed, s.checks_failed, s.checks_warn
    )
}
