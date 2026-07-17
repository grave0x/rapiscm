use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Semaphore;
use tracing::{info, warn};

use crate::config::ScanConfig;
use crate::error::Result;
use crate::types::{Endpoint, ResponseResult};

/// Concurrent HTTP scanner with rate limiting.
///
/// Uses a semaphore for concurrency control. Each task sleeps for
/// `1/rate_limit` before its request, giving an approximate global
/// rate of `rate_limit` requests/sec (not exact, but good enough for CLI).
#[derive(Clone)]
pub struct ScanRunner {
    client: reqwest::Client,
    concurrency: usize,
    rate_delay: Duration,
}

impl ScanRunner {
    pub fn new(config: &ScanConfig) -> Result<Self> {
        let mut builder = reqwest::Client::builder()
            .timeout(config.timeout)
            .danger_accept_invalid_certs(config.insecure)
            .redirect(if config.follow_redirects {
                reqwest::redirect::Policy::limited(10)
            } else {
                reqwest::redirect::Policy::none()
            });
        if let Some(ref proxy_url) = config.proxy {
            let proxy = reqwest::Proxy::all(proxy_url)
                .map_err(|e| crate::error::Error::InvalidUrl(e.to_string()))?;
            builder = builder.proxy(proxy);
        }
        let client = builder.build()?;
        Ok(Self {
            client,
            concurrency: config.concurrency,
            rate_delay: Duration::from_secs_f64(1.0 / config.rate_limit.max(1) as f64),
        })
    }

    /// Fire requests for every endpoint, respecting concurrency and rate limits.
    pub async fn run(&self, endpoints: Vec<Endpoint>) -> Vec<ResponseResult> {
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let total = endpoints.len();
        let mut handles = Vec::with_capacity(total);

        for (i, ep) in endpoints.into_iter().enumerate() {
            // Block loop when at concurrency cap.
            // Semaphore is never closed, so acquire cannot fail.
            let permit = sem
                .clone()
                .acquire_owned()
                .await
                .expect("semaphore should not be closed");
            let client = self.client.clone();
            let delay = self.rate_delay;

            handles.push(tokio::spawn(async move {
                let _permit = permit;
                tokio::time::sleep(delay).await;
                hit_endpoint(&client, ep).await
            }));

            info!("queued {}/{}", i + 1, total);
        }

        let mut results = Vec::with_capacity(total);
        for h in handles {
            match h.await {
                Ok(r) => results.push(r),
                Err(e) => warn!("scan task panicked: {e}"),
            }
        }
        results
    }
}

async fn hit_endpoint(client: &reqwest::Client, ep: Endpoint) -> ResponseResult {
    let start = std::time::Instant::now();
    let method = ep.method.clone();
    let url = ep.url.clone();
    let expected_status = ep.expected_status;

    let mut req = client.request(method.clone(), url.clone());
    for (k, v) in &ep.headers {
        req = req.header(k.as_str(), v.as_str());
    }
    if let Some(body) = &ep.body {
        req = req.json(body);
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let elapsed = start.elapsed();
            let response_headers: Vec<(String, String)> = resp
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
                .collect();
            // Cap response body at 1MB to prevent OOM on large responses
            const MAX_BODY: usize = 1_048_576;
            let response_body = {
                let mut body = resp.bytes().await.unwrap_or_default().to_vec();
                if body.len() > MAX_BODY {
                    body.truncate(MAX_BODY);
                    warn!("response body truncated to 1MB ({url})");
                }
                body
            };
            let size = response_body.len();
            info!("{status} {method} {url} ({elapsed:?})");
            ResponseResult {
                endpoint_method: method.to_string(),
                endpoint_url: url.to_string(),
                status_code: status,
                response_time_ms: elapsed.as_millis() as u64,
                response_size: size,
                response_headers,
                response_body,
                expected_status,
                timestamp: None,
                checks: Vec::new(),
                error: None,
                tags: vec![],
                trackers: vec![],
            }
        }
        Err(e) => {
            let elapsed = start.elapsed();
            warn!("ERR {method} {url}: {e}");
            ResponseResult {
                endpoint_method: method.to_string(),
                endpoint_url: url.to_string(),
                status_code: 0,
                response_time_ms: elapsed.as_millis() as u64,
                response_size: 0,
                response_headers: vec![],
                response_body: vec![],
                expected_status,
                timestamp: None,
                checks: Vec::new(),
                error: Some(e.to_string()),
                tags: vec![],
                trackers: vec![],
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Endpoint;

    fn dummy_endpoint() -> Endpoint {
        Endpoint {
            method: reqwest::Method::GET,
            url: reqwest::Url::parse("http://127.0.0.1:1/nonexistent").unwrap(),
            headers: vec![],
            body: None,
            expected_status: None,
            tags: vec![],
        }
    }

    #[tokio::test]
    async fn test_runner_handles_connection_error() {
        let config = ScanConfig {
            target: crate::types::Target::Url(reqwest::Url::parse("http://127.0.0.1:1").unwrap()),
            method: None,
            headers: vec![],
            auth: None,
            rate_limit: 1000,
            timeout: Duration::from_secs(1),
            concurrency: 1,
            output: crate::types::OutputFormat::Json,
            follow_redirects: false,
            insecure: false,
            paths: vec![],
            tags: vec![],
            filter_tag: vec![],
            exclude_tag: vec![],
            proxy: None,
            log_level: "info".into(),
            log_filter: vec![],
            log_format: "text".into(),
            #[cfg(feature = "browser")]
            browser_kind: crate::scan::browser::BrowserKind::Chrome,
            #[cfg(feature = "browser")]
            headed: false,
            crawl: false,
            depth: 2,
            filter_path: vec![],
            exclude_path: vec![],
            filter_method: vec![],
            exclude_method: vec![],
            filter_status: vec![],
            exclude_status: vec![],
            filter: vec![],
            exclude: vec![],
            show_tags: false,
            trackers: true,
            corp: None,
        };

        let runner = ScanRunner::new(&config).unwrap();
        let results = runner.run(vec![dummy_endpoint()]).await;

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status_code, 0);
        assert!(results[0].error.is_some());
    }
}
