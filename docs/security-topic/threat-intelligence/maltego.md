# Maltego — Link Analysis & Graph Intelligence Platform

Graph-based OSINT and link analysis tool. Visual entity-relationship mapping, transforms connecting data points across public and private data sources.

## How It Works

Maltego displays intelligence as a directed graph where **entities** (people, domains, IPs, emails, URLs, social accounts) are connected by **links** derived from **transforms** — plugins that query data sources. Transforms expand the graph by discovering relationships (e.g., domain → IP → open ports → banners).

**Entity types:**

| Category | Entity Examples |
|----------|----------------|
| **Infrastructure** | IPv4, IPv6, Domain, DNS Name, URL, AS, Netblock |
| **People** | Person, Email, Phone, Social Profile |
| **Documents** | Document, File, Hash, Metadata |
| **Organization** | Company, Organization, Affiliation |
| **Location** | Location, GPS Coordinates, City, Country |

## Manual

```bash
# Launch GUI
maltego &

# Command-line transforms (using Maltego XL/Classic transforms from CLI):
# Maltego is primarily GUI-driven, but supports:
# - TDS (Transform Distribution Server) sharing
# - Python scripting for custom transforms
# - Casefile (offline mode without transforms)

# Custom transform example (Python)
"""
# transforms/my_transform.py
from transforms.api import Transform, InputField

class DomainToIP(Transform):
    input_type = "Domain"
    output_type = "IPv4"
    
    def transform(self, entity):
        domain = entity.value
        # Resolve and yield IPs
        yield self.create_entity("IPv4", "1.2.3.4")
"""
```

### Transforms (Common)

| Transform | Source | Input → Output |
|-----------|--------|----------------|
| To IP | DNS | Domain → IPv4 |
| To Website | Web | Company → Domain |
| To Email | Search engines | Person → Email |
| To Social Media | Social search | Person → Profile |
| To DNS Name | Passive DNS | IP → Domain |
| To Files | Shodan | IP → Open Ports |
| To Threat Intel | VirusTotal | Hash → Reports |

## Install

```bash
# Linux (deb)
wget https://downloads.maltego.com/maltego-v4/20241001/Maltego.v20241001.deb
sudo dpkg -i Maltego.v20241001.deb

# macOS
# Download from https://www.maltego.com/downloads/

# Windows
# Download installer from https://www.maltego.com/downloads/

# Community Edition (free, limited transforms)
# Register at https://www.maltego.com/ce-login/
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.maltego.com/ |
| Downloads | https://www.maltego.com/downloads/ |
| Transforms | https://www.maltego.com/transform-hub/ |
| Docs | https://docs.maltego.com/ |
| Community Edition | https://www.maltego.com/ce-registration/ |
| Python API | https://github.com/paterva/maltego-trx |
