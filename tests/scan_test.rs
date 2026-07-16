use std::path::PathBuf;

use wiremock::matchers::{method, path as mock_path, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

use rapiscm::config::ScanConfig;
use rapiscm::types::{OutputFormat, Target};

fn base_config(port: u16) -> ScanConfig {
    ScanConfig {
        target: Target::Url(reqwest::Url::parse(&format!("http://127.0.0.1:{port}")).unwrap()),
        method: None,
        headers: vec![],
        auth: None,
        rate_limit: 10000,
        timeout: std::time::Duration::from_secs(5),
        concurrency: 10,
        output: OutputFormat::Json,
        follow_redirects: false,
        insecure: false,
        paths: vec![],
        tags: vec![],
        filter_tag: vec![],
        exclude_tag: vec![],
        proxy: None,
        log_level: "off".into(),
        log_filter: vec![],
        log_format: "text".into(),
        #[cfg(feature = "browser")]
        browser_kind: rapiscm::scan::browser::BrowserKind::Chrome,
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
        corp: None,
    }
}

async fn setup_mock(html: &str) -> MockServer {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(mock_path("/"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(html.as_bytes().to_vec(), "text/html".into()),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(mock_path("/api/users"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"users":[]}"#))
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(mock_path("/api/health"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("ok")
                .insert_header("content-security-policy", "default-src 'self'")
                .insert_header("strict-transport-security", "max-age=31536000")
                .insert_header("x-content-type-options", "nosniff")
                .insert_header("x-frame-options", "DENY")
                .insert_header("cache-control", "no-store"),
        )
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(mock_path("/api/login"))
        .respond_with(ResponseTemplate::new(405))
        .mount(&server)
        .await;

    server
}

#[tokio::test]
async fn test_url_mode_discovers_endpoints() {
    let html = include_str!("fixtures/test-page.html");
    let server = setup_mock(html).await;
    let port = server.address().port();

    let config = base_config(port);
    let results = rapiscm::scan::url::run_url_scan(&config).await.unwrap();

    eprintln!("discovered {} results", results.len());
    for r in &results {
        eprintln!("  {} {}", r.endpoint_method, r.endpoint_url);
    }
    assert!(!results.is_empty(), "should discover endpoints");

    let urls: Vec<&str> = results.iter().map(|r| r.endpoint_url.as_str()).collect();
    assert!(
        urls.iter().any(|u| u.contains("/api/users")),
        "should find /api/users"
    );
    assert!(
        urls.iter().any(|u| u.contains("/api/health")),
        "should find /api/health"
    );
    assert!(
        urls.iter().any(|u| u.contains("/api/login")),
        "should find /api/login"
    );
}

#[tokio::test]
async fn test_url_mode_runs_checks() {
    let html = include_str!("fixtures/test-page.html");
    let server = setup_mock(html).await;
    let port = server.address().port();

    let config = base_config(port);
    let results = rapiscm::scan::url::run_url_scan(&config).await.unwrap();

    let health = results
        .iter()
        .find(|r| r.endpoint_url.contains("/api/health"));
    assert!(health.is_some(), "should have scanned /api/health");

    let health = health.unwrap();
    let check_names: Vec<&str> = health.checks.iter().map(|c| c.name.as_str()).collect();
    assert!(check_names.contains(&"CSP"), "should check CSP");
    assert!(check_names.contains(&"HSTS"), "should check HSTS");
}

#[tokio::test]
async fn test_spec_mode_parses_endpoints() {
    let spec_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("test-api.json");

    let endpoints = rapiscm::parser::spec::parse_spec_file(&spec_path).unwrap();
    assert_eq!(endpoints.len(), 4, "should parse 4 endpoints from spec");

    let get_health = endpoints
        .iter()
        .find(|e| e.url.path() == "/health" && e.method == reqwest::Method::GET);
    assert!(get_health.is_some(), "should have GET /health");

    let get_users = endpoints
        .iter()
        .find(|e| e.url.path() == "/users" && e.method == reqwest::Method::GET);
    assert!(get_users.is_some(), "should have GET /users");

    let post_users = endpoints
        .iter()
        .find(|e| e.url.path() == "/users" && e.method == reqwest::Method::POST);
    assert!(post_users.is_some(), "should have POST /users");

    let get_user_id = endpoints
        .iter()
        .find(|e| e.url.path().contains("/users/42") && e.method == reqwest::Method::GET);
    assert!(
        get_user_id.is_some(),
        "should have GET /users/42 with param filled"
    );
}

#[tokio::test]
async fn test_spec_mode_full_scan() {
    use std::io::Write;

    let server = MockServer::start().await;
    let port = server.address().port();

    // Write spec with correct server URL pointing to mock
    let spec_content = format!(
        r#"{{
          "openapi": "3.0.0",
          "info": {{ "title": "Test", "version": "1" }},
          "servers": [{{ "url": "http://127.0.0.1:{}" }}],
          "paths": {{
            "/health": {{ "get": {{ "responses": {{ "200": {{ "description": "OK" }} }} }} }},
            "/users": {{
              "get": {{ "responses": {{ "200": {{ "description": "list" }} }} }},
              "post": {{ "responses": {{ "201": {{ "description": "created" }} }} }}
            }},
            "/users/{{id}}": {{
              "get": {{
                "parameters": [{{ "in": "path", "name": "id", "required": true, "schema": {{ "type": "integer" }}, "example": 42 }}],
                "responses": {{ "200": {{ "description": "detail" }} }}
              }}
            }}
          }}
        }}"#,
        port
    );

    let mut tmp = tempfile::NamedTempFile::new().unwrap();
    write!(tmp, "{}", spec_content).unwrap();
    let spec_path = tmp.into_temp_path();

    Mock::given(method("GET"))
        .and(mock_path("/health"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(mock_path("/users"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"users":[]}"#))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(mock_path("/users"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path_regex("/users/\\d+"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"id":42,"name":"test"}"#))
        .mount(&server)
        .await;

    let config = ScanConfig {
        target: Target::Spec(spec_path.to_path_buf()),
        ..base_config(port)
    };

    let results = rapiscm::scan::spec::run_spec_scan(&config).await.unwrap();
    assert_eq!(results.len(), 4, "all 4 spec endpoints should be scanned");

    // GET endpoints return 200, POST /users returns 201
    for r in &results {
        assert!(
            (200..=299).contains(&r.status_code),
            "{} should be 2xx, got {}",
            r.endpoint_url,
            r.status_code
        );
    }
}
