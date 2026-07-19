//! Response matching and filtering configuration for fuzzing.

use crate::types::ResponseResult;

/// Configuration for matching/filtering fuzz responses.
#[derive(Debug, Clone, Default)]
pub struct MatchConfig {
    /// Status code ranges to match (empty = match all)
    pub match_status: Vec<Range>,
    /// Status code ranges to filter (exclude)
    pub filter_status: Vec<Range>,
    /// Response size ranges to match
    pub match_size: Vec<Range>,
    /// Response size ranges to filter
    pub filter_size: Vec<Range>,
    /// Regex pattern to match in response body
    pub match_regex: Option<String>,
    /// Regex pattern to filter in response body
    pub filter_regex: Option<String>,
    /// Auto-calibrated baseline (size, status) for filtering
    pub baseline: Option<Baseline>,
}

/// A numeric range with inclusive start and end bounds.
#[derive(Debug, Clone)]
pub struct Range {
    /// Start of range (inclusive).
    pub start: u64,
    /// End of range (inclusive).
    pub end: u64,
}

impl Range {
    /// Create a single-value range (`start == end`).
    pub fn single(v: u64) -> Self {
        Range { start: v, end: v }
    }
    /// Check if a value falls within the range (inclusive).
    pub fn contains(&self, v: u64) -> bool {
        v >= self.start && v <= self.end
    }
}

/// Baseline response characteristics for filtering common (uninteresting) responses.
#[derive(Debug, Clone)]
pub struct Baseline {
    /// Baseline HTTP status code.
    pub status: u16,
    /// Baseline response body size.
    pub size: usize,
}

impl MatchConfig {
    /// Evaluate a response against this config. Returns true if the response matches.
    pub fn evaluate(&self, result: &ResponseResult) -> bool {
        // If there was a connection error, skip matching
        if result.status_code == 0 {
            return false;
        }

        let status = result.status_code as u64;
        let size = result.response_size as u64;

        // Apply status filter (exclude matches)
        for r in &self.filter_status {
            if r.contains(status) {
                return false;
            }
        }

        // Apply size filter
        for r in &self.filter_size {
            if r.contains(size) {
                return false;
            }
        }

        // If baseline is set, exclude matches that match baseline
        if let Some(ref base) = self.baseline
            && result.status_code == base.status
            && result.response_size == base.size
        {
            return false;
        }

        // Apply status matcher
        if !self.match_status.is_empty() {
            let matched = self.match_status.iter().any(|r| r.contains(status));
            if !matched {
                return false;
            }
        }

        // Apply size matcher
        if !self.match_size.is_empty() {
            let matched = self.match_size.iter().any(|r| r.contains(size));
            if !matched {
                return false;
            }
        }

        // Apply regex matchers
        if let Some(ref pattern) = self.match_regex {
            let body = String::from_utf8_lossy(&result.response_body);
            if !regex::Regex::new(pattern).is_ok_and(|re| re.is_match(&body)) {
                return false;
            }
        }
        if let Some(ref pattern) = self.filter_regex {
            let body = String::from_utf8_lossy(&result.response_body);
            if regex::Regex::new(pattern).is_ok_and(|re| re.is_match(&body)) {
                return false;
            }
        }

        true
    }
}

/// Parse a match/filter string like `"200,201,400-499"` into `Vec<Range>`.
pub fn parse_range_list(s: &str) -> Vec<Range> {
    s.split(',')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }
            if let Some((a, b)) = part.split_once('-') {
                let start = a.trim().parse().ok()?;
                let end = b.trim().parse().ok()?;
                Some(Range { start, end })
            } else {
                let v = part.parse().ok()?;
                Some(Range::single(v))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_result(status: u16, size: usize) -> ResponseResult {
        ResponseResult {
            endpoint_method: "GET".into(),
            endpoint_url: "http://test/".into(),
            status_code: status,
            response_time_ms: 0,
            response_size: size,
            response_headers: vec![],
            response_body: vec![],
            expected_status: None,
            checks: vec![],
            timestamp: None,
            error: None,
            tags: vec![],
            trackers: vec![],
        }
    }

    #[test]
    fn test_match_status() {
        let config = MatchConfig {
            match_status: vec![Range::single(200), Range::single(201)],
            ..Default::default()
        };
        assert!(config.evaluate(&sample_result(200, 100)));
        assert!(config.evaluate(&sample_result(201, 100)));
        assert!(!config.evaluate(&sample_result(404, 100)));
    }

    #[test]
    fn test_filter_status() {
        let config = MatchConfig {
            filter_status: vec![Range::single(404)],
            ..Default::default()
        };
        assert!(config.evaluate(&sample_result(200, 100)));
        assert!(!config.evaluate(&sample_result(404, 100)));
    }

    #[test]
    fn test_parse_range_list() {
        let ranges = parse_range_list("200,201,400-499");
        assert_eq!(ranges.len(), 3);
        assert!(ranges[0].contains(200));
        assert!(ranges[2].contains(404));
        assert!(ranges[2].contains(499));
        assert!(!ranges[2].contains(500));
    }

    #[test]
    fn test_baseline_filter() {
        let config = MatchConfig {
            baseline: Some(Baseline {
                status: 404,
                size: 50,
            }),
            ..Default::default()
        };
        assert!(!config.evaluate(&sample_result(404, 50))); // filtered as baseline
        assert!(config.evaluate(&sample_result(200, 100))); // different → matched
    }

    #[test]
    fn test_empty_config_matches_all() {
        let config = MatchConfig::default();
        assert!(config.evaluate(&sample_result(200, 0)));
        assert!(config.evaluate(&sample_result(404, 0)));
    }
}
