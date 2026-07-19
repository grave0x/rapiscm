# Threat Intelligence

Tools and techniques for cyber threat intelligence — collection, analysis, sharing, and operationalization of threat data across the intelligence lifecycle.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| CTI Platforms | MISP, OpenCTI, TheHive for IOC management and case tracking |
| STIX/TAXII | STIX 2.1 data model, TAXII 2.1 transport for structured sharing |
| Detection-as-Code | Sigma rules → Splunk, Elastic, Sentinel, QRadar |
| Threat Research | Adversary profiling, TTP mapping to MITRE ATT&CK |
| OSINT Collection | DNS, certificate transparency, social media, breach data |
| Vulnerability Intel | EPSS, CISA KEV, CVSS scoring and prioritization |
| Dark Web Monitoring | Forums, marketplaces, paste sites, ransomware leak sites |
| Campaign Correlation | Diamond model, shared IoCs/TTPs/infrastructure linking |
| Feed Management | Aggregation, enrichment, deduplication, distribution |
| TLP Governance | Traffic Light Protocol for controlled sharing |

## Methods

- **Platform-driven analysis** — ingest raw threat data into MISP events or OpenCTI knowledge graph; automatic correlation; enrich with ATT&CK galaxies
- **STIX/TAXII workflows** — publish IoC bundles via TAXII collections; consume from ISAC/government feeds; native graph model for relationship mapping
- **Sigma rule authoring** — convert CTI-derived TTPs into vendor-agnostic detection logic; compile with pySigma to multiple SIEM formats
- **OSINT gathering** — passive recon via DNS, CT logs, social media, code repos; SpiderFoot automates 200+ modules
- **Vulnerability prioritization** — correlate CVEs with EPSS probability, CISA KEV, CVSS severity
- **Threat actor profiling** — track TTPs, infrastructure, tooling; map to MITRE ATT&CK Groups
- **Campaign correlation** — link events by shared IoCs, TTPs, infrastructure; diamond model analysis

## Tool Comparison

| Tool | Category | Key Strength |
|------|----------|--------------|
| **MISP** | CTI Platform | IOC sharing, auto-correlation, 12k org network |
| **OpenCTI** | CTI Platform | STIX2 knowledge graph, 300+ connectors |
| **TheHive** | IR Platform | Case management, Cortex enrichment, MISP integration |
| **Sigma** | Detection | Vendor-agnostic rules, 1500+ community, pySigma |
| **SpiderFoot** | OSINT | 200+ modules, automated reconnaissance |
| **Maltego** | Link Analysis | Graph transforms, entity resolution |
| **Shodan** | Search Engine | Internet device/service exposure search |
| **EPSS** | Vulnerability Intel | Daily exploitation probability scores |
| **YARA** | Pattern Matching | Malware/host/IOA signatures |
| **PyMISP** | Library | Python REST API for MISP automation |
