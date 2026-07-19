//! OpenAPI spec file parser (JSON/YAML) — extracts endpoints.

use std::path::Path;

use openapiv3::{OpenAPI, Parameter, ReferenceOr, Server};
use tracing::warn;

use crate::error::{Error, Result};
use crate::types::Endpoint;

/// Parse an OpenAPI spec file into a list of endpoints.
pub fn parse_spec_file(path: &Path) -> Result<Vec<Endpoint>> {
    let content = std::fs::read_to_string(path)?;
    let spec: OpenAPI = match path.extension().and_then(|e| e.to_str()) {
        Some("yaml") | Some("yml") => {
            serde_yaml::from_str(&content).map_err(|e| Error::SpecParse(e.to_string()))?
        }
        _ => serde_json::from_str(&content).map_err(|e| Error::SpecParse(e.to_string()))?,
    };
    endpoints_from_spec(&spec)
}

fn endpoints_from_spec(spec: &OpenAPI) -> Result<Vec<Endpoint>> {
    let base_url = resolve_base_url(&spec.servers);

    let mut endpoints: Vec<Endpoint> = Vec::new();

    for (path_template, method, op) in spec.operations() {
        // Collect path parameters from the operation and its path item.
        // We re-collect per operation to keep it simple.
        let mut param_examples: Vec<(String, String)> = Vec::new();
        let mut header_map: Vec<(String, String)> = Vec::new();

        // We don't have easy access to the PathItem here, just the Operation.
        // Collect params from the operation.
        for param_ref in &op.parameters {
            let param = match param_ref {
                ReferenceOr::Item(p) => p,
                ReferenceOr::Reference { reference } => {
                    warn!("unresolved param $ref: {reference}");
                    continue;
                }
            };
            match param {
                Parameter::Path {
                    parameter_data: data,
                    ..
                } => {
                    let example = resolve_param_example(data);
                    param_examples.push((data.name.clone(), example));
                }
                Parameter::Header {
                    parameter_data: data,
                    ..
                } => {
                    if let Some(example) = &data.example
                        && let Some(s) = example.as_str()
                    {
                        header_map.push((data.name.clone(), s.to_string()));
                    }
                }
                _ => {} // skip query, cookie for now
            }
        }

        // Build URL: base_url + path_template
        let mut url_str = format!(
            "{}/{}",
            base_url.trim_end_matches('/'),
            path_template.trim_start_matches('/')
        );

        // Sort by name length descending to avoid partial replacements
        param_examples.sort_by_key(|b| std::cmp::Reverse(b.0.len()));
        for (name, value) in &param_examples {
            url_str = url_str.replace(&format!("{{{name}}}"), value);
        }

        // Fill remaining path params with type-based placeholders
        url_str = fill_remaining_params(&url_str);

        // Skip if URL is relative (spec has no server URL).
        if !url_str.starts_with("http://") && !url_str.starts_with("https://") {
            warn!("skipping relative URL {url_str} — spec has no server URL");
            continue;
        }

        let url = match reqwest::Url::parse(&url_str) {
            Ok(u) => u,
            Err(e) => {
                warn!("skipping invalid URL {url_str}: {e}");
                continue;
            }
        };

        let method_str = method.to_uppercase();
        let method = match reqwest::Method::from_bytes(method_str.as_bytes()) {
            Ok(m) => m,
            Err(e) => {
                warn!("skipping invalid method {method_str}: {e}");
                continue;
            }
        };

        let expected_status = extract_expected_status(op);

        endpoints.push(Endpoint {
            method,
            url,
            headers: header_map,
            body: None, // body generation deferred to later phase
            expected_status,
            tags: vec![],
        });
    }

    Ok(endpoints)
}

/// Resolve the first server URL, substituting variables with defaults.
fn resolve_base_url(servers: &[Server]) -> String {
    let Some(server) = servers.first() else {
        return "/".into();
    };
    let mut url = server.url.clone();
    if let Some(vars) = &server.variables {
        for (name, var) in vars {
            url = url.replace(&format!("{{{name}}}"), &var.default);
        }
    }
    url
}

