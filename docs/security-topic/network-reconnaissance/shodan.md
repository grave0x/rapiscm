# Shodan — Search Engine for Internet-Connected Devices

Search engine that indexes internet-connected devices, services, and their banners. Find exposed databases, ICS devices, cameras, and misconfigurations.

## How It Works

Shodan continuously scans the internet, collecting banners and metadata from all public IPs. Users search via web UI, CLI, or API against fields like port, service, org, location, and vulnerability.

**Key concepts:**

| Concept | Description |
|---------|-------------|
| **Search filters** | `port:443`, `country:US`, `org:Google`, `product:nginx` |
| **Banner data** | Service name, version, options, hostname, SSL cert |
| **Ports** | Default top 100 ports; API plans expand range |
| **Vulnerabilities** | `vuln:CVE-2021-44228` — find vulnerable instances |
| **Reports** | Summary, counts, top ports, top orgs for a query |
| **Monitor** | Track changes to a network over time |

## Manual

### Web Search

```
https://www.shodan.io/search?query=port:22 country:US
```

### CLI (shodan command)

```bash
# Search and get IPs
shodan search --fields ip_str,port "port:443 product:Apache"

# Get summary counts
shodan count "port:22 country:JP"

# Host details
shodan host 8.8.8.8

# My IP info
shodan myip

# Download results
shodan download results "port:3306 product:MySQL"

# Parse downloaded results
shodan parse --fields ip_str,port results.json.gz
```

### API Usage

```python
import shodan

api = shodan.Shodan("API_KEY")
results = api.search("port:443 product:nginx")

for result in results['matches']:
    print(f"{result['ip_str']}:{result['port']} - {result['product']}")
```

### Common Filter Queries

```bash
# Exposed databases
port:27017           # MongoDB (no auth)
port:6379            # Redis
port:5432            # PostgreSQL
port:3306            # MySQL

# ICS / SCADA
port:502             # Modbus
port:1911            # Niagara Fox
port:44818           # EtherNet/IP

# Vulnerable instances
vuln:CVE-2021-44228  # Log4Shell
vuln:CVE-2017-5638   # Struts2

# Camera / IoT
"Netwave IP Camera"  # default creds
"webcam" "7"         # Axis cameras
```

## Install

```bash
# Python CLI tool
pip install shodan

# Initialize with API key
shodan init YOUR_API_KEY
```

### Plan Tiers

| Tier | Scan Speed | Ports | Results | API Queries |
|------|-----------|-------|---------|-------------|
| Free | 1/mo full | Top 100 | 50/page | 100/mo |
| Freelancer | 1/5s | Top 100 | 1K/page | Unlimited |
| Small Business | 1/1s | All | 10K/page | Unlimited |
| Corporate | Streaming | All | No limit | Unlimited |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.shodan.io/ |
| API docs | https://developer.shodan.io/ |
| CLI docs | https://cli.shodan.io/ |
| Shodan blog | https://blog.shodan.io/ |
| Research | https://research.shodan.io/ |
