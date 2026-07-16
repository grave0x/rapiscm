use std::path::PathBuf;

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
}

pub type Result<T> = std::result::Result<T, Error>;
