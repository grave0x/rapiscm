//! Google Analytics ID pivot discovery.
//!
//! Finds domains sharing the same Google Analytics tracking ID.
//! This source requires a third-party API (e.g., BuiltWith, SpyOnWeb) or
//! a local database. In this v1 implementation, it's a stub that returns
//! empty results. Real implementation requires external data source.

use crate::error::{Error, Result};

/// Pivot on Google Analytics IDs to discover related domains.
///
/// Stub — returns empty. Real implementation needs:
/// - A database mapping GA-IDs to domains
/// - An API to query GA-IDs by org name
pub async fn gaid_pivot(_org: &str) -> Result<Vec<String>> {
    // Stub: log and return empty
    tracing::warn!("GA-ID pivot not implemented (requires external data source)");
    Err(Error::DiscoveryHttp {
        src: "gaid",
        detail: "GA-ID pivot requires external API (stub)".into(),
    })
}
