//! Ghost mode — stealth scanning with detection evasion.
//!
//! When `--ghost` is enabled, rapiscm:
//! - Rotates User-Agent across real browser strings
//! - Adds random jitter to request timing (±%)
//! - Randomizes Accept, Accept-Language, and other headers
//! - Rotates through proxies (if --proxy-rotate provided)
//! - Maintains a cookie jar across requests

use rand::Rng;

/// A pool of real browser User-Agent strings.
const UA_POOL: &[&str] = &[
    // Chrome 125 Windows
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
    // Chrome 125 macOS
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36",
    // Firefox 127 Windows
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:127.0) Gecko/20100101 Firefox/127.0",
    // Firefox 127 macOS
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:127.0) Gecko/20100101 Firefox/127.0",
    // Safari 17.5 macOS
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Safari/605.1.15",
    // Edge 125 Windows
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36 Edg/125.0.0.0",
    // Chrome Mobile Android
    "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.6422.113 Mobile Safari/537.36",
    // Safari Mobile iOS
    "Mozilla/5.0 (iPhone14,6; CPU iPhone OS 17_5 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.5 Mobile/15E148 Safari/604.1",
];

/// A pool of Accept values.
const ACCEPT_POOL: &[&str] = &[
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
    "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
    "text/html,application/xhtml+xml;q=0.9,*/*;q=0.8",
];

/// A pool of Accept-Language values.
const LANG_POOL: &[&str] = &[
    "en-US,en;q=0.9",
    "en-GB,en;q=0.9",
    "en-US,en;q=0.8",
    "en;q=0.9",
    "en-US,en;q=0.9,es;q=0.8",
    "en-CA,en;q=0.9,fr;q=0.8",
];

/// A pool of Accept-Encoding values.
const ENC_POOL: &[&str] = &["gzip, deflate, br", "gzip, deflate", "br, gzip, deflate"];

/// Configuration for ghost mode stealth techniques.
#[derive(Debug, Clone)]
pub struct GhostConfig {
    /// Whether ghost mode is enabled.
    pub enabled: bool,
    /// Jitter as percentage (0-100). 30 means ±30% random delay.
    pub jitter_pct: u32,
    /// User-agent rotation strategy: None = default, Some("mobile"/"desktop"/"random") or comma list.
    pub ua_mode: Option<String>,
    /// List of proxy URLs to rotate through.
    pub proxies: Vec<String>,
}

impl GhostConfig {
    pub fn new(enabled: bool, jitter_pct: u32, ua_mode: Option<String>, proxies: Vec<String>) -> Self {
        Self {
            enabled,
            jitter_pct,
            ua_mode,
            proxies,
        }
    }

    pub fn is_active(&self) -> bool {
        self.enabled || self.jitter_pct > 0 || self.ua_mode.is_some() || !self.proxies.is_empty()
    }
}

/// State for a single ghost-mode scan, maintaining rotation counters.
#[derive(Debug)]
pub struct GhostState {
    pub config: GhostConfig,
    ua_index: usize,
    proxy_index: usize,
    rng: rand::rngs::ThreadRng,
}

impl GhostState {
    pub fn new(config: GhostConfig) -> Self {
        Self {
            config,
            ua_index: 0,
            proxy_index: 0,
            rng: rand::thread_rng(),
        }
    }

    /// Get the next User-Agent string (round-robin or random).
    pub fn next_ua(&mut self) -> Option<&'static str> {
        if !self.config.enabled {
            return None;
        }
        match &self.config.ua_mode {
            Some(mode) => {
                let pool = match mode.as_str() {
                    "mobile" => &[UA_POOL[6], UA_POOL[7]],
                    _ => UA_POOL, // "desktop", "random", or comma list fall through to full pool
                };
                if self.config.ua_mode.as_deref() == Some("random") || mode == "random" {
                    Some(pool[self.rng.gen_range(0..pool.len())])
                } else {
                    let ua = pool[self.ua_index % pool.len()];
                    self.ua_index += 1;
                    Some(ua)
                }
            }
            None => {
                let ua = UA_POOL[self.ua_index % UA_POOL.len()];
                self.ua_index += 1;
                Some(ua)
            }
        }
    }

    /// Get randomized Accept header.
    pub fn next_accept(&mut self) -> Option<&'static str> {
        if !self.config.enabled {
            return None;
        }
        let val = ACCEPT_POOL[self.rng.gen_range(0..ACCEPT_POOL.len())];
        Some(val)
    }

    /// Get randomized Accept-Language header.
    pub fn next_lang(&mut self) -> Option<&'static str> {
        if !self.config.enabled {
            return None;
        }
        let val = LANG_POOL[self.rng.gen_range(0..LANG_POOL.len())];
        Some(val)
    }

    /// Get randomized Accept-Encoding header.
    pub fn next_encoding(&mut self) -> Option<&'static str> {
        if !self.config.enabled {
            return None;
        }
        let val = ENC_POOL[self.rng.gen_range(0..ENC_POOL.len())];
        Some(val)
    }

    /// Get next proxy URL (round-robin).
    pub fn next_proxy(&mut self) -> Option<String> {
        if self.config.proxies.is_empty() {
            return None;
        }
        let p = self.config.proxies[self.proxy_index % self.config.proxies.len()].clone();
        self.proxy_index += 1;
        Some(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ua_rotation() {
        let config = GhostConfig::new(true, 0, Some("desktop".into()), vec![]);
        let mut state = GhostState::new(config);
        let ua1 = state.next_ua();
        let ua2 = state.next_ua();
        assert!(ua1.is_some());
        assert!(ua2.is_some());
    }

    #[test]
    fn test_accept_randomization() {
        let config = GhostConfig::new(true, 0, None, vec![]);
        let mut state = GhostState::new(config);
        let accept = state.next_accept();
        assert!(accept.is_some());
        assert!(accept.unwrap().contains("text/html"));
    }

    #[test]
    fn test_proxy_rotation() {
        let config = GhostConfig::new(
            true,
            0,
            None,
            vec!["http://proxy1:8080".into(), "http://proxy2:8080".into()],
        );
        let mut state = GhostState::new(config);
        let p1 = state.next_proxy();
        let p2 = state.next_proxy();
        let p3 = state.next_proxy();
        assert_eq!(p1.unwrap(), "http://proxy1:8080");
        assert_eq!(p2.unwrap(), "http://proxy2:8080");
        assert_eq!(p3.unwrap(), "http://proxy1:8080"); // wraps around
    }

    #[test]
    fn test_ghost_disabled_no_ua() {
        let config = GhostConfig::new(false, 0, None, vec![]);
        let mut state = GhostState::new(config);
        assert!(state.next_ua().is_none());
        assert!(state.next_accept().is_none());
    }
}
