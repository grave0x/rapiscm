# Wireshark — Network Protocol Analyzer

Premier GUI packet analyzer with 3,000+ protocol dissectors. Industry standard for network troubleshooting and protocol analysis.

## How It Works

Wireshark captures packets from a live interface or reads PCAP files. It decodes protocols via its dissection engine, presenting packet structure in a tree view. Supports display filters, coloring rules, follow TCP/UDP/TLS streams, and export objects.

**Key concepts:**

| Concept | Description |
|---------|-------------|
| **Capture filters** | BPF syntax — filter at capture time (`port 443`) |
| **Display filters** | Protocol-field syntax — filter after capture (`http.response.code == 200`) |
| **Follow stream** | Reconstruct TCP/UDP/TLS session in ASCII/hex |
| **Export objects** | Extract files transferred via HTTP, SMB, TFTP |
| **Name resolution** | MAC → vendor, IP → DNS, port → service |

## Manual

### Basic Usage (GUI)

1. Select capture interface
2. Apply capture filter (optional)
3. Start capture
4. Apply display filters
5. Click a packet → inspect tree in middle pane, hex in bottom

### Display Filters

```bash
# Common filters
http
dns
tcp.port == 443
ip.addr == 192.168.1.1
http.request.method == "POST"
tls.handshake.type == 1           # Client Hello
http.response.code >= 400
tcp.flags.syn == 1 && tcp.flags.ack == 0  # SYN packets
!(arp or icmp or dns)             # Exclude noise
```

### CLI (tshark style)

```bash
# Capture 100 packets to file
wireshark -i eth0 -k -a duration:60

# Read and analyze
wireshark -r capture.pcap
```

### Key Analysis Tasks

```bash
# Find slow responses (HTTP)
http.time > 1

# Detect suspicious DNS
dns.qry.name contains "malware"

# Extract HTTP objects (File → Export Objects → HTTP)

# TLS handshake analysis
tls.handshake.type == 11  # Certificate
```

## Build

```bash
git clone https://gitlab.com/wireshark/wireshark.git
cd wireshark
cmake -S . -B build
cmake --build build -j$(nproc)
sudo cmake --install build
```

## Install

```bash
# Debian/Ubuntu
sudo apt install wireshark

# RHEL/Fedora
sudo dnf install wireshark

# macOS
brew install --cask wireshark

# Windows
# Download installer from wireshark.org/download.html

# Docker (CLI only)
docker pull linuxserver/wireshark
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.wireshark.org/ |
| Download | https://www.wireshark.org/download.html |
| User guide | https://www.wireshark.org/docs/wsug_html/ |
| Display filters | https://www.wireshark.org/docs/dfref/ |
| Protocol ref | https://www.wireshark.org/docs/ |
| Sample captures | https://wiki.wireshark.org/SampleCaptures |
