//! JSON output formatter for scan results.

use crate::types::ResponseResult;

/// Format results as pretty-printed JSON.
pub fn format_json(results: &[ResponseResult]) -> String {
    serde_json::to_string_pretty(results).unwrap_or_else(|_| "[]".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result() -> ResponseResult {
        ResponseResult {
            endpoint_method: "GET".into(),
            endpoint_url: "https://api.example.com/test".into(),
            status_code: 200,
            response_time_ms: 100,
            response_size: 64,
            response_headers: vec![],
            response_body: vec![],
            expected_status: None,
            timestamp: None,
            checks: vec![],
            error: None,
            tags: vec![],
            trackers: vec![],
        }
    }

    #[test]
    fn test_format_json_empty() {
        let out = format_json(&[]);
        assert_eq!(out, "[]");
    }

    #[test]
    fn test_format_json_single() {
        let out = format_json(&[result()]);
        assert!(out.contains("api.example.com"));
        assert!(out.contains("GET"));
        assert!(out.contains("200"));
    }

    #[test]
    fn test_format_json_roundtrip() {
        let results = vec![result()];
        let json = format_json(&results);
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0]["endpoint_url"], "https://api.example.com/test");
    }

    #[test]
    fn test_format_json_multiple() {
        let results = vec![result(), result()];
        let out = format_json(&results);
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&out).unwrap();
        assert_eq!(parsed.len(), 2);
    }
}
