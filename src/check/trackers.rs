use crate::analytics::TrackerSignature;
use crate::types::{Check, Severity};

/// Run tracker detection on response body + headers.
pub fn check_trackers(body: &[u8], headers: &[(String, String)]) -> Vec<Check> {
    let body_str = match std::str::from_utf8(body) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let found = crate::analytics::detect_trackers(body_str, headers);
    found.into_iter().map(sig_to_check).collect()
}

/// Classify cookies from Set-Cookie response headers.
pub fn check_cookies(headers: &[(String, String)]) -> Vec<Check> {
    let mut checks: Vec<Check> = Vec::new();
    for (k, v) in headers {
        if !k.eq_ignore_ascii_case("set-cookie") {
            continue;
        }
        // Extract cookie name (everything before the first '=')
        let cookie_name = v.split('=').next().unwrap_or("");
        if cookie_name.is_empty() {
            continue;
        }
        let purpose = crate::analytics::classify_cookie(cookie_name);
        checks.push(Check {
            name: format!("cookie:{}", cookie_name),
            passed: false,
            severity: Severity::Info,
            message: format!(
                "Cookie '{}' classified as {}",
                cookie_name,
                purpose.as_str()
            ),
        });
    }
    checks
}

fn sig_to_check(sig: TrackerSignature) -> Check {
    Check {
        name: format!("tracker:{}", sig.name),
        passed: false,
        severity: Severity::Info,
        message: format!("{} ({}) — {}", sig.name, sig.company, sig.purpose),
    }
}
