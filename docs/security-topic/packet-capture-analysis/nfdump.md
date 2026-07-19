# nfdump — NetFlow Analysis Tools

Tool suite for collecting, processing, and analyzing NetFlow v5/v9/IPFIX data. De facto standard for flow-based network traffic analysis.

## How It Works

nfcapd collects NetFlow exports from routers/switches and stores them in time-indexed files (5-min blocks by default). nfdump reads these files, applies filters, and outputs flow records, statistics, or aggregated views. The flow engine supports aggregation, top-N analysis, and time-series bucketing across multiple collectors.

**Core tools:**

| Tool | Description |
|------|-------------|
| **nfcapd** | NetFlow collection daemon |
| **nfdump** | Query and analysis tool |
| **nfanon** | Anonymize IPs in flow data |
| **nfreplay** | Replay NetFlow data |
| **nfstat** | Collector statistics |

## Manual

### Collector Setup

```bash
# Start collector (port 9995 for NetFlow v5/v9)
nfcapd -l /data/nfcapd -p 9995

# With auto-rotation
nfcapd -l /data/nfcapd -p 9995 -t 300  # 5-min rotation (default)

# Background
nfcapd -l /data/nfcapd -p 9995 -D

# Collect from specific source
nfcapd -l /data/nfcapd -p 9995 -S 192.168.1.1
```

### Query Basics

```bash
# Read most recent data
nfdump -R /data/nfcapd

# Merge all data in directory
nfdump -M /data/nfcapd

# Specific file
nfdump -r /data/nfcapd/nfcapd.202501011200
```

### Filtering

```bash
# Filter by host
nfdump -R /data/nfcapd 'host 192.168.1.100'

# Filter by port
nfdump -R /data/nfcapd 'port 443'

# Filter by protocol
nfdump -R /data/nfcapd 'proto tcp'

# Time range
nfdump -R /data/nfcapd -t 14:00-15:00

# Compound
nfdump -R /data/nfcapd 'src host 10.0.0.1 and dst port 80'
```

### Aggregation

```bash
# Top talkers (by bytes)
nfdump -R /data/nfcapd -s srcip -o "bytes"

# Top destinations
nfdump -R /data/nfcapd -s dstip -o "bytes"

# Top ports
nfdump -R /data/nfcapd -s port -o "packets"

# Protocol breakdown
nfdump -R /data/nfcapd -s proto -o "bytes"
```

### Output Formats

```bash
# Long format (all fields)
nfdump -R /data/nfcapd -o long

# Extended format
nfdump -R /data/nfcapd -o extended

# CSV output
nfdump -R /data/nfcapd -o csv > flows.csv

# JSON output
nfdump -R /data/nfcapd -o json > flows.json

# Custom format
nfdump -R /data/nfcapd -o "fmt:%ts %te %sa %da %pkt"
```

### Statistics

```bash
# Flow statistics
nfdump -R /data/nfcapd -s ip/flows -o "bytes"

# Time series by hour
nfdump -R /data/nfcapd -A hour -o "bytes"

# Top autonomous systems
nfdump -R /data/nfcapd -s as -o "bytes"
```

## Build

```bash
git clone https://github.com/phaag/nfdump.git
cd nfdump
./autogen.sh
./configure --enable-nfexp
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install nfdump

# RHEL/Fedora
sudo dnf install nfdump

# macOS
brew install nfdump

# Source build
git clone https://github.com/phaag/nfdump.git
cd nfdump && ./autogen.sh && ./configure && make && sudo make install
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/phaag/nfdump |
| Docs | https://github.com/phaag/nfdump/wiki |
| NetFlow v5/v9 | https://www.ietf.org/rfc/rfc3954.txt |
| IPFIX | https://www.ietf.org/rfc/rfc7011.txt |
