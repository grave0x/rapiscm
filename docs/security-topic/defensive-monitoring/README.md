# Defensive Monitoring & SIEM

SIEM, EDR/XDR, NIDS/HIDS, SOAR, incident response platforms, threat intelligence, and log management tools for security operations.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| **SIEM Platforms** | Log aggregation, correlation rules, alerting, dashboards, compliance reporting |
| **HIDS (Host Intrusion Detection)** | File integrity monitoring (FIM), log monitoring, rootkit detection, compliance checks |
| **NIDS (Network Intrusion Detection)** | Deep packet inspection, protocol anomaly detection, signature + behavioral matching |
| **EDR / XDR** | Endpoint telemetry, process/network/file monitoring, behavioral detection, automated response |
| **SOAR** | Playbook automation, orchestration across tools, case management, incident response workflows |
| **Threat Intelligence Platforms** | IOC aggregation/management, STIX/TAXII feeds, enrichment pipelines, threat scoring |
| **Log Management** | Centralized collection, parsing/normalization, retention, archival, forensic search |
| **Incident Response Platforms** | Case management, evidence tracking, timeline reconstruction, analyst collaboration |
| **File Integrity Monitoring** | Baseline cryptographic hashing, change detection, alerting on unauthorized modifications |
| **Vulnerability Management** | Asset discovery, scanning, prioritization (CVSS/EPSS/KEV), patch tracking, SLA enforcement |

## Methods

1. **Collection & Ingestion** — Agent deployment, syslog forwarding, API ingestion, Windows Event Forwarding, cloud log export
2. **Normalization & Enrichment** — Log parsing, CEF/LEEF/JSON normalization, GeoIP tagging, threat intel enrichment
3. **Detection** — Correlation rules, threshold-based alerts, statistical baselines, ML anomaly detection, Sigma rule translation
4. **Investigation** — Timeline reconstruction, pivot-hunting (IOC to asset), query language (SPL, KQL, EQL, Kusto), artifacts collection
5. **Response** — Automated containment playbooks, case creation, evidence preservation, escalation workflows, lessons learned

## Tool Comparison

| Tool | Category | License | Key Strength |
|------|----------|---------|--------------|
| **Splunk Enterprise** | SIEM | Commercial | SPL correlation, dashboards, MLTK anomaly detection |
| **Microsoft Sentinel** | SIEM (Cloud) | Commercial (Azure) | KQL, Fusion ML, UEBA, Logic Apps SOAR, 150+ connectors |
| **Elastic Security** | SIEM + EDR | Elastic License 2.0 | EQL/KQL rules, detection engine, Cases, Osquery manager |
| **Wazuh** | HIDS + SIEM | GPLv2 | FIM, vulnerability detection, regulatory compliance, agent-based |
| **OSSEC** | HIDS | GPLv2 | Log analysis, file integrity, rootkit detection, active response |
| **Zeek** | NIDS | BSD | Protocol parsing, scriptable event engine, rich log output |
| **Suricata** | IDS/IPS/NSM | GPLv2 | Multi-threaded, ruleset engine, EVE JSON output, file extraction |
| **CrowdStrike Falcon** | EDR | Commercial | Cloud-native agent, IOA/IOC detection, threat graph |
| **SentinelOne** | XDR | Commercial | Autonomous AI detection, rollback, Ranger risk assessment |
| **TheHive** | IR platform | AGPLv3 | Case management, observable enrichment, MISP integration |
| **DFIR-IRIS** | IR platform | LGPL | Case/alert management, evidence tracking, timeline, MITRE mapping |
| **Shuffle** | SOAR | AGPLv3 | Drag-and-drop workflow editor, 1000+ app integrations |
| **Velociraptor** | Endpoint monitoring + IR | AGPLv3 | VQL hunts, artifact collection, offline/online mode, multi-OS |
| **Plaso** | Timeline generator | Apache 2.0 | Super-timeline, multi-source log/artifact parser |
| **Timesketch** | Forensic timeline | Apache 2.0 | Plaso import, collaborative investigation, tag/search/aggregate |

## Tool Docs

| File | Tool |
|------|------|
| [Splunk.md](Splunk.md) | Splunk Enterprise |
| [Microsoft-Sentinel.md](Microsoft-Sentinel.md) | Microsoft Sentinel |
| [Elastic-Security.md](Elastic-Security.md) | Elastic Security |
| [Wazuh.md](Wazuh.md) | Wazuh |
| [Zeek.md](Zeek.md) | Zeek |
