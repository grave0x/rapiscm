# Burp Suite Professional — Web Security Proxy + DAST

Most widely used web application security testing platform. Proxy, Repeater, Intruder, Scanner, Extender API, BCheck rules.

## How It Works

Burp operates as a local-intercepting proxy (default `localhost:8080`). Traffic flows through Burp for inspection, modification, and automated attack.

**Core tools:**
- **Proxy** — Intercept/modify requests/responses, history, match/replace
- **Repeater** — Manual request replay, modify and resend
- **Intruder** — Automated brute-force/fuzzing across parameters (positions, payloads)
- **Scanner** — DAST automated vulnerability scanning (crawl + audit)
- **Sequencer** — Session token entropy analysis
- **Decoder** — Base64, hex, URL, HTML encode/decode
- **Comparer** — Diff request/response pairs
- **Extender** — Java/Python/Ruby plugin API (BApp Store)
- **BCheck** — Custom scan check definitions (YAML-based)
- **Collaborator** — OOB/interaction detection (DNS/HTTP/SMTP)

## Manual

```bash
# Launch (Linux)
burpsuite

# Headless scan (CLI)
java -jar burpsuite_pro.jar --scan-file target_list.txt --project-file scan.burp

# Headless with config
java -jar burpsuite_pro.jar \
  --config-file config.json \
  --scan-file urls.txt \
  --project-file proj.burp

# Export findings
# Burp -> Target -> Site map -> right-click -> Save selected items

# BCheck validation
# Burp -> Extensions -> BChecks -> Run check
```

## Install

```bash
# Linux
# Download from portswigger.net, run Java JAR
java -jar burpsuite_community.jar
java -jar burpsuite_pro.jar

# macOS
brew install --cask burp-suite

# Windows
# Download installer from portswigger.net

# Headless (CI)
# Use `--collaborator-server` and `--collaborator-port` for OOB
```

## Build

Closed-source. Extensions via BApp Store, BCheck rules, and Extender API (Java, Python, Ruby).

## Package

| Edition | Cost | Features |
|---------|------|----------|
| Community | Free | Proxy, Repeater, basic Intruder, Decoder |
| Professional | Per-user/year | Scanner, advanced Intruder, BChecks, Collaborator, headless CLI |
| Enterprise | Per-target/year | Automated CI scanning, REST API, scheduler |

Distributed via portswigger.net download or package managers.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://portswigger.net/burp |
| Docs | https://portswigger.net/burp/documentation |
| BApp Store | https://portswigger.net/bappstore |
| BCheck spec | https://portswigger.net/burp/extender/bchecks |
| Community forum | https://portswigger.net/burp/community |
| Web Security Academy | https://portswigger.net/web-security |
| GitHub (extensions) | https://github.com/PortSwigger/ |
