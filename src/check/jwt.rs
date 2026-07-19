//! JWT token analysis: decode, validate claims, detect algorithm confusion risks.

use crate::types::{Check, Severity};
use base64::Engine;

use base64::alphabet::URL_SAFE;
use base64::engine::GeneralPurpose;
use base64::engine::GeneralPurposeConfig;

/// Analyze a JWT token string for security issues.
pub fn analyze_jwt(token: &str) -> Vec<Check> {
    let mut checks = Vec::new();

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        checks.push(Check {
            name: "JWT Structure".into(),
            passed: false,
            severity: Severity::Warn,
            message: "JWT does not have 3 dot-separated parts (invalid structure)".into(),
        });
        return checks;
    }

    let (header, payload) = match decode_parts(parts[0], parts[1]) {
        Ok(hp) => hp,
        Err(e) => {
            checks.push(Check {
                name: "JWT Decode".into(),
                passed: false,
                severity: Severity::Warn,
                message: format!("JWT decode failed: {e}"),
            });
            return checks;
        }
    };

    checks.push(Check {
        name: "JWT Structure".into(),
        passed: true,
        severity: Severity::Info,
        message: "JWT has valid 3-part structure".into(),
    });

    // Check algorithm.
    let alg = header.get("alg").and_then(|v| v.as_str()).unwrap_or("none");
    check_algorithm(alg, &mut checks);

    // Check for "none" algorithm bypass.
    if alg.eq_ignore_ascii_case("none") {
        checks.push(Check {
            name: "JWT Algorithm".into(),
            passed: false,
            severity: Severity::Critical,
            message: "JWT uses alg=none — signature verification is disabled".into(),
        });
    }

    // Check token type.
    if let Some(typ) = header.get("typ").and_then(|v| v.as_str())
        && !typ.eq_ignore_ascii_case("jwt")
    {
        checks.push(Check {
            name: "JWT Type".into(),
            passed: false,
            severity: Severity::Info,
            message: format!("JWT typ header is '{typ}', expected 'JWT'"),
        });
    }

    // Validate standard claims.
    if let Some(exp) = payload.get("exp").and_then(|v| v.as_i64()) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        if exp < now {
            checks.push(Check {
                name: "JWT Expiration".into(),
                passed: false,
                severity: Severity::Warn,
                message: "JWT is expired".into(),
            });
        } else {
            checks.push(Check {
                name: "JWT Expiration".into(),
                passed: true,
                severity: Severity::Info,
                message: "JWT is not expired".into(),
            });
        }
    } else {
        checks.push(Check {
            name: "JWT Expiration".into(),
            passed: false,
            severity: Severity::Warn,
            message: "JWT has no exp claim — token never expires".into(),
        });
    }

    // Check for weak HMAC secret heuristic (if header is HS256/HS384/HS512).
    check_hmac_risk(alg, &mut checks);

    checks
}

fn decode_parts(
    header_b64: &str,
    payload_b64: &str,
) -> Result<(serde_json::Value, serde_json::Value), String> {
    let header_bytes = decode_b64_url(header_b64)?;
    let payload_bytes = decode_b64_url(payload_b64)?;
    let header: serde_json::Value =
        serde_json::from_slice(&header_bytes).map_err(|e| format!("header JSON: {e}"))?;
    let payload: serde_json::Value =
        serde_json::from_slice(&payload_bytes).map_err(|e| format!("payload JSON: {e}"))?;
    Ok((header, payload))
}

fn decode_b64_url(input: &str) -> Result<Vec<u8>, String> {
    let engine = GeneralPurpose::new(&URL_SAFE, GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true));
    engine
        .decode(input)
        .map_err(|e| format!("base64 decode: {e}"))
}

fn check_algorithm(alg: &str, checks: &mut Vec<Check>) {
    let is_symmetric = matches!(alg, "HS256" | "HS384" | "HS512");
    let is_asymmetric = matches!(alg, "RS256" | "RS384" | "RS512" | "ES256" | "ES384" | "ES512" | "PS256" | "PS384" | "PS512");
    let is_known = is_symmetric || is_asymmetric || alg.eq_ignore_ascii_case("none");

    if !is_known {
        checks.push(Check {
            name: "JWT Algorithm".into(),
            passed: false,
            severity: Severity::Warn,
            message: format!("Unknown or non-standard JWT algorithm: {alg}"),
        });
    } else {
        checks.push(Check {
            name: "JWT Algorithm".into(),
            passed: true,
            severity: Severity::Info,
            message: format!("JWT uses {alg} algorithm"),
        });
    }
}

