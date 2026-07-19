//! Terminal table output (plain text and markdown).

use crate::types::ResponseResult;
use std::collections::BTreeMap;

fn status_color(code: u16) -> &'static str {
    match code / 100 {
        2 => "\x1b[32m",
        3 => "\x1b[34m",
        4 => "\x1b[33m",
        5 => "\x1b[31m",
        _ => "\x1b[0m",
    }
}
const RESET: &str = "\x1b[0m";

fn check_mark(passed: bool) -> &'static str {
    if passed {
        "\x1b[32m✓\x1b[0m"
    } else {
        "\x1b[31m✗\x1b[0m"
    }
}

/// Format results as a colorized terminal table.
pub fn format_table(results: &[ResponseResult]) -> String {
    let mut out = String::new();
    for r in results {
        let code_str = if r.status_code > 0 {
            format!(
                "{}{}{}{}",
                status_color(r.status_code),
                r.status_code,
                RESET,
                status_suffix(r.status_code)
            )
        } else {
            "\x1b[31mERR\x1b[0m".to_string()
        };
        let time = format_time(r.response_time_ms);
        let checks_str = format_checks(&r.checks);
        let tags_str = format_tags(&r.tags);
        let trackers_str = format_trackers_count(&r.trackers);
        out.push_str(&format!(
            "{} {}  {}  {}  {}  {}  {}",
            r.endpoint_method, r.endpoint_url, code_str, time, tags_str, trackers_str, checks_str,
        ));
        out.push('\n');
    }

    // Append tracker summary section.
    let summary = tracker_category_summary(results);
    if !summary.is_empty() {
        out.push_str("\n\x1b[1mTracker Summary\x1b[0m\n");
        for (cat, count) in &summary {
            out.push_str(&format!("  {cat}: {count}\n"));
        }
    }

    out
}

/// Format results as a markdown table.
pub fn format_markdown_table(results: &[ResponseResult]) -> String {
    let mut out = String::from("| Method | URL | Status | Time | Tags | Trackers | Checks |\n");
    out.push_str("|--------|-----|--------|------|------|----------|--------|\n");
    for r in results {
        let status = if r.status_code > 0 {
            r.status_code.to_string()
        } else {
            "ERR".into()
        };
        let time = format_time(r.response_time_ms);
        let tags_str = format_tags_md(&r.tags);
        let trackers_str = format_trackers_count(&r.trackers);
        let checks_str = format_checks_md(&r.checks);
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            r.endpoint_method, r.endpoint_url, status, time, tags_str, trackers_str, checks_str
        ));
    }

    // Append tracker breakdown.
    let summary = tracker_category_summary(results);
    if !summary.is_empty() {
        out.push_str("\n### Tracker Categories\n\n| Category | Count |\n|----------|-------|\n");
        for (cat, count) in &summary {
            out.push_str(&format!("| {cat} | {count} |\n"));
        }
        out.push('\n');
    }

    out
}

fn status_suffix(code: u16) -> &'static str {
    match code {
        200 => " OK",
        201 => " Created",
        204 => " No Content",
        301 => " Moved",
        302 => " Found",
        304 => " Not Modified",
        400 => " Bad Request",
        401 => " Unauthorized",
        403 => " Forbidden",
        404 => " Not Found",
        405 => " Method Not Allowed",
        429 => " Too Many Requests",
        500 => " Internal Error",
        502 => " Bad Gateway",
        503 => " Unavailable",
        504 => " Gateway Timeout",
        _ => "",
    }
}

fn format_time(ms: u64) -> String {
    if ms < 1000 {
        format!("{ms}ms")
    } else {
        format!("{}.{:0>3}s", ms / 1000, ms % 1000)
    }
}

fn format_checks(checks: &[crate::types::Check]) -> String {
    if checks.is_empty() {
        return String::new();
    }
    let parts: Vec<String> = checks
        .iter()
        .map(|c| format!("[{}] {}", check_mark(c.passed), c.name))
        .collect();
    parts.join(" ")
}

fn format_tags(tags: &[String]) -> String {
    if tags.is_empty() {
        return String::new();
    }
    tags.join(",")
}

fn format_tags_md(tags: &[String]) -> String {
    if tags.is_empty() {
        return "-".into();
    }
    tags.iter()
        .map(|t| format!("`{t}`"))
        .collect::<Vec<_>>()
        .join("<br>")
}

