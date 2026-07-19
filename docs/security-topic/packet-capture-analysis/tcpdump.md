# tcpdump — Packet Capture (CLI)

Lightweight command-line packet capture and analysis tool. Widely available on all UNIX systems. Uses libpcap.

## How It Works

tcpdump captures raw packets from a network interface, applies a BPF filter, and displays packet summaries. It can write to PCAP files for later analysis and read captured files. Minimal overhead makes it ideal for production servers and embedded systems.

**Key concepts:**

| Concept | Description |
|---------|-------------|
| **BPF filter** | Berkeley Packet Filter — `host`, `port`, `tcp`, `udp`, `icmp` |
| **Promiscuous mode** | Captures all packets on the wire, not just to this host |
| **-w** | Write raw packets to file (binary PCAP) |
| **-r** | Read packets from a PCAP file |
| **-v / -vv / -vvv** | Verbosity levels for packet detail |

## Manual

### Basic Usage

```bash
# Capture on interface eth0
sudo tcpdump -i eth0

# Capture with count limit (100 packets)
sudo tcpdump -i eth0 -c 100

# Capture to file
sudo tcpdump -i eth0 -w capture.pcap

# Read PCAP file
tcpdump -r capture.pcap
```

### Filters

```bash
# By host
sudo tcpdump host 192.168.1.1
sudo tcpdump src host 10.0.0.1
sudo tcpdump dst host 8.8.8.8

# By port
sudo tcpdump port 443
sudo tcpdump portrange 8000-8080

# By protocol
sudo tcpdump icmp
sudo tcpdump arp
sudo tcpdump tcp

# Complex expressions
sudo tcpdump "tcp[tcpflags] & (tcp-syn|tcp-ack) != 0"
sudo tcpdump "src net 192.168.0.0/16 and (port 80 or port 443)"

# Exclude noise
sudo tcpdump not arp and not icmp
```

### Verbose Output

```bash
# Minimal (default)
tcpdump -r capture.pcap

# Verbose
tcpdump -r capture.pcap -v

# Very verbose (full packet)
tcpdump -r capture.pcap -vv -X

# Hex + ASCII
tcpdump -r capture.pcap -X

# Timestamps with microseconds
tcpdump -r capture.pcap -tttt
```

### Advanced

```bash
# Rotating capture files
tcpdump -i eth0 -w capture -G 3600 -C 100
# -G: rotate every N seconds
# -C: max file size in MB

# Snapshot length (snip packets at 256 bytes)
tcpdump -i eth0 -s 256

# Don't resolve addresses (faster + raw)
tcpdump -i eth0 -n
```

## Build

```bash
git clone https://github.com/the-tcpdump-group/tcpdump.git
cd tcpdump
./configure
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install tcpdump

# RHEL/Fedora
sudo dnf install tcpdump

# Alpine
apk add tcpdump

# macOS (pre-installed with dev tools)
# brew install tcpdump  # optional, newer version

# From source
git clone https://github.com/the-tcpdump-group/tcpdump.git
cd tcpdump && ./configure && make && sudo make install
```

## Links

| Resource | URL |
|----------|-----|
| Man page | https://www.tcpdump.org/manpages/tcpdump.1.html |
| Official site | https://www.tcpdump.org/ |
| GitHub | https://github.com/the-tcpdump-group/tcpdump |
