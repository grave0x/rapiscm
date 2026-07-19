# CrowdStrike Falcon — Cloud-Native EDR/XDR Platform

Leading cloud-native endpoint protection (EDR/XDR). Single lightweight agent. AI-powered threat detection, real-time response, and threat intelligence.

## How It Works

CrowdStrike Falcon uses a cloud-native architecture — single agent ("sensor") on endpoints, cloud-based management console.

**Sensor types:**
- **Windows** — Kernel-mode minifilter driver + userland services. Logs process creation, network connections, file operations, registry changes, DLL loads
- **macOS** — System Extension (EndpointSecurity framework)
- **Linux** — eBPF-based sensor. Coverage for containers, Kubernetes nodes

**Prevention vs Detection:**
- **Prevention** — ML-based on-sensor blocking. Zero-day prevention via static/dynamic ML models
- **Detection** — Behavioral IOA (Indicators of Attack). Correlates low-level events into kill-chain steps
- **Response** — Real-time response (RTR) via cloud CLI. Live shell, process kill, quarantine, registry manipulation

**Cloud console capabilities:**
- **Dashboard** — Detections, incidents, prevention activity, real-time graph
- **Investigate** — Search across process, network, file, and registry telemetry (up to 1 year retention)
- **IOA** — Behavioral rules (process tree analysis, suspicious parent/child, LOLBin abuse)
- **IOC Management** — Custom IOC blacklist/whitelist, detonate on hash/domain/IP
- **Falcon OverWatch** — 24/7 human threat hunting team
- **Falcon X** — Threat intelligence + sandbox
- **FalonX Recon** — External attack surface monitoring

## Manual

```bash
# No local CLI — API-driven
# Base URL: https://api.crowdstrike.com

# Auth (OAuth2)
curl -X POST https://api.crowdstrike.com/oauth2/token \
  -d "client_id=<CID>&client_secret=<SECRET>"

# List detections
curl -H "Authorization: Bearer <token>" \
  https://api.crowdstrike.com/detects/queries/detects/v1

# Real-time response (RTR) — run on endpoint
curl -X POST -H "Authorization: Bearer <token>" \
  https://api.crowdstrike.com/real-time-response/entities/command/v1 \
  -d '{"device_id":"<id>","command_string":"ps -aux"}'

# Sensor install (Linux)
sudo dpkg -i falcon-sensor_<version>_amd64.deb
sudo /opt/CrowdStrike/falconctl -s --cid=<CID>
```

## Install

```bash
# Linux (DEB)
sudo dpkg -i falcon-sensor*.deb
sudo /opt/CrowdStrike/falconctl -s --cid=<CID>

# Linux (RPM)
sudo rpm -i falcon-sensor*.rpm
sudo /opt/CrowdStrike/falconctl -s --cid=<CID>

# macOS
sudo installer -pkg FalconSensor.pkg -target /
sudo /Applications/Falcon.app/Contents/Resources/falconctl license <CID>

# Windows
# MSI — deploy via GPO, SCCM, Intune
msiexec /i Falcon_Sensor_Windows_*.msi /qn CID=<CID>

# Docker
docker pull crowdstrike/falcon-sensor:latest
```

## Build

Closed-source. API and SDK (`crowdstrike-falconpy` Python) for automation.

```bash
# FalconPy SDK
pip install crowdstrike-falconpy
```

## Package

Subscription-based. Tiered by module (Falcon Pro, Falcon Enterprise, Falcon Elite). Add-ons: Falcon OverWatch, Falcon X, Falcon Discover, Falcon DevSecOps.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.crowdstrike.com/ |
| Developer docs | https://www.crowdstrike.com/developer/ |
| API reference | https://falcon.crowdstrike.com/documentation |
| FalconPy SDK | https://github.com/CrowdStrike/falconpy |
| Blog | https://www.crowdstrike.com/blog/ |
| CVE advisories | https://www.crowdstrike.com/blog/category/threat-intelligence/ |
