# Gitleaks — Git Secret Scanner

Static analysis tool to detect hardcoded secrets (API keys, passwords, tokens, private keys) in git repositories. Pre-commit hooks, CI/CD integration, and full-repo/history scanning.

## How It Works

Gitleaks scans git commits, files, and diffs against configurable regex patterns. Compares against known secret formats (AWS keys, GitHub tokens, Slack tokens, etc.).

**Scan modes:**
- **Detect** — Scan entire repo history or working directory
- **Protect** — Pre-commit scanning (staged changes only)
- **Directory** — Scan a directory (non-git)

**Output formats:** JSON, CSV, SARIF (GitHub Code Scanning). Each finding includes: commit SHA, file path, line number, secret type, entropy value, and redacted content.

**Detection methods:**
- **Regex** — Pre-defined and custom patterns (`allowlist`, `regex` in config)
- **Entropy** — Shannon entropy scoring for high-randomness strings
- **Path whitelisting** — Exclude test files, vendor directories, `.gitconfig`
- **Commit whitelisting** — Skip specific commits (false positives)

## Manual

```bash
# Scan entire repo history
gitleaks detect -s /path/to/repo -v

# Scan working directory (uncommitted files only)
gitleaks detect --no-git -s /path/to/repo

# Pre-commit hook
gitleaks protect --staged -v

# Generate report with specific config
gitleaks detect -s . -c gitleaks.toml -r report.json

# Scan specific commit range
gitleaks detect --log-opts="HEAD~10..HEAD"

# SARIF output (GitHub Code Scanning)
gitleaks detect -s . -r results.sarif -f sarif

# Baseline (ignore known false positives)
gitleaks detect -s . --baseline-path .gitleaks-baseline.json

# Custom config
cat .gitleaks.toml
# [rules]
# [[rules]]
# id = "my-custom-rule"
# regex = '''(?i)(myapp)_(secret|key|token)'''''
# [allowlist]
# paths = ["tests/", "*.md"]
```

## Build

```bash
git clone https://github.com/gitleaks/gitleaks.git
cd gitleaks
make build
```

## Install

```bash
# Go install
go install github.com/gitleaks/gitleaks@latest

# Binary download
wget https://github.com/gitleaks/gitleaks/releases/download/v8.18.2/gitleaks_8.18.2_linux_x64.tar.gz
tar xzf gitleaks_8.18.2_linux_x64.tar.gz
sudo mv gitleaks /usr/local/bin/

# Docker
docker pull ghcr.io/gitleaks/gitleaks:latest

# Pre-commit hook
cat .pre-commit-config.yaml
# - repo: https://github.com/gitleaks/gitleaks
#   rev: v8.18.2
#   hooks:
#   - id: gitleaks

# macOS
brew install gitleaks
```

## Package

| Manager | Command |
|---------|---------|
| Homebrew | `brew install gitleaks` |
| Go | `go install github.com/gitleaks/gitleaks@latest` |
| Docker | `docker pull ghcr.io/gitleaks/gitleaks` |
| Binary | GitHub releases |
| Pre-commit | `.pre-commit-config.yaml` entry |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/gitleaks/gitleaks |
| Docs | https://github.com/gitleaks/gitleaks#readme |
| Config reference | https://github.com/gitleaks/gitleaks/blob/master/.gitleaks.toml |
| Pre-commit | https://github.com/gitleaks/gitleaks#pre-commit |
| CI/CD integration | https://github.com/gitleaks/gitleaks#github-action |
