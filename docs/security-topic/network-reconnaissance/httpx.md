# HTTPX — HTTP Probe & Analysis Tool

Fast Go tool for probing HTTP/HTTPs services. Probes subdomains or IPs, collects response metadata, and identifies live hosts. Part of ProjectDiscovery toolkit.

## How It Works

HTTPX sends HTTP requests to each input target and collects response attributes: status code, content-length, title, content-type, IP address, TLS certificate details, CDN provider, and technologies in use. It deduplicates and filters results for further analysis or pipelining into other tools.

**Probe capabilities:**

| Feature | Description |
|---------|-------------|
| **Probe modes** | HTTP/HTTPS, HTTP/2, TLS, timeout detection |
| **Response extraction** | Status, title, headers, body, location |
| **TLS info** | Certificate chain, SAN, issuer, expiry |
| **Tech detection** | Wappalyzer-based fingerprinting |
| **CDN detection** | Cloudflare, Akamai, Fastly, etc. |
| **Port probing** | Custom port lists (80, 443, 8443, 8080) |

## Manual

### Basic Usage

```bash
# Probe subdomains
cat subdomains.txt | httpx

# Single target
httpx -u https://target.com

# Output results
cat subdomains.txt | httpx -o live.txt
```

### Filtering

```bash
# Status code filter
cat subdomains.txt | httpx -mc 200,403,301

# Content length filter
cat subdomains.txt | httpx -ml 5000    # max length 5KB

# Response title filter
cat subdomains.txt | httpx -title -ms "Admin"
```

### Response Collection

```bash
# Full metadata
cat subdomains.txt | httpx -title -status-code -content-length -content-type -ip -cname

# Screenshot (requires chromedp)
cat subdomains.txt | httpx -screenshot

# Technology stack
cat subdomains.txt | httpx -tech-detect

# Custom request headers
cat subdomains.txt | httpx -H "Authorization: Bearer token"
```

### Advanced

```bash
# Probe all discovered URLs
cat urls.txt | httpx -title -status-code -o metadata.txt

# Rate limiting
cat subdomains.txt | httpx -rate-limit 50 -t 10

# Input from multiple sources
subfinder -d target.com | httpx -title -o results.txt

# Threaded scanning
cat subdomains.txt | httpx -t 150 -o fast_scan.txt
```

### Output Formats

```bash
# JSON (machine-readable)
cat subdomains.txt | httpx -json -o results.json

# CSV
cat subdomains.txt | httpx -csv -o results.csv

# Custom output format
cat subdomains.txt | httpx -title -status-code -o custom.txt
```

## Build

```bash
git clone https://github.com/projectdiscovery/httpx.git
cd httpx/cmd/httpx
go build
# Binary: ./httpx
```

## Install

```bash
# Go install
go install -v github.com/projectdiscovery/httpx/cmd/httpx@latest

# Prebuilt binary
wget https://github.com/projectdiscovery/httpx/releases/latest/download/httpx_*_linux_amd64.zip
unzip httpx_*.zip && sudo mv httpx /usr/local/bin/

# macOS
brew install httpx

# Docker
docker pull projectdiscovery/httpx

# Kali
sudo apt install httpx
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/projectdiscovery/httpx |
| Releases | https://github.com/projectdiscovery/httpx/releases |
| Docs | https://docs.projectdiscovery.io/tools/httpx |
| Tech detection | https://github.com/projectdiscovery/wappalyzergo |
