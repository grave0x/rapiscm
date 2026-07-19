# Akto — Open-Source API Security Platform

Open-core API security platform combining traffic-based API discovery, automated testing, and runtime posture monitoring. Freemium model — core features free, enterprise features paid.

## How It Works

Akto uses **traffic mirroring** to learn API behavior, then runs tests against the observed traffic, not against a spec.

### Architecture

```
API Gateway / Sidecar
       │
       ▼
Traffic Mirror ──→ Akto Collector ──→ MinIO (S3) ──→ MongoDB ──→ API Dashboard
  (real-time      (parses HTTP         (raw traces)   (metadata,    (UI: endpoints,
   HTTP traffic)   req/resp)                           test results) vulnerabilities)
```

### Discovery

Observes traffic and builds an API inventory **without requiring an OpenAPI spec**:

| Feature | Detail |
|---------|--------|
| **Endpoint inventory** | All observed URL paths + methods + params |
| **Sensitive data detection** | PII, tokens, credentials, internal IDs in request/response bodies |
| **Authentication mapping** | Which endpoints carry auth headers vs. which are exposed |
| **Shadow API detection** | Endpoints with no matching doc or spec reference |
| **Traffic patterns** | Call frequency, error rates, latency percentiles per endpoint |

### Testing Modules

| Module | Vulnerability | Cost |
|--------|--------------|------|
| **BOLA/IDOR** | Object-level authorization — replace IDs across users | Free |
| **Authentication** | Weak tokens, missing auth, expired sessions | Free |
| **Rate limiting** | Brute-force protection bypass | Free |
| **Mass assignment** | Extra field injection | Free |
| **XSS, SQLi, SSTI** | Injection testing | Free |
| **GraphQL** | Introspection, depth-limit, alias brute-fource | Enterprise |
| **Business logic** | Custom flow-based testing | Enterprise |
| **JWT/SSRF** | JWT alg none, debug endpoints | Enterprise |

### Testing Modes

| Mode | How It Works |
|------|-------------|
| **Passive** | Observes traffic, no malicious payloads. Flags sensitive data exposure, missing auth, CORS misconfigs |
| **Active** | Replays requests with modified parameters. Requires confirmation (sandbox/staging recommended) |
| **Scheduled** | Recurring scans (daily/weekly). Diffs results to detect regressions |

## Manual

### Quick Start (SaaS)

```bash
# 1. Sign up at https://app.akto.io
# 2. Install collector in your infrastructure

# Docker collector
docker run -d --name akto-collector \
  -e AKTO_API_KEY=${API_KEY} \
  -e AKTO_API_URL=https://app.akto.io \
  aktoane/akto-collector:latest

# 3. Traffic starts flowing into the dashboard automatically
```

### Docker Compose (Self-Hosted)

```bash
git clone https://github.com/akto-api-security/akto.git
cd akto/deployment/docker-compose
docker-compose up -d

# Access UI: http://localhost:9090
# Default: admin@akto.io / admin123 (change immediately)
```

### Kubernetes Sidecar

```yaml
# Sidecar annotation in your pod spec
annotations:
  sidecar.akto.io/inject: "enabled"
  sidecar.akto.io/api-key: "${AKTO_API_KEY}"
```

### CLI (Traffic Replay)

```bash
# Replay a recorded traffic file for offline testing
akto replay --input traffic.har --output results.json

# Run specific test module
akto test --endpoint "GET /api/v2/users" --module bola
```

## Build

```bash
# Prerequisites: Java 11+, Node 18+, MongoDB, MinIO
git clone https://github.com/akto-api-security/akto.git
cd akto

# Backend (Java)
cd apps/server && ./gradlew build

# Frontend (React)
cd ../dashboard && npm install && npm run build

# Dashboard (React Native for local)
cd ../ui && npm install && npm run build
```

## Install

| Method | Command |
|--------|---------|
| Docker Compose (self-host) | `git clone` + `docker-compose up` |
| Kubernetes (Helm) | `helm repo add akto && helm install akto/akto` |
| AWS AMI | Via AWS Marketplace |
| SaaS | Sign up at app.akto.io — zero install |

## Package

| Component | Distribution |
|-----------|-------------|
| Backend (Java) | Docker image (`aktoane/akto-backend`) |
| Dashboard | Docker image (`aktoane/akto-dashboard`) |
| Collector | Docker image (`aktoane/akto-collector`) |
| Helm chart | GitHub + ArtifactHUB |
| Source | GitHub (`akto-api-security/akto`) |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.akto.io/ |
| GitHub | https://github.com/akto-api-security/akto |
| Documentation | https://docs.akto.io/ |
| SaaS console | https://app.akto.io |
| Self-host guide | https://docs.akto.io/getting-started/self-host |
| Testing modules | https://docs.akto.io/testing/test-modules |
| Blog | https://www.akto.io/blog |
| Slack community | https://www.akto.io/community |
