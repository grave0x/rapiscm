//! Endpoint and result filtering based on scan configuration.

/// Filter engine — applies include/exclude rules to endpoints and results.
use crate::config::ScanConfig;
use crate::fuzz::matcher::Range;
use crate::types::{Endpoint, ResponseResult};

/// Check whether an endpoint passes all configured filters.
pub fn endpoint_passes(ep: &Endpoint, config: &ScanConfig) -> bool {
    // Method filters
    let method = ep.method.as_str();
    if !config.filter_method.is_empty()
        && !config
            .filter_method
            .iter()
            .any(|m| m.eq_ignore_ascii_case(method))
    {
        return false;
    }
    if config
        .exclude_method
        .iter()
        .any(|m| m.eq_ignore_ascii_case(method))
    {
        return false;
    }

    // Path filters (simple substring match for now, could upgrade to glob)
    let path = ep.url.path();
    if !config.filter_path.is_empty()
        && !config.filter_path.iter().any(|p| path.contains(p.as_str()))
    {
        return false;
    }
    if config
        .exclude_path
        .iter()
        .any(|p| path.contains(p.as_str()))
    {
        return false;
    }

    // Tag filters
    if !config.filter_tag.is_empty() && !config.filter_tag.iter().all(|t| ep.tags.contains(t)) {
        return false;
    }
    if config.exclude_tag.iter().any(|t| ep.tags.contains(t)) {
        return false;
    }

    true
}

/// Filter endpoints in-place using all configured filters.
pub fn filter_endpoints(endpoints: &mut Vec<Endpoint>, config: &ScanConfig) {
    endpoints.retain(|ep| endpoint_passes(ep, config));
}

/// Check whether a response result passes status filters.
pub fn result_passes(r: &ResponseResult, config: &ScanConfig) -> bool {
    let status = r.status_code as u64;
    // Filter-status: include only matching
    if !config.filter_status.is_empty() {
        let ranges: Vec<Range> = config
            .filter_status
            .iter()
            .flat_map(|s| crate::fuzz::matcher::parse_range_list(s))
            .collect();
        if !ranges.is_empty() && !ranges.iter().any(|range| range.contains(status)) {
            return false;
        }
    }
    // Exclude-status
    if !config.exclude_status.is_empty() {
        let ranges: Vec<Range> = config
            .exclude_status
            .iter()
            .flat_map(|s| crate::fuzz::matcher::parse_range_list(s))
            .collect();
        if ranges.iter().any(|range| range.contains(status)) {
            return false;
        }
    }
    true
}

/// Filter response results in-place by status.
pub fn filter_results(results: &mut Vec<ResponseResult>, config: &ScanConfig) {
    results.retain(|r| result_passes(r, config));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::GlobalArgs;
    use crate::types::Target;
    use reqwest::Url;

    fn test_ep(method: &str, path: &str) -> Endpoint {
        Endpoint {
            method: reqwest::Method::from_bytes(method.as_bytes()).unwrap(),
            url: Url::parse(&format!("https://example.com{path}")).unwrap(),
            headers: vec![],
            body: None,
            expected_status: None,
            tags: vec![],
        }
    }

    /// Minimal GlobalArgs for testing.
    fn make_args() -> GlobalArgs {
        GlobalArgs {
            method: None,
            headers: vec![],
            auth: None,
            rate_limit: 50,
            timeout: 5,
            concurrency: 10,
            output: "table".into(),
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
            no_trackers: false,
            corp: None,
            save: false,
            task_name: None,
            task_tag: vec![],
            no_bodies: false,
            raw: false,
            task_dir: None,
            resume: None,
            git: false,
            report: None,
        }
    }

    fn config_with(f: impl FnOnce(&mut GlobalArgs)) -> ScanConfig {
        let mut g = make_args();
        f(&mut g);
        ScanConfig::from_cli_global(&g, Target::Url(Url::parse("https://example.com").unwrap()))
            .unwrap()
    }

    #[test]
    fn test_filter_method() {
        let mut eps = vec![test_ep("GET", "/api/users"), test_ep("POST", "/api/users")];
        let config = config_with(|g| g.filter_method = vec!["GET".into()]);
        filter_endpoints(&mut eps, &config);
        assert_eq!(eps.len(), 1);
        assert_eq!(eps[0].method.as_str(), "GET");
    }

    #[test]
    fn test_exclude_method() {
        let mut eps = vec![test_ep("GET", "/api/users"), test_ep("POST", "/api/users")];
        let config = config_with(|g| g.exclude_method = vec!["POST".into()]);
        filter_endpoints(&mut eps, &config);
        assert_eq!(eps.len(), 1);
        assert_eq!(eps[0].method.as_str(), "GET");
    }

    #[test]
    fn test_filter_path() {
        let mut eps = vec![test_ep("GET", "/api/users"), test_ep("GET", "/api/admin")];
        let config = config_with(|g| g.filter_path = vec!["/api/users".into()]);
        filter_endpoints(&mut eps, &config);
        assert_eq!(eps.len(), 1);
    }

    #[test]
    fn test_exclude_path() {
        let mut eps = vec![test_ep("GET", "/api/users"), test_ep("GET", "/api/admin")];
        let config = config_with(|g| g.exclude_path = vec!["admin".into()]);
        filter_endpoints(&mut eps, &config);
        assert_eq!(eps.len(), 1);
    }
}
