# tshark — CLI Packet Analyzer (Wireshark's Terminal Interface)

Command-line version of Wireshark. Scriptable packet analysis, capture, and field extraction.

## How It Works

tshark uses the same dissection engine as Wireshark but operates in the terminal. It can capture live traffic, read PCAP files, and extract specific protocol fields using display filter syntax — ideal for automated pipelines and CI.

**Key concepts:**

| Concept | Description |
|---------|-------------|
| **-T fields** | Extract specific fields as tabular output |
| **-Y** | Display filter (post-capture) |
| **-f** | Capture filter (BPF, pre-capture) |
| **-z** | Statistics (conversations, IO graph, protocol hierarchy) |
| **-r** | Read from file (instead of live capture) |

## Manual

### Capture

```bash
# Capture on interface, write to file
tshark -i eth0 -w capture.pcap -a duration:60

# Capture with BPF filter
tshark -i eth0 -f "port 443" -w https.pcap

# Capture 1000 packets then stop
tshark -i eth0 -c 1000
```

### Analyze

```bash
# Display filter on existing PCAP
tshark -r capture.pcap -Y "http.request"

# Extract specific fields
tshark -r capture.pcap -T fields -e ip.src -e ip.dst -e tcp.port

# HTTP request/response pairs
tshark -r capture.pcap -Y "http" -T fields \
  -e http.request.method -e http.request.uri -e http.response.code

# DNS queries
tshark -r capture.pcap -Y "dns.flags.response == 0" \
  -T fields -e dns.qry.name

# TLS SNI extraction
tshark -r capture.pcap -Y "tls.handshake.extensions_server_name" \
  -T fields -e tls.handshake.extensions_server_name
```

### Statistics

```bash
# Protocol hierarchy
tshark -r capture.pcap -z io,phs

# Top conversations
tshark -r capture.pcap -z conv,tcp

# HTTP request rates
tshark -r capture.pcap -z http,tree

# IO graph (for time-series)
tshark -r capture.pcap -z io,stat,1
```

### Automation

```bash
# Count HTTP methods
tshark -r access.log.pcap -Y "http.request" \
  -T fields -e http.request.method | sort | uniq -c

# List unique server IPs
tshark -r capture.pcap -Y "tcp.flags.syn == 1 && tcp.flags.ack == 0" \
  -T fields -e ip.dst | sort -u
```

## Build

```bash
# Same build as Wireshark (tshark is in the wireshark repo)
git clone https://gitlab.com/wireshark/wireshark.git
cd wireshark
cmake -S . -B build
cmake --build build -j$(nproc)
sudo cmake --install build
```

## Install

```bash
# Debian/Ubuntu
sudo apt install tshark

# RHEL/Fedora
sudo dnf install wireshark-cli

# macOS
brew install wireshark  # includes tshark

# Docker
docker pull linuxserver/wireshark
# tshark is available in the Wireshark Docker image
```

## Links

| Resource | URL |
|----------|-----|
| Man page | https://www.wireshark.org/docs/man-pages/tshark.html |
| Display filters | https://www.wireshark.org/docs/dfref/ |
| Wireshark site | https://www.wireshark.org/ |
