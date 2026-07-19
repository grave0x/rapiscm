# Kismet — Wireless IDS / Network Detector

Wireless intrusion detection system supporting 802.11, BT, BLE, and SDR-based device detection, logging, and alerting.

## How It Works

Kismet operates as a passive wireless sniffer. Server captures packets from one or more radio interfaces, parses them, and logs device/network metadata. Web UI or REST API provides real-time visibility.

**Key features:**
- Multi-interface capture (WiFi + BT/BLE + SDR simultaneously)
- Device fingerprinting (OS, manufacturer, device class)
- GPS logging and wardriving integration
- Alert system — deauth floods, probe requests, rogue AP
- Plugin system for custom data sources
- PCAPNG log output

## Manual

```bash
# Start Kismet server
kismet -c wlan0 -c wlan1

# Web UI access
# http://localhost:2501

# Capture + logging only (no UI)
kismet -c wlan0 --log-to-file /path/to/logs

# View log folder for alerts
kismet_logs/
  Kismet-CCYYMMDD-HHMMSS-1.netxml    # Network list
  Kismet-CCYYMMDD-HHMMSS-1.pcapng    # Packet capture
  Kismet-CCYYMMDD-HHMMSS-1.alert     # Alerts
  Kismet-CCYYMMDD-HHMMSS-1.gps       # GPS data

# REST API examples
curl http://localhost:2501/messagebus/all.json
curl http://localhost:2501/devices/any/all_devices.json
```

## Build

```bash
git clone https://github.com/kismetwireless/kismet.git
cd kismet
./configure
make -j$(nproc)
sudo make install
sudo ldconfig
```

## Install

```bash
# Debian/Ubuntu
sudo apt install kismet

# macOS
brew install kismet

# Arch
sudo pacman -S kismet

# Docker
docker pull kismet/kismet-dev
```

## Package

Debian/Ubuntu repos, Homebrew, Arch AUR. Daily dev builds available from Kismet website. Kali includes it by default.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.kismetwireless.net/ |
| GitHub | https://github.com/kismetwireless/kismet |
| Docs | https://www.kismetwireless.net/docs/ |
| REST API | https://www.kismetwireless.net/docs/readme/restapi/ |
| Capturing sources | https://www.kismetwireless.net/docs/readme/datasources/ |
| Alert types | https://www.kismetwireless.net/docs/readme/alerts/ |
