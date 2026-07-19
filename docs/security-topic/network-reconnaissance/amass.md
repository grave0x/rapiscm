# Amass (OWASP) — Attack Surface Mapping

In-depth attack surface mapping and external asset discovery, combining passive OSINT with active enumeration.

## How It Works

Amass aggregates data from 45+ sources (Certificate Transparency, DNS, search engines, APIs) then runs active enumeration to build a comprehensive graph of subdomains, IPs, ASNs, and certificate relationships.

**Data sources:** crt.sh, Shodan, AlienVault, VirusTotal, DNS brute-force, reverse DNS, search engine scraping, CommonCrawl, Wayback Machine.

**Enumeration phases:**

| Phase | Description |
|-------|-------------|
| **Passive** | Collect intel from 45+ third-party APIs |
| **Active** | DNS brute-force, zone transfer attempts, reverse DNS |
| **Resolution** | Verify discovered names resolve to IPs |
| **Graph building** | Link assets by certificate, ASN, IP relationships |
| **Track merging** | Combine data across multiple runs into sessions |

## Manual

### Basic Usage

```bash
# Passive enumeration
amass enum -passive -d target.com

# Full enumeration (passive + active)
amass enum -d target.com

# With config file (API keys for data sources)
amass enum -d target.com -config config.ini

# Output to file
amass enum -d target.com -o subdomains.txt

# JSON output for machine processing
amass enum -d target.com -json results.json

# Visualize graph
amass viz -d target.com -enum <dir>

# Track changes across runs
amass track -d target.com
```

### Subcommands

| Subcommand | Description |
|------------|-------------|
| `enum` | Main enumeration engine |
| `intel` | Open-source intel gathering (no target domain needed) |
| `track` | Compare differences between enumeration runs |
| `viz` | Generate D3.js visualizations |
| `db` | Query the Amass graph database |

### Intel Subcommand

```bash
# Find root domains from company name
amass intel -org "Target Corp"

# Find ASNs owned by org
amass intel -asn 12345

# Reverse DNS from CIDR
amass intel -cidr 192.168.0.0/24
```

## Build

```bash
git clone https://github.com/owasp-amass/amass.git
cd amass
go build -o amass ./cmd/amass
```

## Install

```bash
# Linux (script)
wget https://github.com/owasp-amass/amass/releases/latest/download/amass_linux_amd64.zip
unzip amass_linux_amd64.zip
sudo mv amass /usr/local/bin/

# Docker
docker pull caffix/amass

# Go install
go install -v github.com/owasp-amass/amass/v4/...@master

# macOS
brew install amass

# Snap
sudo snap install amass
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/owasp-amass/amass |
| OWASP project | https://owasp.org/www-project-amass/ |
| Docs | https://github.com/owasp-amass/amass/blob/master/doc/user_guide.md |
