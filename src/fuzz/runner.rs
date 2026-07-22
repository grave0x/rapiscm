//! Core fuzz runner: dispatches path/param/method/header/body fuzz loops.
//!
//! All modes use keyword-based URL construction: if the base URL contains
//! the keyword (default: "FUZZ"), each word replaces it. Otherwise, words
//! are appended to the URL path.

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

/// Common headers for header fuzzing. "FUZZ" in the value is replaced with the word.
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
    ("Content-Type", "application/FUZZ"),
    ("Accept", "application/FUZZ"),
    ("Origin", "https://FUZZ.evil.com"),
    ("Referer", "https://FUZZ.evil.com/"),
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

    /// Build URLs by replacing keyword in the base URL with each word.
    /// Falls back to path-appending if the URL doesn't contain the keyword.
    fn build_urls(base_url: &reqwest::Url, words: &[String], keyword: &str) -> Vec<String> {
        let base_str = base_url.to_string();
        let base_no_trailing = base_str.trim_end_matches('/');

        if base_str.contains(keyword) {
            words
                .iter()
                .map(|w| base_str.replace(keyword, w))
                .collect()
        } else {
            words
                .iter()
                .map(|w| format!("{base_no_trailing}{w}"))
                .collect()
        }
    }

    /// Fuzz path endpoints — replaces keyword in URL path or appends words.
    pub async fn fuzz_paths(
        &self,
        base_url: &reqwest::Url,
        words: &[String],
        matcher: &MatchConfig,
        keyword: &str,
    ) -> Vec<ResponseResult> {
        let urls = Self::build_urls(base_url, words, keyword);
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::with_capacity(urls.len());
        for url_str in urls {
            let permit = sem.clone().acquire_owned().await.unwrap();
            let client = self.client.clone();
            let delay = self.rate_delay;
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

    /// Fuzz query parameters — appends `?word=value` to keyword-replaced URLs.
    pub async fn fuzz_params(
        &self,
        base_url: &reqwest::Url,
        words: &[String],
        matcher: &MatchConfig,
        keyword: &str,
    ) -> Vec<ResponseResult> {
        let urls = Self::build_urls(base_url, words, keyword);
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::with_capacity(words.len() * urls.len().max(1));
        for url_str in &urls {
            for word in words {
                let permit = sem.clone().acquire_owned().await.unwrap();
                let client = self.client.clone();
                let delay = self.rate_delay;
                let param_url = format!("{url_str}?{word}=FUZZ");
                let matcher = matcher.clone();
                handles.push(tokio::spawn(async move {
                    let _permit = permit;
                    tokio::time::sleep(delay).await;
                    let result = hit_get(&client, &param_url).await;
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

    /// Fuzz HTTP methods — tries each method on each keyword-replaced URL.
    pub async fn fuzz_methods(
        &self,
        base_url: &reqwest::Url,
        words: &[String],
        matcher: &MatchConfig,
        keyword: &str,
    ) -> Vec<ResponseResult> {
        let urls = Self::build_urls(base_url, words, keyword);
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::with_capacity(HTTP_METHODS.len() * urls.len());
        for url_str in &urls {
            for method in HTTP_METHODS {
                let permit = sem.clone().acquire_owned().await.unwrap();
                let client = self.client.clone();
                let delay = self.rate_delay;
                let url = url_str.clone();
                let m = method.to_string();
                let matcher = matcher.clone();
                handles.push(tokio::spawn(async move {
                    let _permit = permit;
                    tokio::time::sleep(delay).await;
                    let result = hit_method(&client, &url, &m).await;
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

    /// Fuzz request headers — replaces keyword in header values, sends to keyword URLs.
    pub async fn fuzz_headers(
        &self,
        base_url: &reqwest::Url,
        words: &[String],
        matcher: &MatchConfig,
        keyword: &str,
    ) -> Vec<ResponseResult> {
        let urls = Self::build_urls(base_url, words, keyword);
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::with_capacity(FUZZ_HEADERS.len() * urls.len());
        for url_str in &urls {
            for (header_name, header_template) in FUZZ_HEADERS {
                for word in words {
                    let permit = sem.clone().acquire_owned().await.unwrap();
                    let client = self.client.clone();
                    let delay = self.rate_delay;
                    let url = url_str.clone();
                    let hdr = header_name.to_string();
                    let val = header_template.replace(keyword, word);
                    let matcher = matcher.clone();
                    handles.push(tokio::spawn(async move {
                        let _permit = permit;
                        tokio::time::sleep(delay).await;
                        let result =
                            hit_with_header(&client, &url, &hdr, &val).await;
                        if matcher.evaluate(&result) {
                            Some(result)
                        } else {
                            None
                        }
                    }));
                }
            }
        }
        collect(handles).await
    }

    /// Fuzz request bodies — POSTs word-based bodies to keyword-replaced URLs.
    pub async fn fuzz_bodies(
        &self,
        base_url: &reqwest::Url,
        words: &[String],
        matcher: &MatchConfig,
        keyword: &str,
        request_file: Option<&Path>,
    ) -> Vec<ResponseResult> {
        let urls = Self::build_urls(base_url, words, keyword);
        let sem = Arc::new(Semaphore::new(self.concurrency));
        let mut handles = Vec::with_capacity(words.len() * urls.len());
        let template = request_file
            .and_then(|p| std::fs::read_to_string(p).ok())
            .unwrap_or_default();
        for url_str in &urls {
            for word in words {
                let permit = sem.clone().acquire_owned().await.unwrap();
                let client = self.client.clone();
                let delay = self.rate_delay;
                let url = url_str.clone();
                let body = if template.is_empty() {
                    format!("{{\"{word}\":\"test\"}}")
                } else {
                    template.replace(keyword, word)
                };
                let matcher = matcher.clone();
                handles.push(tokio::spawn(async move {
                    let _permit = permit;
                    tokio::time::sleep(delay).await;
                    let result = hit_with_body(&client, &url, &body).await;
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
        Ok(resp) => {
            make_result("GET", url, resp, start.elapsed().as_millis() as u64, None).await
        }
        Err(e) => error_result("GET", url, start.elapsed().as_millis() as u64, &e.to_string()),
    }
}

async fn hit_method(client: &reqwest::Client, url: &str, method: &str) -> ResponseResult {
    use reqwest::Method;
    let m = Method::from_bytes(method.as_bytes()).unwrap_or(Method::GET);
    let start = std::time::Instant::now();
    let req = client.request(m, url);
    match req.send().await {
        Ok(resp) => {
            make_result(method, url, resp, start.elapsed().as_millis() as u64, None).await
        }
        Err(e) => {
            error_result(method, url, start.elapsed().as_millis() as u64, &e.to_string())
        }
    }
}

async fn hit_with_header(
    client: &reqwest::Client,
    url: &str,
    header_name: &str,
    header_val: &str,
) -> ResponseResult {
    let start = std::time::Instant::now();
    match client
        .get(url)
        .header(header_name, header_val)
        .send()
        .await
    {
        Ok(resp) => {
            make_result("GET", url, resp, start.elapsed().as_millis() as u64, None).await
        }
        Err(e) => {
            error_result("GET", url, start.elapsed().as_millis() as u64, &e.to_string())
        }
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
            make_result("POST", url, resp, start.elapsed().as_millis() as u64, Some(body.len()))
                .await
        }
        Err(e) => error_result("POST", url, start.elapsed().as_millis() as u64, &e.to_string()),
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
