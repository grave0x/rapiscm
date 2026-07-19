# SentinelOne — Autonomous AI-Powered EDR/XDR

AI-driven endpoint protection platform (EDR/XDR). Single agent for Windows, macOS, Linux, and Kubernetes. Autonomous detection, prevention, and response.

## How It Works

SentinelOne Singularity uses a single agent with on-device AI models for real-time prevention. Cloud console for centralized management, investigation, and response.

**Key capabilities:**
- **Static AI** — Pre-execution ML. Analyzes file at rest before execution. Detection without hash lookup
- **Behavioral AI** — Runtime detection. Monitors process behavior, file operations, registry, network connections
- **Storyline** — Correlates related events into a single "Storyline" across processes, threads, network
- **Rangers** — Network discovery tool (agent-based LAN scanning for connected devices)
- **RemoteScript** — PowerShell, bash, Python, or cmd execution across fleet
- **Vulnerability Management** — Built-in agent-based CVE scanning (no separate scanner needed)
- **Purge** — Rollback malicious changes. Reverses registry, file, and service changes from ransomware/ malware

**Detection methodology:**
- Pre-execution: Static AI scores file at first sight
- Execution: Behavioral AI analyzes process chain, anomalies
- Post-execution: Storyline maps full kill chain from initial access to objective
- Cloud detection: Cross-endpoint correlation via Purple Fox AI models

## Manual

```bash
# API-based management
# Base URL: https://<site>.sentinelone.net

# Auth
curl -X POST https://<site>.sentinelone.net/web/api/v2.1/users/login \
  -d '{"username":"<user>","password":"<pass>"}'

# List threats
curl -H "Authorization: ApiToken <token>" \
  https://<site>.sentinelone.net/web/api/v2.1/threats

# Initiate remote script
curl -X POST -H "Authorization: ApiToken <token>" \
  https://<site>.sentinelone.net/web/api/v2.1/remote-scripts/execute \
  -d '{"script":"ls -la","scriptType":"bash","agentIds":["<id>"]}'

# Initiate purge (rollback)
curl -X POST -H "Authorization: ApiToken <token>" \
  https://<site>.sentinelone.net/web/api/v2.1/activities/initiate-purge
```

## Install

```bash
# Linux (DEB)
sudo dpkg -i sentinelone_agent*.deb
sudo settings sentinelone set --mgmt <console-url>
sudo systemctl start sentinelone

# Linux (RPM)
sudo rpm -i sentinelone_agent*.rpm
sudo /opt/sentinelone/bin/sentinelone set --mgmt <console-url>

# macOS
sudo installer -pkg SentinelOneAgent.pkg -target /

# Windows
msiexec /i SentinelOneAgent_<version>.msi /quiet

# Kubernetes (DaemonSet)
kubectl apply -f sentinelone-agent.yaml
```

## Build

Closed-source. REST API v2.1 for automation. Python SDK (unofficial: `sentinelone-sdk`).

## Package

Subscription-based. Tiered: Singularity Core (NGAV), Singularity Control (EDR), Singularity Complete (XDR + Purple + Ranger + Vulnerability).

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.sentinelone.com/ |
| Platform overview | https://www.sentinelone.com/platform/ |
| API docs | https://<site>.sentinelone.net/api-doc/ |
| Blog | https://www.sentinelone.com/blog/ |
| Threat research | https://www.sentinelone.com/labs/ |
| Support | https://support.sentinelone.com/ |
