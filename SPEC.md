# Rapiscm — Complete Spec + Plan

Rust API scanner. Point at an API spec or a URL to scan.

---

## Table of Contents

1. [CLI Reference](#1-cli-reference)
2. [Modes](#2-modes)
3. [Checks](#3-checks)
4. [Browser Feature](#4-browser-feature)
5. [Output Formats](#5-output-formats)
6. [Shared Types](#6-shared-types)
7. [Dependencies](#7-dependencies)
8. [Architecture](#8-architecture)
9. [Feature Gates](#9-feature-gates)
10. [Phase Plan](#10-phase-plan)
11. [Quality Cycle](#11-quality-cycle)
12. [Task System](#12-task-system)
13. [Tracker Analytics](#13-tracker-analytics-engine)
14. [Non-goals](#14-non-goals)

---

## 1. CLI Reference

### Subcommands

```
rapiscm spec <file>            scan from OpenAPI spec (JSON/YAML)
rapiscm url <url>              scan a URL / API base URL
rapiscm scan <target>          auto-detect: spec file or URL
rapiscm fuzz <target>          fuzz endpoints with wordlist
rapiscm ip <target>            IP/range/CIDR port scan + service detection
rapiscm proxy                  HTTP/HTTPS MITM proxy (explicit)
rapiscm mitm <interface>       Packet sniffing MITM (passive, root)
rapiscm tasks                  manage scan history
```

### Global flags (all subcommands)

| Flag | Default | Purpose |
|------|---------|---------|
| `--method` | all | Restrict HTTP method |
| `-H/--header` | — | Custom header (repeatable) |
| `--auth` | — | `bearer:<token>` / `basic:<u:p>` / `header:<n:v>` |
| `--rate-limit` | 50 | Approximate requests/sec |
| `--timeout` | 30s | Per-request timeout |
| `--concurrency` | 10 | Max concurrent in-flight |
| `-o/--output` | table | Output format: `table`, `json`, `md` |
| `--follow-redirects` | false | Follow 3xx |
| `-k/--insecure` | false | Skip TLS verification |
| `--paths` | — | Comma-separated path filter |
| `--tags` | — | Comma-separated tag filter |
| `--proxy` | — | Upstream proxy URL |
| `--filter-tag` | — | Only endpoints with all specified tags |
| `--exclude-tag` | — | Exclude endpoints with any specified tags |
| `--filter-path` | — | Glob path include filter |
| `--exclude-path` | — | Glob path exclude filter |
| `--filter-method` | — | Method include filter |
| `--exclude-method` | — | Method exclude filter |
| `--filter-status` | — | Status range include filter |
| `--exclude-status` | — | Status range exclude filter |
| `--filter` | — | Expression filter (`tag:rest AND tag:v2 AND status:2xx`) |
| `--exclude` | — | Expression exclude |
| `--show-tags` | false | Show tags in report |
| `--script` | — | Script file (`rhai:./x.rhai`, `lua:./x.lua`, `pipe:./x.py`) |
| `--log-level` | info | Log level: `error`, `warn`, `info`, `debug`, `trace` |
| `--log-filter` | — | Module-level filter (`rapiscm::scan=debug,rapiscm::proxy=info`) |
| `--log-format` | text | Log output: `text` or `json` |
| `--deep-spec` | false | Produce deep technical site breakdown |
| `--tracker-analysis` | false | Detect analytics, ads, fingerprinting, trackers |
| `--tracker-report` | false | Detailed tracker analysis report output |
| `--save` | false | Save scan results as a task |
| `--task-name` | — | Custom name for the saved task |
| `--task-tags` | — | Tags for task categorization |
| `--task-dir` | — | Custom tasks storage directory |
| `--no-bodies` | false | Don't store response bodies in saved task |
| `--raw` | false | Store full HTTP request/response pairs |
| `--resume` | — | Resume from partially-completed task ID |
| `--git` | false | Attach git SHA to task (auto-detected) |

### Tasks subcommand

```
rapiscm tasks list [--older <N>d] [--tag <tag>]     List saved tasks
rapiscm tasks show <id>                               Show task details
rapiscm tasks rebuild <id> [--no-save]                Re-run a saved task
rapiscm tasks diff <a> <b>                            Compare two task results
rapiscm tasks export <id> --format <fmt>              Export task (json|md|sarif|html)
rapiscm tasks queue <targets...>                      Queue targets for later scan
rapiscm tasks queue --list targets.txt                Queue from file
rapiscm tasks run [--parallel <N>]                    Process the queue
rapiscm tasks status                                  Show queue / progress
rapiscm tasks prune --older <N>d                      Remove old tasks
rapiscm tasks prune --keep <N>                        Keep N most recent

Flags:
  --format <fmt>    Export format (json, md, sarif, html)
  --parallel <N>    Queue processing concurrency (default 1)
  --older <N>d      Filter/prune tasks older than N days
  --keep <N>        Keep N most recent tasks
  --tag <tag>       Filter by task tag
  --no-save         Rebuild without creating a new task entry
```

### Spec mode flags

| Flag | Default | Purpose |
|------|---------|---------|
| `--tags` | — | Comma-separated OpenAPI tag filter |

### Fuzz mode flags

| Flag | Default | Purpose |
|------|---------|---------|
| `-w/--wordlist` | built-in | Wordlist file or built-in name |
| `--mode` | path | Fuzzing mode: `path`, `param`, `method`, `header`, `body` |
| `-e/--extensions` | — | Extensions to append (`.json,.xml`) |
| `-d/--depth` | 0 | Recursion depth |
| `--mc` | — | Match status codes |
| `--fc` | — | Filter status codes |
| `--ms` | — | Match response size range |
| `--fs` | — | Filter response size range |
| `--mr` | — | Regex match on body |
| `--fr` | — | Regex filter on body |
| `--ac` | false | Auto-calibrate filters |
| `--keyword` | FUZZ | Keyword to replace in target URL |
| `--request` | — | Raw HTTP request file |

### IP mode flags

| Flag | Default | Purpose |
|------|---------|---------|
| `--ports` | — | Explicit ports: `80,443,8000-8100` |
| `--top-ports` | 1000 | Top N common ports |
| `--scan-type` | connect | `connect`, `syn` (root), `udp` (root) |
| `--service-detect` | false | Banner grab + service fingerprint |
| `--os-detect` | false | OS fingerprint |
| `--device-id` | false | Device classification |
| `--mac-vendors` | false | MAC OUI vendor lookup |
| `--concurrent-hosts` | 10 | Max parallel hosts |
| `--ping-sweep` | false | ICMP sweep before scan (root) |
| `--no-ping` | false | Skip host discovery |
| `--dns-resolve` | true | Reverse DNS |

### Proxy mode flags

All use `--mitm-proxy-*` prefix to avoid collisions:

| Flag | Default | Purpose |
|------|---------|---------|
| `--mitm-proxy-port` | 8080 | Local proxy port |
| `--mitm-proxy-install-ca` | false | Install root CA and exit |
| `--mitm-proxy-no-cleanup` | false | Keep CA after exit |
| `--mitm-proxy-no-serve-cert` | false | Disable CA distribution endpoints |
| `--mitm-proxy-cert-url` | auto | Public URL for QR code |
| `--mitm-proxy-crawl` | — | Crawl mode: `same-origin` or `<N>` hops |
| `--mitm-proxy-depth` | 3 | Max crawl depth |
| `--mitm-proxy-rules` | — | Path to YAML rules file |
| `--mitm-proxy-scan` | false | Auto-scan proxied endpoints |
| `--mitm-proxy-scan-rate-limit` | 10 | Max scans/min |

### MITM sniffer mode flags

| Flag | Default | Purpose |
|------|---------|---------|
| `--filter` | — | BPF filter expression |
| `--ports` | — | Port whitelist |
| `--promisc` | true | Promiscuous mode |
| `--auto-tls` | false | TLS keylog decryption |
| `--output` | — | PCAP output file |
| `--pcap-size` | 65536 | Max bytes per packet |
| `--active` | false | ARP spoof + forward (full MITM) |

---

## 2. Modes

### 2.1 Spec mode (`rapiscm spec ./openapi.json`)

1. Parse OpenAPI 3.0/3.1 spec (JSON/YAML)
2. Resolve server URL (first `servers` entry, substitute `{variables}` with defaults)
3. Walk every `{path} {method}` — extract params, security, response codes
4. Fill path params with examples (spec `example` → type-based → `"123"` for IDs)
5. Skip relative URLs (spec without server URL) with warning
6. Apply global `--header` + `--auth` to every endpoint
7. Run tagger on each endpoint (URL patterns, method, spec metadata)
8. Apply `--filter-tag`, `--exclude-tag`, `--paths`, `--tags` filters
9. Fire concurrent HTTP (respecting `--concurrency` + `--rate-limit`)
10. Run checks (sync: headers/schema, async: CORS/auth)
11. If `--deep-spec`: run deep analysis passes
12. If `--save`: store full result as a task
13. Report results

### 2.2 URL mode (`rapiscm url https://api.example.com`)

1. Fetch the given URL
2. Extract endpoints from response (content-type dispatched)
3. Probe common API paths from built-in wordlist
4. If `--features browser`: use headless Chrome/Firefox for JS-rendered discovery
5. If `--crawl`: recursively extract + queue discovered endpoints to `--depth`
6. Deduplicate + normalize discovered URLs
7. Apply `--filter-path`, `--filter-tag`, `--exclude-tag` filters
8. Build endpoints with `--method` (default GET), global `--header` + `--auth`
9. Fire concurrent HTTP
10. Run checks
11. If `--deep-spec`: run deep analysis passes
12. If `--save`: store full result as a task
13. Report

### 2.3 Scan mode (`rapiscm scan <target>`)

Auto-detect: ends in `.json`, `.yaml`, `.yml` → spec mode. Otherwise → URL mode.

### 2.4 Fuzz mode (`rapiscm fuzz <target>`)

1. Parse CLI → FuzzConfig (wordlist, mode, matchers, filters, depth)
2. Load wordlist (built-in or file)
3. If `--ac`: auto-calibrate against baseline request
4. For each word in wordlist: construct URL, fire request, evaluate via MatchConfig
5. If match: log, store, optionally recurse
6. If `--depth > 0` and matched path acts as directory: recurse with depth-1
7. Report

Fuzzing modes: path, param, method, header, body.
Wordlist modes: sniper, pitchfork, clusterbomb.

### 2.5 IP mode (`rapiscm ip <target>`)

Target formats: single IP, CIDR, range, comma-separated, DNS-resolved hostname.

1. Parse target → list of IPs
2. If `--ping-sweep`: ICMP probe to skip dead hosts
3. For each host: port scan, service detection, OS fingerprint, device ID, DNS reverse lookup
4. Report

### 2.6 Proxy mode (`rapiscm proxy`)

```
Client → TCP CONNECT → rapiscm proxy → target server
Client → HTTP GET → rapiscm proxy (hyper mode) → target server
```

TLS decryption via dynamic root CA. Auto-install + cleanup. QR code + iOS mobileconfig served on proxy port. Two code paths: Fork A (raw TCP bridge) and Fork B (hyper-based with full body inspection + rules engine).

### 2.7 MITM sniffer mode (`rapiscm mitm <interface>`)

```
NIC → pcap → BPF filter → packet parser → TCP reassembly → script hooks → report
```

Future `--active` flag: ARP spoofing + IP forwarding for full MITM position.

---

## 3. Checks

| # | Check | Type | Description |
|---|-------|------|-------------|
| 1 | HTTP status | sync | 4xx/5xx tracked as issues |
| 2 | Response time | sync | p50/p90/p99 |
| 3 | Response size | sync | logged per endpoint |
| 4 | Security headers | sync | CSP, HSTS, X-CT-O, X-FO, Cache-Control |
| 5 | CORS | async | OPTIONS + `Origin: https://evil.com` |
| 6 | Auth required | async | Re-request without auth → 200 means no enforcement |
| 7 | Schema validation | sync | Expected status match + valid JSON body |

---

## 4. Browser Feature

Feature gate: `browser`. Deps: chromiumoxide, fantoccini.

| Engine | Driver | Proxy | Interactive |
|--------|--------|-------|-------------|
| Chrome | chromiumoxide (CDP) | `--proxy-server` | Click links, submit forms |
| Firefox | fantoccini (WebDriver) | geckodriver on :4444 | Click links |

---

## 5. Output Formats

### Table (default)

ANSI-colored terminal. Columns: method, URL, status (color-coded), time, checks inline.

### JSON

`serde_json::to_string_pretty`. Full array of `ResponseResult` objects.

### Markdown

Summary section + table per endpoint group.

---

## 6. Shared Types

```rust
pub enum Target { Spec(PathBuf), Url(reqwest::Url) }
pub enum AuthConfig { Bearer(String), Basic {..}, Header {..} }

pub struct Endpoint {
    pub method: reqwest::Method, pub url: reqwest::Url,
    pub headers: Vec<(String,String)>, pub body: Option<Value>,
    pub expected_status: Option<u16>, pub tags: Vec<String>,
}

pub struct ResponseResult {
    pub endpoint_method: String, pub endpoint_url: String,
    pub status_code: u16, pub response_time_ms: u64,
    pub response_size: usize, pub response_headers: Vec<(String,String)>,
    pub response_body: Vec<u8>, pub expected_status: Option<u16>,
    pub tags: Vec<String>, pub checks: Vec<Check>, pub error: Option<String>,
}

pub struct Check {
    pub name: String, pub passed: bool, pub severity: Severity,
    pub message: String,
}

pub enum Severity { Info, Warn, Critical }
pub enum OutputFormat { Table, Json, Markdown }
```

---

## 7. Dependencies

### Runtime (13 core)

clap, tokio, reqwest, serde+serde_json+serde_yaml, anyhow, thiserror,
tracing+tracing-subscriber, regex, futures-util, openapiv3

### Optional browser (2): chromiumoxide, fantoccini
### Optional proxy (4): rcgen, rustls, tokio-rustls, pem
### Optional proxy-full (+4): hyper, http, http-body, h2
### Optional mitm-sniff (2): pcap, pnet
### Optional script-lua (1): mlua

---

## 8. Architecture

### Data flow (full)

```
CLI (clap) → Config → (spec | url | scan | fuzz | ip | proxy | mitm)
    → endpoints (+ tags + filters) → ScanRunner → ResponseResult
    → checks (sync + async) → deepspec (if --deep-spec) → report
    → task store (if --save)
```

### Task system flow

```
Scan completes
    │
    ├─ No --save flag → print report, exit
    │
    └─ --save flag:
        ├─ Generate task_id (monotonic counter)
        ├─ Create task directory: ~/.local/share/rapiscm/tasks/<id>/
        ├─ Write task.json (config + metadata + summary)
        ├─ Write results.json (full ResponseResult array)
        ├─ If --raw: write raw/0001-req.txt, raw/0001-resp.txt per endpoint
        ├─ If --git: capture HEAD SHA + branch
        ├─ Write to task index: tasks/index.json
        ├─ Print report
        └─ Exit

Requeue happens at exit: add `--rerun` flag → task config goes back to queue
Queue stored as tasks/queue.json → persistent across crashes
```

---

## 9. Feature Gates

```toml
[features]
default = []
browser = ["dep:chromiumoxide", "dep:fantoccini"]
proxy = ["dep:rcgen", "dep:rustls", "dep:tokio-rustls", "dep:pem"]
proxy-full = ["proxy", "dep:hyper", "dep:http", "dep:http-body", "dep:h2"]
mitm-sniff = ["dep:pcap", "dep:pnet"]
script-lua = ["dep:mlua"]
```

---

## 10. Phase Plan

See full plan in [PLAN.md](./PLAN.md) for detailed phase breakdown.

---

## 11. Quality Cycle

After each phase (or every 4 phases for larger groups), run 3 turns of:
Review → Test → Deep review → Test → Optimize → Refactor → Test → Exploit.

---

## 12. Task System

### 12.1 Overview

Every scan can be saved as a **task** — a self-contained JSON directory storing the full scan configuration, all response results, and computed summary. Tasks can be listed, inspected, re-run, compared, and exported.

### 12.2 Storage layout

```
~/.config/rapiscm/tasks/
├── index.json              # task metadata index (lightweight, grep-friendly)
├── queue.json              # pending task queue (persistent, crash-recoverable)
│
├── 0001-example-api-prod/  # named task directory
│   ├── task.json           # full config + metadata + summary
│   ├── results.json        # Vec<ResponseResult> (full scan output)
│   ├── results-nb.json     # same as results.json without response bodies (--no-bodies)
│   └── raw/                # raw HTTP request/response pairs (--raw)
│       ├── 0001-req.txt    # raw request: GET /api/users HTTP/1.1
│       ├── 0001-resp.txt   # raw response: HTTP/1.1 200 OK
│       └── ...
│
├── 0002-...
│
└── artifacts/              # deep-spec reports, screenshots, GPG signatures
    ├── 0001-deepspec.yaml
    └── ...
```

**Platform defaults:**

| Platform | Path |
|----------|------|
| Linux | `~/.local/share/rapiscm/tasks/` |
| macOS | `~/Library/Application Support/com.rapiscm/tasks/` |
| Windows | `%APPDATA%/rapiscm/tasks/` |

Override: `RAPISCM_TASKS_DIR` env var or `--task-dir` CLI flag.

### 12.3 task.json specification

```json
{
  "task_id": 1,
  "task_name": "example-api-prod",
  "task_tags": ["api", "production", "sprint-42"],
  "cli_version": "0.1.0",
  "created_at": "2026-07-16T15:00:00Z",
  "duration_seconds": 47.3,
  "command": "url",
  "target": "https://api.example.com",
  "config": {
    "method": null,
    "headers": [["Authorization", "Bearer ***"]],
    "rate_limit": 50,
    "timeout": 30,
    "concurrency": 10,
    "output": "json",
    "follow_redirects": false,
    "insecure": false,
    "paths": [],
    "proxy": null,
    "log_level": "info",
    "ghost": false,
    "deep_spec": false,
    "crawl": null,
    "diff_auth": false,
    "tags": []
  },
  "git": {
    "sha": "a1b2c3d4e5f6...",
    "branch": "main",
    "message": "fix: add rate limiting to /api/search",
    "dirty": false
  },
  "endpoint_count": 47,
  "result_summary": {
    "total": 47,
    "successful": 35,
    "failed": 8,
    "errors": 4,
    "checks_passed": 189,
    "checks_failed": 23,
    "checks_warn": 12,
    "p50_ms": 42,
    "p90_ms": 127,
    "p99_ms": 890
  },
  "endpoints_by_tag": {
    "rest": 40,
    "v2": 25,
    "v1": 22,
    "auth-required": 30,
    "slow": 3,
    "error-5xx": 2
  },
  "storage": {
    "has_bodies": true,
    "has_raw": false,
    "has_artifacts": ["deepspec"],
    "results_size_bytes": 482000
  },
  "exit_code": 0
}
```

### 12.4 Save modes

| Mode | Flag | Bodies | Raw HTTP | Disk | Use case |
|------|------|--------|----------|------|----------|
| **Full** | `--save` | ✅ | ❌ | ~500KB | Default. Good for diff, rebuild, export. |
| **Bodyless** | `--save --no-bodies` | ❌ | ❌ | ~5KB | CI/CD, privacy (no data stored). |
| **Raw** | `--save --raw` | ✅ | ✅ | ~1MB+ | Forensics, Wireshark analysis, MITM replay. |

### 12.5 index.json

Lightweight index for fast listing without loading every task.json:

```json
[
  {
    "task_id": 1,
    "task_name": "example-api-prod",
    "command": "url",
    "target": "https://api.example.com",
    "created_at": "2026-07-16T15:00:00Z",
    "duration_seconds": 47.3,
    "endpoint_count": 47,
    "checks_failed": 23,
    "exit_code": 0,
    "task_tags": ["api", "production"],
    "git_sha": "a1b2c3d4"
  },
  ...
]
```

### 12.6 queue.json

Persistent queue. Survives crashes (atomically written after each mutation):

```json
{
  "concurrency": 3,
  "items": [
    {
      "queue_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
      "command": "url",
      "target": "https://api3.example.com",
      "config_snapshot": { "rate_limit": 100, "save": true, ... },
      "status": "pending",
      "created_at": "2026-07-16T14:00:00Z",
      "retries": 0,
      "error": null
    },
    {
      "queue_id": "b2c3d4e5-...",
      "status": "running",
      "started_at": "2026-07-16T14:30:00Z"
    },
    {
      "queue_id": "c3d4e5f6-...",
      "status": "completed",
      "task_id": 1,
      "completed_at": "2026-07-16T15:00:00Z"
    }
  ]
}
```

**Crash recovery:**
- On startup: scan queue for `running` items
- If any `running` items exist and the process PID no longer exists → mark as `failed`
- Re-queue `failed` and `pending` items up to `concurrency` limit
- Queue mutations are atomic: write to temp file, rename over old file

### 12.7 Rebuild

`rapiscm tasks rebuild 1`:

1. Read `tasks/0001-*/task.json` for full config
2. Execute scan from scratch (respecting current `--rate-limit`, `--timeout`, etc.)
3. Save results as a **new task** (original task 1 is preserved as-is)
4. Show diff summary:

```
=== Rebuild of task 1: example-api-prod ===

Config: identical (no CLI overrides)
Duration: 52.1s (was 47.3s)  [+10%]

Endpoint changes:
  + /api/v2/export          POST     [new]
  - /api/v1/legacy/import   POST     [removed]

Status changes:
  /api/users         200 → 200       [unchanged]
  /api/admin/login   200 → 401       [now requires auth ✅]

Check changes:
  + HSTS: /api/v2/export     [new endpoint]
  - CSP:  /api/v1/legacy     [endpoint removed]
  ! HSTS: /api/health        [WARN: now missing on 1 endpoint]
```

**Rebuild config resolution:**
- `task.json` config is the base
- CLI flags override (e.g., `rapiscm tasks rebuild 1 --rate-limit 200`)
- Missing or invalid values fall back to defaults (backward-compatible rebuilds)

### 12.8 Diff

`rapiscm tasks diff 1 2`:

```
=== Task Diff: 1 (prod) vs 2 (staging) ===

Scan metadata:
  Duration: 47.3s vs 52.1s
  Targets:  https://api.example.com vs https://staging-api.example.com

Endpoint changes (6):
  + /api/v2/export           [POST, auth-required, rest]
  + /api/v2/import           [POST, auth-required, rest]
  - /api/v1/legacy/import    [deprecated]
  - /api/v1/health           [internal]

Status changes (2):
  GET /api/users           200 → 200    [same]
  GET /api/admin/settings  200 → 401    [staging requires auth, prod did not]

Security regression (1):
  GET /api/health          HSTS missing (prod has it)

Security improvement (3):
  POST /api/login          CSP now present
  GET /api/logs            Auth now required
  GET /api/config          401 instead of 200

Performance regression (2):
  POST /api/search         42ms → 890ms   [+2100%]
  GET  /api/feed           15ms → 120ms   [+700%]
```

**Diff algorithm:**
1. Group both result sets by `parse::url::fingerprint_path()`
2. Match fingerprints between old and new
3. For matched: compare status, JSON body structure (field names + types), checks (passed/failed names), timing
4. For unmatched in old only: removed
5. For unmatched in new only: added
6. Classify changes: security (check severity), performance (>2x timing change), stability (status code range change)

### 12.9 Export

| Format | Description | Use case |
|--------|-------------|----------|
| `json` | Full task.json + results.json merged | Machine processing |
| `markdown` | Human-readable report (PRs, wikis) | Documentation |
| `sarif` | Static Analysis Results Interchange Format | CI/CD integration |
| `html` | Standalone HTML report with CSS + JS | Visual sharing |

```
rapiscm tasks export 1 --format sarif         # → tasks/0001-*/export.sarif
rapiscm tasks export 1 --format html          # → tasks/0001-*/export.html
rapiscm tasks export 1 --format markdown -o ./report.md
```

### 12.10 Git integration

When `--git` is set (or `--save --git`):

1. Run `git rev-parse HEAD` at scan start
2. Run `git rev-parse --abbrev-ref HEAD` for branch name
3. Run `git log -1 --format=%s` for commit message
4. Run `git status --porcelain` to detect dirty working tree
5. Store in `task.json` under `"git"` key

**Use cases:**
- CI/CD: link scan results to the commit that triggered them
- `tasks diff 1 2` shows git SHAs instead of task IDs when available
- `tasks show 1` shows "Scan of commit a1b2c3d on main: 'fix auth on /api/admin'"
- `tasks list --git-branch staging` filter by branch

```rust
pub fn capture_git_info() -> Option<GitInfo> {
    let sha = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"]).output().ok()?;
    let branch = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"]).output().ok()?;
    let message = std::process::Command::new("git")
        .args(["log", "-1", "--format=%s"]).output().ok()?;
    let dirty = std::process::Command::new("git")
        .args(["status", "--porcelain"]).output().ok()?;
    Some(GitInfo {
        sha: String::from_utf8(sha.stdout).ok()?.trim().into(),
        branch: String::from_utf8(branch.stdout).ok()?.trim().into(),
        message: String::from_utf8(message.stdout).ok()?.trim().into(),
        dirty: !dirty.stdout.is_empty(),
    })
}
```

### 12.11 Resume

`rapiscm url https://api.example.com --resume 1`:

1. Load `tasks/0001-*/task.json`
2. Reconstruct the scan configuration
3. Resume: re-run but skip endpoints that already have results
4. Only endpoints with `status_code == 0` (connection error or timeout) are re-attempted
5. Merge old results + new results → unified result set
6. If `--save`: update task in place (preserving git info, plus resume metadata)

**Resume breakpoint:** After each successful endpoint scan, the endpoint index is written to a checkpoint file. On resume, this checkpoint is read and completed endpoints are skipped.

### 12.12 New files

| File | Purpose | LOC | Deps |
|------|---------|-----|------|
| `src/task/mod.rs` | Task struct, TaskConfig, TaskId, TaskMeta | 80 | serde |
| `src/task/store.rs` | JSON file store: save, load, list, delete, prune | 160 | serde_json |
| `src/task/index.rs` | index.json management (lightweight scan) | 60 | serde_json |
| `src/task/queue.rs` | queue.json management, crash recovery | 100 | serde_json |
| `src/task/rebuild.rs` | Rebuild from task.json, diff computation | 180 | — |
| `src/task/diff.rs` | TaskDiff struct, diff algorithm, classification | 200 | — |
| `src/task/export.rs` | Export to md, sarif, html | 180 | — |
| `src/task/resume.rs` | Resume logic, checkpoint read/write | 80 | — |
| `src/cli/tasks.rs` | `tasks` subcommand + all flags | 120 | clap |
| `src/config.rs` | Add `save`, `task_name`, `task_tags`, `task_dir`, `no_bodies`, `raw`, `resume_from`, `git` fields | +30 | — |
| `src/scan/runner.rs` | Checkpoint write after each endpoint | +20 | — |
| `src/main.rs` | Dispatch `Command::Tasks`, wire `--save` at scan completion | +20 | — |

**Total new LOC:** ~1,230
**New deps:** Zero (all JSON via existing serde_json)

### 12.13 Tests

| Test | What it verifies |
|------|-----------------|
| `test_task_save_load` | Save a scan result → load it back → fields match |
| `test_task_list` | Multiple tasks saved → list returns correct count |
| `test_task_list_filter_tag` | List with `--tag` returns filtered subset |
| `test_task_delete` | Delete task → not in list, files removed |
| `test_task_prune_older` | Prune older than 30d → only recent remain |
| `test_task_prune_keep` | Keep N → only N most recent remain |
| `test_queue_add` | Add to queue → shows in list |
| `test_queue_process` | Process queue → creates tasks |
| `test_queue_crash_recovery` | Simulate crash during run → `running` items become `failed` |
| `test_rebuild_preserves_original` | Rebuild creates new task, original unchanged |
| `test_diff_added_removed` | Diff detects added + removed endpoints |
| `test_diff_status_change` | Diff detects status code changes |
| `test_diff_security_regression` | Diff flags check failures |
| `test_diff_performance` | Diff flags >2x timing changes |
| `test_resume_skips_completed` | Resume skips endpoints with status > 0 |
| `test_export_markdown` | Export generates markdown |
| `test_git_info` | Git SHA + branch captured correctly |
| `test_save_mode_full` | Full save includes bodies |
| `test_save_mode_bodyless` | Bodyless save has empty response_body |
| `test_save_mode_raw` | Raw save creates req/resp files |
| `test_index_sync` | index.json matches actual tasks |
| `test_metadata_integrity` | Corrupted task.json returns error |

**~22 new tests.**

---

## 13. Tracker Analytics Engine

### 13.1 Overview

When `--tracker-analysis` is set, rapiscm catalogs every tracker, analytics script, ad network, fingerprinting technique, consent platform, and data exfiltration destination in every scanned page. Output is a structured report usable for privacy audits, competition analysis, and ghost-mode validation.

### 13.2 Detection categories

| Category | Examples | Detection method |
|----------|----------|-----------------|
| **Analytics** | GA4, Meta Pixel, Hotjar, Clarity, Amplitude, Mixpanel, Heap, PostHog, Matomo | Script `src` URL patterns, `window.dataLayer`, `gtag()` calls |
| **Advertising** | Google Ads, Amazon Associates, Criteo, Taboola, Outbrain, AdSense | Script/iframe URL patterns, `navigator.sendBeacon` destinations |
| **Consent Management** | Cookiebot, OneTrust, Consent Manager, CookieYes, Osano | Script URL + `window.__tcfapi`, `window.googlefc`, `window.Cookiebot` |
| **Fingerprinting** | FingerprintJS, Cloudflare Turnstile, Akamai BMP, Distil | Canvas `.toDataURL()` calls, WebGL `getParameter()`, AudioContext |
| **Session Replay** | FullStory, Hotjar, Mouseflow, SessionCam, Smartlook | Script patterns capturing DOM events, `rrweb` library |
| **Social Media** | Facebook, Twitter, LinkedIn, TikTok, Pinterest, Instagram | Embed scripts, share buttons, iframes, pixel tracking |
| **CDN/Utility** | Cloudflare, Akamai, Fastly, jsDelivr, unpkg, CDNJS | Response headers, script source patterns |

### 13.3 Tracker signature database

Embedded ~300-entry database, ~5KB data, zero deps:

```rust
pub struct TrackerSignature {
    pub name: &'static str,
    pub category: TrackerCategory,
    pub company: &'static str,
    pub domains: &'static [&'static str],
    pub script_patterns: &'static [&'static str],
    pub cookie_names: &'static [&'static str],
    pub purpose: &'static str,
    pub privacy_url: Option<&'static str>,
}

pub enum TrackerCategory {
    Analytics, Advertising, ConsentManagement,
    Fingerprinting, SessionReplay, SocialMedia, CDN, Utility,
}
```

### 13.4 Cookie classification

Each `Set-Cookie` header is classified by purpose:

| Purpose | Examples | Description |
|---------|----------|-------------|
| **Necessary** | `sessionid`, `csrf_token`, `__cf_bm` | Required for basic functionality |
| **Preferences** | `language`, `currency`, `theme` | User preference storage |
| **Statistics** | `_ga`, `_gid`, `_clck` | Analytics, anonymous usage data |
| **Marketing** | `_fbp`, `_gcl_au`, `IDE` | Ad targeting, cross-site tracking |

```rust
pub fn classify_cookie(name: &str) -> CookiePurpose {
    match name {
        "_ga" | "_gid" | "_gat" | "_clck" | "_clsk" => CookiePurpose::Statistics,
        "_fbp" | "_gcl_au" | "IDE" | "test_cookie" => CookiePurpose::Marketing,
        "sessionid" | "csrf" | "__cf_bm" => CookiePurpose::Necessary,
        "language" | "currency" | "theme" => CookiePurpose::Preferences,
        _ => CookiePurpose::Unclassified,
    }
}
```

### 13.5 Data export detection

In MITM proxy or mitm sniffer mode, rapiscm tracks all outbound requests and cross-references destinations against the tracker database:

```
Data sent to 12 third-party domains:
  google-analytics.com            page_view, scroll_depth, device_info, user_agent
  facebook.net                    page_view, referrer, user_agent
  hotjar.com                      mouse_move, click, form_interaction
  criteo.com                      product_view, user_id, session_id
  doubleclick.net                 page_view, device_info, browser_language
```

### 13.6 Device profile reconstruction

From available browser fingerprint signals, rapiscm reconstructs the device profile that trackers build:

```
Device Profile (8 signals):
  User-Agent:     Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ...
  Screen:         1512 × 982 (2.0 DPR)
  Color Depth:    24-bit
  Timezone:       America/New_York (UTC-5)
  Language:       en-US, en
  Fonts:          183 installed
  WebGL Vendor:   Google Inc. (Apple)
  Canvas Hash:    a1b2c3d4e5f6...
Stability: High (8/8 signals consistent)
Likely Bot: No
```

### 13.7 Output format

```json
{
  "trackers": {
    "total_detected": 14,
    "by_category": { "analytics": 4, "advertising": 3, ... },
    "trackers": [{ "name": "GA4", "category": "analytics", ... }],
    "cookies": { "total": 14, "by_purpose": { "necessary": 2, ... } },
    "device_profile": { "fingerprint_signals": 8, "stability": "high" },
    "third_party_connections": 12
  }
}
```

### 13.8 Integration with existing features

| Feature | How analytics enhances it |
|---------|--------------------------|
| MITM proxy | Real-time tracker inventory per proxied response |
| Ghost mode | Validate evasion: "are trackers seeing the real device?" |
| Deep-spec | Tracker inventory added to deep-spec YAML report |
| Task system | Tracker data saved as artifact, diff across rebuilds |
| Extract engine | Tracker URLs extracted from JS/HTML automatically |

### 13.9 New files

| File | Purpose | LOC |
|------|---------|-----|
| `src/analytics/mod.rs` | TrackerAnalysis, dispatch, output | 60 |
| `src/analytics/detect.rs` | 300-entry signature DB + matching | 200 |
| `src/analytics/cookies.rs` | Cookie purpose classification | 80 |
| `src/analytics/export.rs` | Data export detection + reporting | 100 |
| `src/analytics/profile.rs` | Device profile from fingerprint signals | 80 |

**Total:** ~520 LOC. Zero new deps.

### 13.10 Tests

| Test | What it verifies |
|------|-----------------|
| `test_detect_ga4` | GA4 detected from gtag/js URL |
| `test_detect_fb_pixel` | Meta Pixel detected from fbevents.js |
| `test_classify_cookie_ga` | `_ga` → Statistics |
| `test_classify_cookie_fbp` | `_fbp` → Marketing |
| `test_export_destinations` | Outbound URLs matched to known trackers |
| `test_no_trackers` | Clean page → empty tracker list |
| `test_consent_framework` | TCF v2.2 detected from `__tcfapi` |

---

## 14. Non-goals (v1)

- No OAuth / token-refresh flows
- No performance/load testing
- No GUI or daemon mode
- No spec generation from responses
- No database persistence of scan results
- No cloud/SaaS integration
- No distributed scanning
- No fuzzing beyond API parameter fuzzing (no UI/buffer/heap fuzzing)
- No Bluetooth/BLE MITM
- No GSM/SS7 interception
- No hardware keylogger integration
