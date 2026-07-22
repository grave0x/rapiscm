# Implementation Plan: rapiscm v0.2.0 — Distribution & Feature Expansion

## Overview

Ship rapiscm as a distributable tool (crates.io, binaries, Homebrew, Docker) and
add five new capabilities: OpenAPI 3.1 parsing, SARIF 2.1 compliance, IP/network
scanning mode, expanded fuzzing engine, and a batch task queue with crash recovery.

## Architecture Decisions

- **Feature gates**: IP mode behind `--features ip`; browser mode already gated;
  keep the default build lean.
- **OpenAPI 3.1 is additive**: Extend the existing parser rather than rewrite.
- **SARIF via hand-rolled JSON**: Avoid a heavy SARIF crate dependency; the spec
  fields needed are finite.
- **Queue reuses task store**: The batch queue extends `task/store.rs` rather than
  introducing a new storage backend.

## Task List

### Phase 0: Prefactor
- [ ] Task 0: Dead code audit — review 17 `#[allow(dead_code)]` sites, remove or gate

### Checkpoint: Prefactor
- [ ] `cargo clippy` clean, `cargo test` passes

### Phase 1: Distribution
- [ ] Task 1: crates.io publish + GitHub Actions binary builds
- [ ] Task 2: Homebrew tap + multi-arch Docker image

### Checkpoint: Distribution
- [ ] `cargo install rapiscm` works; `brew install` works; `docker run` works

### Phase 2: Features
- [ ] Task 3: OpenAPI 3.1 support
- [ ] Task 4: SARIF 2.1 compliance
- [ ] Task 5: `rapiscm ip` mode (TCP scan, service detect, OS fingerprint)
- [ ] Task 6: Fuzz mode expansion (param/method/header/body/wordlist)
- [ ] Task 7: Batch task queue (queue/run/status + crash recovery)

### Checkpoint: Features
- [ ] All new subcommands functional; `cargo test` passes

### Phase 3: Release
- [ ] Task 8: Version bump, CHANGELOG, tag, full CI green

### Checkpoint: Complete
- [ ] v0.2.0 released; all ACs met

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| crates.io name taken | High | Check availability early; have backup names ready |
| OpenAPI 3.1 parsing breaks 3.0 | High | Extensive regression tests; 3.0 test suite must stay green |
| IP mode requires root for raw sockets | Medium | Use TCP connect scan (no root needed); fallback message |
| SARIF validator is strict | Medium | Iterate with validator in CI; fail build on invalid output |

## Open Questions

- Should IP mode be a separate binary (`rapiscm-ip`) or integrated as a subcommand?
  Leaning: integrated subcommand with feature gate.
