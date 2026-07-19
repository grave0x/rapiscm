# Wifiphisher — Evil Twin + Captive Portal Phishing

Captive portal and evil twin phishing framework. Creates a rogue AP, performs deauth on target, and serves a phishing captive portal.

## How It Works

Three-phase attack:
1. **Rogue AP** — Creates a cloned AP with the same SSID (evil twin)
2. **Deauth** — Deauthenticates clients from the real AP, forcing reassociation to the rogue AP
3. **Captive Portal** — Serves a phishing page mimicking router config, browser update, or login page

**Phishing scenarios (scenario files):**
- Router configuration page (password capture)
- Browser update notification
- Firmware upgrade page
- Social media login page
- OAuth consent page

## Manual

```bash
# Interactive mode
sudo wifiphisher

# Specify target AP BSSID
sudo wifiphisher -aI wlan0 -jI wlan0 -p PK -b AA:BB:CC:DD:EE:FF

# Options
# -aI: Access point interface
# -jI: Internet/upstream interface (for portal)
# -p: Phishing scenario (PK = "Connectivity Check")
# -b: Target BSSID
# -c: Channel
# -e: ESSID

# List available scenarios
sudo wifiphisher --list-scenarios

# Force a specific scenario
sudo wifiphisher -p firmware-upgrade

# With upstream internet to clients
sudo wifiphisher -aI wlan0 -jI eth0 -p connectivity-check
```

## Install

```bash
# Debian/Ubuntu
sudo apt install wifiphisher

# Git clone + install
git clone https://github.com/wifiphisher/wifiphisher.git
cd wifiphisher
sudo python3 setup.py install

# Kali
# Pre-installed
```

## Build

```bash
# No build needed — Python3 project
git clone https://github.com/wifiphisher/wifiphisher.git
sudo pip3 install -r requirements.txt
```

## Package

Debian/Ubuntu repos (`wifiphisher`). Kali Linux includes it. GitHub provides release tarballs.

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/wifiphisher/wifiphisher |
| Docs | https://wifiphisher.readthedocs.io/ |
| Scenario list | https://github.com/wifiphisher/wifiphisher/tree/master/wifiphisher/data/phishing-pages |
| Blog/wiki | https://wifiphisher.org/ |
