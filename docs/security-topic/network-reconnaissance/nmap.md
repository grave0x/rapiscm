# Nmap — Network Mapper

Industry-standard port scanner, service detection, and NSE scripting engine. De facto tool for network reconnaissance.

## How It Works

Nmap sends raw IP packets to determine which hosts are up, what ports are open, what services are running, and what OS is in use. The **Nmap Scripting Engine (NSE)** extends this with 600+ Lua scripts for vulnerability detection, brute-force, and service enumeration.

**Scan types:**

| Type | Description |
|------|-------------|
| **TCP SYN scan (-sS)** | Half-open scan, fast and stealthy. Default. |
| **TCP connect scan (-sT)** | Full TCP handshake. No raw socket needed. |
| **UDP scan (-sU)** | Slow, spans 65K UDP ports. |
| **Ping sweep (-sn)** | Host discovery only. No port scan. |
| **Version detection (-sV)** | Probe open ports for service + version. |
| **OS detection (-O)** | TCP/IP stack fingerprinting. |
| **NSE scripts (--script)** | 600+ categories: vuln, exploit, brute, discovery. |

## Manual

### Basic Usage

```bash
# Quick scan of top 1000 ports
nmap -sS -T4 target.com

# Full port scan + version + OS
nmap -sS -sV -O -p- -T4 target.com

# Ping sweep /24 subnet
nmap -sn 192.168.1.0/24

# UDP top 100 ports
nmap -sU --top-ports 100 target.com

# NSE vulnerability scan
nmap --script vuln target.com

# NSE with service detection
nmap -sV --script=http-title,ssl-cert target.com

# Output all formats
nmap -oA scan_result target.com
```

### Common Output Formats

```bash
# Normal, XML, grepable
nmap -oN scan.nmap -oX scan.xml -oG scan.gnmap target.com

# Convert XML to HTML
xsltproc scan.xml -o scan.html
```

### NSE Script Categories

| Category | Description | Examples |
|----------|-------------|---------|
| `vuln` | Vulnerability checks | http-vuln-cve2017-5638 |
| `exploit` | Active exploitation | smb-vuln-ms17-010 |
| `brute` | Credential brute-force | http-brute, ssh-brute |
| `discovery` | Service/service discovery | dns-brute, whois-ip |
| `safe` | No disruption risk | ssh-hostkey, ssl-cert |
| `intrusive` | May affect target | http-sql-injection |

### CI/CD Integration

```yaml
- name: Network scan
  run: |
    nmap -sV --script vuln -oX report.xml $TARGET
```

## Build

```bash
git clone https://github.com/nmap/nmap.git
cd nmap
./configure
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install nmap

# RHEL/CentOS
sudo dnf install nmap

# macOS
brew install nmap

# Windows
# Download installer from https://nmap.org/download.html

# Docker
docker pull instrumentisto/nmap
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://nmap.org/ |
| GitHub | https://github.com/nmap/nmap |
| NSE docs | https://nmap.org/nsedoc/ |
| Reference guide | https://nmap.org/docs.html |
| Book | https://nmap.org/book/ |
