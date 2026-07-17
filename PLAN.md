# rapiscm plan

16 phases split into 4 groups of 4. After each group: quality cycle (3 turns).

---

## Dependencies (minimal)

Every dep justified by essential functionality. No table-lib, color-lib, progress-bar, uuid, or chrono — replaced with std + manual formatting.

### Runtime

| Crate | Why |
|-------|-----|
| `clap` (derive) | CLI arg parsing |
| `tokio` (full) | Async runtime for concurrent HTTP |
| `reqwest` (json) | HTTP client |
| `serde` + `serde_json` | Spec parsing, JSON output |
| `anyhow` | Error propagation |
| `thiserror` | Typed error enums |
| `log` + `env_logger` | Structured debug logging |

### Spec parsing (phases 4, 8)

| Crate | Why |
|-------|-----|
| `openapiv3` | OpenAPI 3.0/3.1 Rust types |
| `serde_yaml` | YAML spec files |

### Dev only

| Crate | Why |
|-------|-----|
| `tempfile` | Temp files for spec fixture tests |

### Eliminated (std replacement)

| Planned dep | Replaced with |
|-------------|---------------|
| `tabled` | Manual `write!` formatting |
| `colored` | Raw ANSI escape codes |
| `indicatif` | Simple `log!` per request |
| `chrono` | `std::time::SystemTime` |
| `uuid` | `AtomicU64` counter |
| `tracing` | `log` + `env_logger` |
| `url` | `reqwest::Url` (re-exported) |
| `reqwest mock` | `wiremock` crate or local test server |
| `criterion` | Skip benchmarks for v1 |

### Dependency graph

```
rapiscm
├── clap             (CLI → Config)
├── tokio            (async runtime)
├── reqwest          (HTTP scan runner)
├── serde + serde_json + serde_yaml
│   └── openapiv3    (spec parse)
├── log + env_logger (debug output)
├── thiserror        (error types)
└── anyhow           (bail, context)
```

---

## Architecture

### Data flow

```
CLI args (clap)
    │
    ▼
  Config
    │
    ├─ Target::Spec(path) ──► SpecParser ──┐
    └─ Target::Url(url)  ──► UrlDiscover ──┤
                                           ▼
                                     Vec<Endpoint>
                                           │
                                           ▼
                                     ScanRunner
                                   (concurrent HTTP)
                                           │
                                           ▼
                                     Vec<ResponseResult>
                                           │
                                           ▼
                                     CheckRunner
                                    (security, cors, etc.)
                                           │
                                           ▼
                                     Report (Table/Json/Md)
```

### Module tree

```
src/
  main.rs               # tokio::main, run scan, print report
  cli.rs                # clap derive struct + parse()
  config.rs             # ScanConfig builder from Cli
  error.rs              # Error enum (thiserror)
  types.rs              # Endpoint, ResponseResult, Check, etc.

  scan/
    mod.rs              # scan entry, dispatch
    runner.rs           # ScanRunner: concurrent HTTP exec + rate limiter
    spec.rs             # SpecScanner: spec → endpoints → scan
    url.rs              # UrlScanner: url → discover → scan

  parser/
    mod.rs
    spec.rs             # openapiv3 wrapper → Vec<Endpoint>
    url.rs              # URL normalize, join

  check/
    mod.rs
    security.rs         # Security header presence
    cors.rs             # CORS probe
    auth.rs             # Missing-auth check

  report/
    mod.rs
    table.rs            # manual string formatting
    json.rs             # serde_json formatter
    summary.rs          # Stats aggregation
```

---

## Phases

### Group 1: Foundation + Spec Scanner (phases 1–4)

**Phase 1 — Project scaffold + CLI** (done)
- `cargo init --name rapiscm`
- `Cargo.toml` with minimal deps
- `cli.rs`: `Cli` struct with `spec`, `url`, `scan` subcommands + all flags
- `main.rs`: parse CLI, dispatch to mode
- `.gitignore`, `cargo clippy` clean

