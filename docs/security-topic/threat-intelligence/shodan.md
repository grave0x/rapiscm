# Shodan — Internet Device Search Engine

Search engine for internet-connected devices and services. Scans the entire IPv4 address space continuously. Find exposed services, industrial controls, vulnerable devices, and infrastructure.

## How It Works

Shodan crawls the internet by sending probes to every IPv4 address on common ports, collecting banners (HTTP, SSH, FTP, Modbus, etc.), and indexing device metadata. Users search via web UI, CLI, or API.

**Search facets:**

| Facet | Examples |
|-------|----------|
| **Service** | HTTP, SSH, FTP, Telnet, MongoDB, MySQL, VNC, RDP |
| **Port** | 22, 80, 443, 445, 8080, 3389, 9200 |
| **OS** | Linux, Windows, iOS, embedded Linux, VxWorks |
| **Product** | Apache, nginx, IIS, OpenSSH, vsftpd |
| **CVE** | Filter by known vulnerability, severity, CVSS |
| **Geo** | Country, city, coordinates, ISP, ASN |
| **ICS** | Modbus, Siemens S7, BACnet, DNP3, Niagara Fox |

## Manual

```bash
# CLI search
shodan search "apache country:US product:nginx"

# Count results
shodan count "port:22 ssh"

# Host details
shodan host 8.8.8.8

# Download results
shodan download results.json.gz "product:mongodb"

# Parse downloaded results
shodan parse --fields ip_str,port,org results.json.gz

# My IP info
shodan myip

# Account info
shodan info
```

### Search Filters

```
# Common filters
city:London
country:US
port:3389
org:"Google"
hostname:example.com
os:"Windows Server 2019"
product:nginx
version:1.21
before:2024-01-01
after:2024-01-01
has_vuln:true
cve:CVE-2021-44228
net:192.168.1.0/24

# ICS specific
modbus unit address:1
s7-300
bacnet device id:123
```

### Python API

```python
import shodan

api = shodan.Shodan("API_KEY")

# Search
results = api.search("product:mongodb country:DE")
for r in results["matches"]:
    print(f"{r['ip_str']}:{r['port']} — {r['org']}")

# Host info
host = api.host("8.8.8.8")
for service in host["data"]:
    print(f"Port {service['port']}: {service['product']} {service.get('version','')}")

# Count
count = api.count("openssh")
```

## Install

```bash
# macOS
brew install shodan

# pip
pip install shodan

# Set API key
shodan init "API_KEY"
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.shodan.io/ |
| Developer API | https://developer.shodan.io/ |
| CLI Docs | https://cli.shodan.io/ |
| Search Examples | https://www.shodan.io/search/examples |
| ICS Dashboard | https://www.shodan.io/explore/category/industrial-control-systems |
| InternetDB | https://internetdb.shodan.io/ |
