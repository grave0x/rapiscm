//! Device profile reconstruction from HTTP response signals.
#![allow(dead_code)]
//!
//! Analyzes response headers to reconstruct the device profile that trackers
//! observe, including browser, OS, screen, language, and fingerprint signals.

/// Device profile reconstructed from available fingerprint signals.
#[derive(Debug, Clone)]
pub struct DeviceProfile {
    /// User-Agent string.
    pub user_agent: Option<String>,
    /// Detected browser family.
    pub browser: Option<String>,
    /// Detected browser version.
    pub browser_version: Option<String>,
    /// Detected operating system.
    pub os: Option<String>,
    /// Screen resolution (from viewport dimensions, if available).
    pub screen: Option<String>,
    /// Color depth (typically 24-bit).
    pub color_depth: Option<String>,
    /// Timezone from Accept-Language or explicit headers.
    pub timezone: Option<String>,
    /// Primary language from Accept-Language header.
    pub language: Option<String>,
    /// How many fingerprint signals were collected.
    pub signal_count: usize,
    /// Stability assessment of the fingerprint.
    pub stability: Stability,
    /// Whether the profile looks like a bot.
    pub likely_bot: bool,
}

/// How stable/distinctive a device fingerprint is.
#[derive(Debug, Clone, PartialEq)]
pub enum Stability {
    High,
    Medium,
    Low,
}

impl Stability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Stability::High => "high",
            Stability::Medium => "medium",
            Stability::Low => "low",
        }
    }
}

/// Reconstruct device profile from HTTP response headers.
///
/// The more signals available, the higher the stability rating.
/// A proxy/VPN or headless browser may indicate a likely bot.
pub fn reconstruct_profile(headers: &[(String, String)]) -> DeviceProfile {
    let mut profile = DeviceProfile {
        user_agent: None,
        browser: None,
        browser_version: None,
        os: None,
        screen: None,
        color_depth: None,
        timezone: None,
        language: None,
        signal_count: 0,
        stability: Stability::Low,
        likely_bot: false,
    };

    // Headers are request headers from the client, not response headers.
    // In scan mode, we have the request headers we sent and response headers we received.
    // For device profile reconstruction, we look at what the SERVER sees — which is
    // the request headers that we sent. Since rapiscm sends minimal headers,
    // we reconstruct from response headers that indicate what was sent.

    // Check response headers for evidence of what was sent
    for (name, value) in headers {
        let lower = name.to_lowercase();

        if lower == "user-agent" || lower == "x-device-user-agent" {
            profile.user_agent = Some(value.clone());
            profile.signal_count += 1;
            let (browser, os) = parse_ua(value);
            profile.browser = browser;
            profile.os = os;

            // Detect headless browsers by UA patterns
            if value.contains("HeadlessChrome")
                || value.contains("PhantomJS")
                || value.contains("Headless")
            {
                profile.likely_bot = true;
            }
        }

        if lower == "accept-language" || lower == "content-language" {
            profile.language = Some(value.split(',').next().unwrap_or(value).trim().to_string());
            profile.signal_count += 1;
        }

        if lower == "x-device-timezone" || lower == "timezone" {
            profile.timezone = Some(value.clone());
            profile.signal_count += 1;
        }

        if lower == "x-device-screen" || lower == "viewport" {
            profile.screen = Some(value.clone());
            profile.signal_count += 1;
        }

        if lower == "x-device-dpr" || lower == "dpr" {
            profile.color_depth = Some(value.clone());
            profile.signal_count += 1;
        }
    }

    // If we have explicit X-Device-* headers, the fingerprint is more complete
    // If we only have User-Agent + Accept-Language, it's less definitive
    profile.stability = match profile.signal_count {
        0 => Stability::Low,
        1..=2 => Stability::Low,
        3..=4 => Stability::Medium,
        _ => Stability::High,
    };

    profile
}

/// Parse a User-Agent string to extract browser and OS.
fn parse_ua(ua: &str) -> (Option<String>, Option<String>) {
    let browser = if ua.contains("Chrome/") && !ua.contains("Edg/") && !ua.contains("OPR/") {
        Some("Chrome".into())
    } else if ua.contains("Firefox/") && !ua.contains("Seamonkey/") {
        Some("Firefox".into())
    } else if ua.contains("Safari/") && !ua.contains("Chrome/") {
        Some("Safari".into())
    } else if ua.contains("Edg/") {
        Some("Edge".into())
    } else if ua.contains("OPR/") || ua.contains("Opera/") {
        Some("Opera".into())
    } else {
        None
    };

    let os = if ua.contains("Windows NT") {
        Some("Windows".into())
    } else if ua.contains("Mac OS X") || ua.contains("macOS") {
        Some("macOS".into())
    } else if ua.contains("Linux") && !ua.contains("Android") {
        Some("Linux".into())
    } else if ua.contains("Android") {
        Some("Android".into())
    } else if ua.contains("iPhone") || ua.contains("iPad") || ua.contains("iOS") {
        Some("iOS".into())
    } else if ua.contains("CrOS") || ua.contains("Chromebook") {
        Some("ChromeOS".into())
    } else {
        None
    };

    (browser, os)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chrome_ua() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/125.0.0.0 Safari/537.36";
        let (browser, os) = parse_ua(ua);
        assert_eq!(browser.as_deref(), Some("Chrome"));
        assert_eq!(os.as_deref(), Some("Windows"));
    }

    #[test]
    fn test_parse_firefox_ua() {
        let ua =
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:127.0) Gecko/20100101 Firefox/127.0";
        let (browser, os) = parse_ua(ua);
        assert_eq!(browser.as_deref(), Some("Firefox"));
        assert_eq!(os.as_deref(), Some("macOS"));
    }

    #[test]
    fn test_reconstruct_profile() {
        let headers = vec![
            (
                "User-Agent".into(),
                "Mozilla/5.0 (X11; Linux x86_64) Chrome/125.0.0.0 Safari/537.36".into(),
            ),
            ("Accept-Language".into(), "en-US,en;q=0.9".into()),
        ];
        let profile = reconstruct_profile(&headers);
        assert_eq!(profile.browser.as_deref(), Some("Chrome"));
        assert_eq!(profile.os.as_deref(), Some("Linux"));
        assert_eq!(profile.language.as_deref(), Some("en-US"));
        assert!(!profile.likely_bot);
    }

    #[test]
    fn test_detect_headless() {
        let headers = vec![
            ("User-Agent".into(), "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/125.0.0.0 Safari/537.36".into()),
        ];
        let profile = reconstruct_profile(&headers);
        assert!(profile.likely_bot);
    }

    #[test]
    fn test_no_signals() {
        let profile = reconstruct_profile(&[]);
        assert_eq!(profile.signal_count, 0);
        assert_eq!(profile.stability, Stability::Low);
    }

    #[test]
    fn test_stability_scaling() {
        let headers = vec![
            ("User-Agent".into(), "Mozilla/5.0 Chrome/125".into()),
            ("Accept-Language".into(), "en-US".into()),
            ("X-Device-Screen".into(), "1512x982".into()),
            ("X-Device-Timezone".into(), "America/New_York".into()),
        ];
        let profile = reconstruct_profile(&headers);
        assert_eq!(profile.signal_count, 4);
        assert_eq!(profile.stability, Stability::Medium);
    }
}