**Phase 2 — Types + Config + Errors** (done)
- `types.rs`: `Target`, `AuthConfig`, `Endpoint`, `ResponseResult`, `Check`, `Severity`, `OutputFormat`
- `config.rs`: `ScanConfig::from_cli()` with header/auth/output parsing
- `error.rs`: `Error` enum with thiserror
- Unit tests for config builders

**Phase 3 — HTTP runner**
- Write `scan/runner.rs`: `ScanRunner` struct
  - Takes `Vec<Endpoint>` + config
  - Spawns `concurrency` workers, feeds endpoints via channel
  - Rate-limit via tokio sleep or semaphore
  - Returns `Vec<ResponseResult>`
  - Handles timeouts, connection errors gracefully
  - Logs each request with `log!`
- Verify: unit tests with local test server (e.g. `wiremock` or manual)

**Phase 4 — Spec parser + spec-mode scan**
- Write `parser/spec.rs`:
  - Parse JSON/YAML → `openapiv3::OpenAPI`
  - Walk paths, extract each method
  - Resolve `$ref`, resolve server URL + variables
  - Fill path params with example values or `<type>` placeholders
  - Build `Endpoint` vec
- Write `scan/spec.rs`:
  - `SpecScanner`: parse spec → scan runner → results
- Wire into `main.rs` for `rapiscm spec <file>`
- Verify: parse a real OpenAPI spec (petstore?), scan against test server

```
┌─────────────────────────────────────────────────┐
│         QUALITY CYCLE (3 turns)                  │
│                                                 │
│  Turn:                                           │
│  ┌─────────────────────────────────────────┐    │
│  │ 1. review         — read all code       │    │
│  │ 2. test           — run test suite      │    │
│  │ 3. deep review    — line-by-line audit  │    │
│  │ 4. test           — rerun, add edge cas │    │
│  │ 5. optimize       — perf, allocations   │    │
│  │ 6. refactor       — simplify, extract   │    │
│  │ 7. test           — regression check    │    │
│  │ 8. exploit        — find bugs, breaks   │    │
│  └─────────────────────────────────────────┘    │
│             ↺ 3 times                            │
└─────────────────────────────────────────────────┘
```

---

### Group 2: URL Scanner + Checks (phases 5–8)

**Phase 5 — URL discovery + URL-mode scan**
- Write `parser/url.rs`: URL normalization, base resolution
- Write `scan/url.rs`:
  - Fetch URL, parse HTML for links/forms
  - Probe common API paths (built-in wordlist)
  - Build `Endpoint` vec from discovered paths
  - Run scan runner
- Wire into `main.rs` for `rapiscm url <url>`
- Verify: test against a known API, verify endpoint discovery

**Phase 6 — Security header checks**
- Write `check/security.rs`:
  - Check each header presence + value pattern
  - `Content-Security-Policy`, `Strict-Transport-Security`, `X-Content-Type-Options`, `X-Frame-Options`, `Cache-Control`
- Integrate into scan flow: after `ResponseResult` received, run checks
- Each check → `Check { name, passed, severity, message }`
- Verify: test against server with known headers, without, and with weak values

**Phase 7 — CORS + Auth checks**
- Write `check/cors.rs`:
  - Send preflight `OPTIONS` request with `Origin: <attacker-site>`
  - Check `Access-Control-Allow-Origin` response
- Write `check/auth.rs`:
  - Re-send request without auth headers
  - If 200 → report "auth not required"
- Verify: test against known CORS configurations, known auth-gated endpoints

**Phase 8 — Schema validation**
- In spec mode: validate response body against spec schema
- Use `openapiv3` schema types, validate JSON structure
- Report schema mismatches as `Check`
- Verify: test with valid response, invalid response, missing fields

```
┌─────────────────────────────────────────────────┐
│         QUALITY CYCLE (3 turns)                  │
└─────────────────────────────────────────────────┘
```

---

### Group 3: Output + Polish (phases 9–12)

