# Maltego — Link Analysis & OSINT Platform

Graph-based intelligence platform for connecting entities (domains, IPs, emails, people) across public data sources. Industry standard for OSINT link analysis.

## How It Works

Maltego operates through **Transforms** — modular data-fetching components that query public APIs (DNS, WHOIS, Shodan, social media, breach data) and return related entities. Each transform feeds new entities into the graph, enabling iterative discovery. Paterva's **Transform Hub** provides 100+ transforms from first-party and third-party sources.

**Core concepts:**

| Concept | Description |
|---------|-------------|
| **Entity** | Data node — domain, IP, email, person, phone, URL |
| **Transform** | Query that returns related entities from an input entity |
| **Machine** | Automated multi-step transform sequence (playbook) |
| **Hub** | Marketplace for commercial transforms (Paterva, Social Links, etc.) |
| **Graph** | Interactive visualization of entity relationships |

## Manual

### GUI Workflow

1. Choose an entity palette (Domain, IP, Email, Person)
2. Drag entity onto graph
3. Right-click → Run Transform (e.g., Domain → DNS → MX record)
4. Iterate: select new entities → more transforms
5. Export graph as image, CSV, or report

### CLI (CaseFile / TDS)

```bash
# Maltego TDS (Transform Distribution Server) CLI
# Windows only native; Linux via Wine or Docker
maltego.exe --cli script.maltego
```

### Key Transform Chains

```
Domain → DNS A Record → IP Address
Domain → WHOIS → Email Contact → Social Media Profile
IP Address → Shodan API → Open Ports / Services
Email → Have I Been Pwned → Breach Data
Company → LinkedIn → Employee Names
```

### Export Formats

| Format | Usage |
|--------|-------|
| GraphML | Import into Gephi for advanced visualization |
| CSV | Spreadsheet for reports |
| Maltego export (.mtgl) | Share with other analysts |
| Image (PNG/SVG) | Presentation-friendly |

## Build

Maltego is proprietary — no source build. Transforms are extensible via the **Maltego Python API** (`canari` framework):

```bash
pip install canari
canari create-transform my-transform
# Develop custom transforms in Python
```

## Install

```bash
# Linux (via Wine)
# Download Maltego CE from https://www.maltego.com/downloads/
wine maltego_setup.exe

# macOS
brew install --cask maltego

# Windows
# Download and run installer from maltego.com

# Docker (headless TDS)
docker pull paterva/tds:latest
```

### Editions

| Edition | Cost | Transforms |
|---------|------|------------|
| Maltego CE (Community) | Free | Limited set (replaced by XL) |
| Maltego XL | €999/yr | All transforms, unlimited graph size |
| Maltego Pro | €2,499/yr | XL + TDS + collaboration |
| Maltego API (TDS) | Custom | Headless transform execution |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.maltego.com/ |
| Transform Hub | https://www.maltego.com/transform-hub/ |
| Docs | https://docs.maltego.com/ |
| Canari (Python API) | https://github.com/allfro/canari |
| CE download | https://www.maltego.com/downloads/ |
