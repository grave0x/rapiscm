//! JSON output formatter for scan results.

use crate::types::ResponseResult;

/// Format results as pretty-printed JSON.
pub fn format_json(results: &[ResponseResult]) -> String {
    serde_json::to_string_pretty(results).unwrap_or_else(|_| "[]".into())
}
