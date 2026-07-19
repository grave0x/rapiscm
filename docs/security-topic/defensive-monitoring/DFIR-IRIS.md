# DFIR-IRIS — Incident Response Collaborative Platform

Open-source incident response platform. Designed for CSIRTs/SOCs. Case management, artifact ingestion, IOC correlation, timeline building, and report generation.

## How It Works

DFIR-IRIS is a web-based IR platform (Flask + Celery + PostgreSQL). Cases are the central object — each incident creates a case with associated evidence, artifacts, tasks, alerts, and timeline entries.

**Core data model:**
- **Cases** — Title, severity, status, owner, customer, SOC ID
- **IOCs** — Observable objects (IP, domain, hash, URL, email). Type, TLP, analysis status
- **Artifacts** — Files uploaded to case (registry hives, memory dumps, log files). Parsed by IRIS modules (EVTX, Plaso, Hayabusa, Chainsaw)
- **Timeline** — Unified chronological view across all evidence. Graph/table display
- **Tasks** — Per-case task management with assignee, status, description
- **Alerts** — SIEM/webhook ingestion via MISP, Splunk, Sentinel, etc.

**Pipeline workflow:**
1. Alert arrives (manual, API, MISP, TheHive)
2. Create case with auto-assigned SOC ID
3. Add IOCs from alert or manual entry
4. Upload artifacts → automated parsing (EVTX → timeline entries)
5. Investigate via timeline, graph, and gallery views
6. Assign tasks, link IOCs to detections
7. Export case report (PDF template)

## Manual

```bash
# Docker Compose deployment
git clone https://github.com/dfir-iris/iris-web.git
cd iris-web
cp .env.model .env
# Edit .env with DB passwords, secret key
docker compose up -d

# Access web UI at https://localhost:443
# Default: admin@admin.local / admin

# REST API
curl -X POST https://localhost/api/auth/login \
  -d '{"login":"admin@admin.local","password":"admin"}'

# List cases
curl -H "Authorization: Bearer <token>" \
  https://localhost/api/cases

# Create IOC
curl -X POST -H "Authorization: Bearer <token>" \
  https://localhost/api/iocs \
  -d '{"case_id":1, "ioc_type":"hash", "ioc_value":"<sha256>", "tlp":2}'

# Search timeline
curl -H "Authorization: Bearer <token>" \
  https://localhost/api/timeline?case=1&search=svchost
```

## Build

```bash
git clone https://github.com/dfir-iris/iris-web.git
cd iris-web
docker compose build

# Or manual (Python 3.10+)
python3 -m venv venv
source venv/bin/activate
pip install -r requirements/dev.txt
flask db upgrade
```

## Install

```bash
# Docker Compose (recommended)
docker compose up -d

# Production (Docker Swarm/Kubernetes)
# Kubernetes manifests available at
# https://github.com/dfir-iris/iris-web/tree/develop/deploy/k8s
```

## Package

Docker Compose deployment. No package manager install. Source available on GitHub. Image on Docker Hub (`dfiriris/irisweb`).

## Links

| Resource | URL |
|----------|-----|
| Official site | https://dfir-iris.org/ |
| GitHub | https://github.com/dfir-iris/iris-web |
| Docs | https://docs.dfir-iris.org/ |
| API reference | https://docs.dfir-iris.org/development/api/ |
| Community | https://github.com/dfir-iris/iris-web/discussions |
| Docker Hub | https://hub.docker.com/u/dfiriris |
