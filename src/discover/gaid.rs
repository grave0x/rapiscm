//! Google Analytics ID pivot discovery.
//!
//! Finds domains sharing the same Google Analytics tracking ID.
//! This source requires a third-party API (e.g., BuiltWith, SpyOnWeb) or
//! a local database. Currently returns empty — see ApiKeys.ga_api_key.

use crate::error::Result;

/// Pivot on Google Analytics IDs to discover related domains.
///
/// Requires an external data source mapping GA-IDs to domains
/// (e.g., BuiltWith, SpyOnWeb). Without a configured API key,
/// returns empty without error.
pub async fn gaid_pivot(_org: &str, _api_key: Option<&str>) -> Result<Vec<String>> {
    if _api_key.is_none() {
        tracing::info!("GA-ID pivot skipped (no API key)");
        return Ok(vec![]);
    }
    tracing::warn!("GA-ID pivot not implemented — needs external API integration (BuiltWith/SpyOnWeb)");
    Ok(vec![])
}
