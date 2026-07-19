# SpiderFoot — OSINT Automation Framework

Automated OSINT reconnaissance tool with 200+ modules. Collects intelligence on IPs, domains, email, names, and ASNs from public sources.

## How It Works

SpiderFoot ingests a target (domain, IP, email, ASN, name) and runs modular scans across search engines, DNS, social media, breach databases, certificate transparency logs, and threat intelligence feeds. Results are correlated into a graph showing relationships between entities.

**Scan modes:**

| Mode | Description |
|------|-------------|
| **SpiderFoot (CLI)** | Headless command-line scanning for automation / CI |
| **SFWeb (GUI)** | Web interface with project management, graph view, export |
| **Scan types** | Footprint, Investigate, Passive, All — vary scope and intensity |
| **Correlation engine** | Links related entities (IP → domain → email → social profile) |

**Module categories:**

| Category | Examples |
|----------|----------|
| DNS | MX, SPF, DMARC, zone transfer, reverse lookup |
| Search engines | Google, Bing, Shodan, Censys, VirusTotal |
| Social media | LinkedIn, Twitter, Facebook, Instagram |
| Breach data | Have I Been Pwned, Dehashed, leaked databases |
| CT logs | crt.sh, CertSpotter |
| Geolocation | IP2Location, GeoIP |

## Manual

### CLI Basic Usage

```bash
# Run module scan on domain
spiderfoot -s target.com -t all -o json > results.json

# Passive scan only (no direct interaction with target)
spiderfoot -s target.com -t all -o html -T passive

# Single module
spiderfoot -s target.com -t 1001 -o csv > results.csv

# List available modules
spiderfoot -m list

# Seed scan from file
spiderfoot -f targets.txt -o json
```

### Web UI

```bash
# Start web interface
spiderfoot -l 0.0.0.0:5001
# Access http://localhost:5001
```

### Scan Types

```bash
# Footprint — comprehensive (60-90 min)
spiderfoot -s target.com -t all -o html > footprint.html

# Investigate — deep dive with correlation
spiderfoot -s target.com -t all -o json | jq .

# Passive — no direct touch
spiderfoot -s target.com -t all -T passive -o csv
```

## Build

```bash
git clone https://github.com/smicallef/spiderfoot.git
cd spiderfoot
pip install -r requirements.txt
python3 sf.py -h
```

## Install

```bash
# Docker
docker pull ghcr.io/smicallef/spiderfoot:latest

# Pip
pip install spiderfoot

# Manual (latest)
git clone https://github.com/smicallef/spiderfoot.git
cd spiderfoot
pip install -r requirements.txt
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/smicallef/spiderfoot |
| Docs | https://www.spiderfoot.net/documentation/ |
| Module list | https://github.com/smicallef/spiderfoot/tree/master/modules |
| Docker image | https://github.com/smicallef/spiderfoot/pkgs/container/spiderfoot |
