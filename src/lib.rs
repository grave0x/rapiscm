//! rapiscm — Rust API scanner.
//!
//! Library interface for integration tests and programmatic use.
//! Supports scanning from OpenAPI specs, URLs, session replays, fuzzing, and domain discovery.
//!
//! # Quick start
//!
//! ```rust,ignore
//! // Use the CLI entry point for full config resolution:
//! use rapiscm::cli::{Cli, Command};
//! use clap::Parser;
//!
//! let cli = Cli::parse();
//! // Then run via rapiscm::main::dispatch(&cli).await
//! ```
//!
//! For programmatic scans, build a [`config::ScanConfig`] via
//! [`config::ScanConfig::from_cli_global`] or construct one directly
//! (see the CLI module for an example of each field).
//!
//! # Feature flags
//!
//! | Flag | Description | Dependencies |
//! |------|-------------|-------------|
//! | `browser` | Browser-based endpoint discovery (headless Chrome/Firefox) | `chromiumoxide`, `fantoccini` |
//!
//! The `browser` feature enables JavaScript-rendered page scanning via
//! headless browser automation. It is **optional** and off by default
//! because it adds significant compile time and runtime dependencies.
//!
//! ```toml
//! [dependencies]
//! rapiscm = { version = "0.1", features = ["browser"] }
//! ```
//!
//! # Module overview
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`analytics`] | Tracker/analytics detection in responses |
//! | [`check`] | Security and compliance checks (CORS, headers, auth) |
//! | [`cli`] | CLI argument types (clap derive) |
//! | [`config`] | Scan configuration parsing |
//! | [`discover`] | Domain discovery (ASN, crt.sh, RDAP, Shodan) |
//! | [`error`] | Unified error types |
//! | [`extract`] | URL extraction from responses (HTML, JSON, JS, headers) |
//! | [`filter`] | Endpoint/result filtering by tag, method, path, status |
//! | [`fuzz`] | Wordlist-driven fuzzing engine |
//! | [`parser`] | OpenAPI spec parsing and URL analysis |
//! | [`report`] | Output formatting (table, JSON, markdown) |
//! | [`scan`] | Scan orchestrators (spec, URL, browser, session) |
//! | [`tag`] | Auto-tagging of endpoints and responses |
//! | [`types`] | Core data types (Endpoint, ResponseResult, Check, …) |

pub mod analytics;
pub mod check;
pub mod cli;
pub mod config;
pub mod discover;
pub mod error;
pub mod extract;
pub mod filter;
pub mod fuzz;
pub mod ghost;
pub mod parser;
pub mod report;
pub mod scan;
pub mod tag;
pub mod types;
