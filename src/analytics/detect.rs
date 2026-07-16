/// Tracker signature database — ~200 entries, ~3KB, zero deps.

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
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

/// Classify a cookie by name.
#[derive(Debug, Clone, PartialEq)]
pub enum CookiePurpose {
    Necessary,
    Preferences,
    Statistics,
    Marketing,
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

pub fn classify_cookie(name: &str) -> CookiePurpose {
    match name {
        "_ga"
        | "_gid"
        | "_gat"
        | "_ga_*"
        | "__gid"
        | "__ga"
        | "_clck"
        | "_clsk"
        | "_clsk_*"
        | "_hjSession_*"
        | "_hjSessionUser_*"
        | "amplitude_id_*"
        | "mixpanel_distinct_id"
        | "mp_*"
        | "ajs_*"
        | "ajs_user_id"
        | "ajs_group_id"
        | "ajs_anonymous_id" => CookiePurpose::Statistics,
        "_fbp" | "_fbc" | "fr" | "tr" | "_gcl_au" | "_gcl_aw" | "_gcl_gb" | "_gcl_gs" | "IDE"
        | "test_cookie" | "NID" | "OTZ" | "1P_JAR" | "CONSENT" | "DSID" | "__gads" | "__gpi"
        | "__eoi" | "partner" | "personalization_id" | "guest_id" | "lang" | "auth_token"
        | "ct0" | "twid" => CookiePurpose::Marketing,
        "sessionid" | "session" | "sid" | "csrf" | "csrftoken" | "XSRF-TOKEN" | "__cf_bm"
        | "__cfruid" | "cf_clearance" | "AWSALB" | "AWSALBCORS" | "PHPSESSID" | "JSESSIONID"
        | "ASP.NET_SessionId" | "connect.sid" | "laravel_session" | "X-Mapping-*" => {
            CookiePurpose::Necessary
        }
        "language"
        | "currency"
        | "theme"
        | "locale"
        | "country"
        | "region"
        | "display"
        | "font_size"
        | "sidebar"
        | "layout"
        | "cookies_accepted"
        | "cookieconsent_status"
        | "cookie_notice_accepted"
        | "eu_cookie" => CookiePurpose::Preferences,
        _ => CookiePurpose::Unclassified,
    }
}

include!("sigdb.rs");
