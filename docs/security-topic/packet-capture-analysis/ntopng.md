# ntopng — Network Traffic Analysis Web UI

High-performance web-based traffic analysis and flow monitoring tool. Provides real-time network visibility with historical reporting.

## How It Works

ntopng captures packets or ingests NetFlow/IPFIX/sFlow flows and presents traffic data through a web UI. It maintains per-host, per-protocol, and per-ASN statistics with historical DB storage (RRD or InfluxDB). The analysis engine identifies active hosts, protocol breakdown (DPI), application-layer mappings, top talkers, and anomalous traffic patterns.

**Core concepts:**

| Concept | Description |
|---------|-------------|
| **Host** | Individual IP with its own traffic dashboard |
| **Flow** | Uni/bi-directional traffic session metadata |
| **DPI (nDPI)** | Deep packet inspection — 300+ protocol classification |
| **Historical** | RRD/MySQL-based time-series data |
| **Alerts** | Threshold-based: throughput, protocol anomaly, host behavior |

## Manual

### Basic Usage

```bash
# Start on default port 3000 (no auth)
ntopng -i eth0

# With authentication
ntopng -i eth0 -u admin:password

# Read PCAP file (offline)
ntopng -i capture.pcap

# Specify data directory
ntopng -i eth0 --data-dir /var/ntopng

# Listen on specific interface
ntopng -i eth0 -w 0.0.0.0:3000
```

### NetFlow/sFlow Collector

```bash
# Run as flow collector (port 2055 for NetFlow v5/v9/IPFIX)
ntopng -i nf:eth0 -n 1.2.3.4

# sFlow (port 6343)
ntopng -i sflow:eth0

# Multiple interfaces
ntopng -i "tcp:*:2055" -i eth0
```

### Docker

```bash
docker run -d --net=host \
  -v /etc/ntopng:/etc/ntopng \
  ntop/ntopng:stable \
  -i eth0
```

### CLI Options

| Flag | Description |
|------|-------------|
| `-i <interface>` | Capture interface or file |
| `-w <addr:port>` | Web interface bind address |
| `-m <net/mask>` | Local network (for categorization) |
| `--community` | Community edition mode |
| `--shutdown-when-done` | Exit after PCAP processing |

## Build

```bash
git clone https://github.com/ntop/ntopng.git
cd ntopng
./autogen.sh
./configure
make -j$(nproc)
sudo make install
```

### Dependencies

```bash
# Requires nDPI, hiredis, json-c, libpcap, sqlite3, zeromq
sudo apt install libpcap-dev libjson-c-dev libhiredis-dev libsqlite3-dev libzmq3-dev
git clone https://github.com/ntop/nDPI.git
cd nDPI && ./autogen.sh && make && sudo make install
```

## Install

```bash
# Debian/Ubuntu (official repo)
sudo apt install ntopng

# Docker
docker pull ntop/ntopng:stable

# Source build (see Build section)
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.ntop.org/products/traffic-analysis/ntop/ |
| GitHub | https://github.com/ntop/ntopng |
| Docs | https://www.ntop.org/guides/ntopng/ |
| nDPI | https://github.com/ntop/nDPI |
| Docker | https://hub.docker.com/r/ntop/ntopng |
| Community | https://www.ntop.org/community/ |
