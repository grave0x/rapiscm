# aircrack-ng — WiFi Security Assessment Suite

Suite for WiFi monitoring, handshake capture, packet injection, WEP/WPA PSK cracking. Core toolset for wireless assessment.

## How It Works

Monitor mode on wireless interface -> capture packets (handshake, PMKID, data) -> crack PSK offline.

**Key components:**
- `airmon-ng` — enable/disable monitor mode on wireless interfaces
- `airodump-ng` — packet capture, AP/client discovery, channel hopping
- `aireplay-ng` — packet injection (deauth, ARP replay, fragmentation)
- `aircrack-ng` — WEP/WPA PSK cracking engine (PTW/KoreK/FMS for WEP, dictionary/table for WPA)
- `airdecap-ng` — decrypt WEP/WPA captures with known key
- `airgraph-ng` — generate client-AP relationship graphs

## Manual

```bash
# Enable monitor mode
airmon-ng start wlan0

# Discover networks + clients
airodump-ng wlan0mon

# Capture handshake on specific channel/bssid
airodump-ng -c 6 --bssid AA:BB:CC:DD:EE:FF -w capture wlan0mon

# Deauth client to force handshake
aireplay-ng -0 2 -a AA:BB:CC:DD:EE:FF -c CLIENT_MAC wlan0mon

# Crack WPA2 PSK
aircrack-ng -w wordlist.txt -b AA:BB:CC:DD:EE:FF capture-01.cap
```

## Build

```bash
git clone https://github.com/aircrack-ng/aircrack-ng.git
cd aircrack-ng
autoreconf -i
./configure --with-experimental
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install aircrack-ng

# Arch
sudo pacman -S aircrack-ng

# macOS
brew install aircrack-ng

# Windows
# Use WSL or download compiled binary from GitHub releases
```

## Package

Distribution archives on GitHub releases. Also packaged in Kali Linux, Pentoo, BackBox.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.aircrack-ng.org/ |
| GitHub | https://github.com/aircrack-ng/aircrack-ng |
| Docs | https://www.aircrack-ng.org/doku.php |
| Wiki | https://github.com/aircrack-ng/aircrack-ng/wiki |
| GUI (airgraph-ng) | https://www.aircrack-ng.org/doku.php?id=airgraph-ng |
