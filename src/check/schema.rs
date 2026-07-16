use crate::types::{Check, Severity};

/// Validate a response against known expectations.
///
/// Checks performed:
/// - If the endpoint has an expected status code, compare with actual.
/// - If the body looks like JSON, verify it's valid.
pub fn check_response_schema(
    status_code: u16,
    response_body: &[u8],
    expected_status: Option<u16>,
) -> Vec<Check> {
    let mut checks = Vec::new();

    // Expected status check.
    if let Some(expected) = expected_status {
        if status_code != expected {
            checks.push(Check {
                name: "Response Status".into(),
                passed: false,
                severity: Severity::Warn,
                message: format!("expected status {expected}, got {status_code}",),
            });
        } else {
            checks.push(Check {
                name: "Response Status".into(),
                passed: true,
                severity: Severity::Info,
                message: format!("returned {status_code} as expected"),
            });
        }
    }

    // Body schema check — try to parse as JSON and validate basic structure.
    if status_code > 0 && !response_body.is_empty() {
        match serde_json::from_slice::<serde_json::Value>(response_body) {
            Ok(val) => {
                // Basic structural check: is it an object or array?
                if val.is_object() || val.is_array() {
                    checks.push(Check {
                        name: "Response Body".into(),
                        passed: true,
                        severity: Severity::Info,
                        message: "valid JSON body".into(),
                    });
                } else {
                    checks.push(Check {
                        name: "Response Body".into(),
                        passed: false,
                        severity: Severity::Info,
                        message: format!("unexpected JSON type: {}", json_type_name(&val)),
                    });
                }
            }
            Err(e) => {
                checks.push(Check {
                    name: "Response Body".into(),
                    passed: false,
                    severity: Severity::Info,
                    message: format!("response is not valid JSON: {e}"),
                });
            }
        }
    }

    checks
}

fn json_type_name(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_status_match() {
        let checks = check_response_schema(200, b"{}", Some(200));
        assert!(
            checks
                .iter()
                .any(|c| c.name == "Response Status" && c.passed)
        );
    }

    #[test]
    fn test_expected_status_mismatch() {
        let checks = check_response_schema(404, b"{}", Some(200));
        assert!(
            checks
                .iter()
                .any(|c| c.name == "Response Status" && !c.passed)
        );
    }

    #[test]
    fn test_valid_json_body() {
        let checks = check_response_schema(200, br#"{"id": 1}"#, None);
        assert!(checks.iter().any(|c| c.name == "Response Body" && c.passed));
    }

    #[test]
    fn test_invalid_json_body() {
        let checks = check_response_schema(200, b"not json", None);
        assert!(
            checks
                .iter()
                .any(|c| c.name == "Response Body" && !c.passed)
        );
    }
}
