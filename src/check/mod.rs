pub mod auth;
pub mod cors;
pub mod schema;
pub mod security;

use std::sync::Arc;
use std::time::Duration;

use crate::config::ScanConfig;
use crate::types::ResponseResult;

/// Run all synchronous checks on a completed scan result.
pub fn run_checks(result: &mut ResponseResult) {
    let header_checks = security::check_security_headers(&result.response_headers);
    result.checks.extend(header_checks);

    let schema_checks = schema::check_response_schema(
        result.status_code,
        &result.response_body,
        result.expected_status,
    );
    result.checks.extend(schema_checks);
}

/// Run asynchronous checks (CORS preflight, auth probe) on scan results.
/// Spawns one task per result; collects via index tracking.
pub async fn run_async_checks(config: &ScanConfig, results: &mut [ResponseResult]) {
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .danger_accept_invalid_certs(config.insecure);
    if let Some(ref proxy_url) = config.proxy
        && let Ok(proxy) = reqwest::Proxy::all(proxy_url)
    {
        builder = builder.proxy(proxy);
    }
    let Ok(client) = builder.build() else { return };
    let client = Arc::new(client);

    // Build a list of (index, task) for endpoints that got a response.
    let mut tasks: Vec<(usize, tokio::task::JoinHandle<Vec<crate::types::Check>>)> = Vec::new();
    for (i, r) in results.iter().enumerate() {
        if r.status_code == 0 {
            continue;
        }
        let client = client.clone();
        let url = r.endpoint_url.clone();
        let method = r.endpoint_method.clone();
        let auth = config.auth.clone();
        tasks.push((
            i,
            tokio::spawn(async move {
                let mut c = Vec::new();
                c.extend(cors::check_cors(&client, &url, &method).await);
                if auth.is_some() {
                    c.extend(auth::check_auth_required(&client, &url, &method, &auth).await);
                }
                c
            }),
        ));
    }

    for (i, handle) in tasks {
        if let Ok(checks) = handle.await {
            results[i].checks.extend(checks);
        }
    }
}
