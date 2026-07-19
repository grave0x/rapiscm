# OWASP ZAP — Zed Attack Proxy

Most widely used free web app + API security scanner. Maintained by OWASP Foundation. Used for both automated DAST and manual penetration testing.

## How It Works

ZAP operates as a **local-intercepting proxy**. Routes traffic through ZAP (default `localhost:8080`). Records requests/responses, builds site tree, launches automated attacks against discovered endpoints.

**Scanning modes:**

| Mode | Description |
|------|-------------|
| **Passive Scan** | Analyzes traffic flowing through proxy. Zero malicious payloads. Checks headers, cookies, comments, JS |
| **Active Scan** | Sends attack payloads (XSS, SQLi, path traversal) against each endpoint. Configurable scan policy |
| **Spider** | Crawls app following links and forms. AJAX Spider uses headless browser for SPAs |
| **API Scan** | Reads OpenAPI/Swagger/GraphQL/WSDL spec, extracts endpoints, attacks each one |
| **Fuzzer** | Brute-forces parameters with wordlists. Supports payload generation and reflection detection |

**Detection methods:** Reflection detection, timing analysis (blind injection), status code diff, content comparison, script-based (Graal.js/Jython/Zest).

## Manual

```bash
# GUI
zap.sh                # Linux
Zap.exe               # Windows
zap.sh -daemon        # headless (CI)

# API scan against OpenAPI spec
zap-api-scan.py -t openapi.json -f openapi -c zap.conf -r report.html

# Baseline scan (passive + quick active)
zap-baseline.py -t https://target.com -r baseline.html

# Full active scan
zap-full-scan.py -t https://target.com -r full.html -a

# Docker
docker run -u zap -p 8080:8080 -v "$PWD:/zap/wrk" \
  ghcr.io/zaproxy/zaproxy:stable \
  zap-api-scan.py -t https://target.com/openapi.json -r report.html

# Context for authenticated scanning
zap-cli context create "my-app"
zap-cli context include "my-app" "https://target.com/.*"
```

## Build

```bash
git clone https://github.com/zaproxy/zaproxy.git
cd zaproxy
./gradlew build
# Artifact: zaproxy/build/distributions/ZAP_<version>.zip
```

## Install

```bash
# Debian/Ubuntu
wget -q -O- https://apt.key | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/zaproxy.gpg
echo "deb https://apt.hc.guru/ stable main" | sudo tee /etc/apt/sources.list.d/zaproxy.list
sudo apt update && sudo apt install zaproxy

# macOS
brew install --cask zap

# Windows
choco install zap

# Docker
docker pull ghcr.io/zaproxy/zaproxy:stable
```

## Package

| Manager | Command |
|---------|---------|
| Homebrew | `brew install --cask zap` |
| Chocolatey | `choco install zap` |
| Snap | `snap install zaproxy --classic` |
| Scoop | `scoop bucket add extras && scoop install zap` |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.zaproxy.org/ |
| GitHub | https://github.com/zaproxy/zaproxy |
| Docs | https://www.zaproxy.org/docs/ |
| API scan docs | https://www.zaproxy.org/docs/docker/api-scan/ |
| Docker images | https://github.com/zaproxy/zaproxy/pkgs/container/zaproxy |
| Add-ons | https://www.zaproxy.org/addons/ |
| OWASP project | https://owasp.org/www-project-zap/ |
