# Rapiscm — remaining work todo

## Phase 0: Fix current build + finish in-progress

### Task 0.1: Fix compile errors
**Files:** `parser/js_bundle.rs`, `ghost.rs`, `cli.rs`, `config.rs`, `main.rs`, `scan/url.rs`, `scan/runner.rs`, `scan/browser.rs`
**Deps:** None
**Size:** M (5+ files)

- [ ] Fix `parser/js_bundle.rs` — return type mismatch (wrap in Ok()), fix test imports
- [ ] Fix `ghost.rs` — borrow errors in `build_client()`, make GhostState properly mutable
- [ ] Fix `cli.rs` — CrawlMode enum, Capture subcommand variant in from_cli
- [ ] Fix `config.rs` — new fields in both from_cli methods
- [ ] Fix `scan/url.rs` — ghost borrow + crawl_mode type
- [ ] Fix `scan/runner.rs` — jitter_pct capture in closure
- [ ] Fix `scan/browser.rs` — screenshot function imports
- [ ] Fix `main.rs` — Capture handler, ghost dispatching
- [ ] **Verify:** `cargo check` passes

### Task 0.2: Wire ghost mode fully
**Files:** `ghost.rs`, `scan/runner.rs`, `scan/url.rs`, `main.rs`
**Deps:** Task 0.1
**Size:** S (2-3 files)

- [ ] Ensure GhostState flows from CLI → config → scan pipeline
- [ ] Ghost headers applied to requests in ScanRunner + url fetch
- [ ] Jitter applied correctly in ScanRunner delay
- [ ] Proxy rotation works (round-robin through proxy list)
- [ ] **Verify:** `cargo test`, ghost tests pass

### Task 0.3: Wire browser JS eval
**Files:** `scan/browser.rs`, `scan/url.rs`, `main.rs`
**Deps:** Task 0.1
**Size:** S (2-3 files)

- [ ] `eval_and_extract()` works for Chrome (CDP)
- [ ] `eval_and_extract()` works for Firefox (WebDriver)
- [ ] `--eval` flag triggers in URL mode
- [ ] Returned URLs deduplicated and merged into scan list
- [ ] **Verify:** `cargo test` (requires browser feature)

### Task 0.4: Wire capture subcommand
**Files:** `main.rs`, `scan/browser.rs`
**Deps:** Task 0.1
**Size:** S (2 files)

- [ ] `rapiscm capture <url>` saves index.html
- [ ] `--extract` scans JS bundles, writes api_endpoints.txt
- [ ] `--screenshot` takes full-page screenshot (requires browser)
- [ ] **Verify:** manual test against https://example.com

### Task 0.5: Wire structured docs output
**Files:** `report/doc.rs`, `report/mod.rs`, `config.rs`
**Deps:** Task 0.1
**Size:** XS (1-2 files)

- [ ] `-o doc` produces llm-api-style markdown
- [ ] Contains endpoint table, security checks, tracker section, infrastructure notes
- [ ] Domain/host extracted from first result's URL
- [ ] **Verify:** `cargo test`

### Task 0.6: Verify tasks rebuild
**Files:** `main.rs`
**Deps:** None (already coded this session)
**Size:** XS (1 file)

- [ ] `rapiscm tasks rebuild <id>` re-scans failed endpoints
- [ ] `--all` re-scans everything
- [ ] Merges results, saves as updated task
- [ ] **Verify:** `cargo test`

### Checkpoint: Phase 0
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes
- [ ] `cargo fmt` passes
- [ ] Review with human before proceeding

---

## Phase 1: Analytics stubs

### Task 1.1: Cookie classification
**Files:** `analytics/cookies.rs`, `analytics/mod.rs`, `check/trackers.rs`
**Deps:** None
**Size:** S (2-3 files)

- [ ] `CookiePurpose` enum: Necessary, Preferences, Statistics, Marketing, Unclassified
- [ ] `classify_cookie(name: &str) -> CookiePurpose` with ~20 known cookie patterns
- [ ] Wired into check pipeline alongside existing tracker detection
- [ ] Shown in report output per endpoint
- [ ] **Verify:** `test_classify_cookie_ga`, `test_classify_cookie_fbp` tests pass

### Task 1.2: Data export detection
**Files:** `analytics/export.rs`, `analytics/mod.rs`, `check/trackers.rs`
**Deps:** Task 1.1
**Size:** S (2-3 files)

