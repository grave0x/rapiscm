# OWASP ZAP — Zed Attack Proxy

Most widely used free web app + API security scanner. Maintained by the OWASP Foundation. Used for both automated DAST and manual penetration testing.

## How It Works

ZAP operates as a **local-intercepting proxy**. The user configures their browser/app to route traffic through ZAP (default `localhost:8080`). ZAP records every request/response, builds a site tree, and then launches automated attacks against discovered endpoints.

**Scanning modes:**

| Mode | Description |
|------|-------------|
| **Passive Scan** | Analyzes requests/responses flowing through proxy. Zero malicious payloads. Checks headers, cookies, comments, JS. |
| **Active Scan** | Sends attack payloads (XSS, SQLi, path traversal, etc.) against each endpoint/parameter. Configurable per-scan policy. |
| **Spider** | Crawls the app by following links, parsing forms, and submitting data. AJAX Spider uses a headless browser for SPAs. |
| **API Scan** | Reads OpenAPI/Swagger/GraphQL/WSDL spec, extracts all endpoints + methods + parameters, then attacks each one. |
| **Fuzzer** | Brute-forces parameters with wordlists. Supports payload generation, reflection detection, and custom scripts. |

**Detection methods:**
- **Reflection detection** — echo payload back in response → XSS, parameter injection
- **Timing analysis** — measure response delay for time-based blind injection
- **Status code diff** — `200` vs `403` vs `500` patterns for authorization testing
- **Content comparison** — baseline vs attack response to find hidden field leaks
- **Script-based** — `graal.js` / `jython` / `zest` scripts for custom detection logic

## Manual

### Launch

```bash
# GUI
zap.sh          # Linux
Zap.exe         # Windows
zap.sh -daemon  # headless mode (CI)

# Docker (headless)
docker run -u zap -p 8080:8080 -v "$PWD:/zap/wrk" ghcr.io/zaproxy/zaproxy:stable \
  zap-api-scan.py -t https://target.com/openapi.json -r report.html
```

### Common Commands

```bash
# API scan against OpenAPI spec
zap-api-scan.py -t openapi.json -f openapi -c zap.conf -r report.html

# Baseline scan (passive + quick active, low noise)
zap-baseline.py -t https://target.com -r baseline.html

# Full active scan
zap-full-scan.py -t https://target.com -r full.html -a

# Headless quick scan
zap-cli quick-scan --self-contained --spider -r https://target.com
```

### CI/CD Integration (GitHub Actions)

```yaml
- name: ZAP API Scan
  uses: zaproxy/action-api-scan@v0.7.0
  with:
    target: 'https://staging.example.com/openapi.json'
    rules_file_name: '.zap/rules.tsv'
    cmd_options: '-a'
```

### Context & Authentication

```bash
# Define context for authenticated scanning
zap-cli context create "my-app"
zap-cli context include "my-app" "https://target.com/.*"
zap-cli session load my-session.session

# Script-based authentication (recommended for modern apps)
# Use ZAP Scripts Console -> "Authentication" scripts
# Supports: OAuth2, form-based, JSON API token, NTLM, Bearer
```

## Build

```bash
# From source (Java 11+, Gradle)
git clone https://github.com/zaproxy/zaproxy.git
cd zaproxy
./gradlew build
# Artifact: zaproxy/build/distributions/ZAP_<version>.zip

# Docker image (manual)
docker build -t zap-custom .
```

## Install

### Linux

```bash
# Debian/Ubuntu
wget -q -O- https://apt.key | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/zaproxy.gpg
echo "deb https://apt.hc.guru/ stable main" | sudo tee /etc/apt/sources.list.d/zaproxy.list
sudo apt update && sudo apt install zaproxy

# Or direct download
wget https://github.com/zaproxy/zaproxy/releases/download/v2.15.0/ZAP_2_15_0_unix.sh
chmod +x ZAP_2_15_0_unix.sh && ./ZAP_2_15_0_unix.sh
```

### macOS

```bash
brew install --cask zap
```

### Windows

```powershell
# Chocolatey
choco install zap

# Or download installer from github.com/zaproxy/zaproxy/releases
```

### Docker

```bash
docker pull ghcr.io/zaproxy/zaproxy:stable
docker pull softwaresecurityproject/zap-stable:latest  # deprecated alias
```

### Package Managers

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
| Marketplace (add-ons) | https://www.zaproxy.org/addons/ |
| Community + blog | https://www.zaproxy.org/blog/ |
| Getting Started Guide | https://www.zaproxy.org/getting-started/ |
| OWASP project page | https://owasp.org/www-project-zap/ |
| ZAP in CI | https://www.zaproxy.org/docs/docker/ |
