//! Timing and rate analytics for session replay.
//!
//! Computes inter-request gaps, burst detection, rate-limit events,
//! and per-endpoint timing percentiles from a list of timestamps and
//! results.

use std::collections::HashMap;

use crate::types::ResponseResult;

/// Distribution statistics for a set of timing measurements.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TimingDistribution {
    pub min_ms: u64,
    pub p50_ms: u64,
    pub p90_ms: u64,
    pub p99_ms: u64,
    pub max_ms: u64,
}

/// A detected burst of requests.
#[derive(Debug, Clone)]
pub struct Burst {
    /// ISO-8601 timestamp of the burst window start.
    pub at: String,
    /// Number of requests in that 1-second window.
    pub requests_in_second: u32,
    /// Endpoint URLs that were hit during this burst (deduplicated).
    pub urls: Vec<String>,
}

/// A detected rate-limit event.
#[derive(Debug, Clone)]
pub struct RateLimitEvent {
    /// ISO-8601 timestamp of the rate-limited response.
    pub at: String,
    /// The URL that returned 429/503.
    pub url: String,
    /// Estimated request rate that preceded the limit.
    pub preceding_rate: f64,
}

/// Full timing analytics result.
#[derive(Debug, Clone, Default)]
pub struct TimingAnalytics {
    pub total_requests: usize,
    pub duration_seconds: f64,
    pub requests_per_second: f64,
    pub inter_request_gaps: TimingDistribution,
    pub bursts: Vec<Burst>,
    pub rate_limits_hit: Vec<RateLimitEvent>,
    pub per_endpoint_timing: HashMap<String, TimingDistribution>,
}

/// Compute timing analytics from session data.
///
/// `timestamps` and `results` must have the same length. Timestamps
/// should be ISO-8601 strings; they are sorted before analysis.
pub fn compute_timing_analytics(
    timestamps: &[String],
    results: &[ResponseResult],
) -> TimingAnalytics {
    if timestamps.is_empty() || results.is_empty() {
        return TimingAnalytics::default();
    }

    // Filter to entries where both have timestamp and result.
    let mut pairs: Vec<(u64, &ResponseResult)> = timestamps
        .iter()
        .zip(results.iter())
        .filter_map(|(ts, res)| {
            if ts.is_empty() {
                return None;
            }
            let epoch = parse_iso8601(ts)?;
            Some((epoch, res))
        })
        .collect();

    if pairs.len() < 2 {
        return TimingAnalytics {
            total_requests: pairs.len(),
            ..Default::default()
        };
    }

    // Sort by timestamp.
    pairs.sort_by_key(|(epoch, _)| *epoch);

    let total = pairs.len();
    let first_ts = pairs.first().map(|(e, _)| *e).unwrap_or(0);
    let last_ts = pairs.last().map(|(e, _)| *e).unwrap_or(0);
    let duration_secs = (last_ts.saturating_sub(first_ts)) as f64 / 1000.0;

    // Inter-request gaps.
    let mut gaps_ms: Vec<u64> = pairs
        .windows(2)
        .map(|w| w[1].0.saturating_sub(w[0].0))
        .collect();
    gaps_ms.sort_unstable();

    let gap_dist = if gaps_ms.is_empty() {
        TimingDistribution::default()
    } else {
        TimingDistribution {
            min_ms: *gaps_ms.first().unwrap_or(&0),
            p50_ms: percentile(&gaps_ms, 50),
            p90_ms: percentile(&gaps_ms, 90),
            p99_ms: percentile(&gaps_ms, 99),
            max_ms: *gaps_ms.last().unwrap_or(&0),
        }
    };

    // Burst detection: sliding 1-second window.
    let mut bursts: Vec<Burst> = Vec::new();
    if pairs.len() > 1 {
        let b = detect_bursts(&pairs);
        bursts = b;
    }

    // Rate-limit events: find 429/503 responses.
    let rate_limits = detect_rate_limits(&pairs);

    // Per-endpoint timing.
    let mut endpoint_times: HashMap<String, Vec<u64>> = HashMap::new();
    for (_, res) in &pairs {
        let path = extract_path(&res.endpoint_url);
        endpoint_times
            .entry(path)
            .or_default()
            .push(res.response_time_ms);
    }
    let per_endpoint_timing: HashMap<String, TimingDistribution> = endpoint_times
        .into_iter()
        .map(|(path, mut times)| {
            times.sort_unstable();
            let dist = TimingDistribution {
                min_ms: *times.first().unwrap_or(&0),
                p50_ms: percentile(&times, 50),
                p90_ms: percentile(&times, 90),
                p99_ms: percentile(&times, 99),
                max_ms: *times.last().unwrap_or(&0),
            };
            (path, dist)
        })
        .collect();

    TimingAnalytics {
        total_requests: total,
        duration_seconds: duration_secs,
        requests_per_second: if duration_secs > 0.0 {
            total as f64 / duration_secs
        } else {
            total as f64
        },
        inter_request_gaps: gap_dist,
        bursts,
        rate_limits_hit: rate_limits,
        per_endpoint_timing,
    }
}

