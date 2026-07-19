# Shuffle — Open-Source SOAR Platform

Open-source security orchestration, automation, and response (SOAR) platform. Visual workflow designer, app framework, case management. Alternative to Splunk SOAR and Palo Alto XSOAR.

## How It Works

Shuffle connects security tools via a visual drag-and-drop workflow editor. Nodes represent actions, triggers, and logic. Workflows execute on events (webhook, schedule, email, SIEM alert).

**Architecture:**
- **Workflow Engine** — Executes node graphs. Each node is an action (API call, transform, decision, loop)
- **Apps** — Pre-built integrations (250+). HTTP wrappers + authentication. Community-maintained
- **Triggers** — Webhook, schedule (cron), email (IMAP), SIEM (Splunk, Elastic, Sentinel), MISP, TheHive
- **Variables** — Context passing between workflow nodes. JSON, array, text. Encryption support
- **Case Management** — In-platform ticketing. Assign, track, close incidents
- **Execution Logs** — Full trace per workflow run. Node timings, API responses, errors

**Example workflows:**
- Phishing triage: Parse email → Extract IOCs → Enrich (VT, Shodan) → Create case (TheHive/IRIS) → Slack notification
- Alert enrichment: SIEM alert → IP/domain/hash enrichment → Slack/Teams notification
- Incident containment: Block IP on firewall → Quarantine endpoint in EDR → Email SOC lead

## Manual

```bash
# Self-hosted Docker deployment
git clone https://github.com/Shuffle/shuffle.git
cd shuffle
docker compose up -d

# Access UI: http://localhost:3001

# API
curl -X POST https://<shuffle>/api/v1/auth/register \
  -d '{"username":"admin","password":"<pass>","org":"SOC"}'

# Trigger a workflow by webhook
curl -X POST https://<shuffle>/api/v1/hooks/webhook/<workflow-id> \
  -d '{"event":"phishing","from":"user@example.com"}'

# Search executions
curl -X GET "https://<shuffle>/api/v1/workflows/<id>/executions" \
  -H "Authorization: Bearer <token>"
```

## Build

```bash
git clone https://github.com/Shuffle/shuffle.git
cd shuffle
# Frontend
cd frontend && npm install && npm run build
# Backend
cd backend && go build -o shuffle
# Docker (all-in-one)
docker compose build
```

## Install

```bash
# Docker Compose (recommended)
docker compose up -d

# Cloud (SaaS)
# Sign up at https://shuffler.io/

# Kubernetes
kubectl apply -f https://raw.githubusercontent.com/Shuffle/shuffle/main/kubernetes/shuffle.yaml
```

## Package

Self-hosted (open-source, Docker Compose). SaaS cloud version at shuffler.io. Free tier (limited workflows/executions). Enterprise license for advanced features.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://shuffler.io/ |
| GitHub | https://github.com/Shuffle/shuffle |
| Docs | https://shuffler.io/docs |
| App store | https://shuffler.io/apps |
| Community | https://github.com/Shuffle/shuffle/discussions |
| Slack | https://join.slack.com/t/shuffleresearch/shared_invite/ |
