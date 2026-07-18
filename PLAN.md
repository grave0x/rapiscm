# rapiscm — build plan

## Status

9 commits, 40+ source files across ~20 subsystems. Core scan pipeline (spec + url) works.  
Fuzz, session replay, task system, discovery engine, tracker analytics, tag/filter/extract all built.

---

## Dependencies (actual)

### Runtime (13)

```
clap (derive)      → CLI
tokio (full)       → async runtime
reqwest (json)     → HTTP client
serde + serde_json → spec parse + JSON output
serde_yaml         → YAML spec parse
openapiv3          → OpenAPI 3.0/3.1 types
anyhow             → error propagation
thiserror          → typed errors
tracing            → structured logging
tracing-subscriber → env-filter, JSON log format
regex              → path matching, tracker patterns
futures-util       → async combinators (fuzz runner)
toml               → config file parse
```

### Dev

```
tempfile   → temp dirs in tests
wiremock   → mock HTTP server in tests
```

### Feature-gated

| Feature | Deps | Status |
|---------|------|--------|
| `browser` | chromiumoxide, fantoccini | Not built |

---

## Module tree (actual)

```
src/
  main.rs              # dispatch to all 7 subcommands + --save
  cli.rs               # clap: 7 subcommands, 50+ flags
  config.rs            # ScanConfig from CLI + env + config.toml
  types.rs             # Endpoint, ResponseResult, Check, Target, ...
  error.rs             # Error enum (thiserror)
  util.rs              # ISO timestamp (no chrono)

  scan/
    mod.rs
    runner.rs          # ScanRunner: concurrent HTTP + semaphore rate-limit
    spec.rs            # spec-mode: parse → filter → tag → scan → check
    url.rs             # url-mode: fetch → extract(crawl) → probe(wordlist) → scan

  parser/
    mod.rs
    spec.rs            # openapiv3 → Vec<Endpoint>
    url.rs             # URL normalize, join, fingerprint

  check/
    mod.rs             # run_checks sync + run_async_checks
    security.rs        # CSP, HSTS, X-CT-O, X-FO, Cache-Control
    cors.rs            # OPTIONS + Origin: evil.com
    auth.rs            # re-request without auth → 200 = no enforcement
    schema.rs          # status match + valid JSON

  report/
    mod.rs             # format_results dispatcher
    table.rs           # ANSI-colored terminal
    json.rs            # serde_json pretty
    md.rs              # markdown with summary + tables
    summary.rs         # Stats aggregation

  tag/
    mod.rs             # tag_endpoint(): rest, v1/v2/v3, admin, graphql,
                       #   auth-required, open, health, static, deprecated

  filter/
    mod.rs             # method/path/status/tag include + exclude,
                       #   endpoint_passes(), result_passes()

  extract/
    mod.rs
    html.rs            # href, form action extraction
    js.rs              # string/URL pattern extraction
    json.rs            # URL values from JSON responses
    sitemap.rs         # robots.txt, sitemap.xml parsing

  fuzz/
    mod.rs             # run_fuzz_scan, FuzzOpts
    runner.rs          # FuzzRunner: path fuzzing loop
    matcher.rs         # MatchConfig: status/size/regex + baseline + Range type
    wordlist.rs        # built-in API path wordlist (~100 entries)

  discover/
    mod.rs             # DiscoverConfig, run_discover, save_report
    crtsh.rs           # crt.sh Certificate Transparency search
    rdap.rs            # ARIN RDAP reverse WHOIS
    asn.rs             # BGPView ASN → IP → reverse DNS
    search.rs          # Google Custom Search dork-based discovery
    favicon.rs         # Shodan favicon hash search
    gaid.rs            # Google Analytics ID pivot (stub)

  analytics/
    mod.rs             # TrackerAnalysis, run_tracker_analysis
    detect.rs          # ~200-entry tracker signature DB + matching
    cookies.rs         # NOT BUILT
    export.rs          # NOT BUILT
    profile.rs         # NOT BUILT

  session/
    mod.rs             # SessionRunner, SessionConfig, dispatch
    parse.rs           # JSONL line parser → ResponseResult
    timing.rs          # TimingAnalytics: gaps, bursts, rate-limit events, percentiles

  task/
    mod.rs             # TaskMeta, ResultSummary, StorageInfo
    store.rs           # JSON file store: save/load/list/delete/prune
    index.rs           # index.json management
    queue.rs           # queue.json management, crash recovery
    rebuild.rs         # re-run from task.json config
    diff.rs            # TaskDiff algorithm + classification
    export.rs          # export to md/sarif/html
    resume.rs          # checkpoint-based partial re-run
```

---

## What's built (by commit)

| Commit | Subsystems |
|--------|-----------|
| `b9c4281` initial | Foundation, CLI, types, config, errors, scan runner, spec parser, URL parser, security checks, CORS, auth, schema, table/json/md/summary reports, tag engine, extract engine (html/js/json/sitemap), fuzz runner + matcher + wordlist, session replay + parse + timing, task store + index + queue + rebuild + diff + export + resume, init_logging |
| `13f47e9` | Crawl mode (`--crawl` + `--depth`), recursive BFS discovery |
| `8d43ab6` | Filter engine (method/path/status/tag includes + excludes) |
| `a0a9721` | Discovery engine: crtsh, rdap, asn, search, favicon, gaid |
| `3505757` | Tracker analytics: detect.rs (~200 sigs), wired into pipeline |
| `b134083` | Session replay refactor, fuzz runner refactor |
| `01cbf08` | Dead code cleanup, tracker reporting |
| `dc018b5` | Clippy fixes |

