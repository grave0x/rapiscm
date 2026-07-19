# Sigma — Generic Detection Rule Format

Vendor-agnostic detection rule format. YAML-based with 1,500+ community rules. Compiles to Splunk, Elastic, Sentinel, QRadar, and more via pySigma backends.

## How It Works

Sigma rules describe detection logic in a SIEM-independent YAML format. A rule contains a `detection` block with field-matching expressions (strings, modifiers, aggregation, temporal). `pySigma` backends compile Sigma rules into target SIEM queries (SPL, EQL, KQL, Kusto, etc.).

**Rule structure:**
- **title, id, status** — metadata
- **logsource** — category (process_creation, network_connection, file_event) and product (windows, linux)
- **detection** — selection conditions with modifiers (`contains`, `endswith`, `base64`)
- **condition** — boolean logic combining selections (`selection and not filter`)
- **falsepositives, level, tags** — context + MITRE ATT&CK mapping

## Manual

```bash
# Convert single rule
sigma convert -t splunk -p windows rule.yml

# Convert directory of rules
sigma convert -t splunk -p windows rules/ > queries.spl

# Validate rule syntax
sigma check rule.yml

# List available targets
sigma plugin list
```

### Rule Example

```yaml
title: PowerShell Download from URL
id: 098e23e1-1f53-4a13-9a2e-3e4a5b6c7d8e
status: experimental
logsource:
  category: process_creation
  product: windows
detection:
  selection:
    Image|endswith: '\powershell.exe'
    CommandLine|contains:
      - 'Invoke-WebRequest'
      - 'System.Net.WebClient'
      - 'Start-BitsTransfer'
  filter:
    CommandLine|contains: 'MicrosoftUpdate'
  condition: selection and not filter
falsepositives:
  - Legitimate software updates
level: high
tags:
  - attack.execution
  - attack.t1059.001
```

### Compile to Multiple Targets

```bash
# Splunk SPL
sigma convert -t splunk rule.yml

# Elastic EQL (Elastic Security)
sigma convert -t elasticsearch rule.yml

# Microsoft Sentinel (KQL)
sigma convert -t sentinel rule.yml

# QRadar AQL
sigma convert -t qradar rule.yml

# Microsoft Defender for Endpoint
sigma convert -t mde rule.yml
```

### pySigma Programmatic Usage

```python
from sigma.collection import SigmaCollection
from sigma.backends.splunk import SplunkBackend

rules = SigmaCollection.load_ruleset("rules/windows/process_creation")
backend = SplunkBackend()
queries = backend.convert(rules)

for query in queries:
    print(query)
```

## Build

```bash
git clone https://github.com/SigmaHQ/sigma.git
cd sigma
pip install -r requirements.txt
# Rules are YAML files, no compilation needed for rules themselves
```

## Install

```bash
# pip install pySigma and backends
pip install pysigma pysigma-backend-splunk pysigma-backend-elasticsearch
pip install pysigma-backend-qradar pysigma-backend-insightidr

# sigma CLI
pip install sigma-cli
```

## Links

| Resource | URL |
|----------|-----|
| GitHub (rules) | https://github.com/SigmaHQ/sigma |
| GitHub (spec) | https://github.com/SigmaHQ/sigma-specification |
| pySigma | https://github.com/SigmaHQ/pySigma |
| Backends | https://github.com/SigmaHQ/ |
| Rule format spec | https://github.com/SigmaHQ/sigma-specification |
