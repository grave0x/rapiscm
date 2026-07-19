# Prowler — Cloud Security Platform

## How It Works

Prowler is an open-source cloud security assessment tool that executes 600+ checks against AWS, 190 against Azure, and 109 against GCP — mapped to 47 compliance frameworks.

**Key architecture:**
- **Provider modules** — AWS (boto3), Azure (azure-identity + azure-mgmt-*), GCP (google-cloud-*)
- **Checks** — Python functions querying cloud APIs for misconfigurations, exposed resources, missing encryption, disabled logging
- **Compliance frameworks** — CIS v5, NIST 800-53, PCI DSS 4.0, HIPAA, SOC 2, FedRAMP, ENS (Spain), ISO 27001
- **Output** — CLI table, JSON, CSV, HTML, SARIF, JUnit XML, ASFF (AWS Security Finding Format)
- **Dashboard** — optional web dashboard via AWS QuickSight or local HTML

**Check categories (AWS):**
- Identity and Access Management (IAM) — 100+ checks
- S3 — public access, encryption, versioning, logging
- EC2 — security groups, AMI, snapshots, EBS encryption
- RDS — public access, encryption, backup retention
- GuardDuty / Config / CloudTrail — service enablement, logging validation
- Network — VPC flow logs, security group rules, WAF presence
- Lambda/Kinesis/SQS/SNS — encryption, access policies

## Manual

### Launch

```bash
# Quick assessment (all AWS checks)
prowler aws

# Specific check
prowler aws --checks s3_bucket_public_access

# Specific compliance framework
prowler aws --compliance cis_1.4

# Azure assessment
prowler azure --az-cli-auth

# GCP assessment
prowler gcp --impersonate-idp <sa@project.iam.gserviceaccount.com>
```

### Output

```bash
# HTML report
prowler aws -M html -o /tmp/report

# CSV
prowler aws -M csv -o /tmp/report

# JSON lines (for SIEM ingestion)
prowler aws -M json-lines -o /tmp/report

# AWS Security Finding Format (ASFF)
prowler aws -M asff -o /tmp/report --security-hub
```

### Common Categories

```bash
# IAM checks
prowler aws --categories iam

# S3 checks
prowler aws --categories s3

# Check by severity
prowler aws --severity critical,high
```

## Build

```bash
git clone https://github.com/prowler-cloud/prowler.git
cd prowler
pip install -e .
```

## Install

```bash
# Option 1 — pip
pip install prowler

# Option 2 — Docker
docker pull toniblyx/prowler:latest
docker run --rm -v $PWD:/report toniblyx/prowler:latest aws

# Option 3 — Homebrew
brew install prowler

# Option 4 — AWS ECS / EKS
# prowler-container pattern available in docs
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/prowler-cloud/prowler |
| Docs | https://docs.prowler.com/ |
| Compliance frameworks | https://docs.prowler.com/projects/prowler-open-source/en/latest/tutorials/compliance/ |
| Community | https://join.slack.com/t/prowler-cloud/shared_invite/zt-2ojmxy9z7-1B3XhRx~BqOD~3D~4KH5g |
