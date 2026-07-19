# Suricata — Open-Source IDS/IPS/NSM Engine

High-performance network threat detection engine. Intrusion detection (IDS), intrusion prevention (IPS), and network security monitoring (NSM). Multi-threaded, GPU-accelerated, supports industry-standard rules.

## How It Works

Suricata captures packets via `libpcap` (inline IPS via `nfqueue` or `afpacket`). Inspects against:

**Detection engines:**
- **Signature-based** — Rules engine. Syntax compatible with Snort. Protocol-aware (HTTP, TLS, DNS, SMB, SSH, etc.)
- **Protocol parsing** — Built-in parsers normalize protocol fields, enabling signature-free anomaly detection
- **Application layer** — File extraction, HTTP body inspection, TLS certificate logging, DNS query/response logging
- **IP reputation** — Score-based IP blocking via external lists
- **Lua scripting** — Custom detection logic in rules

**EVE JSON output:** All events (alerts, metadata, flow, DNS, HTTP, TLS, files, stats) in newline-delimited JSON. Consumed by Elastic, Splunk, Logstash, etc.

**Performance:** Multi-threaded worker pool. AF_PACKET capture mode. CUDA-accelerated pattern matching. GSO/GRO offloading support.

## Manual

```bash
# Run IDS mode
suricata -c /etc/suricata/suricata.yaml -i eth0

# Run IPS mode (nfqueue)
suricata -c /etc/suricata/suricata.yaml -q 0

# Run against pcap file
suricata -c /etc/suricata/suricata.yaml -r capture.pcap

# Test config
suricata -T -c /etc/suricata/suricata.yaml

# Check stats
suricatasc -c "dump-counters"

# Update rules (suricata-update)
suricata-update
suricata-update enable-source oisf/trafficid
suricata-update list-sources
```

## Build

```bash
git clone https://github.com/OISF/suricata.git
cd suricata
git checkout suricata-7.0
./autogen.sh
./configure --enable-nfqueue --enable-hiredis --enable-libjansson
make -j$(nproc) && sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo add-apt-repository ppa:oisf/suricata-stable
sudo apt update && sudo apt install suricata

# RHEL/CentOS
sudo yum install epel-release
sudo yum install suricata

# macOS
brew install suricata

# Docker
docker pull jasonish/suricata:latest
docker run -it --net=host --cap-add=NET_ADMIN jasonish/suricata -i eth0

# Windows
# Download installer from suricata.io/download/
```

## Package

| Manager | Command |
|---------|---------|
| apt/PPA | `sudo apt install suricata` |
| Homebrew | `brew install suricata` |
| Docker | `docker pull jasonish/suricata` |
| RPM | `yum install suricata` |
| Source | GitHub releases |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://suricata.io/ |
| GitHub | https://github.com/OISF/suricata |
| Docs | https://docs.suricata.io/ |
| Rules | https://rules.emergingthreats.net/ |
| Suricata-Update | https://github.com/OISF/suricata-update |
| Eve JSON format | https://docs.suricata.io/en/latest/output/eve/index.html |
| Training | https://suricata.io/training/ |
