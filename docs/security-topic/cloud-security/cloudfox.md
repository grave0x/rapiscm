# CloudFox — Cloud Attack Path Enumeration

## How It Works

CloudFox is an open-source offensive cloud security tool by BishopFox that helps operators find exploitable attack paths in AWS and Azure environments.

**Key architecture:**

- **Multi-cloud support** — AWS (primary), Azure (growing), GCP (experimental)
- **Collector modules** — gather IAM policies, resource configurations, trust relationships via cloud provider APIs
- **Relationship mapping** — link IAM principals to resources, trust policies to external accounts, network paths to services
- **Attack path generation** — traverse relationships to discover: privilege escalation chains, lateral movement routes, data exfiltration vectors

**AWS collection:**
- IAM users, groups, roles, policies (inline + managed)
- S3 bucket policies and ACLs
- EC2 instances, security groups, AMIs
- Lambda functions and resource-based policies
- EKS clusters and IAM mappings
- RDS instances and snapshots
- CloudFormation stacks
- Resource-based policies: SQS, SNS, KMS, Secrets Manager

**Azure collection:**
- Azure RBAC role assignments
- Entra ID app registrations and service principals
- Managed identities
- Key Vault access policies

## Manual

### Launch

```bash
# AWS
cloudfox aws --profile mytarget -o /tmp/cloudfox

# Azure
cloudfox azure --export-dir /tmp/cloudfox
```

### Common Commands

```bash
# Enumerate all IAM permissions
cloudfox aws -p mytarget -o ./output

# Find privilege escalation paths
cloudfox aws -p mytarget -o ./output -m cheats

# Check trust policies
cloudfox aws -p mytarget -o ./output -m trusts

# S3 bucket enumeration
cloudfox aws -p mytarget -o ./output -m buckets

# Command cheats (print actionable commands for findings)
cloudfox aws -p mytarget -o ./output -m cheats
```

### Key Modules

```bash
# Check principals — list IAM users, roles, and their effective permissions
cloudfox aws -p mytarget -m check-principals

# Loot — extract secrets from user-data, environment variables, Lambda env
cloudfox aws -p mytarget -m loot

# Inbound connections — security group rules allowing public access
cloudfox aws -p mytarget -m inbound

# Outbound connections — security group egress rules
cloudfox aws -p mytarget -m outbound
```

### Output

```bash
# HTML report
ls -la /tmp/cloudfox/
open /tmp/cloudfox/cloudfox.html  # interactive findings report
```

## Build

```bash
git clone https://github.com/BishopFox/cloudfox.git
cd cloudfox
go build .
# Artifact: ./cloudfox
```

## Install

```bash
# Download pre-built binary from GitHub releases
wget https://github.com/BishopFox/cloudfox/releases/latest/download/cloudfox-linux-amd64.zip
unzip cloudfox-linux-amd64.zip
sudo mv cloudfox /usr/local/bin/

# Also via:
# Homebrew
brew install cloudfox
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/BishopFox/cloudfox |
| Usage docs | https://github.com/BishopFox/cloudfox/blob/main/Usage.md |
| AWS modules | https://github.com/BishopFox/cloudfox/tree/main/aws/collectors |
| BishopFox | https://www.bishopfox.com/ |
