# Implementation Plan: rapiscm — remaining features

## Overview

Build all unbuilt features from SPEC.md across 6 phases. Phase 0 finishes current in-progress work (ghost mode, JS bundle scanning, browser eval, capture, doc output, tasks rebuild). Phases 1-4 cover tier-2 backend gaps, tier-1 major modes, and polish items.

## Architecture Decisions

- **Ghost mode** lives in its own module (`ghost.rs`), threaded through `ScanRunner` and `scan/url.rs` as optional state. No new deps — uses `rand`.
- **JS bundle scanning** extends `parser/js_bundle.rs`, integrates into `scan/url.rs` crawl step. No new deps — existing `regex`.
- **Browser eval** extends `scan/browser.rs` with `eval_and_extract()`. Feature-gated under `browser`.
- **IP mode** is a new subcommand with its own runner (`scan/ip.rs` + `scan/port.rs`). Uses `tokio::net::TcpStream` for connect scans.
- **Proxy/MITM** require new deps (rcgen, rustls, pcap). Feature-gated.
- **Task queue** extends existing `task/queue.rs` — CLI wiring only, no new logic.
- **Fuzz modes** extend `fuzz/runner.rs` — new mode dispatch, existing matchers.
- **Analytics stubs** fill in `analytics/cookies.rs`, `export.rs`, `profile.rs` from SPEC §13.
- **Deep-spec** is a new module `deepspec/mod.rs` — technical breakdown pass after scan.
- **Script filters** are a new module `script/mod.rs` — delegates to rhai/lua/pipe backends.

## Dependency graph

```
Phase 0 (fix current build)
  └── Fix compile errors in parser/js_bundle.rs, ghost.rs, cli.rs, config.rs, main.rs

Phase 1 (analytics stubs — no deps, isolated)
  ├── analytics/cookies.rs
  ├── analytics/export.rs
  ├── analytics/profile.rs
  └── --tracker-report wiring

Phase 2 (CLI-only backend wiring)
  ├── tasks queue/run/status (queue.rs exists, wire CLI)
  ├── --deep-spec (new module)
  ├── --tracker-report (flag → output)
  └── Config file validation

Phase 3 (expand existing subsystems)
  ├── Fuzz mode expansion (param/method/header/body)
  ├── Wordlist modes (sniper/pitchfork/clusterbomb)
  └── Script filters (rhai/lua/pipe)

Phase 4 (Major modes — new deps)
  ├── rapiscm ip <target> (tokio net only)
  ├── rapiscm proxy (rcgen, rustls, tokio-rustls, pem)
  └── rapiscn mitm <interface> (pcap, pnet)
```

## Task List

### Phase 0: Fix current build + finish in-progress features

- [ ] Task 0.1: Fix compile errors — js_bundle.rs borrow/type errors, ghost.rs borrow errors, cli.rs/config.rs mismatches
- [ ] Task 0.2: Wire ghost mode into scan pipeline fully (thread GhostState through ScanRunner)
- [ ] Task 0.3: Wire browser JS eval into URL mode
- [ ] Task 0.4: Wire capture subcommand handler
- [ ] Task 0.5: Wire structured docs output (-o doc)
- [ ] Task 0.6: Verify tasks rebuild works from CLI

### Checkpoint: Phase 0
- [ ] `cargo check` passes with no errors
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes
- [ ] All Phase 0 features functional

### Phase 1: Analytics stubs

- [ ] Task 1.1: `analytics/cookies.rs` — CookiePurpose enum, classify_cookie(), wired into check pipeline
- [ ] Task 1.2: `analytics/export.rs` — DataExport struct, detect outbound destinations from Set-Cookie / script src
- [ ] Task 1.3: `analytics/profile.rs` — DeviceProfile struct, reconstruct fingerprint signals from headers
- [ ] Task 1.4: Wire `--tracker-report` flag to produce detailed analytics output section

### Checkpoint: Phase 1
- [ ] `cargo test` passes
- [ ] Cookie classification works in scan output
- [ ] `--tracker-report` produces additional output

### Phase 2: Backend wiring

- [ ] Task 2.1: `tasks queue/run/status` CLI wiring — queue.rs exists, just needs clap subcommands + dispatch
- [ ] Task 2.2: `--deep-spec` backend — new `deepspec/mod.rs` module, technical breakdown YAML output
- [ ] Task 2.3: Config file validation — warn on unknown/missing fields, validate ranges
- [ ] Task 2.4: `--save` auto-naming — use target + timestamp instead of hardcoded prefix

### Checkpoint: Phase 2
- [ ] `tasks queue` adds items, `tasks run` processes, `tasks status` shows progress
- [ ] `--deep-spec` produces valid YAML output
- [ ] Invalid config produces actionable warnings

### Phase 3: Expand subsystems

- [ ] Task 3.1: Fuzz mode dispatch — add param/method/header/body modes to fuzz runner
- [ ] Task 3.2: Wordlist modes — sniper (default), pitchfork (parallel), clusterbomb (cartesian)
- [ ] Task 3.3: Script filters — `--script rhai:./check.rhai` executes custom check via rhai
- [ ] Task 3.4: Script filters — `--script lua:./check.lua` via mlua
- [ ] Task 3.5: Script filters — `--script pipe:./check.py` via stdin/stdout subprocess

### Checkpoint: Phase 3
- [ ] Fuzz works in all 5 modes
- [ ] Wordlist modes produce expected combinations
- [ ] Script filters execute and modify results

### Phase 4: Major modes

- [ ] Task 4.1: `rapiscm ip` — `scan/port.rs` TCP connect scanner, concurrent host/port iterator
- [ ] Task 4.2: `rapiscm ip` — Service fingerprint (banner grab), OS detection heuristics
- [ ] Task 4.3: `rapiscm ip` — CIDR parsing, DNS reverse, MAC vendor lookup
- [ ] Task 4.4: `rapiscm proxy` — TLS MITM proxy with dynamic root CA (feature-gated: proxy)
- [ ] Task 4.5: `rapiscm proxy` — Hyper-based body inspection + rules engine (feature-gated: proxy-full)
- [ ] Task 4.6: `rapiscm mitm` — pcap sniffer, BPF filter, TCP reassembly (feature-gated: mitm-sniff)

### Checkpoint: Phase 4
- [ ] `rapiscm ip 10.0.0.1` shows open ports and services
- [ ] `rapiscm proxy` intercepts HTTP/HTTPS traffic
- [ ] `rapiscm mitm eth0` captures and analyzes packets
- [ ] All feature gates work correctly

### Final Checkpoint
- [ ] All acceptance criteria met
- [ ] All tests pass, clippy clean, fmt clean
- [ ] README updated with new flags and subcommands
- [ ] SPEC.md updated to mark features as built

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| chromiumoxide API changes | Browser features break | Pin dependency version, test with --browser flag |
| pcap requires root/sudo | mitm sniffer unusable without elevated perms | Document requirement, detect at runtime |
| rhai/mlua API compatibility | Script filters fragile across versions | Pin deps, extensive test coverage |
| Scope creep on fuzz modes | Phase 3 blows up | Ship path+param modes first, method/header/body as follow-up |
| proxy TLS certs complex | High implementation effort | Use rcgen + rustls, test with real browser traffic |

## Open Questions

- Should `rapiscm ip` support SYN scan (requires raw sockets / root)? For now, connect scan only.
- Should proxy mode store decrypted traffic as sessions (JSONL)? Yes — aligns with session replay.
- What scope for deep-spec? Start with endpoint dependency graph + response structure fingerprint.
