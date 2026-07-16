/// Fuzzing runner — dispatches path/param/method/header fuzz loops.
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Semaphore;

use crate::config::ScanConfig;
use crate::error::Result;
use crate::fuzz::matcher::MatchConfig;
use crate::types::ResponseResult;

pub struct FuzzRunner {
    client: reqwest::Client,
    concurrency: usize,
    rate_delay: Duration,
}

impl FuzzRunner {
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

    /// Fuzz path endpoints using a wordlist.
    pub async fn fuzz_paths(
        &self,
        base_url: &reqwest::Url,
        words: &[String],
        matcher: &MatchConfig,
    ) -> Vec<ResponseResult> {
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::with_capacity(words.len());

        for word in words {
            let permit = sem.clone().acquire_owned().await.unwrap();
            let client = self.client.clone();
            let delay = self.rate_delay;
            let url_str = format!("{}{}", base_url.to_string().trim_end_matches('/'), word);
            let matcher = matcher.clone();

            handles.push(tokio::spawn(async move {
                let _permit = permit;
                tokio::time::sleep(delay).await;
                let result = hit(&client, &url_str).await;
                if matcher.evaluate(&result) {
                    Some(result)
                } else {
                    None
                }
            }));
        }

        let mut results = Vec::new();
        for h in handles {
            if let Some(r) = h.await.unwrap_or(None) {
                results.push(r);
            }
        }
        results
    }
}

async fn hit(client: &reqwest::Client, url: &str) -> ResponseResult {
    let start = std::time::Instant::now();
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let headers: Vec<(String, String)> = resp
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
                .collect();
            let body = resp.bytes().await.unwrap_or_default().to_vec();
            ResponseResult {
                endpoint_method: "GET".into(),
                endpoint_url: url.into(),
                status_code: status,
                response_time_ms: start.elapsed().as_millis() as u64,
                response_size: body.len(),
                response_headers: headers,
                response_body: body,
                expected_status: None,
                checks: vec![],
                error: None,
                tags: vec![],
            }
        }
        Err(e) => ResponseResult {
            endpoint_method: "GET".into(),
            endpoint_url: url.into(),
            status_code: 0,
            response_time_ms: start.elapsed().as_millis() as u64,
            response_size: 0,
            response_headers: vec![],
            response_body: vec![],
            expected_status: None,
            checks: vec![],
            error: Some(e.to_string()),
            tags: vec![],
        },
    }
}
