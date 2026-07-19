# 42Crunch — API Security Platform

Shift-left API security platform that audits OpenAPI specs for misconfigurations *before code is written*, then performs dynamic conformance scanning against running endpoints. Foundational tool for the "spec-first security" approach.

## How It Works

42Crunch operates in two independent phases that complement each other:

### Phase 1: Static Spec Audit (Shift-Left)

A **lint engine** parses the OpenAPI/Swagger spec (JSON or YAML) and scores it against 150+ security rules organized into 4 categories:

| Category | What It Catches |
|----------|----------------|
| **Data Exposure** | Leaking internal model fields, missing `format`, overly permissive `additionalProperties`, schema recursion depth |
| **Auth & Access** | Missing or weak security schemes, permissive API key scopes, OAuth2 flow misconfig |
| **Server Security** | Missing TLS enforcement, CORS misconfig, unversioned paths, wildcard paths |
| **Schema & Validation** | Type mismatches, unconstrained array sizes, nullable misuse, missing required fields |

Each check returns:
- **Severity** (info / low / medium / high / critical)
- **Location** (exact JSON path in spec)
- **Suggestion** (replacement YAML/JSON snippet)
- **WSTG mapping** (OWASP testing guide reference)

A global **audit score** (0–100) is computed from all findings. Scores below 70 are considered high risk.

### Phase 2: Dynamic API Conformance Scan

Deploys lightweight **API Conformance Scanner** (Docker container) that reads the OpenAPI spec and sends attack sequences against the live API endpoint:

1. **Schema enforcement** — sends valid, invalid, extra, missing fields for each endpoint; detects mass-assignment and excessive data exposure
2. **Auth enforcement** — sends requests without auth, with expired tokens, with invalid signatures
3. **Injection testing** — SQLi, NoSQLi, OS command injection, XSS, SSRF probes on every parameter
4. **CRUD abuse** — tests PUT/POST/DELETE endpoints for unauthorized modification
5. **Protocol abuse** — HTTP method override, content-type switching, header smuggling

Scans follow a **conformance profile** that defines allowed HTTP status codes per endpoint. Any response outside the expected range is flagged.

### Audit Trail & Compliance

All findings are stored in the 42Crunch cloud console (or on-prem) with:
- Timestamp and scanner identity (who ran what when)
- Diff tracking between spec versions
- SLAs per severity (configurable)
- SIEM export (JSON, Jira, ServiceNow, Splunk)

## Manual

### CLI — Static Audit

```bash
# Install
npm install -g @42crunch/cli

# Audit a spec
42c audit petstore.yaml --output report.json --format json

# CI-friendly exit codes
42c audit petstore.yaml --fail-on high
echo $?   # 0 = pass, 1 = fail

# Compare two spec versions
42c diff v1.yaml v2.yaml
```

### Docker — Conformance Scan

```bash
# Run conformance scanner
docker run -v "$PWD:/data" 42crunch/api-security-audit \
  --spec /data/petstore.yaml \
  --target https://api.staging.example.com \
  --concurrency 5 \
  --output /data/scan-results.json

# Test single endpoint
docker run 42crunch/api-security-audit \
  --spec /data/petstore.yaml \
  --target https://api.staging.example.com \
  --filter-path "/api/v2/users" \
  --filter-method POST
```

### IDE Integration

- **VS Code Extension** — live linting on save; shows severity + fix inline
- **IntelliJ Plugin** — spec audit as you type
- **JetBrains Gateway** — remote dev support

## Build

42Crunch is **closed-source SaaS** (cloud console + Docker scanner). Not buildable from source.

- Cloud console: https://platform.42crunch.com
- On-prem: Air-gapped appliance available for enterprise (Docker Compose stack: audit engine + scanner + MongoDB + console)

## Install

### 42c CLI

```bash
npm install -g @42crunch/cli
# Verify
42c version
```

Requires a 42Crunch API token from platform.42crunch.com → Profile → API Keys.

### API Conformance Scanner

```bash
docker pull 42crunch/api-security-audit
```

No other install methods — scanner is Docker-only.

### CI Integrations

| Platform | Method |
|----------|--------|
| GitHub Actions | `42crunch/action-audit` @ marketplace |
| GitLab CI | `42crunch/api-security-audit` Docker image |
| Jenkins | Freestyle job with Docker build step |
| CircleCI | Orb: `42crunch/audit` |

## Package

| Artifact | Distribution |
|----------|-------------|
| CLI (`@42crunch/cli`) | npm registry |
| Scanner Docker image | Docker Hub (`42crunch/api-security-audit`) |
| VS Code extension | VS Code Marketplace |
| IntelliJ plugin | JetBrains Marketplace |

No Homebrew / apt / rpm packages.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://42crunch.com/ |
| Platform (login) | https://platform.42crunch.com |
| Documentation | https://docs.42crunch.com/ |
| CLI reference | https://docs.42crunch.com/latest/cli/cli-reference |
| Docker scanner | https://hub.docker.com/r/42crunch/api-security-audit |
| GitHub Actions | https://github.com/marketplace/actions/42crunch-api-security-audit |
| VS Code extension | https://marketplace.visualstudio.com/items?itemName=42Crunch.vscode-openapi |
| Blog | https://42crunch.com/blog/ |
| OWASP API Top 10 mapping | https://docs.42crunch.com/latest/audit/rules/owasp-api-top-10 |
| Pricing | https://42crunch.com/pricing/ |
