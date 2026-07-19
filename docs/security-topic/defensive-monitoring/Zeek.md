# Zeek — Network Security Monitor (NIDS)

Passive network security monitor — protocol parsing (HTTP, DNS, SSL, SMB, Kerberos, etc.), scriptable event engine, structured log output. Formerly Bro.

## How It Works

Zeek sits passively on a network tap or span port, captures traffic, and analyzes protocols at the application layer. It produces structured, tab-separated logs per protocol/event type — no signatures needed for baseline visibility.

**Key log types:**

| Log File | Content |
|----------|---------|
| `conn.log` | Connection summaries (IP, port, protocol, bytes, duration) |
| `dns.log` | DNS queries + answers |
| `http.log` | HTTP requests, methods, URIs, user-agents, status codes |
| `ssl.log` | TLS handshake, certificate info, JA3 fingerprints |
| `smb.log` | SMB file shares, named pipes |
| `kerberos.log` | Kerberos authentication |
| `notice.log` | Policy violations and anomalies |
| `files.log` | Extracted file metadata (MD5/SHA1) |

**Detection:** Zeek scripts (`.zeek` files written in Zeek scripting language) define event handlers that react to protocol events. Policy scripts for alerting, malware detection, and anomaly identification.

## Manual

```bash
# Start Zeek on interface
sudo zeek -i eth0

# Read PCAP offline
zeek -r capture.pcap

# Load specific scripts
zeek -i eth0 local
zeek -r capture.pcap ../../scripts/policy/protocols/ssl/decryption-keys.zeek

# Processed logs directory
ls /usr/local/zeek/logs/current/

# Query conn.log
zeek-cut uid proto service orig_bytes < conn.log
```

## Install

```bash
# Debian/Ubuntu
sudo apt install zeek

# macOS
brew install zeek

# Docker
docker pull zeek/zeek
docker run -i -t --rm --net=host zeek/zeek zeek -i eth0

# Manual (binary package)
wget https://github.com/zeek/zeek/releases/download/v6.0.4/zeek-6.0.4-linux-x86_64.tar.gz
tar xzf zeek-6.0.4-linux-x86_64.tar.gz -C /opt/
```

## Build

```bash
git clone --recursive https://github.com/zeek/zeek.git
cd zeek
./configure
make -j$(nproc)
sudo make install
```

## Package

| Manager | Command |
|---------|---------|
| DEB/RPM | Official Zeek repo |
| macOS | `brew install zeek` |
| Docker | `zeek/zeek` |
| Source | Build from GitHub |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://zeek.org/ |
| GitHub | https://github.com/zeek/zeek |
| Docs | https://docs.zeek.org/ |
| Script reference | https://docs.zeek.org/en/master/scripting/index.html |
| Package manager | https://packages.zeek.org/ |
| Community scripts | https://github.com/zeek/zeek-scripts |
| Zeek Enterprise | https://www.zeek.org/enterprise/ |
