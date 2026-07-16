use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "rapiscm",
    version,
    about = "Rust API scanner — point at an API spec or URL to scan"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone)]
pub enum Command {
    /// Scan from an OpenAPI spec file (JSON or YAML)
    Spec {
        /// Path to the OpenAPI spec file
        file: PathBuf,

        #[command(flatten)]
        global: GlobalArgs,
    },

    /// Scan a URL
    Url {
        /// URL to scan
        url: String,

        #[command(flatten)]
        global: GlobalArgs,
    },

    /// Auto-detect: spec file or URL
    Scan {
        /// Target to scan (file path or URL)
        target: String,

        #[command(flatten)]
        global: GlobalArgs,
    },

    /// Fuzz endpoints with a wordlist
    Fuzz {
        /// Target URL to fuzz
        target: String,

        /// Wordlist file path (or built-in name)
        #[arg(short = 'w', long)]
        wordlist: Option<String>,

        /// Extensions to append (comma-separated)
        #[arg(short = 'e', long, value_delimiter = ',')]
        extensions: Vec<String>,

        /// Match status codes (e.g. 200,200-299)
        #[arg(long)]
        mc: Option<String>,

        /// Filter status codes
        #[arg(long)]
        fc: Option<String>,

        /// Match response size range
        #[arg(long)]
        ms: Option<String>,

        /// Filter response size
        #[arg(long)]
        fs: Option<String>,

        /// Regex match on response body
        #[arg(long)]
        mr: Option<String>,

        /// Regex filter on response body
        #[arg(long)]
        fr: Option<String>,

        /// Auto-calibrate filters
        #[arg(long)]
        ac: bool,

        #[command(flatten)]
        global: GlobalArgs,
    },
}

#[derive(clap::Args, Debug, Clone)]
pub struct GlobalArgs {
    /// HTTP method to use (e.g. GET, POST). Default: all methods defined in spec, or GET for URL mode
    #[arg(long)]
    pub method: Option<String>,

    /// Custom header (repeatable, e.g. -H "X-API-Key: secret")
    #[arg(short = 'H', long = "header", value_name = "KEY:VALUE")]
    pub headers: Vec<String>,

    /// Auth configuration: bearer:<token>, basic:<user:pass>, or header:<name:value>
    #[arg(long)]
    pub auth: Option<String>,

    /// Requests per second cap
    #[arg(long, default_value = "50")]
    pub rate_limit: u64,

    /// Per-request timeout in seconds
    #[arg(long, default_value = "30")]
    pub timeout: u64,

    /// Max concurrent requests
    #[arg(long, default_value = "10")]
    pub concurrency: usize,

    /// Output format: table, json, md
    #[arg(short = 'o', long, default_value = "table", value_parser = ["table", "json", "md"])]
    pub output: String,

    /// Follow 3xx redirects
    #[arg(long)]
    pub follow_redirects: bool,

    /// Skip TLS certificate verification
    #[arg(short = 'k', long)]
    pub insecure: bool,

    /// Comma-separated path filter (e.g. /api/users,/api/posts)
    #[arg(long, value_delimiter = ',')]
    pub paths: Vec<String>,

    /// Comma-separated OpenAPI tag filter
    #[arg(long, value_delimiter = ',')]
    pub tags: Vec<String>,

    /// Only include endpoints matching ALL of these tags (repeatable, e.g. --filter-tag rest --filter-tag v2)
    #[arg(long, value_delimiter = ',')]
    pub filter_tag: Vec<String>,

    /// Exclude endpoints matching ANY of these tags (repeatable)
    #[arg(long, value_delimiter = ',')]
    pub exclude_tag: Vec<String>,

    /// Proxy URL for all HTTP traffic (e.g. http://127.0.0.1:8080)
    #[arg(long)]
    pub proxy: Option<String>,

    /// Log level: error, warn, info, debug, trace
    #[arg(long, default_value = "info")]
    pub log_level: String,

    /// Module-level log filters (e.g. rapiscm::scan=debug,rapiscm::proxy=info)
    #[arg(long)]
    pub log_filter: Vec<String>,

    /// Log output format: text or json
    #[arg(long, default_value = "text")]
    pub log_format: String,

    /// Browser engine for JS-rendered endpoint discovery
    #[cfg(feature = "browser")]
    #[arg(long, default_value = "chrome", value_parser = ["chrome", "firefox"])]
    pub browser: String,

    /// Show browser GUI during scan (non-headless)
    #[cfg(feature = "browser")]
    #[arg(long)]
    pub headed: bool,
}
