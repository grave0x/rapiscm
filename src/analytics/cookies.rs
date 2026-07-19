//! Cookie purpose classification and security analysis.
//!
//! Classifies cookies by purpose (Necessary, Preferences, Statistics, Marketing)
//! and detects missing security flags (Secure, HttpOnly, SameSite, expiry).

use crate::types::{Check, Severity};

/// Cookie purpose classification.
#[derive(Debug, Clone, PartialEq)]
pub enum CookiePurpose {
    /// Required for basic functionality (session, CSRF, load balancers).
    Necessary,
    /// User preference storage (language, currency, theme).
    Preferences,
    /// Analytics, anonymous usage data.
    Statistics,
    /// Ad targeting, cross-site tracking.
    Marketing,
    /// Unknown or uncategorized.
    Unclassified,
}

impl CookiePurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            CookiePurpose::Necessary => "necessary",
            CookiePurpose::Preferences => "preferences",
            CookiePurpose::Statistics => "statistics",
            CookiePurpose::Marketing => "marketing",
            CookiePurpose::Unclassified => "unclassified",
        }
    }
}

/// Classify a cookie by its name into a purpose category.
pub fn classify_cookie(name: &str) -> CookiePurpose {
    match name {
        // Analytics / statistics
        "_ga" | "_gid" | "_gat" | "_gat_gtag" | "__utma" | "__utmb" | "__utmc" | "__utmt"
        | "__utmz" | "_clck" | "_clsk" | "_clsc" | "_cltc" | "_clzu" | "_hjSessionUser_"
        | "_hjSession_" | "_hjid" | "_hjs" | "hp" | "hblid" | "amplitude_id" | "AMP_"
        | "__tdli" | "ajs_anonymous_id" | "ajs_user_id" | "mixpanel" | "mp_" | "mtm_"
        | "_pk_id" | "_pk_ses" | "_pk_ref" => CookiePurpose::Statistics,

        // Advertising / marketing
        "_fbp" | "_fbc" | "_gcl_au" | "_gcl_gs" | "_gcl_dc" | "IDE" | "test_cookie" | "fr"
        | "tr" | "_pin_unauth" | "_tt_enable_cookie" | "_ttp" | "personalization_id"
        | "muc_ads" | "NID" | "ANID" | "DSID" | "FLC" | "AID" | "TAID" | "_uetsid" | "_uetvid"
        | "taboola_" | "criteo_" | "uid" => CookiePurpose::Marketing,

        // Necessary / functional
        "sessionid" | "session" | "sid" | "PHPSESSID" | "JSESSIONID" | "ASP.NET_SessionId"
        | "connect.sid" | "laravel_session" | "XSRF-TOKEN" | "csrf_token" | "csrf" | "__csrf"
        | "csrf-state" | "__cf_bm" | "__cfruid" | "cf_clearance" | "_cfduid" | "AWSALB"
        | "AWSALBCORS" | "SERVERID" | "lb" | "__ddg1_" | "__ddg2_" | "__ddg3_" | "__ddgid"
        | "ARRAffinity" | "ak_bmsc" | "bm_sv" | "akavpau_ppsd" | "TS01" | "TS" | "incap_ses_" => {
            CookiePurpose::Necessary
        }

        // Preferences
        "language" | "lang" | "locale" | "i18n" | "currency" | "theme" | "pref" | "country"
        | "region" | "timezone" | "tz" | "font_size" | "email" => CookiePurpose::Preferences,

        _ => CookiePurpose::Unclassified,
    }
}

