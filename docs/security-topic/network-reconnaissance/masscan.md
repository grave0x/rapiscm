# Masscan — Mass IP Port Scanner

Ultra-fast port scanner that transmits packets at line rate. Scans the entire internet in under 5 minutes.

## How It Works

Masscan uses a custom TCP/IP stack and asynchronous packet transmission to achieve scan rates of millions of packets per second. It supports the same option syntax as Nmap but is optimized for speed over depth — use for discovery, hand off to Nmap for detail.

**Key concepts:**

| Concept | Description |
|---------|-------------|
| **Rate (-rate)** | Packets per second. 100,000 typical; 1,000,000+ on good hardware. |
| **Randomize (--randomize-hosts)** | Scatters scan order to avoid detection. |
| **Exclude file (--excludefile)** | Skips IP ranges (critical for responsible scanning). |
| **Banner grab (--banners)** | Reads service banners from open ports. |
| **Output formats** | XML, JSON, binary, grepable, list. |

## Manual

### Basic Usage

```bash
# Scan a subnet for port 80
masscan 192.168.1.0/24 -p80

# Full TCP scan of a /8 (use caution)
masscan 10.0.0.0/8 -p0-65535 --rate=100000

# Top 100 ports across a range
masscan target.com -p1-100,443,8080 --rate=1000

# Banner grabbing
masscan target.com -p80,443 --banners --rate=1000

# Output as JSON
masscan target.com -p80,443 --output-format json -o results.json

# With exclude file
masscan target.com -p0-65535 --excludefile exclude.txt
```

### Compare with Nmap

```bash
# Nmap is slow but detailed
nmap -sV -p80,443 target.com

# Masscan is fast but basic
masscan target.com -p80,443 --banners

# Best: masscan for discovery → nmap for detail
masscan target.com -p0-65535 --rate=100000 -oG masscan.gnmap
# Parse and feed open ports to nmap
```

### Output Formats

| Format | Flag | Use Case |
|--------|------|----------|
| XML | `-oX file.xml` | Machine parsing |
| JSON | `-oJ file.json` | Programmatic ingestion |
| Binary | `-oB file.bin` | Resume with `--resume` |
| Grepable | `-oL file.lst` | Simple list of IP:port |

## Build

```bash
git clone https://github.com/robertdavidgraham/masscan.git
cd masscan
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install masscan

# macOS
brew install masscan

# From source (recommended for latest)
git clone https://github.com/robertdavidgraham/masscan.git
cd masscan
make -j$(nproc)
sudo make install

# Docker
docker pull masscan/masscan
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/robertdavidgraham/masscan |
| Docs | https://github.com/robertdavidgraham/masscan/blob/master/README.md |
