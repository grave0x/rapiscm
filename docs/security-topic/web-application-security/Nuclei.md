# Nuclei — YAML-Based Vulnerability Scanner

Fast vulnerability scanner powered by YAML templates. Protocol-based scanning with 10k+ community templates. Multi-step, extensible, CI-friendly.

## How It Works

Nuclei uses **templates** (YAML) that define attack logic per vulnerability type. Each template specifies protocol (HTTP, TCP, DNS, SSL, etc.), request/response matchers, and extractors.

**Template structure:**
```yaml
id: example-cve
info:
  name: Example Vulnerability
  severity: high
requests:
  - method: GET
    path:
      - "{{BaseURL}}/vuln"
    matchers:
      - type: word
        words:
          - "vulnerable"
```

**Key features:**
- 10k+ community templates (CVE, default creds, exposures, misconfigs)
- Multi-protocol: HTTP, TCP, DNS, SSL, HTTP2, WebSocket, JavaScript (headless)
- Flow: conditional execution, multi-step requests
- Model-based: automatically match severity, CVE, OWASP category
- Deduplication: automatic dedup based on request/response hash
- Rate limiting: configurable concurrency, delay, retry
- CI/CD: JSON/jUnit output, GitHub Actions integration

## Manual

```bash
# Single target
nuclei -u https://target.com

# Multiple targets
nuclei -l targets.txt

# Specific template category
nuclei -u https://target.com -t cves/

# Specific template ID
nuclei -u https://target.com -id CVE-2023-xxxxx

# Custom template directory
nuclei -u https://target.com -t /path/to/templates/

# JSON output
nuclei -u https://target.com -o results.json -json

# Rate limiting
nuclei -u https://target.com -rl 150 -c 25

# Filter by severity
nuclei -u https://target.com -severity critical,high

# Headless mode (JavaScript templates)
nuclei -u https://target.com -headless
```

## Install

```bash
# Download binary
wget https://github.com/projectdiscovery/nuclei/releases/latest/download/nuclei_linux_amd64.zip
unzip nuclei_linux_amd64.zip
sudo mv nuclei /usr/local/bin/

# Homebrew
brew install nuclei

# Docker
docker pull projectdiscovery/nuclei
docker run projectdiscovery/nuclei -u https://target.com

# Go install
go install -v github.com/projectdiscovery/nuclei/v3/cmd/nuclei@latest
```

## Build

```bash
git clone https://github.com/projectdiscovery/nuclei.git
cd nuclei/v3/cmd/nuclei
go build .
```

## Package

| Manager | Command |
|---------|---------|
| Brew | `brew install nuclei` |
| Docker | `projectdiscovery/nuclei` |
| Go | `go install` |
| Binary | GitHub releases |

Templates: `nuclei -update-templates` or `git clone https://github.com/projectdiscovery/nuclei-templates.git`

## Links

| Resource | URL |
|----------|-----|
| Official site | https://nuclei.projectdiscovery.io/ |
| GitHub | https://github.com/projectdiscovery/nuclei |
| Docs | https://docs.projectdiscovery.io/tools/nuclei |
| Templates | https://github.com/projectdiscovery/nuclei-templates |
| Template guide | https://nuclei.projectdiscovery.io/templating-guide/ |
| Discord | https://discord.gg/projectdiscovery |
