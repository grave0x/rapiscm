# MISP — Malware Information Sharing Platform

Threat intelligence sharing platform. Event-based IOC management, automatic correlation, galaxy clusters, taxonomies. 12,000+ org sharing network.

## How It Works

MISP organizes threat data into **Events** containing **Attributes** (IoCs). Automatic correlation links attributes across events. Galaxies provide MITRE ATT&CK mapping, taxonomies for classification, and warning lists for false positive prevention. Feeds enable push/pull sharing between instances.

**Key concepts:**
- **Event** — container for a set of attributes (a report, campaign, incident)
- **Attribute** — single indicator (IP, hash, domain, YARA rule, etc.)
- **Object** — complex attribute with multiple fields (file with name/hash/size)
- **Galaxy** — MITRE ATT&CK, threat actors, malware, ransomware clusters
- **Taxonomy** — classification tags (TLP, cyber kill chain, etc.)
- **Warning List** — known good indicators (RFC1918, known CDNs) to suppress false positives
- **Feed** — external MISP instance connection for automated sharing

## Manual

```bash
# Web UI: http://localhost:8080
# Default credentials: admin@admin.test / admin

# REST API examples
# Create event
curl -X POST http://localhost:8080/events \
  -H "Authorization: <API_KEY>" \
  -H "Content-Type: application/json" \
  -d '{"Event": {"info": "Suspicious activity", "distribution": 1}}'

# Add attribute
curl -X POST http://localhost:8080/attributes \
  -H "Authorization: <API_KEY>" \
  -H "Content-Type: application/json" \
  -d '{"Attribute": {"event_id": "1", "type": "ip-src", "value": "1.2.3.4"}}'
```

### PyMISP

```python
from pymisp import PyMISP, MISPEvent

misp = PyMISP('https://localhost:8080', 'API_KEY', False)
event = MISPEvent()
event.info = 'Automated event'
event.add_attribute('ip-src', '1.2.3.4')
event.add_attribute('md5', 'd41d8cd98f00b204e9800998ecf8427e')
misp.add_event(event)
```

### MISP to SIEM

```bash
# Export as IDS/NSM rules
curl -H "Authorization: <API_KEY>" \
  "http://localhost:8080/events/nids/suricata/download"
```

## Build

```bash
git clone https://github.com/MISP/MISP.git
cd MISP
git submodule update --init --recursive
# Server setup requires PHP, MySQL, Redis, Python 3
# See install docs for detailed steps
```

## Install

```bash
# Docker (recommended for evaluation)
docker pull ghcr.io/misp/misp-docker:latest

# Debian/Ubuntu
# Automated install scripts at https://github.com/MISP/MISP/tree/2.5/INSTALL

# Manual install requires:
# - PHP 8.x + Apache/Nginx
# - MariaDB 10+
# - Redis
# - Python 3 + pip packages
# - composer dependencies
```

### Docker Quick Start

```bash
# Clone misp-docker
git clone https://github.com/MISP/misp-docker.git
cd misp-docker
cp template.env .env
# Edit .env with passwords
docker-compose up -d
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.misp-project.org/ |
| GitHub | https://github.com/MISP/MISP |
| PyMISP | https://github.com/MISP/PyMISP |
| Galaxies | https://github.com/MISP/misp-galaxy |
| Taxonomies | https://github.com/MISP/misp-taxonomies |
| Warning Lists | https://github.com/MISP/misp-warninglists |
| Docker | https://github.com/MISP/misp-docker |
