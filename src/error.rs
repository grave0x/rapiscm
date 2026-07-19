//! Unified error types for the rapiscm crate.

use std::path::PathBuf;

/// Crate-wide error type covering spec parsing, HTTP, IO, and session errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid auth format: {0}")]
    InvalidAuthFormat(String),

    #[error("invalid header format: {0}")]
    InvalidHeaderFormat(String),

    #[error("invalid output format: {0}")]
    InvalidOutputFormat(String),

    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    #[error("spec file not found: {0}")]
    SpecFileNotFound(PathBuf),

    #[error("failed to parse spec: {0}")]
    SpecParse(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("discovery HTTP error from {src}: {detail}")]
    DiscoveryHttp { src: &'static str, detail: String },

    #[error("discovery parse error from {src}: {detail}")]
    DiscoveryParse { src: &'static str, detail: String },

    #[error("session parse error: {0}")]
    SessionParse(String),

    #[error("task error: {0}")]
    Task(String),
}

/// Shorthand alias for `std::result::Result<T, Error>`.
pub type Result<T> = std::result::Result<T, Error>;
