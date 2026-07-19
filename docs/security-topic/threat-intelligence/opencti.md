# OpenCTI — Open Cyber Threat Intelligence Platform

STIX2-native knowledge graph platform. Models threat actors, malware, campaigns, TTPs, infrastructure. 300+ connectors. Maintained by Filigran.

## How It Works

OpenCTI builds a knowledge graph using the STIX 2.1 data model. Entities (Malware, ThreatActor, Campaign, Indicator) and relationships (uses, targets, mitigates) form a navigable graph. Connectors ingest data from external sources (MISP, VirusTotal, Shodan, MITRE ATT&CK) and enrich existing entities.

**Key concepts:**
- **Knowledge** — entities + relationships (native STIX2 graph)
- **Workspace** — dashboards, investigations, reports
- **Connectors** — ingest, enrichment, export, streaming
- **Ingestion** — CSV, STIX bundles, TAXII, MISP feeds
- **Rules** — automatic relationship generation (e.g., "uses" from campaign to malware)
- **Audit trail** — full history of entity changes

## Manual

```bash
# Web UI: http://localhost:8080
# Default: admin@opencti.io / admin

# REST API
curl http://localhost:8080/graphql \
  -H "Authorization: Bearer <TOKEN>" \
  -H "Content-Type: application/json" \
  -d '{"query": "{ stixCoreObjects { edges { node { id entity_type } } } }"}'
```

### pycti

```python
from pycti import OpenCTIApiClient

client = OpenCTIApiClient('https://opencti:8080', 'API_TOKEN')

# Create indicator
indicator = client.indicator.create(
    name='Suspicious IP',
    pattern_type='stix',
    pattern="[ipv4-addr:value = '1.2.3.4']",
    valid_from='2024-01-01T00:00:00Z'
)

# Create relationship
client.stix_core_relationship.create(
    from_id=indicator['id'],
    to_id=malware_id,
    relationship_type='indicates'
)

# Search for entities
threat_actors = client.threat_actor.list(search='Lazarus')
```

## Build

```bash
git clone https://github.com/OpenCTI-Platform/opencti.git
cd opencti
# Requires Node.js, Yarn, Python 3, Elasticsearch, RabbitMQ, Redis
# See deployment documentation for detailed build steps
```

## Install

```bash
# Docker (recommended)
git clone https://github.com/OpenCTI-Platform/docker.git
cd docker
docker-compose up -d
# Access http://localhost:8080

# Connectors
docker-compose -f docker-compose.yml -f connector-misp.yml up
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://filigran.io/ |
| GitHub | https://github.com/OpenCTI-Platform/opencti |
| pycti | https://github.com/OpenCTI-Platform/client-python |
| Connectors | https://github.com/OpenCTI-Platform/connectors |
| Docs | https://docs.opencti.io/ |
| Demo | https://demo.opencti.io/ |
