# Wiz — Cloud Security Platform

## How It Works

Wiz is a cloud-native security platform (CNAPP) that performs agentless vulnerability scanning, IaC security, CI/CD integration, and cloud security posture management across multi-cloud environments.

**Key architecture:**

- **Agentless scanning** — creates instant snapshots of cloud resources (compute, storage, serverless) and analyzes them out-of-band:
  - **Volume scanning** — EBS snapshots → mount read-only → file-level package/OS scanning
  - **Serverless** — Lambda function code inspection, dependency scan
  - **Containers** — registry-side image scan (ECR, ACR, GCR, Docker Hub via proxy)
  - **Storage** — S3/Azure Blob/GCS object-level malware and secret scanning
- **Cloud graph** — builds a graph of all cloud resources, their configurations, and IAM relationships
  - Queryable via GraphQL: `Security Graph`
- **CI/CD integration** — scan IaC templates (Terraform, CloudFormation) pre-deployment
- **Vulnerability correlation** — map CVEs to actual attack paths in the environment (contextual risk)
- **Compliance** — CIS, NIST 800-53, PCI DSS, SOC 2, HIPAA, ISO 27001 benchmarks

**Detection categories:**
- VM/container vulnerabilities (OS packages, language deps)
- IaC misconfigurations (public buckets, open security groups, overly permissive IAM)
- Secrets in code/configs (API keys, passwords, tokens)
- Malware in storage/artifacts
- Kubernetes RBAC and admission control gaps
- Toxic combinations (public-facing VM + critical vuln + high-privilege IAM role)

## Manual

### Web UI

```bash
# https://app.wiz.io
# Navigation: Vulnerabilities → Compute → Serverless → Containers
# Security Graph → GraphQL query builder
# Issues → prioritized finding list
```

### Security Graph Queries (GraphQL)

```graphql
# Find all EC2 instances with public IP and a critical vulnerability
query {
  computeInstances(
    filter: { 
      field: "HasInternetConnection", 
      value: "true" 
    }
  ) {
    nodes {
      id
      name
      publicIp
      effectiveVulnerabilityStats { criticalCount }
    }
  }
}
```

### CLI (wizcli)

```bash
# IaC scan
wizcli iac scan --path ./

# Container image scan
wizcli container scan --image nginx:latest

# Serverless scan
wizcli serverless scan --path ./function

# Policy check (custom rules)
wizcli iac scan --policy policy.rego
```

### API (REST)

```bash
# List issues
curl -H "Authorization: Bearer $TOKEN" \
  https://api.us1.app.wiz.io/v1/issues

# Get vulnerability details
curl -H "Authorization: Bearer $TOKEN" \
  https://api.us1.app.wiz.io/v1/vulnerabilities
```

## Install

### wizcli

```bash
# Linux
curl -L https://github.com/wiz-sec/wizcli/releases/latest/download/wizcli-linux-amd64 \
  -o wizcli && chmod +x wizcli
sudo mv wizcli /usr/local/bin/

# Authenticate
wizcli auth --token $WIZ_API_TOKEN --client-id $CLIENT_ID
```

### Connectors

```bash
# Cloud connectors (deployed in customer account/subscription)
# AWS: CloudFormation stack
# Azure: app registration + Terraform module
# GCP: service account + project deployment
# Kubernetes: Helm chart / Operator
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.wiz.io/ |
| Docs | https://docs.wiz.io/ |
| wizcli | https://github.com/wiz-sec/wizcli |
| Security Graph | https://docs.wiz.io/docs/security-graph |
| API reference | https://docs.wiz.io/docs/api-overview |
| Compliance frameworks | https://docs.wiz.io/docs/compliance-overview |
| Blog | https://www.wiz.io/blog |
