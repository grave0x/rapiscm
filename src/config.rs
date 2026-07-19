//! Scan configuration, API keys, and output format parsing.

use std::path::PathBuf;
use std::time::Duration;

use crate::cli::{Cli, Command, CrawlMode};
use crate::error::{Error, Result};
use crate::types::{ApiKeys, AuthConfig, OutputFormat, Target};

/// Resolved, validated configuration from CLI args.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ScanConfig {
    pub target: Target,
    pub method: Option<String>,
    pub headers: Vec<(String, String)>,
    pub auth: Option<AuthConfig>,
    pub rate_limit: u64,
    pub timeout: Duration,
    pub concurrency: usize,
    pub output: OutputFormat,
    pub follow_redirects: bool,
    pub insecure: bool,
    pub paths: Vec<String>,
    pub tags: Vec<String>,
    pub filter_tag: Vec<String>,
    pub exclude_tag: Vec<String>,
    pub proxy: Option<String>,
    pub log_level: String,
    pub log_filter: Vec<String>,
    pub log_format: String,

    #[cfg(feature = "browser")]
    pub browser_kind: crate::scan::browser::BrowserKind,
    #[cfg(feature = "browser")]
    pub headed: bool,

    pub crawl_mode: Option<CrawlMode>,
    pub depth: usize,
    pub filter_path: Vec<String>,
    pub exclude_path: Vec<String>,
    pub filter_method: Vec<String>,
    pub exclude_method: Vec<String>,
    pub filter_status: Vec<String>,
    pub exclude_status: Vec<String>,
    pub filter: Vec<String>,
    pub exclude: Vec<String>,
    pub show_tags: bool,
    pub trackers: bool,
    pub tracker_report: bool,
    pub corp: Option<String>,

    // Task system fields.
    pub save: bool,
    pub task_name: Option<String>,
    pub task_tags: Vec<String>,
    pub no_bodies: bool,
    pub raw: bool,
    pub task_dir: Option<PathBuf>,
    pub git: bool,

    // Deep spec flag.
    pub deep_spec: bool,

    // Ghost mode fields.
    pub ghost: bool,
    pub jitter_pct: u32,
    pub ua_rotate: Option<String>,
    pub proxy_rotate: Vec<String>,
    pub eval_js: Option<String>,
}

/// Validate config values and log warnings for suspicious settings.
fn validate_config(rate_limit: u64, timeout: u64, concurrency: usize, jitter_pct: u32) {
    if rate_limit > 1000 {
        tracing::warn!(
            "rate_limit={rate_limit} is very high — may trigger rate limiting on target"
        );
    }
    if rate_limit == 0 {
        tracing::warn!("rate_limit=0 means unlimited requests — use with caution");
    }
    if timeout < 1 {
        tracing::warn!("timeout={timeout}s is too low — requests may fail");
    }
    if concurrency > 100 {
        tracing::warn!("concurrency={concurrency} is very high — may overwhelm target");
    }
    if jitter_pct > 80 {
        tracing::warn!(
            "jitter={jitter_pct}% is very high — request timing will vary significantly"
        );
    }
}

impl ScanConfig {
    /// Build a ScanConfig from GlobalArgs + a Target (for fuzz mode).
    pub fn from_cli_global(global: &crate::cli::GlobalArgs, target: Target) -> Result<Self> {
        let headers = parse_headers(&global.headers)?;
        let auth = parse_auth(global.auth.as_deref())?;
        let output = parse_output(&global.output)?;
        validate_config(
            global.rate_limit,
            global.timeout,
            global.concurrency,
            global.jitter,
        );
        Ok(ScanConfig {
            target,
            method: global.method.clone(),
            headers,
            auth,
            rate_limit: global.rate_limit,
            timeout: Duration::from_secs(global.timeout),
            concurrency: global.concurrency,
            output,
            follow_redirects: global.follow_redirects,
            insecure: global.insecure,
            paths: global.paths.clone(),
            tags: global.tags.clone(),
            filter_tag: global.filter_tag.clone(),
            exclude_tag: global.exclude_tag.clone(),
            proxy: global.proxy.clone(),
            log_level: global.log_level.clone(),
            log_filter: global.log_filter.clone(),
            log_format: global.log_format.clone(),
            #[cfg(feature = "browser")]
            browser_kind: crate::scan::browser::BrowserKind::Chrome,
            #[cfg(feature = "browser")]
            headed: global.headed,
            crawl_mode: global.crawl,
            depth: global.depth,
            filter_path: global.filter_path.clone(),
            exclude_path: global.exclude_path.clone(),
            filter_method: global.filter_method.clone(),
            exclude_method: global.exclude_method.clone(),
            filter_status: global.filter_status.clone(),
            exclude_status: global.exclude_status.clone(),
            filter: global.filter.clone(),
            exclude: global.exclude.clone(),
            show_tags: global.show_tags,
            trackers: !global.no_trackers,
            tracker_report: global.tracker_report,
            corp: global.corp.clone(),
            save: global.save,
            task_name: global.task_name.clone(),
            task_tags: global.task_tag.clone(),
            no_bodies: global.no_bodies,
            raw: global.raw,
            task_dir: global.task_dir.clone(),
            git: global.git,
            deep_spec: global.deep_spec,
            ghost: global.ghost,
            jitter_pct: global.jitter,
            ua_rotate: global.ua_rotate.clone(),
            proxy_rotate: global.proxy_rotate.clone(),
            eval_js: global.eval.clone(),
        })
    }

