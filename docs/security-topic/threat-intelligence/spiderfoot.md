# SpiderFoot — OSINT Automation Platform

Open-source intelligence automation tool. 200+ modules for footprinting, reconnaissance, and threat intelligence gathering. CLI and web UI.

## How It Works

SpiderFoot operates as a scan engine. A scan starts with a target (domain, IP, email, ASN, etc.) and runs configured modules to collect, correlate, and deduplicate OSINT data from public sources.

**Module categories:**

| Category | Examples |
|----------|----------|
| **DNS** | Passive DNS, brute force subdomains, zone transfer, reverse lookup |
| **WHOIS** | Domain registrant info, reverse WHOIS, registrar history |
| **Web** | Content analysis, technology fingerprinting, URL extraction |
| **Social** | Social media profiles, mentions, email addresses |
| **Threat Intel** | AbuseIPDB, VirusTotal, AlienVault OTX, Shodan, Censys |
| **Dark Web** | Tor hidden service discovery, paste sites |
| **Leaks** | Breach database lookups, credential dumps |

## Manual

```bash
# Start web UI
spiderfoot -l 127.0.0.1:5001

# CLI scan
spiderfoot -s target.com -t all -o results.csv

# CLI scan with specific modules
spiderfoot -s target.com -t TARGET_DOMAIN_OWNER,TARGET_WEB_CONTENT

# Output formats
spiderfoot -s target.com -t all -o json -q
spiderfoot -s target.com -t all -o csv
spiderfoot -s target.com -t all -o html

# Correlation
spiderfoot -s target.com -t all -o json | \
  python3 -c "import sys,json; data=json.load(sys.stdin); print(json.dumps(data, indent=2))"
```

### Python API

```python
from spiderfoot import SpiderFoot

sf = SpiderFoot()
sf.setup(modules=["sfp_dnsresolve", "sfp_whois", "sfp_shodan"])

results = []
for event in sf.scan("target.com", "TARGET_DOMAIN_NAME"):
    results.append(event.data)
    print(f"{event.module}: {event.data}")
```

## Build

```bash
git clone https://github.com/smicallef/spiderfoot.git
cd spiderfoot
pip install -r requirements.txt
# Run: python3 spiderfoot.py
```

## Install

```bash
# Docker
docker pull ghcr.io/smicallef/spiderfoot:latest
docker run -p 5001:5001 ghcr.io/smicallef/spiderfoot:latest

# Manual
git clone https://github.com/smicallef/spiderfoot.git
cd spiderfoot
pip install -r requirements.txt
python3 spiderfoot.py -l 127.0.0.1:5001
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.spiderfoot.net/ |
| GitHub | https://github.com/smicallef/spiderfoot |
| Docs | https://github.com/smicallef/spiderfoot/wiki |
| Module list | https://github.com/smicallef/spiderfoot/tree/master/modules |
| Docker | https://github.com/smicallef/spiderfoot/pkgs/container/spiderfoot |