- [ ] Detect outbound data destinations from Set-Cookie domains, script src, img src, iframe src
- [ ] Match against known tracker/ad/analytics domains
- [ ] Count third-party connections per page
- [ ] **Verify:** `test_export_destinations` passes

### Task 1.3: Device profile reconstruction
**Files:** `analytics/profile.rs`, `analytics/mod.rs`
**Deps:** None
**Size:** S (1-2 files)

- [ ] Reconstruct device profile from response signals (UA → OS/browser, headers → language/encoding)
- [ ] `DeviceProfile` struct with fingerprint signals (UA, screen, color, timezone, language, fonts, WebGL, canvas)
- [ ] Stability score: high/medium/low based on signal consistency
- [ ] Bot likelihood heuristic
- [ ] **Verify:** test_device_profile produces reasonable output

### Task 1.4: Wire --tracker-report
**Files:** `cli.rs`, `config.rs`, `main.rs`, `report/mod.rs`
**Deps:** Tasks 1.1-1.3
**Size:** S (2-3 files)

- [ ] `--tracker-report` flag exists in CLI
- [ ] Produces detailed tracker analysis section in output
- [ ] Includes cookie breakdown by purpose, third-party connections, device profile
- [ ] **Verify:** `cargo test`

### Checkpoint: Phase 1
- [ ] `cargo test` passes
- [ ] Cookie classification works in scan output
- [ ] `--tracker-report` produces additional output

---

## Phase 2: Backend wiring

### Task 2.1: Tasks queue commands
**Files:** `cli.rs`, `main.rs`, `task/queue.rs`
**Deps:** None (queue.rs exists)
**Size:** S (2-3 files)

- [ ] `tasks queue <targets...>` adds items to queue.json
- [ ] `tasks queue --list targets.txt` adds from file
- [ ] `tasks run [--parallel N]` processes queue
- [ ] `tasks status` shows queue progress
- [ ] Crash recovery: running items check on startup
- [ ] **Verify:** `test_queue_add`, `test_queue_process`, `test_queue_crash_recovery` pass

### Task 2.2: --deep-spec backend
**Files:** `deepspec/mod.rs`, `cli.rs`, `config.rs`, `main.rs`, `scan/spec.rs`, `scan/url.rs`
**Deps:** None
**Size:** M (3-5 files)

- [ ] DeepSpec struct: endpoint dependency graph, response structure fingerprint
- [ ] Run after main scan completes
- [ ] Output YAML to stdout or file
- [ ] Integrates with task system (saved as artifact)
- [ ] **Verify:** `cargo test`

### Task 2.3: Config file validation
**Files:** `config.rs`
**Deps:** None
**Size:** XS (1 file)

- [ ] Validate rate_limit > 0, timeout >= 1, concurrency >= 1
- [ ] Warn on unknown config keys in config.toml
- [ ] Log warnings for suspicious values (rate_limit > 1000)
- [ ] **Verify:** `cargo test`

### Task 2.4: --save auto-naming
**Files:** `main.rs`
**Deps:** None
**Size:** XS (1 file)

- [ ] Default task name uses `{target-host}-{timestamp}` instead of `scan-{timestamp}`
- [ ] **Verify:** manual check

### Checkpoint: Phase 2
- [ ] `tasks queue` adds, `tasks run` processes, `tasks status` shows progress
- [ ] `--deep-spec` produces valid YAML
- [ ] Invalid config warns

---

## Phase 3: Expand subsystems

### Task 3.1: Fuzz mode dispatch
**Files:** `fuzz/mod.rs`, `fuzz/runner.rs`, `cli.rs`
**Deps:** None
**Size:** M (3-4 files)

- [ ] New `--mode` flag: path (current), param, method, header, body
- [ ] Param mode: fuzz query params and POST body fields
- [ ] Method mode: try all HTTP methods
- [ ] Header mode: fuzz custom header values
- [ ] Body mode: fuzz request body content type + structure
- [ ] **Verify:** `cargo test`

### Task 3.2: Wordlist modes
**Files:** `fuzz/runner.rs`, `fuzz/mod.rs`, `cli.rs`
**Deps:** Task 3.1
**Size:** M (2-3 files)

