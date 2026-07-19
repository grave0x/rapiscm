# TheHive — Incident Response Platform

Security incident response platform with case management, alert ingestion, MISP integration, and Cortex observable enrichment. Maintained by TheHive Project / StrangeBee.

## How It Works

TheHive organizes incidents into **Cases** containing **Tasks**, **Observables**, and **Logs**. Alerts from SIEM, MISP, email, or custom sources create Cases automatically. Observables are enriched via Cortex (analyzer engine) — VirusTotal, AbuseIPDB, Shodan, etc. Playbooks automate case progression.

**Key concepts:**
- **Case** — incident container (title, description, severity, tags)
- **Task** — step within a case (assignable, trackable)
- **Observable** — IoC attached to a case (IP, hash, URL, file)
- **Alert** — incoming notification from external sources
- **Cortex** — observable analysis engine (responders + analyzers)
- **MISP Integration** — sync events, push/pull observables

## Manual

```bash
# Web UI: http://localhost:9000
# Default: admin@thehive.local / secret

# REST API
# Create case
curl -X POST http://localhost:9000/api/v1/case \
  -H "Authorization: Bearer <API_KEY>" \
  -H "Content-Type: application/json" \
  -d '{"title": "Phishing report", "severity": 2, "tags": ["phishing"]}'

# Add observable
curl -X POST http://localhost:9000/api/v1/case/<ID>/observable \
  -H "Authorization: Bearer <API_KEY>" \
  -H "Content-Type: application/json" \
  -d '{"dataType": "ip", "data": "1.2.3.4"}'
```

### TheHive4py

```python
from thehive4py import TheHiveApi

hive = TheHiveApi('http://localhost:9000', 'API_KEY')

# Create case
case = hive.case.create(
    title='Phishing report',
    severity=2,
    tags=['phishing', 'suspicious']
)

# Add observable
hive.observable.create(
    case_id=case['id'],
    data_type='ip',
    data='1.2.3.4'
)

# Create alert from sentinel
hive.alert.create(
    title='SIEM Alert',
    description='Suspicious login',
    severity=2,
    type='sentinel',
    source='SOC',
    source_ref='SOC001'
)
```

## Build

```bash
git clone https://github.com/TheHive-Project/TheHive.git
cd TheHive
# Java/SBT build
sbt clean stage
# Artifact in target/universal/stage/
```

## Install

```bash
# Docker (recommended)
git clone https://github.com/TheHive-Project/TheHive-docker.git
cd TheHive-docker
docker-compose up -d

# DEB/RPM package
# Download from https://github.com/TheHive-Project/TheHive/releases
sudo dpkg -i thehive*.deb  # Debian/Ubuntu
sudo rpm -i thehive*.rpm   # RHEL/CentOS

# Requirements: Java 11+, Elasticsearch 7.x, Cortex (optional)
```

### Docker Quick Start

```yaml
# docker-compose.yml
services:
  thehive:
    image: thehiveproject/thehive:latest
    ports:
      - "9000:9000"
    environment:
      - ELASTICSEARCH_URI=http://elasticsearch:9200
    volumes:
      - thehive-data:/opt/thp/data
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/TheHive-Project/TheHive |
| Docs | https://docs.thehive-project.org/ |
| Cortex | https://github.com/TheHive-Project/Cortex |
| TheHive4py | https://github.com/TheHive-Project/TheHive4py |
| Docker | https://github.com/TheHive-Project/TheHive-docker |
