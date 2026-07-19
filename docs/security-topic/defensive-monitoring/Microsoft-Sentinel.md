# Microsoft Sentinel — Cloud-Native SIEM

Cloud-native SIEM and SOAR built on Azure. KQL queries, Fusion ML, UEBA, Logic Apps SOAR, 150+ data connectors.

## How It Works

Sentinel ingests data from Azure services, Microsoft 365, and third-party sources into a Log Analytics workspace. Detection rules (written in Kusto Query Language) trigger incidents. SOAR playbooks (Azure Logic Apps) automate response.

**Key components:**
- **Data Connectors** — 150+ pre-built connectors (Azure AD, AWS, GCP, syslog, CEF, custom)
- **Analytics Rules** — Scheduled, NRT, ML Behavior Analytics, Fusion, Anomaly, Security Insights
- **UEBA** — Entity behavior profiling, peer comparison, timeline anomalies
- **Incident Management** — Case management, alert grouping, MITRE ATT&CK mapping
- **Playbooks** — Logic Apps-based SOAR, 600+ connectors
- **Workbooks** — Interactive dashboards (Azure Monitor Workbooks)
- **Hunting** — Notebooks (Jupyter), KQL queries, bookmarks, livestream

## Manual

```bash
# Via Azure CLI
# Create workspace
az monitor log-analytics workspace create \
  -g my-rg -n sentinel-workspace

# Enable Sentinel
az security sentinel create \
  --resource-group my-rg \
  --workspace-name sentinel-workspace

# Query via KQL
SecurityAlert
| where TimeGenerated > ago(7d)
| summarize AlertCount=count() by AlertName, Severity
| order by AlertCount desc
```

## Install

```bash
# Azure CLI (module)
az extension add --name sentinel

# Enable via Azure Portal or Bicep/ARM/Terraform
```

## Build

Closed-source. Extensions via Logic Apps custom connectors, Azure Functions, and KQL functions.

## Package

Pricing: Pay-as-you-go (Log Analytics ingestion + Sentinel charges). No upfront cost.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.microsoft.com/en-us/security/business/siem-and-xdr/microsoft-sentinel |
| Docs | https://docs.microsoft.com/en-us/azure/sentinel/ |
| KQL reference | https://learn.microsoft.com/en-us/azure/data-explorer/kusto/query/ |
| Playbook gallery | https://github.com/Azure/Azure-Sentinel |
| GitHub (samples) | https://github.com/Azure/Azure-Sentinel |
| Pricing | https://azure.microsoft.com/en-us/pricing/details/microsoft-sentinel/ |
