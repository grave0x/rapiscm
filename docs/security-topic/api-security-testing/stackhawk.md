# StackHawk — Developer-First API DAST

CI/CD-native dynamic application security testing platform built on OWASP ZAP. Designed to run in pipelines, report findings to developers, and enforce security gates before merge.

## How It Works

StackHawk is a **wrapped + extended ZAP** with a developer-friendly CLI, cloud reporting, and opinionated defaults. It does not replace ZAP's engine — it operationalizes it.

### Architecture

```
Pipeline (GA/GitLab/Jenkins)
   ↓
hawk scan --api-key ${KEY} --openapi-file openapi.yaml
   ↓
StackHawk CLI (wraps ZAP engine)
   ├── Parses OpenAPI/GraphQL spec → extracts endpoints + params
   ├── Applies authentication context (from stackhawk.yml)
   ├── Runs active scan per ZAP scan policy
   ├── Deduplicates by anomaly fingerprint
   └── Uploads results to cloud dashboard
   ↓
StackHawk Cloud Dashboard
   ├── Findings grouped by endpoint, severity, CWE
   ├── Diff view across scan runs
   ├── Jira/GitHub Issues integration
   ├── SLA tracking
   └── API for export to SIEM
```

### StackHawk Configuration (`stackhawk.yml`)

```yaml
app:
  applicationId: my-app-id        # from cloud dashboard
  host: https://staging.example.com
  env: staging

auth:
  type: BEARER                    # BEARER | HEADER | FORM | OAUTH2
  token: ${HAWK_AUTH_TOKEN}

api:
  openApiFilePath: openapi.yaml   # Auto-discover endpoints
  # OR graphQlEndpoint: https://api.example.com/graphql

  scan:
    policy: API Scanning          # ZAP policy name or custom
    appendToPolicy:               # Additional rules
      rules:
        - id: 40012              # Cross Site Scripting (Reflected)
          alertThreshold: HIGH
    concurrency: 5
    maxDurationMinutes: 30
```

### Detection Pipeline

1. **Passive analysis** — all ZAP passive scan rules run against discovered endpoints
2. **Active scanning** — injection, XSS, SSRF, auth bypass, path traversal payloads
3. **Fingerprinting** — responses hashed to create anomaly signatures; same response = same finding deduplicated
4. **Severity mapping** — findings mapped to CWE + severity based on response pattern
5. **No proof-based validation** (unlike Invicti) — findings are probabilistic, categorized as "Potential" unless manually verified

### Authentication Methods

| Method | Config |
|--------|--------|
| Bearer token | `auth.type: BEARER` + token |
| Header | `auth.type: HEADER` + `headerName: X-API-Key` |
| Form login | `auth.type: FORM` + login endpoint + credentials |
| OAuth2 client credentials | `auth.type: OAUTH2` + token URL + client ID/secret |

### Scan Policies (Pre-Built)

| Policy | Coverage |
|--------|----------|
| `API Scanning` | Default OWASP API Top 10 coverage (6/10 risks) |
| `Quick Scan` | Passive + critical active only (~5 min) |
| `Full Scan` | All ZAP active scan rules (~30-60 min) |
| `GraphQL Scanning` | Introspection → query depth, alias brute-force, batch attack |

## Manual

### CLI

```bash
# Install
curl -sSL https://get.stackhawk.com | bash

# Quick scan
hawk scan --api-key ${HAWK_API_KEY}

# Scan with specific config
hawk scan --api-key ${HAWK_API_KEY} --config stackhawk.yml

# Scan with OpenAPI override
hawk scan --api-key ${HAWK_API_KEY} --openapi-file ./openapi.yaml

# Validate config (no scan)
hawk validate --config stackhawk.yml

# Output local JSON (no cloud upload)
hawk scan --offline --output results.json
```

### GitHub Actions

```yaml
- name: StackHawk Scan
  uses: stackhawk/hawkscan@v2
  with:
    apiKey: ${{ secrets.HAWK_API_KEY }}
    configurationFiles: stackhawk.yml
```

### GitLab CI

```yaml
stackhawk-scan:
  image: stackhawk/hawkscan:latest
  script:
    - hawkscan -k $HAWK_API_KEY -f stackhawk.yml
```

### Docker

```bash
docker run --rm \
  -e HAWK_API_KEY=${HAWK_API_KEY} \
  -v "$PWD/stackhawk.yml:/stackhawk.yml" \
  stackhawk/hawkscan:latest
```

## Build

StackHawk CLI wraps ZAP — the core scanning engine is OWASP ZAP (open source). The CLI + cloud backend are closed-source.

```bash
# Not directly buildable from source.
# The underlying ZAP engine can be built:
git clone https://github.com/zaproxy/zaproxy.git
cd zaproxy && ./gradlew build
```

## Install

### Linux / macOS

```bash
# Script (recommended)
curl -sSL https://get.stackhawk.com | bash

# Or download binary
wget https://download.stackhawk.com/hawk/linux/hawk-linux-x64.tar.gz
tar xzf hawk-linux-x64.tar.gz
sudo mv hawk /usr/local/bin/
```

### Docker

```bash
docker pull stackhawk/hawkscan:latest
```

### Package Managers

| Manager | Command |
|---------|---------|
| Homebrew | `brew install stackhawk/tap/hawkscan` |
| Script | `curl -sSL https://get.stackhawk.com \| bash` |

## Package

| Artifact | Distribution |
|----------|-------------|
| CLI binary (Linux/macOS) | https://download.stackhawk.com/hawk/ |
| Docker image | Docker Hub (`stackhawk/hawkscan`) |
| Homebrew | `stackhawk/tap/hawkscan` |
| GA action | GitHub Marketplace (`stackhawk/hawkscan`) |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://stackhawk.com/ |
| Documentation | https://docs.stackhawk.com/ |
| GitHub | https://github.com/stackhawk/hawkscan |
| Getting Started | https://docs.stackhawk.com/getting-started/ |
| Configuration reference | https://docs.stackhawk.com/stackhawk.yml/ |
| GitHub Actions | https://github.com/marketplace/actions/hawkscan |
| Blog | https://stackhawk.com/blog/ |
| Pricing | https://stackhawk.com/pricing/ |
| Status | https://status.stackhawk.com/ |
