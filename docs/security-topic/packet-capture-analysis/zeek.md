# Zeek — Network Security Monitor

Formerly Bro. A powerful network analysis framework that passively monitors traffic and produces structured, application-layer logs for security analysis.

## How It Works

Zeek sits on a SPAN/mirror port or TAP and processes traffic in two layers: the **event engine** (low-level packet parsing, protocol detection) and the **policy layer** (Zeek scripts that generate actionable logs). It produces 50+ log types covering connections, DNS, HTTP, SSL, SMTP, DHCP, FTP, and more.

**Key log types:**

| Log | Description |
|-----|-------------|
| `conn.log` | Every TCP/UDP/ICMP connection — duration, bytes, state |
| `dns.log` | All DNS queries and responses |
| `http.log` | HTTP requests, methods, URIs, user agents, MIME types |
| `ssl.log` | TLS handshake details — cipher, cert, SNI, JA3 |
| `smb.log` | SMB/CIFS file access and commands |
| `notice.log` | Zeek-generated alerts (scanning, malware, policy violations) |
| `weird.log` | Protocol anomalies and malformed packets |
| `files.log` | Extracted file metadata (hashes, MIME) |
| `x509.log` | Certificate details |

## Manual

### Basic Usage

```bash
# Capture live on interface
zeek -i eth0

# Read PCAP file
zeek -r capture.pcap

# Load custom script
zeek -i eth0 local.zeek

# Specific output directory
zeek -i eth0 -C LogDir:/var/log/zeek

# Disable checksums (-C for virtual interfaces)
zeek -i eth0 -C
```

### Common Analysis

```bash
# Find hosts with most connections
cat conn.log | zeek-cut id.orig_h | sort | uniq -c | sort -rn

# Top DNS queries
cat dns.log | zeek-cut query | sort | uniq -c | sort -rn

# HTTP methods breakdown
cat http.log | zeek-cut method | sort | uniq -c | sort -rn

# Failed connections
cat conn.log | zeek-cut id.orig_h,id.resp_h,service,conn_state | grep -v SF

# TLS cipher analysis
cat ssl.log | zeek-cut cipher | sort | uniq -c | sort -rn
```

### Integration with Elastic

```yaml
# Filebeat config to ship Zeek logs
filebeat.inputs:
  - type: log
    paths: /opt/zeek/logs/current/*.log
    fields:
      log_type: zeek
```

### Useful Scripts

```bash
# Load all standard scripts
zeek -i eth0 policy/protocols/conn/mac-logging
zeek -i eth0 policy/frameworks/files/extract-all-files
```

## Build

```bash
git clone --recursive https://github.com/zeek/zeek.git
cd zeek
./configure
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install zeek

# macOS
brew install zeek

# Docker
docker pull zeek/zeek

# Pre-built packages
# https://docs.zeek.org/en/current/install/install.html
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://zeek.org/ |
| GitHub | https://github.com/zeek/zeek |
| Documentation | https://docs.zeek.org/ |
| Package manager | https://packages.zeek.org/ |
| Book | https://www.zeek.org/book/ |
