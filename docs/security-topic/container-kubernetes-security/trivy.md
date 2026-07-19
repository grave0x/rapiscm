# Trivy — All-in-One Container & Cloud Scanner

All-in-one vulnerability scanner for containers, Kubernetes, IaC, repositories, and SBOMs. Maintained by Aqua Security.

## How It Works

Trivy scans by comparing package/artifact metadata against vulnerability databases (NVD, GHSA, OSV, RedHat, Ubuntu, Alpine, etc.). It downloads and caches DBs on first run, supports offline mode, and produces SARIF, CycloneDX, SPDX, HTML, JSON, and table output.

**Scan types:**

| Type | Targets |
|------|---------|
| Container image | OCI images, Docker tarballs, podman/skopeo |
| Filesystem | Directories, rootfs |
| Git repository | git repos (commit-by-commit) |
| Kubernetes | Cluster resources, Helm charts |
| IaC | Terraform, CloudFormation, Dockerfile, K8s YAML |
| SBOM | CycloneDX/SPDX input |
| Rootfs | Full filesystem scan |

## Manual

```bash
# Scan container image
trivy image nginx:latest

# Scan with severity filter
trivy image --severity CRITICAL,HIGH alpine:3.18

# Scan filesystem
trivy fs /path/to/project

# Scan K8s cluster
trivy k8s cluster --report summary

# Scan IaC
trivy config --severity CRITICAL terraform/

# Generate SBOM in CycloneDX
trivy image --format cyclonedx --output result.cdx.json alpine:3.18

# Output formats: table, json, sarif, html, cyclonedx, spdx, github
trivy image --format sarif --output report.sarif alpine:3.18
```

### CI/CD (GitHub Actions)

```yaml
- name: Trivy scan
  uses: aquasecurity/trivy-action@master
  with:
    image-ref: '${{ env.IMAGE }}'
    format: 'sarif'
    output: 'trivy-results.sarif'
    severity: 'CRITICAL,HIGH'
```

## Build

```bash
git clone https://github.com/aquasecurity/trivy.git
cd trivy
make build
# Binary in ./trivy
```

## Install

```bash
# Linux (script)
curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh

# macOS
brew install trivy

# Docker
docker pull ghcr.io/aquasecurity/trivy:latest
docker run ghcr.io/aquasecurity/trivy:latest image alpine:3.18

# APT (Debian/Ubuntu)
sudo apt-get install trivy

# RPM (RHEL/CentOS/Fedora)
sudo dnf install trivy
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/aquasecurity/trivy |
| Docs | https://trivy.dev/ |
| Docker images | https://github.com/aquasecurity/trivy/pkgs/container/trivy |
| Trivy Operator | https://github.com/aquasecurity/trivy-operator |