/// Get an example value for a parameter.
fn resolve_param_example(data: &openapiv3::ParameterData) -> String {
    if let Some(ex) = &data.example {
        if let Some(s) = ex.as_str() {
            return s.to_string();
        }
        if let Some(n) = ex.as_i64() {
            return n.to_string();
        }
        if let Some(f) = ex.as_f64() {
            return f.to_string();
        }
        return ex.to_string();
    }
    // Fall back to schema type
    if let openapiv3::ParameterSchemaOrContent::Schema(schema_ref) = &data.format
        && let ReferenceOr::Item(schema) = schema_ref
        && let openapiv3::SchemaKind::Type(t) = &schema.schema_kind
    {
        return match t {
            openapiv3::Type::String { .. } => "string".into(),
            openapiv3::Type::Integer { .. } => "1".into(),
            openapiv3::Type::Number { .. } => "1.0".into(),
            openapiv3::Type::Boolean { .. } => "true".into(),
            openapiv3::Type::Array { .. } => "[]".into(),
            openapiv3::Type::Object { .. } => "{}".into(),
        };
    }
    // Last resort
    "example".into()
}

/// Extract the first concrete success status code from an operation's responses.
fn extract_expected_status(op: &openapiv3::Operation) -> Option<u16> {
    use openapiv3::StatusCode;
    for (code, _) in &op.responses.responses {
        match code {
            StatusCode::Code(n) if (200..=299).contains(n) => return Some(*n),
            _ => continue,
        }
    }
    None
}

/// Replace any remaining `{param}` placeholders with sensible defaults.
fn fill_remaining_params(url: &str) -> String {
    let mut result = String::with_capacity(url.len());
    let mut rest = url;
    while let Some(start) = rest.find('{') {
        result.push_str(&rest[..start]);
        let after_open = &rest[start + 1..];
        if let Some(end) = after_open.find('}') {
            let name = &after_open[..end];
            let value = if name.to_lowercase().contains("id") {
                "123"
            } else {
                "example"
            };
            result.push_str(value);
            rest = &after_open[end + 1..];
        } else {
            result.push('{');
            rest = after_open;
        }
    }
    result.push_str(rest);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_invalid_json_spec() {
        let dir = std::env::temp_dir();
        let path = dir.join("rapiscm_test_invalid.json");
        std::fs::write(&path, "not valid json").unwrap();
        let result = parse_spec_file(&path);
        assert!(result.is_err());
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_parse_minimal_spec_no_paths() {
        let dir = std::env::temp_dir();
        let path = dir.join("rapiscm_test_minimal.json");
        std::fs::write(
            &path,
            r#"{"openapi":"3.0.0","info":{"title":"test","version":"1.0"},"paths":{}}"#,
        )
        .unwrap();
        let result = parse_spec_file(&path);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn test_fill_remaining_params() {
        assert_eq!(fill_remaining_params("/api/users/{id}"), "/api/users/123");
        assert_eq!(
            fill_remaining_params("/api/{version}/items/{itemId}"),
            "/api/example/items/123"
        );
        assert_eq!(fill_remaining_params("/no-params"), "/no-params");
    }

    #[test]
    fn test_resolve_base_url_no_vars() {
        let servers = vec![Server {
            url: "https://api.example.com/v3".into(),
            ..Default::default()
        }];
        assert_eq!(resolve_base_url(&servers), "https://api.example.com/v3");
    }

    #[test]
    fn test_resolve_base_url_with_vars() {
        let servers: Vec<Server> =
            serde_json::from_str(r#"[{"url": "https://{region}.api.example.com", "variables": {"region": {"default": "us-east"}}}]"#)
                .unwrap();
        assert_eq!(
            resolve_base_url(&servers),
            "https://us-east.api.example.com"
        );
    }

    #[test]
    fn test_resolve_base_url_empty() {
        assert_eq!(resolve_base_url(&[]), "/");
    }
}
