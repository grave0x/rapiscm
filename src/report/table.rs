use crate::types::ResponseResult;

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
        out.push_str(&format!(
            "{} {}  {}  {}  {}  {}", // space-aligned approximately
            r.endpoint_method, r.endpoint_url, code_str, time, tags_str, checks_str,
        ));
        out.push('\n');
    }
    out
}

/// Format results as a markdown table.
pub fn format_markdown_table(results: &[ResponseResult]) -> String {
    let mut out = String::from("| Method | URL | Status | Time | Tags | Checks |\n");
    out.push_str("|--------|-----|--------|------|------|--------|\n");
    for r in results {
        let status = if r.status_code > 0 {
            r.status_code.to_string()
        } else {
            "ERR".into()
        };
        let time = format_time(r.response_time_ms);
        let tags_str = format_tags_md(&r.tags);
        let checks_str = format_checks_md(&r.checks);
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            r.endpoint_method, r.endpoint_url, status, time, tags_str, checks_str
        ));
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
