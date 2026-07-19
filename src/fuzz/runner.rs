//! Core fuzz runner: dispatches path/param/method/header/body fuzz loops.

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Semaphore;

use crate::config::ScanConfig;
use crate::error::Result;
use crate::fuzz::matcher::MatchConfig;
use crate::types::ResponseResult;

/// HTTP methods for method fuzzing.
const HTTP_METHODS: &[&str] = &[
    "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "TRACE",
];

/// Common headers for header fuzzing.
const FUZZ_HEADERS: &[(&str, &str)] = &[
    ("X-Forwarded-For", "127.0.0.1"),
    ("X-Forwarded-Host", "localhost"),
    ("X-Real-IP", "127.0.0.1"),
    ("X-Originating-IP", "127.0.0.1"),
    ("X-Remote-IP", "127.0.0.1"),
    ("X-Client-IP", "127.0.0.1"),
    ("X-API-Key", "FUZZ"),
    ("Authorization", "Bearer FUZZ"),
    ("Authorization", "Basic FUZZ"),
    ("Content-Type", "FUZZ"),
    ("Accept", "FUZZ"),
    ("Origin", "https://evil.com"),
    ("Referer", "https://evil.com/"),
];

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
        if let Some(ref proxy_url) = config.proxy
            && let Ok(proxy) = reqwest::Proxy::all(proxy_url)
        {
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
                let result = hit_get(&client, &url_str).await;
                if matcher.evaluate(&result) {
                    Some(result)
                } else {
                    None
                }
            }));
        }
        collect(handles).await
    }

    /// Fuzz query parameters using a wordlist.
    pub async fn fuzz_params(
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
            let url_str = format!(
                "{}?{}={}",
                base_url.to_string().trim_end_matches('/'),
                word,
                word
            );
            let matcher = matcher.clone();
            handles.push(tokio::spawn(async move {
                let _permit = permit;
                tokio::time::sleep(delay).await;
                let result = hit_get(&client, &url_str).await;
                if matcher.evaluate(&result) {
                    Some(result)
                } else {
                    None
                }
            }));
        }
        collect(handles).await
    }

    /// Fuzz HTTP methods using a wordlist (replaces the keyword in target URL).
    pub async fn fuzz_methods(
        &self,
        base_url: &reqwest::Url,
        words: &[String],
        matcher: &MatchConfig,
    ) -> Vec<ResponseResult> {
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::with_capacity(HTTP_METHODS.len() * words.len().max(1));
        let base_str = base_url.to_string();
        for method in HTTP_METHODS {
            for word in words {
                let permit = sem.clone().acquire_owned().await.unwrap();
                let client = self.client.clone();
                let delay = self.rate_delay;
                let url_str = base_str.trim_end_matches('/').to_string() + word;
                let method_str = method.to_string();
                let matcher = matcher.clone();
                handles.push(tokio::spawn(async move {
                    let _permit = permit;
                    tokio::time::sleep(delay).await;
                    let result = hit_method(&client, &url_str, &method_str).await;
                    if matcher.evaluate(&result) {
                        Some(result)
                    } else {
                        None
                    }
                }));
            }
        }
        collect(handles).await
    }

    /// Fuzz request headers using a wordlist.
    pub async fn fuzz_headers(
        &self,
        base_url: &reqwest::Url,
        words: &[String],
        matcher: &MatchConfig,
        keyword: &str,
    ) -> Vec<ResponseResult> {
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::with_capacity(FUZZ_HEADERS.len() * words.len().max(1));
        let base_str = base_url.to_string().trim_end_matches('/').to_string();
        for (header_name, default_value) in FUZZ_HEADERS {
            for word in words {
                let permit = sem.clone().acquire_owned().await.unwrap();
                let client = self.client.clone();
                let delay = self.rate_delay;
                let url_str = base_str.clone() + word;
                let hdr_name = header_name.to_string();
                let hdr_val = if default_value.contains(keyword) {
                    word.clone()
                } else {
                    default_value.replace(keyword, word)
                };
                let matcher = matcher.clone();
                handles.push(tokio::spawn(async move {
                    let _permit = permit;
                    tokio::time::sleep(delay).await;
                    let result = hit_with_header(&client, &url_str, &hdr_name, &hdr_val).await;
                    if matcher.evaluate(&result) {
                        Some(result)
                    } else {
                        None
                    }
                }));
            }
        }
        collect(handles).await
    }

    /// Fuzz request bodies using a wordlist.
    pub async fn fuzz_bodies(
        &self,
        base_url: &reqwest::Url,
        words: &[String],
        matcher: &MatchConfig,
        keyword: &str,
        request_file: Option<&Path>,
    ) -> Vec<ResponseResult> {
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::with_capacity(words.len());
        let base_str = base_url.to_string().trim_end_matches('/').to_string();
        // Load request body template if provided
        let template = request_file
            .and_then(|p| std::fs::read_to_string(p).ok())
            .unwrap_or_default();
        for word in words {
            let permit = sem.clone().acquire_owned().await.unwrap();
            let client = self.client.clone();
            let delay = self.rate_delay;
            let url_str = base_str.clone();
            let body = if template.is_empty() {
                format!("{{\"{}\":\"test\"}}", word)
            } else {
                template.replace(keyword, word)
            };
            let matcher = matcher.clone();
            handles.push(tokio::spawn(async move {
                let _permit = permit;
                tokio::time::sleep(delay).await;
                let result = hit_with_body(&client, &url_str, &body).await;
                if matcher.evaluate(&result) {
                    Some(result)
                } else {
                    None
                }
            }));
        }
        collect(handles).await
    }
}

