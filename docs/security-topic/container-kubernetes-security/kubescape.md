# Kubescape — Kubernetes Security Posture Scanner

Kubernetes security scanner by ARMO. Scans clusters, YAML, and Helm charts against NSA MITRE, CIS Benchmarks, and custom frameworks.

## How It Works

Kubescape runs as a CLI, operator, or CI/CD scanner. It evaluates cluster resources (or static manifests) against security frameworks, calculates risk scores, and provides remediation guidance.

**Scan targets:**

| Type | Description |
|------|-------------|
| Cluster | Live cluster scan using kubeconfig. RBAC, network policies, pod security, etc. |
| Manifest | Scan local YAML files without a cluster |
| Helm chart | Extract and scan rendered templates |
| Repository | Git repository scanning |

**Frameworks:**
- NSA Kubernetes Hardening Guide
- CIS Kubernetes Benchmark
- MITRE ATT&CK
- Custom controls via OPA/Rego rules

## Manual

```bash
# Scan cluster
kubescape scan framework nsa --format sarif --output report.sarif

# Scan YAML manifests
kubescape scan --file manifest.yaml --format json

# Scan Helm chart
kubescape scan --chart path/to/chart/

# List available frameworks
kubescape list frameworks

# Scan with specific controls
kubescape scan control C-0009 --format json

# Run as operator (continuous scanning)
kubescape operator install
```

### CI/CD (GitHub Actions)

```yaml
- name: Kubescape scan
  uses: kubescape/action@v1
  with:
    format: sarif
    output: results.sarif
```

## Build

```bash
git clone https://github.com/kubescape/kubescape.git
cd kubescape
make build
# Binary in ./kubescape
```

## Install

```bash
# Linux (script)
curl -s https://raw.githubusercontent.com/kubescape/kubescape/master/install.sh | /bin/bash

# macOS
brew install kubescape

# Windows
scoop bucket add kubescape https://github.com/kubescape/scoop.git
scoop install kubescape

# Docker
docker pull ghcr.io/kubescape/kubescape:latest
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://kubescape.io/ |
| GitHub | https://github.com/kubescape/kubescape |
| Docs | https://hub.armosec.io/docs/ |
| Frameworks | https://github.com/kubescape/regolibrary |
| Helm chart | https://github.com/kubescape/kubescape-helm |