/// Format timing analytics as a human-readable string.
pub fn format_timing_analytics(ta: &TimingAnalytics) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "\n── Timing Analytics ──\n\
         Duration:  {:.1}s     Total requests: {}\n\
         Req/s:     {:.1}\n\n",
        ta.duration_seconds, ta.total_requests, ta.requests_per_second,
    ));

    out.push_str(&format!(
        "Inter-request gaps:  min {}ms  p50 {}ms  p90 {}ms  p99 {}ms  max {}ms\n",
        ta.inter_request_gaps.min_ms,
        ta.inter_request_gaps.p50_ms,
        ta.inter_request_gaps.p90_ms,
        ta.inter_request_gaps.p99_ms,
        ta.inter_request_gaps.max_ms,
    ));

    if !ta.bursts.is_empty() {
        out.push_str("\nBursts (≥10 req/s):\n");
        for b in &ta.bursts {
            let urls: Vec<&str> = b.urls.iter().map(|s| s.as_str()).collect();
            out.push_str(&format!(
                "  {}    {} req/s  {:?}\n",
                b.at, b.requests_in_second, urls
            ));
        }
    }

    if !ta.rate_limits_hit.is_empty() {
        out.push_str("\nRate limits hit:\n");
        for rl in &ta.rate_limits_hit {
            out.push_str(&format!(
                "  {}    {} on {}  (preceding rate {:.0} req/s)\n",
                rl.at, rl.url, rl.url, rl.preceding_rate
            ));
        }
    }

    if !ta.per_endpoint_timing.is_empty() {
        out.push_str("\nSlowest endpoints (p99):\n");
        let mut endpoints: Vec<(&String, &TimingDistribution)> =
            ta.per_endpoint_timing.iter().collect();
        endpoints.sort_by_key(|b| std::cmp::Reverse(b.1.p99_ms));
        for (path, dist) in endpoints.iter().take(10) {
            out.push_str(&format!(
                "  {}    {}ms (p50 {})ms\n",
                path, dist.p99_ms, dist.p50_ms
            ));
        }
    }

    out
}

// ── Helper functions ──

