# Orca Security — Cloud Security Platform

## How It Works

Orca Security is an agentless cloud security platform (CNAPP) that uses side-scanning of cloud block storage volumes (EBS, disk snapshots) for deep workload analysis without agents.

**Key architecture:**

- **SideScanning™** — agentless workload inspection:
  1. Scan cloud API for compute resources (EC2, ECS, Lambda, AKS nodes, GCE)
  2. Trigger block storage snapshot (EBS snapshot, managed disk snapshot)
  3. Mount snapshot read-only in Orca's analysis environment
  4. Analyze: package versions, file contents, configs, secrets, malware
  5. Delete snapshot — no persistent access to customer data
- **Cloud-to-Org graph** — map all cloud assets (compute, data, identities, network, K8s) with 1:1 resource-to-account relationships
- **Risk prioritization** — contextual risk engine combining: CVE severity, exploitability (EPSS), asset value, network exposure, and IAM blast radius
- **Compliance** — built-in benchmarks for CIS, NIST 800-53, PCI DSS 4.0, HIPAA, SOC 2, ISO 27001, FedRAMP, GDPR
- **Malware detection** — file-level scanning for known malware hashes + YARA rules on snapshot contents

**Coverage:**
- AWS, Azure, GCP, Kubernetes (EKS, AKS, GKE)
- Compute: EC2, ECS, Lambda, VM instances, GCE, AKS nodes, GKE nodes
- Data: S3, RDS, DynamoDB, Azure Blob, GCS, ElastiCache, Redshift
- Identities: IAM users/roles, Azure RBAC, GCP IAM
- Network: VPCs, security groups, load balancers, API gateways
- Containers: registry images, ECS task definitions, pod specs

## Manual

### Web UI

```bash
# https://app.orcasecurity.io
# Inventory → all assets, grouped by cloud/resource type
# Alerts → prioritized findings with contextual risk scoring
# Compliance → per-framework posture scores and failed controls
```

### API

```bash
# Get alerts
curl -H "Authorization: Bearer $API_KEY" \
  https://app.orcasecurity.io/api/alerts

# Get asset inventory
curl -H "Authorization: Bearer $API_KEY" \
  https://app.orcasecurity.io/api/assets

# Query by severity
curl -H "Authorization: Bearer $API_KEY" \
  "https://app.orcasecurity.io/api/alerts?severity=critical"
```

### Risk Scoring

```bash
# Each alert includes contextual risk score (1–100)
# Factors:
#   - CVE/issue severity (CVSS)
#   - Exploitability (EPSS, CISA KEV)
#   - Asset sensitivity (data classification, production tag)
#   - Network exposure (public internet, internal, isolated)
#   - IAM blast radius (privileges of attached role)
#   - Lateral movement potential
```

## Install

### Cloud Connectors

```bash
# AWS — CloudFormation stack with read-only IAM role
# Azure — app registration with Reader role at management group scope
# GCP — service account with Organization-level Security Center viewer role
# Kubernetes — Helm chart with read-only ClusterRole

# Onboarding via web UI → guided cloud account setup → automated deployment
```

### CLI

```bash
# No local CLI tool. All interaction via web UI or REST API.
# Terraform provider available for connector deployment as IaC:
# https://registry.terraform.io/providers/orcasecurity/orca/latest
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://orca.security/ |
| Docs | https://docs.orcasecurity.io/ |
| API reference | https://docs.orcasecurity.io/docs/api-overview |
| Compliance frameworks | https://docs.orcasecurity.io/docs/compliance-frameworks |
| Blog | https://orca.security/blog/ |
| Terraform provider | https://registry.terraform.io/providers/orcasecurity/orca |