    pub fn from_cli(cli: Cli) -> Result<Self> {
        let (target, global) = match cli.command {
            Command::Spec { file, global } => {
                if !file.exists() {
                    return Err(Error::SpecFileNotFound(file));
                }
                (Target::Spec(file), global)
            }
            Command::Url { url, global } => {
                let parsed = parse_url(&url)?;
                (Target::Url(parsed), global)
            }
            Command::Scan { target, global } => {
                // auto-detect: if looks like a file path with known extension → spec, else URL
                let path = PathBuf::from(&target);
                if path
                    .extension()
                    .is_some_and(|ext| matches!(ext.to_str(), Some("json" | "yaml" | "yml")))
                {
                    if !path.exists() {
                        return Err(Error::SpecFileNotFound(path));
                    }
                    (Target::Spec(path), global)
                } else {
                    let parsed = parse_url(&target)?;
                    (Target::Url(parsed), global)
                }
            }
            Command::Fuzz { target, global, .. } => {
                let parsed = parse_url(&target)?;
                (Target::Url(parsed), global)
            }
            Command::Corp { .. } => unreachable!("corp mode handled separately in main"),
            Command::Session { .. } => unreachable!("session mode handled separately in main"),
            Command::Tasks { .. } => unreachable!("tasks mode handled separately in main"),
            Command::Capture { url, global, .. } => {
                let parsed = parse_url(&url)?;
                (Target::Url(parsed), global)
            }
        };

        let headers = parse_headers(&global.headers)?;
        let auth = parse_auth(global.auth.as_deref())?;
        let output = parse_output(&global.output)?;

        Ok(ScanConfig {
            target,
            method: global.method,
            headers,
            auth,
            rate_limit: global.rate_limit,
            timeout: Duration::from_secs(global.timeout),
            concurrency: global.concurrency,
            output,
            follow_redirects: global.follow_redirects,
            insecure: global.insecure,
            paths: global.paths,
            tags: global.tags,
            filter_tag: global.filter_tag,
            exclude_tag: global.exclude_tag,
            proxy: global.proxy.clone(),
            log_level: global.log_level.clone(),
            log_filter: global.log_filter.clone(),
            log_format: global.log_format.clone(),

            #[cfg(feature = "browser")]
            browser_kind: match global.browser.as_str() {
                "firefox" => crate::scan::browser::BrowserKind::Firefox,
                _ => crate::scan::browser::BrowserKind::Chrome,
            },
            #[cfg(feature = "browser")]
            headed: global.headed,
            crawl_mode: global.crawl,
            depth: global.depth,
            filter_path: global.filter_path,
            exclude_path: global.exclude_path,
            filter_method: global.filter_method,
            exclude_method: global.exclude_method,
            filter_status: global.filter_status,
            exclude_status: global.exclude_status,
            filter: global.filter,
            exclude: global.exclude,
            show_tags: global.show_tags,
            trackers: !global.no_trackers,
            tracker_report: global.tracker_report,
            corp: global.corp,
            save: global.save,
            task_name: global.task_name,
            task_tags: global.task_tag,
            no_bodies: global.no_bodies,
            raw: global.raw,
            task_dir: global.task_dir,
            git: global.git,
            deep_spec: global.deep_spec,
            ghost: global.ghost,
            jitter_pct: global.jitter,
            ua_rotate: global.ua_rotate,
            proxy_rotate: global.proxy_rotate,
            eval_js: global.eval,
        })
    }
}

/// Load API keys from ~/.config/rapiscm/config.toml.
/// Returns default (all None) if file missing or unreadable.
pub fn load_config() -> ApiKeys {
    let path = config_path();
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return ApiKeys::default(),
    };

    let table: toml::Value = match content.parse() {
        Ok(t) => t,
        Err(_) => return ApiKeys::default(),
    };

    let keys = table.get("api_keys");
    ApiKeys {
        google_api_key: keys
            .and_then(|k| k.get("google_api_key"))
            .and_then(|v| v.as_str())
            .map(String::from),
        google_cx: keys
            .and_then(|k| k.get("google_cx"))
            .and_then(|v| v.as_str())
            .map(String::from),
        shodan_api_key: keys
            .and_then(|k| k.get("shodan_api_key"))
            .and_then(|v| v.as_str())
            .map(String::from),
    }
}

