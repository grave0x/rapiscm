//! Data export detection — track outbound data sent to third-party domains.
#![allow(dead_code)]
//!
//! Analyzes response headers and body content to identify:
//! - Third-party domains receiving data (script src, img src, iframe src, navigation)
//! - Known tracker domains matched against the signature database
//! - Beacon/navigator.sendBeacon destinations
//! - WebSocket connections to external hosts

use crate::analytics::detect::TrackerCategory;

/// A detected data export destination.
#[derive(Debug, Clone)]
pub struct DataExport {
    /// The third-party domain data was sent to.
    pub domain: String,
    /// How the data export was detected.
    pub method: ExportMethod,
    /// What data category this relates to.
    pub category: TrackerCategory,
    /// The tracker or service name if matched.
    pub service_name: Option<&'static str>,
}

/// How an outbound data transfer was detected.
#[derive(Debug, Clone)]
pub enum ExportMethod {
    /// `<script src="https://third-party.com/...">`
    ScriptSource,
    /// `<img src="https://third-party.com/...">` (tracking pixel)
    ImageSource,
    /// `<iframe src="https://third-party.com/...">`
    IframeSource,
    /// `navigator.sendBeacon('https://third-party.com/...')`
    SendBeacon,
    /// `new WebSocket('wss://third-party.com/...')`
    WebSocket,
    /// `fetch('https://third-party.com/...')` or `XMLHttpRequest`
    Fetch,
    /// Navigation or redirect to third-party
    Navigation,
    /// Set-Cookie domain attribute pointing to third-party
    CookieDomain,
}

/// Detect data exports from response body + headers.
///
/// Uses pattern matching to find third-party resource URLs in HTML/JS bodies
/// and cross-references them against the tracker signature database.
pub fn detect_exports(body: &str, headers: &[(String, String)]) -> Vec<DataExport> {
    let mut exports = Vec::new();

    // 1. Detect script/image/iframe sources: src="https://..."
    for url_str in body.match_iter() {
        if let Some(domain) = extract_domain(&url_str) {
            exports.push(domain_to_export(domain, ExportMethod::ScriptSource));
        }
    }

    // 4. Detect navigator.sendBeacon calls
    for marker in &[
        "sendBeacon('https://",
        "sendBeacon(\"https://",
        "sendBeacon('http://",
        "sendBeacon(\"http://",
    ] {
        let mut pos = 0;
        while let Some(start) = body[pos..].find(marker) {
            let val_start = pos + start + marker.len();
            let quote = if marker.contains('\'') { '\'' } else { '"' };
            if let Some(end) = body[val_start..].find(quote) {
                let url = format!("https://{}", &body[val_start..val_start + end]);
                if let Some(domain) = extract_domain(&url) {
                    exports.push(domain_to_export(domain, ExportMethod::SendBeacon));
                }
            }
            pos = val_start + 1;
        }
    }

    // 5. Detect Set-Cookie domains pointing to third-party
    for (name, value) in headers {
        if !name.eq_ignore_ascii_case("set-cookie") {
            continue;
        }
        let lower = value.to_lowercase();
        if let Some(pos) = lower.find("domain=") {
            let rest = &lower[pos + 7..];
            let domain = rest.split([';', ' ', ',']).next().unwrap_or("");
            if !domain.is_empty()
                && !domain.contains(".example.com")
                && !domain.contains(".localhost")
            {
                exports.push(domain_to_export(
                    domain.to_string(),
                    ExportMethod::CookieDomain,
                ));
            }
        }
    }

    // Deduplicate by domain + method
    exports.dedup_by_key(|e| (e.domain.clone(), format!("{:?}", e.method)));

    // Match against known trackers for service name enrichment
    for export in &mut exports {
        export.service_name = match_tracker_service(&export.domain);
    }

    exports
}

fn extract_domain(url_str: &str) -> Option<String> {
    reqwest::Url::parse(url_str)
        .ok()
        .and_then(|u| u.host_str().map(String::from))
}

fn domain_to_export(domain: String, method: ExportMethod) -> DataExport {
    let category = match &method {
        ExportMethod::ImageSource => TrackerCategory::Advertising,
        ExportMethod::SendBeacon => TrackerCategory::Analytics,
        ExportMethod::Fetch => TrackerCategory::Analytics,
        _ => TrackerCategory::Utility,
    };
    DataExport {
        domain,
        method,
        category,
        service_name: None,
    }
}

