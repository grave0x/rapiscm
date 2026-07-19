# APIClarity — Shadow API Discovery & Spec Reconstruction

Open-source framework for discovering undocumented (shadow) APIs, reconstructing OpenAPI specs from traffic, and detecting API drift between spec and runtime. Originally developed by Cisco, now community-maintained under the CNCF landscape.

## How It Works

APIClarity uses **eBPF-based traffic reflection** to capture live API traffic without modifying the application.

### Architecture

```
                                   ┌──────────────────────┐
                                   │   APIClarity UI       │
                                   │   (React dashboard)  │
                                   └──────┬───────────────┘
                                          │
App / Sidecar ──► eBPF tracer ──► Kafka ──►──► Postgres ──► API spec diff
  (no change)    (kernel-level    │                │
                  HTTP capture)   │                ├── Spec v1 (reconstructed)
                                  │                ├── Spec v2 (reconstructed)
                                  │                └── Drift report
                                  │
                              Reconciler (compares spec versions)
```

### Deployment Modes

| Mode | How It Works | Use Case |
|------|-------------|----------|
| **Kubernetes sidecar** | Envoy-based sidecar intercepts all pod ingress/egress | K8s-native, automatic |
| **eBPF agent (standalone)** | Kernel-level HTTP tracing, no sidecar needed | VM / bare metal / sidecar-averse |
| **Docker compose** | Standalone deployment for dev/testing | Local development |
| **SaaS** (planned) | Cloud-hosted collector + backend | Managed service |

### Capabilities

| Feature | Detail |
|---------|--------|
| **Spec reconstruction** | Builds OpenAPI 3.0 spec from observed HTTP traffic — paths, methods, parameters, request/response schemas |
| **Drift detection** | Compares reconstructed spec against reference spec (provided or prior version). Flags new/deleted/changed endpoints |
| **Shadow API detection** | Flags endpoints that exist in traffic but NOT in the reference spec |
| **Sensitive data discovery** | Detects PII, credentials, tokens, secrets in request/response bodies |
| **Authentication mapping** | Identifies which endpoints require auth vs. which are unauthenticated |
| **Rate analysis** | Tracks endpoint call frequency, latency percentiles, error rates |

### Drift Categories

| Category | Meaning |
|----------|---------|
| **Added** | Endpoint exists in traffic but not in reference spec |
| **Removed** | Endpoint in reference spec but no traffic observed |
| **Modified** | Endpoint exists in both, but request/response schema differs |
| **Deprecated** | Endpoint marked deprecated in spec but still active |
| **Shadow** | Endpoint with no security scheme observed |

## Manual

### CLI

```bash
# Start APIClarity (Docker Compose)
git clone https://github.com/apiclarity/apiclarity.git
cd apiclarity/deployment/docker-compose
docker-compose up -d

# Access UI: http://localhost:8080
# API: http://localhost:8080/api/

# API import (POST spec for comparison)
curl -X POST http://localhost:8080/api/references \
  -H "Content-Type: application/json" \
  -d @openapi.yaml

# List reconstructed specs
curl http://localhost:8080/api/reconstructed-specs

# Get drift for a specific API
curl http://localhost:8080/api/reconstructed-specs/${SPEC_ID}/drift

# Export reconstructed spec
curl http://localhost:8080/api/reconstructed-specs/${SPEC_ID}/export
```

### Kubernetes (Helm)

```bash
# Add Helm chart
helm repo add apiclarity https://apiclarity.github.io/apiclarity
helm install apiclarity apiclarity/apiclarity \
  --set global.trafficSource=service-mesh
```

### CI/CD Integration

```bash
# Scan a recorded HAR file (offline)
curl -X POST http://localhost:8080/api/har \
  -H "Content-Type: multipart/form-data" \
  -F "file=@traffic.har"

# Get drift report as CI artifact
curl http://localhost:8080/api/drift-report/${APP_ID} \
  -o drift-report.json
```

## Build

```bash
# Prerequisites: Go 1.21+, Node 18+, Docker

git clone https://github.com/apiclarity/apiclarity.git
cd apiclarity

# Backend (Go)
cd backend && go build -o apiclarity-backend ./cmd/

# Frontend (React)
cd frontend && npm install && npm run build

# eBPF agent
cd ../tracer && make build

# Or build all with Docker
cd .. && docker-compose build
```

## Install

### Docker Compose (Quick Start)

```bash
git clone https://github.com/apiclarity/apiclarity.git
cd apiclarity/deployment/docker-compose
docker-compose up -d
# UI: http://localhost:8080
```

### Kubernetes (Helm)

```bash
helm repo add apiclarity https://apiclarity.github.io/apiclarity
helm install apiclarity apiclarity/apiclarity \
  --namespace apiclarity --create-namespace
```

### eBPF Agent (Linux Only)

```bash
# Requires: Linux 5.4+, kernel headers, bcc/libbpf
cd deployment/ebpf
./install-agent.sh
```

## Package

| Component | Distribution |
|-----------|-------------|
| Backend (Go) | Docker image (`apiclarity/apiclarity-backend`) |
| Frontend (React) | Docker image (`apiclarity/apiclarity-frontend`) |
| eBPF tracer | Source build |
| Helm chart | apiclarity GitHub Pages |
| Docker Compose | GitHub repo |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/apiclarity/apiclarity |
| Documentation | https://apiclarity.io/docs/ |
| Helm chart | https://apiclarity.github.io/apiclarity |
| Docker images | https://github.com/orgs/apiclarity/packages |
| CNCF landscape | https://landscape.cncf.io/?item=observability--apiclarity |
| Blog (Cisco) | https://blogs.cisco.com/developer/apiclarity-project-01 |
| Community meeting | https://github.com/apiclarity/community |
| Slack channel | https://cloud-native.slack.com/archives/C02E2T8UJJG |
