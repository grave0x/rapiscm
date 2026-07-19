# Architecture

## High-level flow

```
CLI (clap)
  │
  ├── corp  ──→  discover::run_discover()  ──→  OSINT sources (crtsh, RDAP, Shodan, etc.)
  │
  ├── session  ──→  session::run_session()  ──→  replays JSONL with live probes
  │
  ├── tasks  ──→  task::TaskStorage  ──→  list/show/delete/prune/export/diff/rebuild
  │
  ├── fuzz  ──→  fuzz::run_fuzz_scan()
  │
  └── scan/spec/url  ──→  config::ScanConfig
                              │
                              ├── parser (spec or url mode)
                              │     │
                              │     └──  Vec<Endpoint>
                              │
                              ├── scan::runner::ScanRunner
                              │     │
                              │     ├── concurrent HTTP (reqwest)
                              │     ├── rate limiting
                              │     └──  Vec<ResponseResult>
                              │
                              ├── check::run_checks (sync)
                              ├── check::run_async_checks (CORS, auth)
                              │
                              ├── report::format_results
                              │     ├── table (ANSI terminal)
                              │     ├── json (JSON lines)
                              │     └── md (markdown tables)
                              │
                              └── task (--save)  ──→  task::TaskStorage
```

## Module dependencies

```
main.rs
  ├── cli.rs     (no deps on other modules)
  ├── config.rs  (depends on cli, types, error)
  ├── types.rs   (depends on analytics for TrackerSignature)
  ├── error.rs   (no deps)
  ├── util.rs    (depends on task::GitInfo)
  │
  ├── scan/
  │   ├── runner.rs  (depends on config, types, check)
  │   ├── spec.rs    (depends on parser::spec, runner)
  │   ├── url.rs     (depends on extract/*, parser::url, runner)
  │   └── browser.rs (features= browser, depends on runner)
  │
  ├── parser/
  │   ├── spec.rs  (depends on types, openapiv3)
  │   └── url.rs   (depends on types, extract/*)
  │
  ├── check/
  │   ├── mod.rs     (orchestrator)
  │   ├── security.rs
  │   ├── cors.rs
  │   ├── auth.rs
  │   └── trackers.rs
  │
  ├── extract/
  │   ├── html.rs
  │   ├── js.rs
  │   ├── json.rs
  │   ├── headers.rs
  │   └── sitemap.rs
  │
  ├── discover/
  │   ├── crtsh.rs
  │   ├── rdap.rs
  │   ├── favicon.rs
  │   ├── asn.rs
  │   ├── gaid.rs
  │   └── search.rs
  │
  ├── fuzz/
  │   ├── runner.rs
  │   ├── wordlist.rs
  │   └── matcher.rs
  │
  ├── session/
  │   ├── parse.rs
  │   └── timing.rs
  │
  ├── task/
  │   ├── store.rs
  │   ├── index.rs
  │   ├── export.rs
  │   ├── diff.rs
  │   ├── resume.rs
  │   ├── rebuild.rs
  │   └── queue.rs
  │
  ├── analytics/
  │   ├── detect.rs
  │   └── sigdb.rs
  │
  ├── filter/  (expression engine)
  ├── tag/     (tag management)
  ├── report/
  │   ├── table.rs
  │   ├── json.rs
  │   └── summary.rs
  └── lib.rs   (public re-exports)
```

## Data types

### Core scan pipeline types

```
Target → config::ScanConfig → Vec<Endpoint> → ScanRunner → Vec<ResponseResult> → format → output
```

- **`Target`** — `Spec(PathBuf)` or `Url(reqwest::Url)`
- **`Endpoint`** — method, URL, headers, body, expected_status, tags
- **`ResponseResult`** — method, URL, status, timing, size, headers, body, checks, trackers, error, tags
- **`Check`** — name, passed, severity (Info/Warn/Critical), message
- **`ScanConfig`** — all resolved CLI flags + target
- **`DiscoverConfig`** — org name + API keys for domain discovery

### Task system types

- **`TaskMeta`** — id, name, tags, version, timestamps, stats, git info, storage info
- **`TaskStorage`** — file-based persistence in `~/.local/share/rapiscm/tasks/`
- **`TaskDiff`** — two-task comparison with change sets
- **`GitInfo`** — sha, branch, message, dirty flag

### Discovery types

- **`DiscoveredDomain`** — domain, sources, cert_subjects, asn, asn_org, ip_ranges, org_name
- **`ApiKeys`** — google_api_key, google_cx, shodan_api_key

## Design principles

- **Minimal dependencies** — manual ANSI codes, std time, AtomicU64 IDs where possible
- **Async-first** — tokio runtime, concurrent HTTP with configurable concurrency
- **Pluggable checks** — sync checks run inline, async checks spawn per-endpoint
- **File-based task storage** — no database needed, tasks stored as JSON files
- **Feature-gated browser** — chromiumoxide + fantoccini only when `browser` feature is enabled
