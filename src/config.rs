use std::path::PathBuf;
use std::time::Duration;

use crate::cli::{Cli, Command};
use crate::error::{Error, Result};
use crate::types::{AuthConfig, OutputFormat, Target};

/// Resolved, validated configuration from CLI args.
#[derive(Debug, Clone)]
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
}

impl ScanConfig {
    /// Build a ScanConfig from GlobalArgs + a Target (for fuzz mode).
    pub fn from_cli_global(global: &crate::cli::GlobalArgs, target: Target) -> Result<Self> {
        let headers = parse_headers(&global.headers)?;
        let auth = parse_auth(global.auth.as_deref())?;
        let output = parse_output(&global.output)?;
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
        })
    }
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

fn parse_output(raw: &str) -> Result<OutputFormat> {
    match raw {
        "table" => Ok(OutputFormat::Table),
        "json" => Ok(OutputFormat::Json),
        "md" => Ok(OutputFormat::Markdown),
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
}
