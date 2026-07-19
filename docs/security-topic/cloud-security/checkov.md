# Checkov — Infrastructure as Code Scanning

## How It Works

Checkov scans Terraform, CloudFormation, Kubernetes, Helm, ARM, and Dockerfile templates for misconfigurations, security issues, and compliance violations — with 1000+ built-in policies.

**Key architecture:**
- **Parser engine** — HCL (Terraform), YAML (K8s/CF), JSON (ARM), Dockerfile parser
- **Graph framework** — builds resource dependency graph for cross-resource checks (e.g., S3 bucket → KMS key → IAM policy)
- **Policy engine** — policy-as-code in YAML/Python using Custom Policies (YAML DSL) or Python API
- **Platform checks** — pre-computed policies for CIS, PCI DSS, HIPAA, SOC 2, NIST, GDPR
- **Output** — CLI table, JSON, SARIF, Junit XML, HTML, CLI summary

**Check categories:**
- Terraform: IAM, S3, RDS, KMS, VPC, Lambda — all CIS AWS Foundations Benchmark
- Kubernetes: pod security context, RBAC, secrets, network policies, PSP → PSS conversion
- CloudFormation: equivalent coverage parity with Terraform checks
- Docker: USER directive, no root, sensitive env vars, health checks
- Bicep/ARM: Azure-specific resource configs, secure defaults

## Manual

### Launch

```bash
# Scan directory of IaC files
checkov -d /path/to/iac/

# Scan single file
checkov -f main.tf

# Skip specific checks (by policy ID)
checkov -d . --skip-check CKV_AWS_1,CKV_AWS_52

# Run only specific checks
checkov -d . --check CKV_AWS_2,CKV_AWS_3

# External checks directory
checkov -d . --external-checks-dir /custom/checks/
```

### Output

```bash
# JSON output
checkov -d . -o json > results.json

# SARIF (for GitHub Advanced Security)
checkov -d . -o sarif > results.sarif

# HTML report
checkov -d . -o html > report.html

# Quiet mode (only failures)
checkov -d . --quiet
```

### CI/CD Integration

```yaml
# GitHub Actions
- name: Checkov
  uses: bridgecrewio/checkov-action@master
  with:
    directory: .
    framework: terraform,kubernetes
    output_format: sarif
```

### Custom Policy (YAML)

```yaml
# checks/custom/s3_encryption_enabled.yml
metadata:
  id: CUSTOM_S3_ENC
  name: S3 Bucket Encryption Enabled
  category: DATA_PROTECTION
definition:
  and:
    - cond: equals
      resource: "aws_s3_bucket.server_side_encryption_configuration"
      not_null: true
```

## Build

```bash
git clone https://github.com/bridgecrewio/checkov.git
cd checkov
pip install -e .
```

## Install

```bash
# Option 1 — pip
pip install checkov

# Option 2 — Docker
docker pull bridgecrew/checkov:latest
docker run --rm -v $PWD:/scan bridgecrew/checkov -d /scan

# Option 3 — Homebrew
brew install checkov

# Option 4 — GitHub Action
# bridgecrewio/checkov-action
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/bridgecrewio/checkov |
| Docs | https://www.checkov.io/ |
| Policy index | https://www.checkov.io/5.Policy%20Index/ |
| Custom policies | https://www.checkov.io/3.Custom%20Policies/ |
| Bridgecrew (managed) | https://bridgecrew.io/ |
