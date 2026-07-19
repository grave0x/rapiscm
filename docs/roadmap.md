# Roadmap

## Current status — v0.1.0

Working core pipeline with spec-mode and URL-mode scans. Task system, fuzzing, session replay, ghost mode, JS bundle scanning, and corporate domain discovery implemented. Output in table/JSON/Markdown/Doc formats. Browser discovery feature-gated.

## Short-term (next)

- [ ] **crates.io publish** — publish rapiscm to crates.io
- [ ] **Prebuilt binaries** — GitHub releases for Linux x86_64, aarch64
- [ ] **Homebrew tap** — `brew install rapiscm`
- [ ] **Docker image** — multi-arch Docker image for CI
- [ ] **OpenAPI 3.1 support** — full 3.1 schema parsing
- [ ] **SARIF export** — currently partial; complete SARIF 2.1 spec compliance
- [ ] **`rapiscm ip` mode** — TCP port scanning, service detection, OS fingerprint
- [ ] **`--deep-spec` backend** — technical breakdown YAML output
- [ ] **`tasks queue/run/status`** — batch scan queue with crash recovery
- [ ] **Fuzz mode expansion** — param/method/header/body modes + wordlist modes

## Medium-term

- [ ] **GraphQL support** — schema introspection, query depth analysis, brute force
- [ ] **gRPC support** — proto file parsing, endpoint scanning
- [ ] **WebSocket support** — handshake analysis, message fuzzing
- [ ] **OAuth flow detection** — detect OAuth endpoints, test for misconfigurations
- [ ] **JWT analysis** — decode, validate signature, check algorithm confusion
- [ ] **Rate limit testing** — detect rate limits, test for bypasses
- [ ] **API key scanning** — detect leaked keys in responses
- [ ] **CVE matching** — match endpoints/versions against known CVEs
- [ ] **Plugin system** — dynamic check loading via WASM or dynamic libs

## Long-term

- [ ] **CI/CD integration** — GitHub Action, GitLab CI template
- [ ] **Continuous scanning** — daemon mode with scheduled re-scans
- [ ] **Dashboard UI** — web dashboard for scan history, trends, alerts
- [ ] **Multi-user** — team features, shared task storage, permissions
- [ ] **SLSA provenance** — signed builds, attestations
- [ ] **Fuzzing harness** — coverage-guided fuzzing of API parameters

## Known gaps

- Browser build time is long (chromiumoxide → chromium crate compile). Consider prebuilt CI artifacts.
- OpenAPI `$ref` resolution only handles local references — no remote `$ref` support yet.
- No `Content-Type: application/grpc` handling.
- No multipart form data for file upload endpoint testing.
- Session replay format is JSONL — no binary/Protobuf session support.
- `--resume` re-scans all previously failed endpoints; no granular retry.

## Completed milestones

- [x] Core scan pipeline (spec + URL mode)
- [x] Security checks (headers, CORS, auth, schema, trackers, cookies)
- [x] Output formatters (table, JSON, markdown)
- [x] Filter system (path, tag, method, status, expression)
- [x] Task persistence (save, list, show, delete, prune, export, diff)
- [x] Resume failed scans
- [x] Fuzzing engine (wordlist, match/filter, auto-calibrate)
- [x] Corporate domain discovery (crtsh, RDAP, favicon, ASN, Google, Shodan)
- [x] Session replay with timing analytics
- [x] Browser endpoint discovery (Chrome, Firefox)
- [x] Expression-based filter syntax
- [x] Git context capture for tasks
- [x] Tag endpoint filtering (include/exclude by OpenAPI tags)
- [x] Ghost mode (UA rotation, jitter, header randomization, proxy rotation)
- [x] JS bundle scanning (download + parse SPA bundles for API routes)
- [x] Browser JS eval (--eval for dynamic route extraction)
- [x] Page capture (rapiscm capture — HTML + screenshot + JS endpoints)
- [x] Structured docs output (-o doc for llm-api-style markdown)
- [x] Report HTML site (--report generates static HTML report site)
- [x] Cookie security analysis (Secure/HttpOnly/SameSite/expiry detection)
- [x] Expression filter syntax (tag:/status:/path:/method: with AND logic)