/// Analyze a Set-Cookie value for security best practices.
pub fn analyze_cookie_security(value: &str) -> Vec<Check> {
    let mut checks = Vec::new();
    let lower = value.to_lowercase();

    // Extract cookie name
    let name = value.split('=').next().unwrap_or("cookie");
    let prefix = |msg: &str| format!("cookie:{name}: {msg}");

    // Check Secure flag
    if lower.contains("secure") {
        checks.push(Check {
            name: format!("cookie:{name}:secure"),
            passed: true,
            severity: Severity::Info,
            message: prefix("Secure flag present"),
        });
    } else {
        checks.push(Check {
            name: format!("cookie:{name}:secure"),
            passed: false,
            severity: Severity::Warn,
            message: prefix("Missing Secure flag — cookie sent over unencrypted HTTP"),
        });
    }

    // Check HttpOnly flag
    if lower.contains("httponly") {
        checks.push(Check {
            name: format!("cookie:{name}:httponly"),
            passed: true,
            severity: Severity::Info,
            message: prefix("HttpOnly flag present"),
        });
    } else {
        checks.push(Check {
            name: format!("cookie:{name}:httponly"),
            passed: false,
            severity: Severity::Warn,
            message: prefix("Missing HttpOnly flag — accessible via JavaScript"),
        });
    }

    // Check SameSite attribute
    if lower.contains("samesite") {
        if lower.contains("samesite=none") {
            checks.push(Check {
                name: format!("cookie:{name}:samesite"),
                passed: false,
                severity: Severity::Warn,
                message: prefix("SameSite=None — cross-site sending allowed, use with Secure"),
            });
        } else {
            checks.push(Check {
                name: format!("cookie:{name}:samesite"),
                passed: true,
                severity: Severity::Info,
                message: prefix("SameSite attribute present"),
            });
        }
    } else {
        checks.push(Check {
            name: format!("cookie:{name}:samesite"),
            passed: false,
            severity: Severity::Info,
            message: prefix("Missing SameSite attribute — default behavior varies by browser"),
        });
    }

    // Check Expiry / Max-Age for session cookies
    if lower.contains("expires=") || lower.contains("max-age=") {
        checks.push(Check {
            name: format!("cookie:{name}:persistent"),
            passed: false,
            severity: Severity::Info,
            message: prefix("Persistent cookie with explicit expiry — check retention period"),
        });
    }

    checks
}

/// Group tracked cookies by purpose for reporting.
#[cfg_attr(not(test), expect(dead_code))]
pub fn summarize_cookies(headers: &[(String, String)]) -> Vec<(String, CookiePurpose)> {
    let mut results = Vec::new();
    for (k, v) in headers {
        if !k.eq_ignore_ascii_case("set-cookie") {
            continue;
        }
        let name = v.split('=').next().unwrap_or("");
        if name.is_empty() {
            continue;
        }
        results.push((name.to_string(), classify_cookie(name)));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_ga() {
        assert_eq!(classify_cookie("_ga"), CookiePurpose::Statistics);
    }

    #[test]
    fn test_classify_fbp() {
        assert_eq!(classify_cookie("_fbp"), CookiePurpose::Marketing);
    }

    #[test]
    fn test_classify_session() {
        assert_eq!(classify_cookie("sessionid"), CookiePurpose::Necessary);
    }

    #[test]
    fn test_classify_language() {
        assert_eq!(classify_cookie("language"), CookiePurpose::Preferences);
    }

    #[test]
    fn test_classify_unknown() {
        assert_eq!(
            classify_cookie("some_random_cookie"),
            CookiePurpose::Unclassified
        );
    }

    #[test]
    fn test_analyze_secure_flag() {
        let result = analyze_cookie_security("sessionid=abc123; Secure; HttpOnly; SameSite=Lax");
        assert!(result.iter().any(|c| c.name.contains("secure") && c.passed));
    }

    #[test]
    fn test_analyze_missing_secure() {
        let result = analyze_cookie_security("sessionid=abc123; HttpOnly");
        assert!(
            result
                .iter()
                .any(|c| c.name.contains("secure") && !c.passed)
        );
    }

    #[test]
    fn test_analyze_missing_httponly() {
        let result = analyze_cookie_security("sessionid=abc123; Secure");
        assert!(
            result
                .iter()
                .any(|c| c.name.contains("httponly") && !c.passed)
        );
    }

    #[test]
    fn test_analyze_samesite_none() {
        let result = analyze_cookie_security("id=1; SameSite=None; Secure");
        assert!(
            result
                .iter()
                .any(|c| c.name.contains("samesite") && !c.passed)
        );
    }

    #[test]
    fn test_summarize_cookies() {
        let headers = vec![
            (
                "Set-Cookie".into(),
                "_ga=GA1.2.abc; Secure; HttpOnly".into(),
            ),
            ("Set-Cookie".into(), "sessionid=xyz; Secure".into()),
        ];
        let summary = summarize_cookies(&headers);
        assert_eq!(summary.len(), 2);
        assert_eq!(summary[0].1, CookiePurpose::Statistics);
        assert_eq!(summary[1].1, CookiePurpose::Necessary);
    }
}
