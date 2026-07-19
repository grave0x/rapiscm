# TruffleHog — Credential Scanner for Git & Files

High-velocity credential scanner. Scans git repos, filesystems, S3 buckets, and more for hardcoded secrets. Uses entropy detection + regex patterns + custom detectors.

## How It Works

TruffleHog v3+ rewritten in Go. Scans by parsing text into chunks and scanning each with entropy-based and regex-based detectors.

**Scan sources:**
- Git repositories (all branches, tags, history)
- Filesystems and directories
- S3 buckets
- GitHub repositories/issues/wiki
- GitLab projects
- Docker images (layers)
- Files (including archives)
- stdin

**Detector types:**
- **Built-in detectors** — 800+ high-signature detectors. Pre-configured for AWS, GCP, Azure, GitHub, Slack, Discord, Stripe, etc.
- **Verification** — API-based live verification. Confirms credential validity without performing dangerous actions
- **Custom detectors** — Go plugins for organization-specific secret patterns
- **Entropy** — Fallback detection for unknown patterns (high-entropy strings)

## Manual

```bash
# Scan a git repo (full history)
trufflehog git https://github.com/org/repo.git

# Scan local directory
trufflehog filesystem /path/to/repo

# Scan with verification (live API check)
trufflehog git https://github.com/org/repo.git --only-verified

# Scan GitHub org
trufflehog github --org=myorg \
  --token=$GITHUB_TOKEN

# Scan S3 bucket
trufflehog s3 --bucket=my-bucket

# Scan Docker image
trufflehog docker --image alpine:latest

# JSON output
trufflehog git https://github.com/org/repo.git --json > results.json

# Custom directory scan with fail
trufflehog filesystem /path --fail > results.json
```

## Build

```bash
git clone https://github.com/trufflesecurity/trufflehog.git
cd trufflehog
go build -o trufflehog
```

## Install

```bash
# Go install
go install github.com/trufflesecurity/trufflehog/v3@latest

# Binary download
wget https://github.com/trufflesecurity/trufflehog/releases/download/v3.81.10/trufflehog_3.81.10_linux_amd64.tar.gz
tar xzf trufflehog_3.81.10_linux_amd64.tar.gz
sudo mv trufflehog /usr/local/bin/

# Docker
docker pull trufflesecurity/trufflehog:latest
docker run --rm -v "$PWD:/scan" trufflesecurity/trufflehog filesystem /scan

# macOS
brew install trufflehog
```

## Package

| Manager | Command |
|---------|---------|
| Homebrew | `brew install trufflehog` |
| Go | `go install github.com/trufflesecurity/trufflehog/v3@latest` |
| Docker | `docker pull trufflesecurity/trufflehog` |
| Binary | GitHub releases |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/trufflesecurity/trufflehog |
| Docs | https://github.com/trufflesecurity/trufflehog#readme |
| Detectors list | https://github.com/trufflesecurity/trufflehog/tree/main/pkg/detectors |
| Custom detectors | https://github.com/trufflesecurity/trufflehog/blob/main/docs/custom-detectors.md |
| CI/CD | https://github.com/trufflesecurity/trufflehog/blob/main/docs/integrations/ |
| Blog | https://trufflesecurity.com/blog |