/// Match a domain against the tracker signature database.
fn match_tracker_service(domain: &str) -> Option<&'static str> {
    let known: &[(&str, &str)] = &[
        ("google-analytics.com", "Google Analytics"),
        ("googletagmanager.com", "Google Tag Manager"),
        ("doubleclick.net", "DoubleClick"),
        ("facebook.com", "Facebook"),
        ("facebook.net", "Facebook"),
        ("connect.facebook.net", "Facebook"),
        ("twitter.com", "Twitter"),
        ("twttr.com", "Twitter"),
        ("linkedin.com", "LinkedIn"),
        ("hotjar.com", "Hotjar"),
        ("mouseflow.com", "Mouseflow"),
        ("fullstory.com", "FullStory"),
        ("smartlook.com", "Smartlook"),
        ("amplitude.com", "Amplitude"),
        ("mixpanel.com", "Mixpanel"),
        ("segment.io", "Segment"),
        ("segment.com", "Segment"),
        ("cdn.segment.com", "Segment"),
        ("heap.com", "Heap"),
        ("posthog.com", "PostHog"),
        ("matomo.org", "Matomo"),
        ("plausible.io", "Plausible"),
        ("fathom.com", "Fathom"),
        ("criteo.com", "Criteo"),
        ("criteo.net", "Criteo"),
        ("taboola.com", "Taboola"),
        ("outbrain.com", "Outbrain"),
        ("adsrvr.org", "The Trade Desk"),
        ("adnxs.com", "AppNexus"),
        ("rubiconproject.com", "Rubicon"),
        ("openx.net", "OpenX"),
        ("pubmatic.com", "PubMatic"),
        ("amazon-adsystem.com", "Amazon Ads"),
        ("aax.amazon.com", "Amazon Ads"),
        ("adsymptotic.com", "Quantcast"),
        ("quantserve.com", "Quantcast"),
        ("scorecardresearch.com", "ScorecardResearch"),
        ("nr-data.net", "New Relic"),
        ("sentry.io", "Sentry"),
        ("datadoghq.com", "Datadog"),
        ("logrocket.com", "LogRocket"),
        ("bugsnag.com", "Bugsnag"),
        ("rollbar.com", "Rollbar"),
        ("clarity.ms", "Microsoft Clarity"),
        ("bing.com", "Bing"),
        ("pinterest.com", "Pinterest"),
        ("tiktok.com", "TikTok"),
        ("snapchat.com", "Snapchat"),
        ("reddit.com", "Reddit"),
        ("t.co", "Twitter"),
        ("optimizely.com", "Optimizely"),
        ("vwo.com", "VWO"),
        ("crazyegg.com", "CrazyEgg"),
        ("clicky.com", "Clicky"),
        ("cdn.jsdelivr.net", "jsDelivr"),
        ("unpkg.com", "unpkg"),
        ("cdnjs.cloudflare.com", "CDNJS"),
        ("cloudflare.com", "Cloudflare"),
    ];
    for (d, name) in known {
        if domain.contains(d) {
            return Some(name);
        }
    }
    None
}

/// Count third-party connections by category.
pub fn count_by_category(exports: &[DataExport]) -> Vec<(&'static str, usize)> {
    use std::collections::BTreeMap;
    let mut map: BTreeMap<&'static str, usize> = BTreeMap::new();
    for e in exports {
        let cat = e.category.as_str();
        *map.entry(cat).or_insert(0) += 1;
    }
    map.into_iter().collect()
}

/// Simple URL extractor from HTML src attributes.
trait MatchIter {
    fn match_iter(&self) -> SrcMatches<'_>;
}

impl MatchIter for str {
    fn match_iter(&self) -> SrcMatches<'_> {
        SrcMatches { text: self, pos: 0 }
    }
}

struct SrcMatches<'a> {
    text: &'a str,
    pos: usize,
}

impl<'a> Iterator for SrcMatches<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // Handle src="URL" patterns
        for marker in &[
            "src=\"https://",
            "src=\"http://",
            "src='https://",
            "src='http://",
        ] {
            if let Some(start) = self.text[self.pos..].find(marker) {
                let val_start = self.pos + start + marker.len();
                // Find closing quote of the same type
                let quote_close = if marker.contains('\'') { '\'' } else { '"' };
                if let Some(end) = self.text[val_start..].find(quote_close) {
                    let url_part = &self.text[val_start..val_start + end];
                    self.pos = val_start + end + 1;
                    // Prepend scheme for URL parsing
                    return Some(format!("https://{url_part}"));
                }
            }
        }
        // Try sendBeacon('URL') pattern
        let beacon_marker = "sendBeacon('https://";
        if let Some(start) = self.text[self.pos..].find(beacon_marker) {
            let val_start = self.pos + start + beacon_marker.len();
            if let Some(end) = self.text[val_start..].find('\'') {
                let url = &self.text[val_start..val_start + end];
                self.pos = val_start + end + 1;
                return Some(url.to_string());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_script_src() {
        let body =
            r#"<html><script src="https://www.google-analytics.com/analytics.js"></script></html>"#;
        let exports = detect_exports(body, &[]);
        assert!(
            exports
                .iter()
                .any(|e| e.domain.contains("google-analytics.com"))
        );
    }

    #[test]
    fn test_detect_tracking_pixel() {
        let body = r#"<img src="https://www.facebook.com/tr?id=123&ev=PageView">"#;
        let exports = detect_exports(body, &[]);
        assert!(exports.iter().any(|e| e.domain.contains("facebook.com")));
    }

    #[test]
    fn test_detect_send_beacon() {
        let body = r#"navigator.sendBeacon('https://analytics.example.com/collect', data)"#;
        let exports = detect_exports(body, &[]);
        assert!(
            exports
                .iter()
                .any(|e| e.domain.contains("analytics.example.com"))
        );
    }

    #[test]
    fn test_detect_cookie_domain() {
        let headers = vec![(
            "Set-Cookie".into(),
            "_ga=GA1.2.abc; domain=.doubleclick.net".into(),
        )];
        let exports = detect_exports("", &headers);
        assert!(exports.iter().any(|e| e.domain.contains("doubleclick.net")));
    }

    #[test]
    fn test_service_name_enrichment() {
        let body = r#"<script src="https://www.googletagmanager.com/gtag/js"></script>"#;
        let exports = detect_exports(body, &[]);
        assert!(
            exports
                .iter()
                .any(|e| e.service_name == Some("Google Tag Manager"))
        );
    }

    #[test]
    fn test_count_by_category() {
        let body = r#"
            <script src="https://www.google-analytics.com/analytics.js"></script>
            <img src="https://www.facebook.com/tr?id=123">
        "#;
        let exports = detect_exports(body, &[]);
        let counts = count_by_category(&exports);
        assert!(!counts.is_empty());
    }
}
