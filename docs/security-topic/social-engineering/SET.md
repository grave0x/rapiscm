# SET — Social Engineering Toolkit

Multi-vector social engineering framework — spear-phishing, credential harvesting, webpage cloning, infectious media generator, and more.

## How It Works

SET is a menu-driven Python framework. Modules cover the full SE attack lifecycle:

- **Spear-Phishing Attack Vector** — Email with malicious attachment or link, single/mass mailer
- **Website Attack Vectors** — Credential harvester, webpage clone, Java applet, Metasploit browser exploits, Tabnabbing
- **Infectious Media Generator** — USB/DVD payload autorun
- **Teejlic (Tight Security)** — HTML smuggling for bypassing security controls
- **QR Code Generator** — QR code with embedded attack URL
- **PowerShell Attack Vectors** — PowerShell injection via Alfa Shell, web delivery
- **Third-Party Modules** — Fast-Track, Web Attack, SMS Spoofing, Wireless AP

## Manual

```bash
# Interactive menu
sudo setoolkit

# CLI: Website cloner + credential harvester (headless)
sudo python3 set.py --cli -c "1" -s "2" -y -i eth0

# CLI: Mass mailer with template
sudo python3 set.py --cli -c "1" -s "5"

# Web attack vector (Java applet)
sudo python3 set.py --cli -c "2"

# Generate infectious media
sudo python3 set.py --cli -c "3"

# After harvest: results in /root/.set/reports/
ls /root/.set/reports/
```

## Build

```bash
git clone https://github.com/trustedsec/social-engineer-toolkit.git
cd social-engineer-toolkit
sudo pip3 install -r requirements.txt
sudo python3 setup.py install
```

## Install

```bash
# Debian/Ubuntu
sudo apt install set

# macOS
brew install set

# Git + pip (latest)
git clone https://github.com/trustedsec/social-engineer-toolkit.git
cd social-engineer-toolkit
sudo pip3 install -r requirements.txt
sudo python3 setup.py install

# Kali
# Pre-installed
```

## Package

Debian/Ubuntu repos (`set`). Included in Kali Linux by default. GitHub releases. Python package dependencies separately.

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/trustedsec/social-engineer-toolkit |
| TrustedSec | https://www.trustedsec.com/tools/the-social-engineer-toolkit-set/ |
| Docs | https://github.com/trustedsec/social-engineer-toolkit/wiki |
| User manual | https://www.social-engineer-toolkit.readthedocs.io/en/latest/ |
