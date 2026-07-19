//! CLI argument definitions (clap derive).

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

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

/// How to crawl pages for endpoint discovery.
#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum CrawlMode {
    /// Only fetch HTML pages (default crawl behavior).
    Html,
    /// Fetch HTML pages AND download/parse JS bundles for API endpoints.
    Js,
    /// Fetch HTML, JS bundles, and use the browser for SPA rendering.
    Full,
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

    /// Discover domains for a company/organization
    Corp {
        /// Company or organization name to discover domains for
        name: String,

        #[command(flatten)]
        global: GlobalArgs,
    },

    /// Replay a recorded session from JSONL
    Session {
        /// Path to the JSONL session file
        file: PathBuf,

        /// Show timing analytics (bursts, gaps, rate limits)
        #[arg(long)]
        timing: bool,

        /// Max malformed lines allowed before aborting
        #[arg(long, default_value = "10")]
        max_parse_errors: usize,

        /// Skip CORS preflight probes during replay
        #[arg(long)]
        skip_cors: bool,

        /// Skip auth-enforcement probes during replay
        #[arg(long)]
        skip_auth: bool,

        #[command(flatten)]
        global: GlobalArgs,
    },

    /// Manage saved scan tasks (list, show, delete, export, diff, rebuild)
    Tasks {
        #[command(subcommand)]
        action: TasksAction,

        #[command(flatten)]
        global: GlobalArgs,
    },

    /// Capture a page as evidence (HTML + screenshot + JS API endpoints)
    Capture {
        /// URL to capture
        url: String,

        /// Output directory for captured page
        #[arg(short, long, default_value = "capture")]
        output: PathBuf,

        /// Take a screenshot (requires --browser feature)
        #[arg(long)]
        screenshot: bool,

        /// Save rendered HTML
        #[arg(long, default_value = "true")]
        html: bool,

        /// Extract API endpoints from JS bundles
        #[arg(long)]
        extract: bool,

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

#[derive(Subcommand, Clone)]
pub enum TasksAction {
    /// List saved tasks
    List,
    /// Show details for a saved task
    Show {
        /// Task ID
        id: u64,
    },
    /// Delete a saved task
    Delete {
        /// Task ID
        id: u64,
    },
    /// Prune old tasks, keeping at most N newest
    Prune {
        /// Number of tasks to keep
        keep: usize,
    },
    /// Export a task to a file
    Export {
        /// Task ID
        id: u64,
        /// Export format: md, sarif, html
        #[arg(long, default_value = "md")]
        format: String,
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Diff two saved tasks
    Diff {
        /// First task ID
        old_id: u64,
        /// Second task ID
        new_id: u64,
    },
    /// Rebuild a task (re-scan failed endpoints)
    Rebuild {
        /// Task ID to rebuild
        id: u64,
        /// Re-scan ALL endpoints, not just failed ones
        #[arg(long)]
        all: bool,
    },
    /// Add targets to the scan queue
    Queue {
        /// Targets to queue (spec files, URLs)
        targets: Vec<String>,
        /// Read targets from a file (one per line)
        #[arg(long)]
        list: Option<PathBuf>,
    },
    /// Process items in the scan queue
    Run {
        /// Number of items to process concurrently
        #[arg(long, default_value = "1")]
        parallel: usize,
    },
    /// Show queue status (pending, running, completed, failed)
    Status,
}

#[derive(Args, Debug, Clone)]
pub struct GlobalArgs {
    /// HTTP method to use (e.g. GET, POST). Default: all methods defined in spec, or GET for URL mode
    #[arg(long)]
    pub method: Option<String>,

    /// Custom header (repeatable, e.g. -H "X-API-Key: secret")
    #[arg(short = 'H', long = "header", value_name = "KEY:VALUE")]
    pub headers: Vec<String>,

    /// Auth configuration: `bearer:<token>`, `basic:<user:pass>`, or `header:<name:value>`
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

    /// Output format: table, json, md, doc
    #[arg(short = 'o', long, default_value = "table", value_parser = ["table", "json", "md", "doc"])]
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

    /// Crawl mode: html, js, full (default: off). Use --crawl js to scan JS bundles
    #[arg(long, value_enum)]
    pub crawl: Option<CrawlMode>,

    /// Maximum crawl depth (default: 2, only used with --crawl)
    #[arg(long, default_value = "2")]
    pub depth: usize,

    /// Glob path include filter
    #[arg(long)]
    pub filter_path: Vec<String>,

    /// Glob path exclude filter
    #[arg(long)]
    pub exclude_path: Vec<String>,

    /// Method include filter (repeatable)
    #[arg(long)]
    pub filter_method: Vec<String>,

    /// Method exclude filter (repeatable)
    #[arg(long)]
    pub exclude_method: Vec<String>,

    /// Status range include filter (e.g. 200,200-299)
    #[arg(long)]
    pub filter_status: Vec<String>,

    /// Status range exclude filter
    #[arg(long)]
    pub exclude_status: Vec<String>,

    /// Expression filter (tag:rest AND tag:v2 AND status:2xx)
    #[arg(long)]
    pub filter: Vec<String>,

    /// Expression exclude
    #[arg(long)]
    pub exclude: Vec<String>,

    /// Show tags in report
    #[arg(long)]
    pub show_tags: bool,

    /// Disable tracker/analytics detection
    #[arg(long)]
    pub no_trackers: bool,

    /// Detailed tracker analysis report (includes cookie breakdown, third-party connections, device profile)
    #[arg(long)]
    pub tracker_report: bool,

    /// Company/organization name for domain discovery (scan + discover)
    /// Use --corp "Org Name" to discover domains, or --corp (empty) for
    /// auto-detection from target URL.
    #[arg(long, num_args = 0..=1, default_missing_value = "")]
    pub corp: Option<String>,

    /// Save scan results as a task (implies --task-name default)
    #[arg(long)]
    pub save: bool,

    /// Label for the saved task (used with --save)
    #[arg(long)]
    pub task_name: Option<String>,

    /// Tags for the saved task (repeatable, e.g. --task-tag ci --task-tag nightly)
    #[arg(long)]
    pub task_tag: Vec<String>,

    /// Do NOT store response bodies in the task
    #[arg(long)]
    pub no_bodies: bool,

    /// Store raw endpoint files in the task directory
    #[arg(long)]
    pub raw: bool,

    /// Task storage directory (default: ~/.local/share/rapiscm/tasks)
    #[arg(long)]
    pub task_dir: Option<PathBuf>,

    /// Task ID to resume (re-scans failed/incomplete endpoints from a saved task)
    #[arg(long)]
    pub resume: Option<u64>,

    /// Capture git context (SHA, branch, message) when saving a task
    #[arg(long)]
    pub git: bool,

    /// Generate reports (API docs site + security audit) in `reports/<name>/`
    #[arg(long)]
    pub report: Option<String>,

    /// Ghost mode: stealth scanning with UA rotation, request jitter, header randomization
    #[arg(long)]
    pub ghost: bool,

    /// Evaluate JS in browser to extract API endpoints (requires --browser feature)
    #[arg(long)]
    pub eval: Option<String>,

    /// User-agent rotation: "mobile", "desktop", "random", or comma-separated list
    #[arg(long)]
    pub ua_rotate: Option<String>,

    /// Request jitter as percentage (e.g. 30 = ±30% random delay)
    #[arg(long, default_value = "0")]
    pub jitter: u32,

    /// Proxy rotation: comma-separated proxy URLs (overrides --proxy)
    #[arg(long, value_delimiter = ',')]
    pub proxy_rotate: Vec<String>,
}
