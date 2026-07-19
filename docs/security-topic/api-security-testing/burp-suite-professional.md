# Burp Suite Professional — Web Security Testing Platform

Industry-standard intercepting proxy for manual web application and API security testing. Combines automated scanning with deep manual analysis tools. Maintained by PortSwigger.

## How It Works

Burp Suite acts as a **local intercepting proxy** (default `127.0.0.1:8080`). All traffic between browser/app and target routes through Burp, giving full visibility and control.

### Core Architecture

```
Browser → [Burp Proxy ←→ Scanner ←→ Intruder ←→ Repeater] → Target
                ↕
           [Target Scope → Site Map → Scope Rules]
```

Components operate on the same traffic stream — findings from one tool feed another.

### Key Modules

| Module | Purpose |
|--------|---------|
| **Proxy** | Intercept, inspect, modify HTTP/HTTPS in real-time. WebSocket passthrough. |
| **Spider** | Crawl web app, discover endpoints, follow JS/AJAX. Two modes: lightweight + targeted. |
| **Scanner** | Automated DAST — passive (no payloads) + active (injection, XSS, SSRF, etc.). Crawl-based or live traffic-based. |
| **Intruder** | Parameter fuzzing engine. 4 attack types: Sniper, Battering Ram, Pitchfork, Cluster Bomb. |
| **Repeater** | Manual request editor for single-endpoint deep testing. History + diff views. |
| **Sequencer** | Token/cookie entropy analysis. Tests randomness quality for session tokens, CSRF tokens, nonces. |
| **Decoder** | URL/base64/hex/ASCII encoding/decoding. Smart decode chains. |
| **Comparer** | Side-by-side response diff for authorization testing, blind detection. |
| **Extender** (BApp Store) | Plugin system. 500+ community extensions. Python/Ruby/Java APIs. |
| **Collaborator** | OOB (out-of-band) detection server. Catches blind SSRF, XXE, SQLi via DNS/HTTP interactions. |

### Scan Methodology

1. **Passive scan** — analyzes all proxied traffic for info leaks, missing headers, cookie flaws, comment exposure. No malicious payloads sent.
2. **Crawl** — Spider discovers all pages, forms, API endpoints, parameters from the application.
3. **Active scan** — Insertion points defined per parameter, per endpoint. Scan queue with configurable speed, thread pool, and audit options.
4. **Manual deep test** — Repeater for targeted probing, Intruder for brute-force/fuzzing.

### BCheck Scripting (200+ built-in checks)

Detection logic written in Burp's own DSL:

```java
if (response.status == 200 && response.body.contains("admin")) {
    issue("Admin panel exposed", severity: HIGH);
}
```

Replaces custom Python/Ruby extensions for common detection patterns.

## Manual

### Launch

```bash
# GUI (requires Java 17+)
java -jar burpsuite_pro_v2024.12.jar

# Headless (for automation scripts)
java -Djava.awt.headless=true -jar burpsuite_pro_v2024.12.jar

# REST API (Enterprise)
# Enabled via User Options → Misc → REST API
# Default: http://127.0.0.1:1337
```

### Proxy Setup

```text
Proxy → Options → Proxy Listeners
Default: 127.0.0.1:8080
Cert → Regenerate CA cert → Import into browser/device
```

### Scanning

```text
Right-click request → Do an active scan
OR
Dashboard → New Scan → URL or OpenAPI spec → Next
```

### REST API (CI/CD)

```bash
# Trigger scan from CLI
curl -X POST http://127.0.0.1:1337/v0.1/scan \
  -H "Authorization: Bearer ${BURP_API_KEY}" \
  -d '{"urls":["https://staging.example.com"], "name":"CI scan"}'

# Get scan status
curl -H "Authorization: Bearer ${BURP_API_KEY}" \
  http://127.0.0.1:1337/v0.1/scan/${SCAN_ID}

# Export findings
curl -H "Authorization: Bearer ${BURP_API_KEY}" \
  http://127.0.0.1:1337/v0.1/scan/${SCAN_ID}/report
```

### Authentication

| Method | Configuration |
|--------|---------------|
| Form-based | Record login macro; Burp replays session cookie |
| OAuth2 (implicit) | Intercept token grant, inject `Authorization: Bearer` |
| SAML | Record IdP POST, replay SAML response |
| Kerberos/NTLM | Configure OS credentials; Burp negotiates automatically |
| API key | Scope rule → `Add header: X-API-Key: <key>` |
| Certificate (client TLS) | Project options → TLS → Client certificate |

## Build

Burp Suite is **closed-source** (PortSwigger). Not buildable from source.

Community Edition source (old versions) available for audit: https://github.com/portswigger/burp-extensions-montoya-api

## Install

### Linux

```bash
# Download JAR from portswigger.net
wget https://portswigger.net/burp/releases/download?product=pro&version=2024.12&type=jar
java -jar burpsuite_pro_v2024.12.jar
```

### macOS

```bash
brew install --cask burp-suite
```

### Windows

```powershell
# Download installer from portswigger.net
choco install burp-suite-professional
```

### Docker (Enterprise)

```bash
docker pull portswigger/burpsuite-enterprise:latest
```

## Package

| Component | Distribution |
|-----------|-------------|
| Pro (JAR) | Download from portswigger.net (license required) |
| Community (JAR) | Free, feature-limited |
| Enterprise (Docker) | Docker Hub / on-prem appliance |
| BApp Store | Built-in Extender tab |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://portswigger.net/burp |
| Download | https://portswigger.net/burp/releases |
| Documentation | https://portswigger.net/burp/documentation |
| BApp Store | https://portswigger.net/bappstore |
| Blog | https://portswigger.net/blog |
| Web Security Academy | https://portswigger.net/web-security |
| REST API docs | https://portswigger.net/burp/documentation/enterprise/rest-api |
| GitHub (Montoya API) | https://github.com/portswigger/burp-extensions-montoya-api |
| Community forum | https://forum.portswigger.net/ |