**Phase 9 — Table formatter**
- Write `report/table.rs`:
  - Manual string formatting (no `tabled` crate)
  - Columns: status, method, path, time, checks summary
  - Color-code via ANSI codes: 2xx=green, 3xx=blue, 4xx=yellow, 5xx=red
  - Show check pass/fail per endpoint
- Wire into main: when `--output table` (default)
- Verify: run scan, inspect output

**Phase 10 — JSON formatter**
- Write `report/json.rs`:
  - `serde_json::to_string_pretty` on full result set
  - Schema: `{ scan_id, timestamp, target, summary, results: [...] }`
  - Scan ID: monotonic `AtomicU64`, timestamp: `std::time::SystemTime`
- Wire into main: when `--output json`
- Verify: pipe through `jq`, validate structure

**Phase 11 — Summary + markdown**
- Write `report/summary.rs`:
  - `SummaryStats { total, passed, failed, skipped, p50, p90, p99 }`
- Markdown output: `report/md.rs`
  - Headline summary, then tables per endpoint group
- Wire both into main
- Verify: inspect output

**Phase 12 — UX polish + CI**
- `--verbose` / `--quiet` flags
- `--version` flag (automatic from clap)
- Good error messages for common failures (no spec, bad URL, network down)
- Set up `.github/workflows/ci.yml`: `cargo check → test → clippy → fmt`
- Final `cargo clippy` + `cargo fmt` pass

```
┌─────────────────────────────────────────────────┐
│         QUALITY CYCLE (3 turns)                  │
└─────────────────────────────────────────────────┘
```

---

### Group 4: Session Analysis + Advanced Features (phases 13–16)

**Phase 13 — Session replay mode**
- Write `src/session/` module:
  - `mod.rs`: `SessionRunner`, `SessionConfig`, dispatch
  - `parse.rs`: JSONL line parser, validate `timestamp`/`method`/`url`/`status`, convert to `ResponseResult`
  - `timing.rs`: `TimingAnalytics` — inter-request gaps, burst detection, rate-limit event detection
- Wire into `main.rs`: `rapiscm session <file>` dispatches `SessionRunner`
- Session flow: parse JSONL → build `ResponseResult` vec → run check pipeline (sync+async, skip CORS/auth if no headers) → timing pass (if `--timing`) → report
- Round-trip: rapiscm JSONL output is valid session input
- Add `--timing`, `--max-parse-errors`, `--skip-cors`, `--skip-auth` flags
- JSONL format documented in SPEC.md §15.2
- Verify: create test JSONL files, run session replay, compare check results with live scan

**Phase 14 — (planned)** Sitemap discovery from robots.txt, sitemap.xml, common paths
**Phase 15 — (planned)** Authentication replay: session cookie injection, token refresh
**Phase 16 — (planned)** Enhanced security checks: CORS wildcard detection, info disclosure

## Quality Cycle detail

After each group of 4 phases, run this 3 times:

| Step | What |
|------|------|
| 1. Review | Read all files in scope. Note logic errors, missing edge cases, wrong types. |
| 2. Test | `cargo test --all`. Fix failures. |
| 3. Deep review | Line-by-line. Check unwraps, error paths, race conditions, resource leaks. |
| 4. Test | `cargo test --all` + add edge-case tests uncovered in deep review. |
| 5. Optimize | Profile allocation-heavy paths, `clone()` calls, unnecessary `String` allocations. Replace with `Cow`, references. |
| 6. Refactor | Extract duplicate logic, shrink large functions, align module boundaries. |
| 7. Test | `cargo test --all` + `cargo clippy`. Regression check. |
| 8. Exploit | Deliberately try to break it: empty input, malformed spec, unreachable host, infinite redirects, concurrent hammer. Fix each break. |

Each turn produces either fixes or confidence. After 3 turns, move to next phase group.

---

## Summary timeline

```
Phase  1  2  3  4  │QC│  5  6  7  8  │QC│  9 10 11 12  │QC│ 13 14 15 16  │QC│
Week   1  1  2  2   2W  3  3  4  4   4W  5  5  6  6   6W  7  7  8  8   8W
```