fn format_checks_md(checks: &[crate::types::Check]) -> String {
    if checks.is_empty() {
        return "-".into();
    }
    let parts: Vec<String> = checks
        .iter()
        .map(|c| {
            let mark = if c.passed { "✓" } else { "✗" };
            format!("{mark} {}", c.name)
        })
        .collect();
    parts.join("<br>")
}

fn format_trackers_count(trackers: &[crate::analytics::TrackerSignature]) -> String {
    if trackers.is_empty() {
        return String::new();
    }
    trackers.len().to_string()
}

fn tracker_category_summary(results: &[ResponseResult]) -> BTreeMap<&'static str, usize> {
    let mut map: BTreeMap<&'static str, usize> = BTreeMap::new();
    for r in results {
        for t in &r.trackers {
            *map.entry(t.category.as_str()).or_insert(0) += 1;
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Check, Severity};

    fn sample_results() -> Vec<ResponseResult> {
        vec![
            ResponseResult {
                endpoint_method: "GET".into(),
                endpoint_url: "https://api.example.com/users".into(),
                status_code: 200,
                response_time_ms: 150,
                response_size: 1024,
                response_headers: vec![("content-type".into(), "application/json".into())],
                response_body: vec![],
                expected_status: Some(200),
                timestamp: None,
                checks: vec![Check {
                    name: "CSP".into(),
                    passed: true,
                    severity: Severity::Info,
                    message: "CSP present".into(),
                }],
                error: None,
                tags: vec!["api".into(), "v1".into()],
                trackers: vec![],
            },
            ResponseResult {
                endpoint_method: "POST".into(),
                endpoint_url: "https://api.example.com/login".into(),
                status_code: 401,
                response_time_ms: 300,
                response_size: 512,
                response_headers: vec![],
                response_body: vec![],
                expected_status: None,
                timestamp: None,
                checks: vec![Check {
                    name: "HSTS".into(),
                    passed: false,
                    severity: Severity::Warn,
                    message: "HSTS missing".into(),
                }],
                error: None,
                tags: vec![],
                trackers: vec![],
            },
        ]
    }

    #[test]
    fn test_format_table_contains_methods() {
        let out = format_table(&sample_results());
        assert!(out.contains("GET"));
        assert!(out.contains("POST"));
        assert!(out.contains("200"));
        assert!(out.contains("401"));
    }

    #[test]
    fn test_format_markdown_contains_headers() {
        let out = format_markdown_table(&sample_results());
        assert!(out.contains("Method"));
        assert!(out.contains("URL"));
        assert!(out.contains("Status"));
        assert!(out.contains("api.example.com"));
    }

    #[test]
    fn test_format_table_empty() {
        assert_eq!(format_table(&[]), "");
    }

    #[test]
    fn test_format_markdown_empty() {
        let out = format_markdown_table(&[]);
        assert!(out.contains("Method"));
        assert!(out.contains("| Method | URL |"));
    }

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(0), "0ms");
        assert_eq!(format_time(500), "500ms");
        assert_eq!(format_time(1500), "1.500s");
        assert_eq!(format_time(10000), "10.000s");
    }

    #[test]
    fn test_status_suffix() {
        assert_eq!(status_suffix(200), " OK");
        assert_eq!(status_suffix(404), " Not Found");
        assert_eq!(status_suffix(500), " Internal Error");
        assert_eq!(status_suffix(999), "");
    }

    #[test]
    fn test_format_checks_empty() {
        assert_eq!(format_checks(&[]), "");
    }

    #[test]
    fn test_format_checks() {
        let checks = vec![
            Check { name: "CSP".into(), passed: true, severity: Severity::Info, message: "ok".into() },
            Check { name: "HSTS".into(), passed: false, severity: Severity::Warn, message: "missing".into() },
        ];
        let out = format_checks(&checks);
        assert!(out.contains("CSP"));
        assert!(out.contains("HSTS"));
    }

    #[test]
    fn test_format_tags() {
        assert_eq!(format_tags(&vec!["a".into(), "b".into()]), "a,b");
        assert_eq!(format_tags(&[]), "");
    }

    #[test]
    fn test_tracker_category_summary_empty() {
        let results = sample_results();
        let summary = tracker_category_summary(&results);
        assert!(summary.is_empty());
    }
}
