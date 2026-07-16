use crate::types::{AuthConfig, Check, Severity};

/// Check whether an endpoint enforces authentication by re-requesting it
/// without any auth headers. Only runs when `--auth` was provided.
pub async fn check_auth_required(
    client: &reqwest::Client,
    url: &str,
    method: &str,
    config_auth: &Option<AuthConfig>,
) -> Vec<Check> {
    if config_auth.is_none() {
        return vec![];
    }

    let method = match reqwest::Method::from_bytes(method.as_bytes()) {
        Ok(m) => m,
        Err(_) => return vec![],
    };

    let req = client.request(method, url);
    // Deliberately no auth headers.

    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            if (200..=299).contains(&status) || status == 302 || status == 303 {
                vec![Check {
                    name: "Auth Required".into(),
                    passed: false,
                    severity: Severity::Warn,
                    message: format!(
                        "endpoint returned {status} without auth — authentication may not be enforced"
                    ),
                }]
            } else if status == 401 || status == 403 {
                vec![Check {
                    name: "Auth Required".into(),
                    passed: true,
                    severity: Severity::Info,
                    message: format!("endpoint returned {status} without auth — auth is enforced"),
                }]
            } else {
                vec![]
            }
        }
        Err(_) => vec![],
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
    async fn test_auth_skip_if_none() {
        let client = check_client();
        let checks = check_auth_required(&client, "http://127.0.0.1:1/test", "GET", &None).await;
        assert!(checks.is_empty());
    }
}