fn check_hmac_risk(alg: &str, checks: &mut Vec<Check>) {
    if matches!(alg, "HS256" | "HS384" | "HS512") {
        checks.push(Check {
            name: "JWT HMAC".into(),
            passed: false,
            severity: Severity::Info,
            message: format!(
                "JWT uses symmetric {alg} — vulnerable to algorithm confusion if \
                 public key is known or signature is not validated"
            ),
        });
    }
}

/// Extract JWT tokens from a response body using simple pattern matching.
pub fn extract_jwt_tokens(body: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    // Match Authorization: Bearer <token> patterns
    for cap in regex::Regex::new(r"(?i)bearer\s+([A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+)")
        .unwrap()
        .captures_iter(body)
    {
        tokens.push(cap[1].to_string());
    }
    // Match standalone JWT-like strings
    for cap in regex::Regex::new(r"([A-Za-z0-9_-]+\.[A-Za-z0-9_-]+\.[A-Za-z0-9_-]+)")
        .unwrap()
        .captures_iter(body)
    {
        let token = cap[1].to_string();
        // Skip very short tokens (likely not JWTs)
        if token.len() > 40 && !tokens.contains(&token) {
            tokens.push(token);
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_jwt(header: &str, payload: &str) -> String {
        let eng = GeneralPurpose::new(&URL_SAFE, GeneralPurposeConfig::new());
        let h = eng.encode(header);
        let p = eng.encode(payload);
        format!("{h}.{p}.fakesignature")
    }

    #[test]
    fn test_valid_jwt() {
        let token = make_jwt(r#"{"alg":"RS256","typ":"JWT"}"#, r#"{"sub":"123","exp":9999999999}"#);
        let checks = analyze_jwt(&token);
        assert!(checks.iter().any(|c| c.name == "JWT Structure" && c.passed));
        assert!(checks.iter().any(|c| c.name == "JWT Algorithm" && c.passed));
        assert!(checks.iter().any(|c| c.name == "JWT Expiration" && c.passed));
    }

    #[test]
    fn test_expired_jwt() {
        let token = make_jwt(r#"{"alg":"RS256"}"#, r#"{"exp":1}"#);
        let checks = analyze_jwt(&token);
        assert!(checks.iter().any(|c| c.name == "JWT Expiration" && !c.passed));
    }

    #[test]
    fn test_no_exp_claim() {
        let token = make_jwt(r#"{"alg":"RS256"}"#, r#"{"sub":"123"}"#);
        let checks = analyze_jwt(&token);
        assert!(checks.iter().any(|c| c.name == "JWT Expiration" && !c.passed && c.message.contains("no exp")));
    }

    #[test]
    fn test_none_algorithm() {
        let token = make_jwt(r#"{"alg":"none"}"#, r#"{"sub":"admin"}"#);
        let checks = analyze_jwt(&token);
        assert!(checks.iter().any(|c| c.name == "JWT Algorithm" && !c.passed && c.message.contains("none")));
    }

    #[test]
    fn test_invalid_structure() {
        let checks = analyze_jwt("not.a.jwt.extra.part");
        assert!(checks.iter().any(|c| c.name == "JWT Structure" && !c.passed));
    }

    #[test]
    fn test_not_three_parts() {
        let checks = analyze_jwt("justonepart");
        assert!(checks.iter().any(|c| c.name == "JWT Structure" && !c.passed));
    }

    #[test]
    fn test_hs256_flagged() {
        let token = make_jwt(r#"{"alg":"HS256"}"#, r#"{"sub":"123"}"#);
        let checks = analyze_jwt(&token);
        assert!(checks.iter().any(|c| c.name == "JWT HMAC"));
    }

    #[test]
    fn test_unknown_algorithm() {
        let token = make_jwt(r#"{"alg":"FOObar"}"#, r#"{"sub":"123"}"#);
        let checks = analyze_jwt(&token);
        assert!(checks.iter().any(|c| c.name == "JWT Algorithm" && !c.passed));
    }

    #[test]
    fn test_extract_jwt_tokens() {
        let body = "Authorization: Bearer eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjMifQ.fakesig";
        let tokens = extract_jwt_tokens(body);
        assert_eq!(tokens.len(), 1);
        assert!(tokens[0].contains("eyJhbGci"));
    }

    #[test]
    fn test_extract_jwt_standalone() {
        let body = "token=eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjMifQ.fakesig";
        let tokens = extract_jwt_tokens(body);
        assert!(!tokens.is_empty());
    }
}
