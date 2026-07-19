//! Security header analysis (HSTS, CSP, X-Frame-Options, etc).

use crate::types::{Check, Severity};

/// Run all security header checks on a response's headers.
pub fn check_security_headers(headers: &[(String, String)]) -> Vec<Check> {
    vec![
        check_csp(headers),
        check_hsts(headers),
        check_x_content_type_options(headers),
        check_x_frame_options(headers),
        check_cache_control(headers),
    ]
}

fn has_header(headers: &[(String, String)], name: &str) -> bool {
    headers.iter().any(|(k, _)| k.eq_ignore_ascii_case(name))
}

fn check_csp(headers: &[(String, String)]) -> Check {
    let present = has_header(headers, "content-security-policy");
    Check {
        name: "CSP".into(),
        passed: present,
        severity: if present {
            Severity::Info
        } else {
            Severity::Warn
        },
        message: if present {
            "Content-Security-Policy header present".into()
        } else {
            "Content-Security-Policy header missing — prevents XSS via allowed sources".into()
        },
    }
}

fn check_hsts(headers: &[(String, String)]) -> Check {
    let present = has_header(headers, "strict-transport-security");
    Check {
        name: "HSTS".into(),
        passed: present,
        severity: if present {
            Severity::Info
        } else {
            Severity::Warn
        },
        message: if present {
            "Strict-Transport-Security header present".into()
        } else {
            "Strict-Transport-Security header missing — no HTTPS enforcement".into()
        },
    }
}

fn check_x_content_type_options(headers: &[(String, String)]) -> Check {
    let present = has_header(headers, "x-content-type-options");
    Check {
        name: "X-Content-Type-Options".into(),
        passed: present,
        severity: if present {
            Severity::Info
        } else {
            Severity::Warn
        },
        message: if present {
            "X-Content-Type-Options header present".into()
        } else {
            "X-Content-Type-Options header missing — MIME-sniffing allowed".into()
        },
    }
}

fn check_x_frame_options(headers: &[(String, String)]) -> Check {
    let present = has_header(headers, "x-frame-options");
    Check {
        name: "X-Frame-Options".into(),
        passed: present,
        severity: Severity::Info,
        message: if present {
            "X-Frame-Options header present".into()
        } else {
            "X-Frame-Options header missing — page can be framed (clickjacking risk)".into()
        },
    }
}

fn check_cache_control(headers: &[(String, String)]) -> Check {
    let value = headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("cache-control"))
        .map(|(_, v)| v.as_str());
    let no_store = value.is_some_and(|v| v.contains("no-store"));
    Check {
        name: "Cache-Control".into(),
        passed: no_store,
        severity: Severity::Info,
        message: if no_store {
            "Cache-Control: no-store present".into()
        } else {
            match value {
                Some(v) => {
                    format!("Cache-Control: {v} — consider adding no-store for sensitive data")
                }
                None => "Cache-Control header missing — responses may be cached".into(),
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_headers_present() {
        let headers = vec![
            (
                "content-security-policy".into(),
                "default-src 'self'".into(),
            ),
            (
                "strict-transport-security".into(),
                "max-age=31536000".into(),
            ),
            ("x-content-type-options".into(), "nosniff".into()),
            ("x-frame-options".into(), "DENY".into()),
            ("cache-control".into(), "no-store".into()),
        ];
        let checks = check_security_headers(&headers);
        assert!(checks.iter().all(|c| c.passed));
    }

    #[test]
    fn test_all_headers_missing() {
        let headers: Vec<(String, String)> = vec![];
        let checks = check_security_headers(&headers);
        assert!(checks.iter().all(|c| !c.passed));
    }

    #[test]
    fn test_case_insensitive_matching() {
        let headers = vec![(
            "CONTENT-SECURITY-POLICY".into(),
            "default-src 'self'".into(),
        )];
        let checks = check_security_headers(&headers);
        assert!(checks[0].passed);
    }
}
