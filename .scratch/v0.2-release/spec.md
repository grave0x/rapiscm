---
labels: [ready-for-agent]
---

# Spec: rapiscm v0.2.0 — Distribution & Feature Expansion

## Problem Statement

rapiscm v0.1.0 is functionally complete for single-machine use — it scans APIs,
finds security issues, and produces reports. But it is not installable by anyone
other than the developer (no crate, no binaries, no Docker image), it only
supports OpenAPI 3.0 (not 3.1), its SARIF export is partial, and it lacks three
major features planned from the start: IP/network scanning mode, a batch task
queue, and expanded fuzzing capabilities.

## Solution

Ship v0.2.0 as a **distributable tool with four new capabilities**:

1. **Distribution**: crates.io publish, prebuilt Linux binaries (x86_64 + aarch64),
   Homebrew tap, multi-arch Docker image.
2. **OpenAPI 3.1**: full schema parsing with 3.1-specific features (nullable as
   first-class, path items as $ref targets, webhooks).
3. **SARIF 2.1 compliance**: complete the partial SARIF export to pass the SARIF
   validator.
4. **rapiscm ip mode**: TCP port scanning, service version detection, OS
   fingerprinting — analogous to a lightweight nmap for API-relevant ports.
5. **Fuzz expansion**: parameter fuzzing, HTTP method fuzzing, header fuzzing,
   body fuzzing, and wordlist-driven fuzzing modes.
6. **Batch task queue**: `tasks queue/run/status` with crash recovery — queue
   multiple scans, run them sequentially, resume on failure.

## User Stories

1. As a security engineer, I want to `cargo install rapiscm` so that I can use
   it without cloning the repo.
2. As a CI pipeline operator, I want a prebuilt Linux binary to drop into a
   Dockerfile so that I don't need the Rust toolchain.
3. As a macOS user, I want `brew install rapiscm` so that it stays updated with
   my system package manager.
4. As a Kubernetes operator, I want a multi-arch Docker image so that I can run
   rapiscm in my arm64 clusters.
5. As an API developer using OpenAPI 3.1, I want rapiscm to parse my spec without
   errors so that I can scan my 3.1-defined APIs.
6. As a security auditor, I want SARIF output that passes the SARIF validator so
   that I can import results into GitHub Code Scanning.
7. As a penetration tester, I want `rapiscm ip <target>` to discover open ports
   and running services so that I can map the attack surface of a target host.
8. As a red team operator, I want to fuzz API parameters with custom wordlists so
   that I can discover hidden endpoints and edge cases.
9. As a SOC analyst running recurring scans, I want to queue multiple scans and
   have them run in order with crash recovery so that I don't lose work on failure.
10. As a developer integrating rapiscm, I want `rapiscm tasks status` to show
    progress and estimated completion time so that I can plan my workflow.

## Implementation Decisions

- **Distribution**: Use `cargo release` for crates.io. GitHub Actions matrix for
  prebuilt binaries (build on ubuntu-latest, target x86_64-unknown-linux-gnu and
  aarch64-unknown-linux-gnu). Homebrew formula in a dedicated tap repo. Docker
  multi-arch via `docker buildx`.
- **OpenAPI 3.1**: Extend the existing `parser/spec.rs` to handle 3.1-specific
  JSON Schema constructs. The 3.1 spec is a superset of 3.0; existing 3.0 parsing
  must not regress.
- **SARIF**: Complete the existing `report/` SARIF path to emit valid SARIF 2.1.
  Target the `sarif-rs` crate or hand-roll the remaining fields. Validate against
  the SARIF SDK validator.
- **IP mode**: New `scan/ip.rs` module. Use raw sockets or a TCP connect scan.
  Service detection via banner grabbing on common ports. OS fingerprint via TCP
  stack analysis (TTL, window size, options ordering). Feature-gated behind
  `--features ip` to avoid bloat.
- **Fuzz expansion**: Refactor `fuzz/runner.rs` to accept a `FuzzMode` enum
  (Param, Method, Header, Body, Wordlist). Each mode generates payloads differently.
  Wordlist mode reads from `--wordlist <file>`.
- **Batch queue**: Extend the existing `task/` system with a `queue` submodule.
  Use the same JSONL-based persistence as the existing task store. A queue runner
  reads pending tasks and executes them sequentially. Crash recovery re-reads the
  task file and marks interrupted tasks as failed.

## Testing Decisions

- Good tests assert external behaviour: parse output, scan results, SARIF validity,
  queue ordering.
- Distribution is tested via CI (builds pass for all targets, Docker image starts).
- IP mode tests mock network responses or use localhost listeners.
- Fuzz tests use known-good wordlists and assert expected hit counts.
- Prior art: existing unit tests in `tests/` for the scan pipeline, filter system,
  task store. Follow the same pattern.

## Out of Scope

- GraphQL, gRPC, WebSocket support (medium-term roadmap)
- OAuth flow detection, JWT analysis, CVE matching (medium-term)
- Plugin system (long-term)
- CI/CD integration as GitHub Action (long-term)
- Dashboard UI (long-term)
- Remote `$ref` resolution in OpenAPI specs (known gap, deferred)

## Further Notes

- 17 instances of `#[allow(dead_code)]` exist, mostly from feature-gated browser
  code. These should be audited as part of the release — any truly dead paths
  should be removed.
- The existing `docs/roadmap.md` has 25 unchecked short-term items. This spec
  covers the highest-priority subset. Remaining items (e.g. `--resume` granular
  retry) are candidates for v0.2.1.
