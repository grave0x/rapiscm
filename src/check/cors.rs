use crate::types::{Check, Severity};

/// Check CORS configuration by sending an OPTIONS preflight with a
/// cross-origin `Origin` header and inspecting the response.
pub async fn check_cors(client: &reqwest::Client, url: &str, method: &str) -> Vec<Check> {
    let mut req = client.request(reqwest::Method::OPTIONS, url);
    req = req.header("Origin", "https://evil.com");
    req = req.header("Access-Control-Request-Method", method);

    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let acao = resp
                .headers()
                .get("access-control-allow-origin")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let acac = resp
                .headers()
                .get("access-control-allow-credentials")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let mut checks = Vec::new();

            match acao.as_deref() {
                Some("*") => checks.push(Check {
                    name: "CORS".into(),
                    passed: false,
                    severity: Severity::Warn,
                    message: "Access-Control-Allow-Origin: * — any site can read responses".into(),
                }),
                Some(origin) if origin == "https://evil.com" => checks.push(Check {
                    name: "CORS".into(),
                    passed: false,
                    severity: Severity::Warn,
                    message: format!(
                        "Access-Control-Allow-Origin mirrors attacker origin ({origin})"
                    ),
                }),
                Some(origin) => checks.push(Check {
                    name: "CORS".into(),
                    passed: true,
                    severity: Severity::Info,
                    message: format!("Access-Control-Allow-Origin: {origin}"),
                }),
                None => {
                    if status == 200 || status == 204 {
                        // OPTIONS succeeded but no ACAO header
                        checks.push(Check {
                            name: "CORS".into(),
                            passed: true,
                            severity: Severity::Info,
                            message: "No CORS headers — cross-origin reads not allowed".into(),
                        });
                    }
                    // Non-2xx OPTIONS means preflight rejected → no CORS risk
                }
            }

            if let Some(creds) = acac
                && creds == "true"
                && acao.as_deref() == Some("*")
            {
                checks.push(Check {
                    name: "CORS".into(),
                    passed: false,
                    severity: Severity::Critical,
                    message: "Wildcard origin with credentials — credentials leak to any site"
                        .into(),
                });
            }

            checks
        }
        Err(e) => {
            vec![Check {
                name: "CORS".into(),
                passed: true,
                severity: Severity::Info,
                message: format!("CORS preflight failed — endpoint may not support OPTIONS ({e})"),
            }]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_client() -> reqwest::Client {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(1))
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn test_cors_unreachable() {
        let client = check_client();
        let checks = check_cors(&client, "http://127.0.0.1:1/test", "GET").await;
        assert!(checks.iter().any(|c| c.name == "CORS"));
    }
}
