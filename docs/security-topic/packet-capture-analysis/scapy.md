# Scapy — Packet Manipulation Library

Python library for packet crafting, sniffing, dissection, and network protocol fuzzing. Use it interactively or as a library for custom network tools.

## How It Works

Scapy operates at Layer 2–7, constructing packets as Python objects composed of protocol layers. Each layer is a class with fields that can be set arbitrarily. Packets are sent via raw sockets (L2) or BPF (L3), and responses are captured and matched. The library includes implementations for 300+ protocols and can be extended with custom protocol classes.

**Core concepts:**

| Concept | Description |
|---------|-------------|
| **Packet layers** | `Ether()/IP()/TCP()` — compose layers with `/` operator |
| **Sniffing** | BPF-based capture with per-packet callback |
| **Answering** | `sr()`, `sr1()`, `srloop()` — send/receive matching |
| **Routing** | Built-in routing table for L3 packet sending |
| **ls()** | List fields for any protocol layer |
| **Packet fields** | Bit-field, int, string, conditional — field types |

## Manual

### Interactive Shell

```bash
scapy
```

### Packet Crafting

```python
# Build a TCP SYN packet
pkt = IP(dst="target.com")/TCP(dport=80, flags="S")

# Send and receive
ans = sr1(pkt, timeout=2)
ans.show()

# ARP request
arp = ARP(pdst="192.168.1.1")
ans = sr1(arp, timeout=2)
print(ans.hwsrc)  # MAC address

# DNS query
dns = IP(dst="8.8.8.8")/UDP(dport=53)/DNS(rd=1, qd=DNSQR(qname="target.com"))
ans = sr1(dns, timeout=2)
ans[DNS].an.rdata
```

### Sniffing

```python
# Simple packet sniffer
pkts = sniff(count=100, iface="eth0")
pkts.summary()

# With BPF filter
pkts = sniff(filter="tcp port 443", count=50)

# Per-packet callback
def analyze(pkt):
    if pkt.haslayer(TCP):
        print(pkt[IP].src, pkt[TCP].dport)

sniff(prn=analyze, count=100)

# Save and load
wrpcap("capture.pcap", pkts)
pkts = rdpcap("capture.pcap")
```

### Common Tasks

```python
# TCP port scan
ans, unans = sr(IP(dst="target.com")/TCP(dport=[80,443,22], flags="S"), timeout=2)
for s, r in ans:
    if r.haslayer(TCP) and r[TCP].flags & 0x12:  # SYN-ACK
        print(f"{r[IP].src}:{r[TCP].sport} open")

# ICMP ping sweep
ans, unans = sr(IP(dst="192.168.1.0/24")/ICMP(), timeout=2)
ans.summary(lambda s,r: r.sprintf("%IP.src% is alive"))

# Traceroute
ans = traceroute("target.com")
ans.show()
```

## Install

```bash
# Debian/Ubuntu
sudo apt install python3-scapy

# Pip
pip install scapy

# macOS
brew install scapy

# Docker
docker pull secsi/scapy
```

### Optional Dependencies

```bash
# For packet visualization
pip install matplotlib

# For graphviz visualization
pip install graphviz
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://scapy.net/ |
| GitHub | https://github.com/secdev/scapy |
| Docs | https://scapy.readthedocs.io/ |
| API reference | https://scapy.readthedocs.io/en/latest/api/scapy.html |
| Packet layers | https://scapy.readthedocs.io/en/latest/usage.html |
| Interactive tutorial | https://scapy.readthedocs.io/en/latest/usage.html#interactive-tutorial |
