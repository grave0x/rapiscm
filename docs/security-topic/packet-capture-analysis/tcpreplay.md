# Tcpreplay — PCAP Replay & Editing Suite

Suite of tools for replaying, editing, and transforming PCAP files through live network interfaces. Used for testing IDS/IPS, firewalls, and network monitoring tools.

## How It Works

Tcpreplay reads PCAP files and replays packets through a network interface at controlled speeds. It rewrites Layer 2/3 headers to ensure packets are relevant to the test network. The suite includes tools for splitting, merging, rewriting MAC/IP addresses, and generating traffic profiles.

**Core tools:**

| Tool | Description |
|------|-------------|
| **tcpreplay** | Replay PCAP through interface |
| **tcprewrite** | Rewrite L2/L3 headers in PCAP files |
| **tcpcapinfo** | Display PCAP metadata |
| **tcpprep** | Create cache files for dual-NIC replay |
| **tcpreplay-edit** | Replay with inline packet editing |
| **flowtop** | Flow-based top talkers from PCAP |

## Manual

### Basic Replay

```bash
# Simple replay (single interface)
tcpreplay -i eth0 capture.pcap

# Loop replay
tcpreplay -i eth0 --loop=10 capture.pcap

# Speed control (multiplier)
tcpreplay -i eth0 --mbps=100 capture.pcap      # 100 Mbps
tcpreplay -i eth0 --pps=1000 capture.pcap       # 1000 pkts/sec
tcpreplay -i eth0 --multiplier=2.0 capture.pcap # 2x original speed
```

### Packet Editing

```bash
# Rewrite destination MAC
tcprewrite --dstmac=00:11:22:33:44:55 -i capture.pcap -o edited.pcap

# Rewrite source MAC
tcprewrite --srcmac=aa:bb:cc:dd:ee:ff -i capture.pcap -o edited.pcap

# Rewrite IP addresses
tcprewrite --dstip=192.168.1.100 -i capture.pcap -o edited.pcap

# Rewrite both MAC and IP
tcprewrite --endpoints=192.168.1.1:10.0.0.1 -i capture.pcap -o edited.pcap

# Remove VLAN tags
tcprewrite --delete-vlan -i capture.pcap -o edited.pcap
```

### Advanced Replay

```bash
# Dual-NIC replay (client + server)
tcpprep -a client -i capture.pcap -o cache.file
tcpreplay -i eth0 --dualfile -c cache.file capture.pcap

# Replay with timing accuracy
tcpreplay -i eth0 --pnat=192.168.1.0/24:10.0.0.0/24 capture.pcap

# Limit packets
tcpreplay -i eth0 --limit=1000 capture.pcap

# Unique IP for each replay loop
tcpreplay -i eth0 --unique-ip --loop=5 capture.pcap
```

### PCAP Info & Stats

```bash
# Show PCAP details
tcpcapinfo capture.pcap

# Show cache info
tcpprep -I cache.file
```

## Build

```bash
git clone https://github.com/appneta/tcpreplay.git
cd tcpreplay
./autogen.sh
./configure --enable-dynamic-link
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install tcpreplay

# RHEL/Fedora
sudo dnf install tcpreplay

# macOS
brew install tcpreplay

# Source build
git clone https://github.com/appneta/tcpreplay.git
cd tcpreplay && ./autogen.sh && ./configure && make && sudo make install
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/appneta/tcpreplay |
| Docs | https://tcpreplay.appneta.com/ |
| FAQ | https://tcpreplay.appneta.com/faq.html |
| Features | https://tcpreplay.appneta.com/wiki/features.html |