/// Compute the p-th percentile from a sorted slice.
fn percentile(sorted: &[u64], p: u64) -> u64 {
    if sorted.is_empty() {
        return 0;
    }
    let idx = ((p as f64 / 100.0) * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Parse an ISO-8601-ish timestamp to milliseconds since Unix epoch.
/// Supports formats like "2026-07-16T14:30:00Z" and "2026-07-16T14:30:00.123Z".
fn parse_iso8601(s: &str) -> Option<u64> {
    // Extract date parts.
    let s = s.trim();
    if s.len() < 19 {
        return None;
    }
    let chars: Vec<char> = s.chars().collect();

    // Year: read 4 digits individually (parse2 only handles 2 digits)
    let year = chars[0].to_digit(10)? * 1000
        + chars[1].to_digit(10)? * 100
        + chars[2].to_digit(10)? * 10
        + chars[3].to_digit(10)?;
    let month = parse2(&chars[5..=6])?;
    let day = parse2(&chars[8..=9])?;
    let hour = if chars.len() > 11 {
        parse2(&chars[11..=12])?
    } else {
        0
    };
    let min = if chars.len() > 14 {
        parse2(&chars[14..=15])?
    } else {
        0
    };
    let sec = if chars.len() > 17 {
        parse2(&chars[17..=18])?
    } else {
        0
    };

    // Parse fractional seconds.
    let mut millis = 0u64;
    if chars.len() > 19 && chars[19] == '.' {
        let end = chars.len().min(23);
        let frac: String = chars[20..end].iter().collect();
        let frac_val: u64 = frac.parse().unwrap_or(0);
        let frac_len = end - 20;
        millis = match frac_len {
            1 => frac_val * 100,
            2 => frac_val * 10,
            3 => frac_val,
            _ => 0,
        };
    }

    // Days since epoch (simplified, not handling leap seconds).
    let days = date_to_days(year, month, day)?;
    let total_secs = days * 86400 + hour as u64 * 3600 + min as u64 * 60 + sec as u64;
    Some(total_secs * 1000 + millis)
}

fn parse2(chars: &[char]) -> Option<u32> {
    if chars.len() < 2 {
        return None;
    }
    let tens = chars[0].to_digit(10)?;
    let ones = chars[1].to_digit(10)?;
    Some(tens * 10 + ones)
}

fn date_to_days(year: u32, month: u32, day: u32) -> Option<u64> {
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let (y, m) = if month <= 2 {
        (year as i64 - 1, month as i64 + 9)
    } else {
        (year as i64, month as i64 - 3)
    };
    let c = y / 100;
    let ya = y - 100 * c;
    let doy = (146097 * c) / 4 + (36525 * ya) / 100 + (153 * m + 2) / 5 + day as i64;
    let epoch_adjust: i64 = 719469; // days from year 0 to 1970-01-01
    let days = doy - epoch_adjust;
    if days >= 0 { Some(days as u64) } else { None }
}

/// Detect bursts using a sliding 1-second window.
fn detect_bursts<'a>(pairs: &'a [(u64, &'a ResponseResult)]) -> Vec<Burst> {
    let mut bursts: Vec<Burst> = Vec::new();
    let mut i = 0;
    let n = pairs.len();

    while i < n {
        let window_start = pairs[i].0;
        let window_end = window_start + 1000; // 1 second
        let mut count = 0u32;
        let mut urls: Vec<String> = Vec::new();
        let mut j = i;
        while j < n && pairs[j].0 <= window_end {
            let url = extract_path(&pairs[j].1.endpoint_url);
            if !urls.contains(&url) {
                urls.push(url);
            }
            count += 1;
            j += 1;
        }

        if count >= 10 {
            // Find the ISO timestamp for this burst.
            let at_ts = pairs[i]
                .1
                .timestamp
                .clone()
                .unwrap_or_else(|| format_epoch(pairs[i].0));
            bursts.push(Burst {
                at: at_ts,
                requests_in_second: count,
                urls,
            });
            i = j; // skip past the burst
        } else {
            i += 1;
        }
    }
    bursts
}

/// Detect rate-limit events: 429/503 responses preceded by elevated rate.
fn detect_rate_limits(pairs: &[(u64, &ResponseResult)]) -> Vec<RateLimitEvent> {
    let mut events = Vec::new();
    let n = pairs.len();

    for i in 0..n {
        let status = pairs[i].1.status_code;
        if status != 429 && status != 503 {
            continue;
        }

        // Compute rate in the 3 seconds before this request.
        let window_start = pairs[i].0.saturating_sub(3000);
        let mut count = 0u64;
        for j in (0..i).rev() {
            if pairs[j].0 >= window_start {
                count += 1;
            } else {
                break;
            }
        }

        let window_size = (pairs[i].0 - window_start) as f64;
        let duration_secs = window_size / 1000.0;
        let rate = count as f64 / duration_secs.max(0.001);

        let at_ts = pairs[i]
            .1
            .timestamp
            .clone()
            .unwrap_or_else(|| format_epoch(pairs[i].0));

        events.push(RateLimitEvent {
            at: at_ts,
            url: pairs[i].1.endpoint_url.clone(),
            preceding_rate: rate,
        });
    }

    events
}

/// Extract path portion from a URL string.
fn extract_path(url: &str) -> String {
    if let Some(pos) = url.find("://") {
        let after_scheme = &url[pos + 3..];
        if let Some(slash) = after_scheme.find('/') {
            let path_and_query = &after_scheme[slash..];
            // Strip query string for grouping.
            if let Some(qpos) = path_and_query.find('?') {
                path_and_query[..qpos].to_string()
            } else {
                path_and_query.to_string()
            }
        } else {
            "/".to_string()
        }
    } else {
        url.to_string()
    }
}

/// Format epoch millis as ISO-8601 string (UTC, no subseconds).
fn format_epoch(ms: u64) -> String {
    let secs = ms / 1000;
    let days_since_epoch = secs / 86400;
    let day_secs = secs % 86400;
    let hour = day_secs / 3600;
    let min = (day_secs % 3600) / 60;
    let sec = day_secs % 60;

    // Days since 1970-01-01.
    let epoch_day = days_since_epoch as i64;
    let z = epoch_day + 719468;
    let era = if z >= 0 { z } else { z - 146096 };
    let doe = era % 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + (era / 146097) * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 {
        mp as u32 + 3
    } else {
        mp as u32 - 9
    };
    let year = if m <= 2 { y as u32 + 1 } else { y as u32 };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, m, d, hour, min, sec
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ResponseResult;

    fn make_result(url: &str, status: u16, time_ms: u64, ts: &str) -> (String, ResponseResult) {
        let r = ResponseResult {
            timestamp: Some(ts.to_string()),
            endpoint_method: "GET".into(),
            endpoint_url: url.into(),
            status_code: status,
            response_time_ms: time_ms,
            response_size: 0,
            response_headers: vec![],
            response_body: vec![],
            expected_status: None,
            checks: vec![],
            error: None,
            tags: vec![],
            trackers: vec![],
        };
        (ts.to_string(), r)
    }

    #[test]
    fn test_parse_iso8601_basic() {
        let ms = parse_iso8601("2026-07-16T14:30:00Z").unwrap();
        // 2026-07-16 in epoch ms: let's compute approximately
        assert!(ms > 1_784_000_000_000);
        assert!(ms < 1_800_000_000_000);
    }

    #[test]
    fn test_parse_iso8601_with_millis() {
        let ms = parse_iso8601("2026-07-16T14:30:00.123Z").unwrap();
        assert_eq!(ms % 1000, 123);
    }

    #[test]
    fn test_parse_iso8601_invalid() {
        assert!(parse_iso8601("not-a-date").is_none());
        assert!(parse_iso8601("").is_none());
    }

    #[test]
    fn test_percentile() {
        let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        assert_eq!(percentile(&data, 50), 60); // nearest-rank: sorted[5]
        assert_eq!(percentile(&data, 90), 90); // nearest-rank: sorted[8]
        assert_eq!(percentile(&data, 100), 100);
    }

    #[test]
    fn test_extract_path() {
        assert_eq!(extract_path("https://example.com/api/users"), "/api/users");
        assert_eq!(
            extract_path("https://example.com/api/users?page=1"),
            "/api/users"
        );
        assert_eq!(extract_path("https://example.com/"), "/");
        assert_eq!(extract_path("https://example.com"), "/");
    }

    #[test]
    fn test_compute_empty() {
        let ta = compute_timing_analytics(&[], &[]);
        assert_eq!(ta.total_requests, 0);
    }

    #[test]
    fn test_compute_single() {
        let (ts, res) = make_result("https://example.com/", 200, 10, "2026-07-16T14:30:00Z");
        let ta = compute_timing_analytics(&[ts], &[res]);
        assert_eq!(ta.total_requests, 1);
    }

    #[test]
    fn test_detect_bursts() {
        let base = parse_iso8601("2026-07-16T14:30:00Z").unwrap();
        let mut results = Vec::new();
        for _i in 0..15 {
            results.push(ResponseResult {
                timestamp: None,
                endpoint_method: "GET".into(),
                endpoint_url: "https://example.com/api".into(),
                status_code: 200,
                response_time_ms: 0,
                response_size: 0,
                response_headers: vec![],
                response_body: vec![],
                expected_status: None,
                checks: vec![],
                error: None,
                tags: vec![],
                trackers: vec![],
            });
        }
        let pairs: Vec<(u64, &ResponseResult)> = results
            .iter()
            .enumerate()
            .map(|(i, r)| (base + i as u64 * 50, r))
            .collect();
        let bursts = detect_bursts(&pairs);
        assert!(!bursts.is_empty());
        assert!(bursts[0].requests_in_second >= 15);
    }

    #[test]
    fn test_detect_rate_limits() {
        let base = parse_iso8601("2026-07-16T14:30:00Z").unwrap();
        let mut results = Vec::new();
        for i in 0..10 {
            results.push(ResponseResult {
                timestamp: if i == 9 {
                    Some("2026-07-16T14:30:00.500Z".into())
                } else {
                    None
                },
                endpoint_method: "GET".into(),
                endpoint_url: "https://example.com/api/search".into(),
                status_code: if i == 9 { 429 } else { 200 },
                response_time_ms: 0,
                response_size: 0,
                response_headers: vec![],
                response_body: vec![],
                expected_status: None,
                checks: vec![],
                error: None,
                tags: vec![],
                trackers: vec![],
            });
        }
        let pairs: Vec<(u64, &ResponseResult)> = results
            .iter()
            .enumerate()
            .map(|(i, r)| (base + i as u64 * 100, r))
            .collect();
        let events = detect_rate_limits(&pairs);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].url, "https://example.com/api/search");
    }

    #[test]
    fn test_format_epoch_roundtrip() {
        let original = "2026-07-16T14:30:00Z";
        let ms = parse_iso8601(original).unwrap();
        let formatted = format_epoch(ms);
        assert_eq!(formatted, original);
    }
}
