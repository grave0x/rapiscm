# Wazuh — HIDS + SIEM Platform

Open-source host-based intrusion detection + SIEM — FIM, vulnerability detection, regulatory compliance (PCI-DSS, HIPAA, CIS), agent-based monitoring.

## How It Works

Wazuh extends OSSEC with modern architecture:

- **Wazuh Agent** — Deployed on monitored endpoints (Linux, Windows, macOS)
- **Wazuh Manager (Server)** — Central analysis, alerting, log collection
- **Wazuh Indexer** — Elasticsearch-based storage (forked from OpenSearch)
- **Wazuh Dashboard** — Kibana-based UI for alerts, compliance, management

**Detection capabilities:**
- File Integrity Monitoring (FIM) — real-time/scheduled checksum verification
- Log Data Analysis — syslog, Windows Event, JSON logs with decoders/rules
- Vulnerability Detection — feeds from NVD, CVE, RedHat, Canonical, Arch Linux
- Malware Detection — YARA integration, hash matching
- Regulatory Compliance — PCI DSS, HIPAA, GPG13, TSC, NIST 800-53, CIS benchmarks
- Active Response — automated threat containment (firewall blocks, agent commands)
- Security Configuration Assessment (SCA) — CIS benchmark scanning

## Manual

```bash
# Add agent
sudo systemctl enable wazuh-agent
sudo systemctl start wazuh-agent

# Manager control
sudo /var/ossec/bin/ossec-control enable agentless
sudo /var/ossec/bin/ossec-control restart

# Query alerts (API)
curl -u wazuh:wazuh -k -X GET "https://localhost:55000/alerts?limit=10" | jq

# Agent upgrade
sudo /var/ossec/bin/agent_upgrade -l
```

## Install

```bash
# Quick install (all-in-one)
curl -sO https://packages.wazuh.com/4.9/wazuh-install.sh
sudo bash wazuh-install.sh -a

# Agent (Linux)
wget https://packages.wazuh.com/4.x/apt/pool/main/w/wazuh-agent/wazuh-agent_4.9.0-1_amd64.deb
sudo WAZUH_MANAGER="10.0.0.2" dpkg -i wazuh-agent_*.deb

# Agent (Docker)
docker run -d --name wazuh-agent \
  -e WAZUH_MANAGER=10.0.0.2 \
  wazuh/wazuh-agent
```

## Build

```bash
git clone https://github.com/wazuh/wazuh.git
cd wazuh
# Build requires autoconf, automake, libtool, cmake
make -j$(nproc)
```

## Package

| Manager | Command |
|---------|---------|
| DEB/RPM | Wazuh repository (add key + apt/yum) |
| Tarball | Build from source |
| Docker | `wazuh/wazuh-manager`, `wazuh/wazuh-indexer`, `wazuh/wazuh-dashboard` |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://wazuh.com/ |
| GitHub | https://github.com/wazuh/wazuh |
| Docs | https://documentation.wazuh.com/ |
| Rules gallery | https://github.com/wazuh/wazuh-ruleset |
| Compliance | https://documentation.wazuh.com/current/compliance/index.html |