fn config_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config/rapiscm/config.toml")
}

/// Parse a URL string, trying `https://` prefix if no scheme is present.
fn parse_url(raw: &str) -> Result<reqwest::Url> {
    if raw.contains("://") {
        reqwest::Url::parse(raw).map_err(|e| Error::InvalidUrl(e.to_string()))
    } else {
        let with_scheme = format!("https://{raw}");
        reqwest::Url::parse(&with_scheme).map_err(|e| Error::InvalidUrl(e.to_string()))
    }
}

fn parse_headers(raw: &[String]) -> Result<Vec<(String, String)>> {
    raw.iter()
        .map(|h| {
            let mut parts = h.splitn(2, ':');
            let name = parts
                .next()
                .ok_or_else(|| Error::InvalidHeaderFormat("empty header".into()))?
                .trim()
                .to_string();
            let value = parts
                .next()
                .ok_or_else(|| Error::InvalidHeaderFormat(format!("header {name} has no value")))?
                .trim()
                .to_string();
            Ok((name, value))
        })
        .collect()
}

fn parse_auth(raw: Option<&str>) -> Result<Option<AuthConfig>> {
    let Some(s) = raw else { return Ok(None) };
    let mut parts = s.splitn(2, ':');
    let scheme = parts.next().unwrap_or("");
    let rest = parts.next().unwrap_or("");

    match scheme {
        "bearer" => Ok(Some(AuthConfig::Bearer(rest.to_string()))),
        "basic" => {
            let mut creds = rest.splitn(2, ':');
            let username = creds.next().unwrap_or("").to_string();
            let password = creds.next().unwrap_or("").to_string();
            Ok(Some(AuthConfig::Basic { username, password }))
        }
        "header" => {
            let mut kv = rest.splitn(2, ':');
            let name = kv.next().unwrap_or("").to_string();
            let value = kv.next().unwrap_or("").to_string();
            Ok(Some(AuthConfig::Header { name, value }))
        }
        other => Err(Error::InvalidAuthFormat(format!(
            "unknown auth scheme '{other}'. Use bearer:<token>, basic:<user:pass>, or header:<name:value>"
        ))),
    }
}

pub fn parse_output(raw: &str) -> Result<OutputFormat> {
    match raw {
        "table" => Ok(OutputFormat::Table),
        "json" => Ok(OutputFormat::Json),
        "md" => Ok(OutputFormat::Markdown),
        "doc" => Ok(OutputFormat::Doc),
        other => Err(Error::InvalidOutputFormat(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_auth_bearer() {
        let result = parse_auth(Some("bearer:tok123")).unwrap().unwrap();
        assert!(matches!(result, AuthConfig::Bearer(t) if t == "tok123"));
    }

    #[test]
    fn test_parse_auth_basic() {
        let result = parse_auth(Some("basic:admin:hunter2")).unwrap().unwrap();
        assert!(
            matches!(result, AuthConfig::Basic { ref username, ref password }
            if username == "admin" && password == "hunter2")
        );
    }

    #[test]
    fn test_parse_auth_header() {
        let result = parse_auth(Some("header:X-API-Key:secret123"))
            .unwrap()
            .unwrap();
        assert!(matches!(result, AuthConfig::Header { ref name, ref value }
            if name == "X-API-Key" && value == "secret123"));
    }

    #[test]
    fn test_parse_auth_none() {
        assert!(parse_auth(None).unwrap().is_none());
    }

    #[test]
    fn test_parse_auth_invalid_scheme() {
        assert!(parse_auth(Some("unknown:value")).is_err());
    }

    #[test]
    fn test_parse_output() {
        assert_eq!(parse_output("table").unwrap(), OutputFormat::Table);
        assert_eq!(parse_output("json").unwrap(), OutputFormat::Json);
        assert_eq!(parse_output("md").unwrap(), OutputFormat::Markdown);
        assert!(parse_output("csv").is_err());
    }

    #[test]
    fn test_parse_url_with_scheme() {
        let url = parse_url("https://api.example.com").unwrap();
        assert_eq!(url.as_str(), "https://api.example.com/");
    }

    #[test]
    fn test_parse_url_without_scheme() {
        let url = parse_url("api.example.com").unwrap();
        assert_eq!(url.as_str(), "https://api.example.com/");
    }

    #[test]
    fn test_parse_url_invalid() {
        assert!(parse_url("").is_err());
    }

    #[test]
    fn test_parse_headers() {
        let raw = vec![
            "Content-Type: application/json".into(),
            "X-Custom: val".into(),
        ];
        let result = parse_headers(&raw).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result[0],
            ("Content-Type".into(), "application/json".into())
        );
    }

    #[test]
    fn test_load_config_default() {
        let keys = load_config();
        // Should return default regardless of whether file exists
        assert!(keys.google_api_key.is_none());
    }
}