---

## What's NOT built (gaps vs SPEC.md)

### Tier 1 — announced CLI commands, zero code

| Feature | SPEC § | LOC | Why |
|---------|--------|-----|-----|
| `rapiscm ip <target>` | §2.5 | ~400 | Port scan, service/OS detect, DNS, CIDR |
| `rapiscm proxy` | §2.6 | ~600 | TLS MITM proxy, dynamic CA, hyper bridge |
| `rapiscm mitm <iface>` | §2.7 | ~500 | pcap sniffer, BPF, TCP reassembly |
| `--git` integration | §12.10 | ~50 | GitInfo type exists, capture_git_info unwired |

### Tier 2 — CLI flags exist, backend incomplete

| Feature | SPEC § | LOC | Blockers |
|---------|--------|-----|----------|
| `--resume` | §12.11 | ~80 | resume.rs exists, not wired in cli.rs config |
| `tasks queue` / `run` / `status` | §1 | ~150 | queue.rs exists, CLI unwired |
| Fuzz modes (param/method/header/body) | §2.4 | ~300 | Only path mode works |
| Wordlist modes (sniper/pitchfork/clusterbomb) | §2.4 | ~150 | Only basic wordlist load |
| `--deep-spec` | §1 | ~300 | Flag exists in CLI, no backend |
| Expression filters (`tag:rest AND tag:v2`) | §1 | ~100 | Filter engine exists, syntax parser unwired |
| Script filters (rhai/lua/pipe) | §1 | ~200 | No implementation |
| `analytics/cookies.rs` | §13.4 | ~80 | Cookie classification stubbed |
| `analytics/export.rs` | §13.5 | ~100 | Data export detection |
| `analytics/profile.rs` | §13.6 | ~80 | Device profile reconstruction |
| Browser feature (chromiumoxide/fantoccini) | §4 | ~300 | Feature-gated deps listed, no code |
| Ghost mode | §13.1 | ~200 | Flag exists, no logic |

### Tier 3 — spec-lifecycle polish

| Feature | LOC | Why |
|---------|-----|-----|
| `--corp` auto-detect + dedup | ~50 | Already works for single target, no batch |
| Config file validation | ~80 | Missing fields silently default |
| `--save` auto-naming | ~30 | Uses hardcoded `scan-{timestamp}` prefix |
| Rate-limit tuning | ~40 | Semaphore-based, no adaptive |
| Test coverage for session/task | ~200 | ~30 existing tests, gaps in edge cases |

---

## Next iteration plan

### Wave A — finish what's wired in CLI (shallow work)

1. `--git` integration (50 LOC) — call `capture_git_info()` in main.rs save path
2. `analytics/{cookies,export,profile}.rs` (260 LOC) — 3 files from SPEC.md
3. `--deep-spec` backend (300 LOC) — technical breakdown pass after scan
4. `--resume` wiring (80 LOC) — config parse + dispatch in main.rs

### Wave B — CLI commands that exist but need backend

5. Task queue commands (`tasks queue/run/status`) — 150 LOC
6. Fuzz modes (param, method, header, body) — 300 LOC
7. Expression filters — 100 LOC

### Wave C — new major features

8. IP mode (`rapiscm ip`) — 400 LOC
9. Proxy mode (`rapiscm proxy`) — 600 LOC + 4 new deps
10. MITM sniffer (`rapiscm mitm`) — 500 LOC + 2 new deps

---

## File map (all 41 source files)

```
src/main.rs           (344)    src/types.rs          (225)
src/cli.rs            (463)    src/config.rs         (340)
src/error.rs          ( 40)    src/util.rs           ( 62)

src/scan/mod.rs       ( 10)    src/scan/runner.rs    (225)
src/scan/spec.rs      ( 76)    src/scan/url.rs       (223)

src/parser/mod.rs     (  2)    src/parser/spec.rs    (293)
src/parser/url.rs     ( 61)

src/check/mod.rs      ( 51)    src/check/security.rs (114)
src/check/cors.rs     (112)    src/check/auth.rs     (100)
src/check/schema.rs   ( 74)

src/report/mod.rs     ( 16)    src/report/table.rs   (166)
src/report/json.rs    ( 21)    src/report/md.rs      ( 98)
src/report/summary.rs ( 70)

src/tag/mod.rs        (202)

src/filter/mod.rs     (190)

src/extract/mod.rs    ( 30)    src/extract/html.rs   ( 63)
src/extract/js.rs     ( 35)    src/extract/json.rs   ( 29)
src/extract/sitemap.rs(107)

src/fuzz/mod.rs       ( 96)    src/fuzz/runner.rs    (123)
src/fuzz/matcher.rs   (203)    src/fuzz/wordlist.rs  (120)

src/discover/mod.rs   (205)
src/discover/crtsh.rs (155)    src/discover/rdap.rs  (179)
src/discover/asn.rs   (239)    src/discover/search.rs(111)
src/discover/favicon.rs(102)   src/discover/gaid.rs  ( 22)

src/analytics/mod.rs  (  2)    src/analytics/detect.rs(157)

src/session/mod.rs    (130)    src/session/parse.rs  (131)
src/session/timing.rs (593)

src/task/mod.rs       (322)    src/task/store.rs     (265)
src/task/index.rs     (178)    src/task/queue.rs     (207)
src/task/rebuild.rs   (220)    src/task/diff.rs      (238)
src/task/export.rs    (227)    src/task/resume.rs    (244)
```

(LOC in parens, `wc -l` counts. Total: ~7,600 lines of Rust.)
