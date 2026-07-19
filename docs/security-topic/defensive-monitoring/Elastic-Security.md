# Elastic Security — SIEM + EDR

Open SIEM + EDR platform built on the Elastic Stack. EQL/KQL rules, detection engine, timelines, Cases, Osquery manager. Free tier available.

## How It Works

Elastic Security extends the Elastic Stack (ELK) with security detection, endpoint protection, and SIEM capabilities.

**Key components:**
- **Elasticsearch** — Scalable search + analytics engine, data store
- **Kibana Security** — Detection rules UI, timelines, cases, dashboards
- **Elastic Agent** — Unified agent for log collection, metrics, and EDR
- **Fleet Server** — Centralized agent management
- **Endpoint** — EDR sensor (process, network, file events), malware prevention
- **Osquery Manager** — Fleet-wide osquery via Kibana

**Detection:** EQL (Event Query Language), KQL (Kibana Query Language), machine learning jobs, indicator match rules, threshold rules, new-term rules.

## Manual

```kql
# KQL — find process executions by unknown binaries
event.category : "process" and event.type : "start"
  and not process.executable : ("/usr/bin/*", "/bin/*", "/sbin/*")

# EQL — detect sequence of events
sequence by user.name
  [ authentication where event.action == "failed_logon" ]
  [ authentication where event.action == "successful_logon" ]
```

## Install

```bash
# Elastic Cloud (managed)
# https://cloud.elastic.co

# Docker Compose (self-managed)
git clone https://github.com/elastic/elastic-stack-installers
docker compose -f docker-compose.yml up -d

# Tarball
wget https://artifacts.elastic.co/downloads/elasticsearch/elasticsearch-8.15.0-linux-x86_64.tar.gz
tar xzf elasticsearch-*.tar.gz
cd elasticsearch-*/ && ./bin/elasticsearch

# Kibana
wget https://artifacts.elastic.co/downloads/kibana/kibana-8.15.0-linux-x86_64.tar.gz
# Fleet + Elastic Agent
sudo ./elastic-agent install --url=https://fleet-server:8220 --enrollment-token=token
```

## Build

Elastic is open-source (Elastic License 2.0). Build from source:

```bash
git clone https://github.com/elastic/elasticsearch.git
cd elasticsearch
./gradlew assemble
```

## Package

| Manager | Command |
|---------|---------|
| DEB/RPM | `apt install elasticsearch` (from Elastic repo) |
| Tarball | Extract from artifacts.elastic.co |
| Docker | `docker.elastic.co/elasticsearch/elasticsearch:8.15.0` |
| Cloud | Elastic Cloud, AWS/GCP/Azure marketplace |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.elastic.co/security |
| Docs | https://www.elastic.co/guide/en/security/current/index.html |
| GitHub | https://github.com/elastic/security-docs |
| Detection rules | https://github.com/elastic/detection-rules |
| Elastic Stack | https://github.com/elastic/elasticsearch |