async fn collect(
    handles: Vec<tokio::task::JoinHandle<Option<ResponseResult>>>,
) -> Vec<ResponseResult> {
    let mut results = Vec::new();
    for h in handles {
        if let Ok(Some(r)) = h.await {
            results.push(r);
        }
    }
    results
}

async fn hit_get(client: &reqwest::Client, url: &str) -> ResponseResult {
    let start = std::time::Instant::now();
    match client.get(url).send().await {
        Ok(resp) => make_result("GET", url, resp, start.elapsed().as_millis() as u64, None).await,
        Err(e) => error_result(
            "GET",
            url,
            start.elapsed().as_millis() as u64,
            &e.to_string(),
        ),
    }
}

async fn hit_method(client: &reqwest::Client, url: &str, method: &str) -> ResponseResult {
    use reqwest::Method;
    let m = Method::from_bytes(method.as_bytes()).unwrap_or(Method::GET);
    let start = std::time::Instant::now();
    let req = client.request(m, url);
    match req.send().await {
        Ok(resp) => make_result(method, url, resp, start.elapsed().as_millis() as u64, None).await,
        Err(e) => error_result(
            method,
            url,
            start.elapsed().as_millis() as u64,
            &e.to_string(),
        ),
    }
}

async fn hit_with_header(
    client: &reqwest::Client,
    url: &str,
    header_name: &str,
    header_val: &str,
) -> ResponseResult {
    let start = std::time::Instant::now();
    match client.get(url).header(header_name, header_val).send().await {
        Ok(resp) => make_result("GET", url, resp, start.elapsed().as_millis() as u64, None).await,
        Err(e) => error_result(
            "GET",
            url,
            start.elapsed().as_millis() as u64,
            &e.to_string(),
        ),
    }
}

async fn hit_with_body(client: &reqwest::Client, url: &str, body: &str) -> ResponseResult {
    let start = std::time::Instant::now();
    match client
        .post(url)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
    {
        Ok(resp) => {
            make_result(
                "POST",
                url,
                resp,
                start.elapsed().as_millis() as u64,
                Some(body.len()),
            )
            .await
        }
        Err(e) => error_result(
            "POST",
            url,
            start.elapsed().as_millis() as u64,
            &e.to_string(),
        ),
    }
}

async fn make_result(
    method: &str,
    url: &str,
    resp: reqwest::Response,
    time_ms: u64,
    body_size_hint: Option<usize>,
) -> ResponseResult {
    let status = resp.status().as_u16();
    let headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
        .collect();
    let body = resp.bytes().await.unwrap_or_default().to_vec();
    let size = body.len().max(body_size_hint.unwrap_or(0));
    ResponseResult {
        endpoint_method: method.into(),
        endpoint_url: url.into(),
        status_code: status,
        response_time_ms: time_ms,
        response_size: size,
        response_headers: headers,
        response_body: body,
        expected_status: None,
        timestamp: None,
        checks: vec![],
        error: None,
        tags: vec![],
        trackers: vec![],
    }
}

fn error_result(method: &str, url: &str, time_ms: u64, error: &str) -> ResponseResult {
    ResponseResult {
        endpoint_method: method.into(),
        endpoint_url: url.into(),
        status_code: 0,
        response_time_ms: time_ms,
        response_size: 0,
        response_headers: vec![],
        response_body: vec![],
        expected_status: None,
        timestamp: None,
        checks: vec![],
        error: Some(error.into()),
        tags: vec![],
        trackers: vec![],
    }
}
