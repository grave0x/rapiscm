# theHarvester — Email & Subdomain OSINT Gatherer

Passive OSINT tool for harvesting emails, subdomains, IPs, and URLs from public sources. Used in reconnaissance phase of penetration tests.

## How It Works

theHarvester queries search engines, social media, certificate transparency logs, and DNS sources for information associated with a target domain. Results are deduplicated and categorized by type (email, subdomain, IP, URL).

**Search sources:**

| Source | Type | Data |
|--------|------|------|
| Google / Bing / Yahoo | Search engine | Emails, subdomains |
| Baidu | Search engine | Subdomains |
| DuckDuckGo | Search engine | Emails, subdomains |
| crt.sh | Certificate transparency | Subdomains |
| DNS brute-force | DNS | Subdomains via wordlist |
| LinkedIn | Social media | Employee names, positions |
| PGP search | PGP keyserver | Emails |
| Threat intelligence feeds | Passive DNS | Subdomains, IPs |

## Manual

### Basic Usage

```bash
# Search all sources for domain
theharvester -d target.com -b all

# Limit to specific sources
theharvester -d target.com -b google,linkedin,crt

# With DNS brute-force
theharvester -d target.com -b all -l 500

# Save results
theharvester -d target.com -b all -f results.html
theharvester -d target.com -b all -f results.xml

# Output to file
theharvester -d target.com -b all -s -o result.html
```

### Options

| Flag | Description |
|------|-------------|
| `-d <domain>` | Target domain |
| `-b <sources>` | Comma-separated sources or `all` |
| `-l <count>` | Limit results (default 500) |
| `-f <file>` | Output filename |
| `-s` | Use Shodan |
| `-h` | Use DNS brute-force with hostnames file |

### DNS Brute-force

```bash
# Custom wordlist
theharvester -d target.com -b none -l /usr/share/wordlists/subdomains.txt
```

## Build

```bash
git clone https://github.com/laramies/theHarvester.git
cd theHarvester
pip install -r requirements/base.txt
python3 theHarvester.py -h
```

## Install

```bash
# Debian/Ubuntu
sudo apt install theharvester

# Pip
pip install theHarvester

# Docker
docker pull laramies/theharvester

# Manual (latest)
git clone https://github.com/laramies/theHarvester.git
cd theHarvester
pip install -r requirements/base.txt
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/laramies/theHarvester |
| Docs | https://github.com/laramies/theHarvester/wiki |
| Sources | https://github.com/laramies/theHarvester/wiki/Supported-Sources |
| Docker | https://hub.docker.com/r/laramies/theharvester |