- [ ] Sniper mode (default): one wordlist, one position
- [ ] Pitchfork mode: N wordlists, parallel position i from each
- [ ] Clusterbomb mode: N wordlists, cartesian product
- [ ] **Verify:** `cargo test`

### Task 3.3-3.5: Script filters
**Files:** `script/mod.rs`, `script/rhai.rs`, `script/lua.rs`, `script/pipe.rs`, `cli.rs`, `filter/mod.rs`
**Deps:** `dep:mlua` (optional), `dep:rhai` (optional)
**Size:** L (5-8 files)

- [ ] `--script rhai:./check.rhai` — execute Rhai script, access endpoint/result variables
- [ ] `--script lua:./check.lua` — execute Lua via mlua, same interface
- [ ] `--script pipe:./check.py` — pipe endpoint data as JSON to subprocess, read modified result
- [ ] Scripts can add/modify checks, tags, severity
- [ ] Feature-gated: `script-rhai`, `script-lua`
- [ ] **Verify:** `cargo test --features script-rhai,script-lua`

### Checkpoint: Phase 3
- [ ] Fuzz works in all 5 modes
- [ ] Wordlist modes produce correct combinations
- [ ] Script filters execute checks

---

## Phase 4: Major modes

### Task 4.1-4.3: rapiscm ip
**Files:** `scan/ip.rs`, `scan/port.rs`, `scan/service.rs`, `cli.rs`, `config.rs`, `main.rs`
**Deps:** None (tokio net built-in)
**Size:** L (5-8 files)

- [ ] `rapiscm ip 10.0.0.1` — TCP connect scan on --top-ports (default 1000)
- [ ] CIDR range expansion: `10.0.0.0/24`
- [ ] Port range: `--ports 80,443,8000-8100`
- [ ] Concurrent host scanning (`--concurrent-hosts`)
- [ ] Banner grab + service fingerprint
- [ ] OS detection heuristics
- [ ] DNS reverse lookup
- [ ] MAC OUI vendor lookup
- [ ] **Verify:** `cargo test`

### Task 4.4-4.5: rapiscm proxy
**Files:** `proxy/mod.rs`, `proxy/ca.rs`, `proxy/handler.rs`, `proxy/rules.rs`, `cli.rs`, `Cargo.toml`
**Deps:** rcgen, rustls, tokio-rustls, pem (proxy feature); hyper, http, http-body, h2 (proxy-full)
**Size:** L (5-8 files)

- [ ] Dynamic root CA generation + TLS interception
- [ ] QR code + iOS mobileconfig served on proxy port
- [ ] Hyper-based body inspection (proxy-full)
- [ ] Rules engine (YAML config for modify/block/log)
- [ ] Auto-scan proxied endpoints (`--mitm-proxy-scan`)
- [ ] **Verify:** curl -x http://localhost:8080 https://example.com

### Task 4.6: rapiscm mitm
**Files:** `mitm/mod.rs`, `mitm/sniffer.rs`, `mitm/tcp.rs`, `cli.rs`, `Cargo.toml`
**Deps:** pcap, pnet (mitm-sniff feature)
**Size:** L (5-8 files)

- [ ] pcap live capture on interface
- [ ] BPF filter expression
- [ ] TCP reassembly + stream reconstruction
- [ ] TLS keylog decryption (SSLKEYLOGFILE)
- [ ] `--active`: ARP spoofing + IP forwarding
- [ ] Output PCAP file
- [ ] **Verify:** `cargo test --features mitm-sniff`

### Checkpoint: Phase 4
- [ ] `rapiscm ip localhost` shows open ports
- [ ] `rapiscm proxy` intercepts browser traffic
- [ ] `rapiscm mitm` captures packets
- [ ] All feature gates work

---

## Summary

| Phase | Tasks | Size | New Deps | LOC (est) |
|-------|-------|------|----------|-----------|
| 0: Fix build | 6 tasks | M | — | ~100 |
| 1: Analytics stubs | 4 tasks | S | — | ~260 |
| 2: Backend wiring | 4 tasks | M | — | ~530 |
| 3: Expand subsystems | 5 tasks | L | mlua, rhai | ~650 |
| 4: Major modes | 6 tasks | XL | rcgen, rustls, pcap, pnet, hyper | ~1,500 |
| **Total** | **25 tasks** | | **6 new deps** | **~3,040** |
