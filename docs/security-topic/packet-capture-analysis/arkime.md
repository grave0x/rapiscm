# Arkime — Full Packet Capture & Search

Formerly Moloch. Large-scale full packet capture, indexing, and search system with a web interface.

## How It Works

Arkime consists of three components: **capture** (reads packets, writes PCAP, extracts metadata), **elasticsearch** (indexes sessions for search), and **viewer** (web UI for querying, PCAP download, timeline analysis). Designed for hundreds of Gbps of traffic.

**Components:**

| Component | Role |
|-----------|------|
| **capture** | Multi-threaded packet capture, session metadata extraction, PCAP storage |
| **elasticsearch** | Indexes session metadata for near-real-time search |
| **viewer** | Node.js web UI — queries, sessions, PCAP download, SPI graph |
| **parliament** | Multi-cluster management dashboard |

**Key features:**
- Full PCAP retention for forensic reconstruction
- Session-based search (not raw packets)
- SPI (Session Profile Intelligence) graph for visual correlation
- PCAP download per session for Wireshark analysis
- Multi-cluster support via Parliament

## Manual

### Basic Commands

```bash
# Initialize elasticsearch
arkime_init.sh

# Add admin user
cd /opt/arkime
node db/db.pl localhost:9200 addUser admin "Admin User" password --admin

# Run capture
cd /opt/arkime
./capture -c /opt/arkime/etc/config.ini

# Viewer
cd /opt/arkime/viewer
node viewer.js -c /opt/arkime/etc/config.ini
```

### Web Search Queries

```bash
# Simple field searches
ip == 8.8.8.8
port == 443
protocol == dns
country == RU

# Compound
ip == 10.0.0.0/8 && port != 80
http.host == "example.com"
tags == "malware"

# Time-based
firstPacket > "2024-01-01"
lastPacket < "2024-06-01"

# Regex
http.uri ~= "/api/.*"
userAgent ~= "curl/.*"
```

### API

```bash
# Search sessions
curl -u admin:password \
  "http://arkime:8005/api/sessions?date=-1&expression=port%3D%3D443"

# Get PCAP
curl -u admin:password \
  "http://arkime:8005/api/sessions.pcap?date=-1&expression=ip%3D%3D8.8.8.8" \
  -o session.pcap

# Unique field values
curl -u admin:password \
  "http://arkime:8005/api/unique?date=-1&field=http.host&expression=port%3D%3D80"
```

## Build

```bash
git clone https://github.com/arkime/arkime.git
cd arkime
./easybutton-build.sh
```

## Install

```bash
# Elasticsearch 7.x+ and Node.js 16+ required

# Quick install script
wget https://github.com/arkime/arkime/releases/latest/download/arkime_*.deb
sudo dpkg -i arkime_*.deb

# Docker (recommended)
docker pull arkime/arkime
docker pull elasticsearch:7.17

# Compose config available at:
# https://github.com/arkime/arkime-docker
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://arkime.com/ |
| GitHub | https://github.com/arkime/arkime |
| Docs | https://arkime.com/documentation |
| Docker | https://github.com/arkime/arkime-docker |
