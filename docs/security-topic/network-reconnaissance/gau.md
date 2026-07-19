# GAU — Get All URLs

Fast URL discovery tool that aggregates URLs from multiple passive sources. Used in recon to build a comprehensive list of known endpoints for a target domain.

## How It Works

GAU queries Wayback Machine, URLScan, AlienVault OTX, CommonCrawl, and GitHub for historical or publicly indexed URLs belonging to a domain. It deduplicates results and supports filtering by MIME type, status code, subsource, and match pattern.

**Data sources:**

| Source | Description |
|--------|-------------|
| Wayback Machine (CDX) | Largest archive of historical web pages |
| URLScan | Screenshot & URL scanning service database |
| AlienVault OTX | Open Threat Exchange indicators |
| CommonCrawl | Petabyte-scale open crawl corpus |
| GitHub | API key leaks, config files (optional) |

## Manual

### Basic Usage

```bash
# Get all URLs for a domain
gau target.com

# Multiple domains
gau target.com example.com

# Output to file
gau target.com > urls.txt
```

### Filtering

```bash
# Filter by MIME type
gau --o text target.com          # text/*
gau --o application target.com   # application/* (JSON, JS, PDF)

# Filter by response status
gau --status-ok target.com       # 2xx only

# Exclude subsources
gau --exclude wayback target.com

# Include subsource in output (3rd column)
gau --verbose target.com
```

### Piping to Other Tools

```bash
# Find live hosts
gau target.com | httpx -mc 200 -title

# Filter for parameters (potential injection points)
gau target.com | grep '=' | qsreplace 'PAYLOAD' | httpx -x POST

# Extract JavaScript files
gau target.com | grep '\.js$' | httpx -mc 200 -content-type

# Nuclei scanning
gau target.com | grep '\.js$' | nuclei -t ~/nuclei-templates/
```

### With Subdomain Input

```bash
# Pipe subdomains
cat subdomains.txt | gau

# Recursive per subdomain
subfinder -d target.com | gau --subs
```

## Build

```bash
git clone https://github.com/lc/gau.git
cd gau
go build
# Binary: ./gau
```

## Install

```bash
# Go install
go install github.com/lc/gau/v2/cmd/gau@latest

# Prebuilt binary
wget https://github.com/lc/gau/releases/latest/download/gau_*_linux_amd64.tar.gz
tar xzf gau_*.tar.gz && sudo mv gau /usr/local/bin/

# macOS
brew install gau

# Build from source
git clone https://github.com/lc/gau.git
cd gau && go build
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/lc/gau |
| Releases | https://github.com/lc/gau/releases |
| Config | https://github.com/lc/gau#configuration |
| Providers | https://github.com/lc/gau#providers |
