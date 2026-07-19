# AWS Security Hub — Cloud Security Posture Management (CSPM)

## How It Works

AWS Security Hub is a managed CSPM service that aggregates, correlates, and prioritizes security findings from AWS services and third-party tools into a single dashboard.

**Key architecture:**

- **Finding aggregation** — ingests findings from:
  - **AWS-native:** GuardDuty, Inspector, Macie, IAM Access Analyzer, Firewall Manager, WAF, Config
  - **Third-party:** 75+ partner integrations (CrowdStrike, Palo Alto, Tenable, Qualys, Sumo Logic, etc.)
  - **Custom findings** — via `BatchImportFindings` API (security automation friendly)
- **Compliance standards** — automated checks against:
  - CIS AWS Foundations Benchmark v1.4 / v3.0
  - NIST SP 800-53 Rev 5
  - PCI DSS 3.2.1
  - AWS Foundational Security Best Practices (FSBP)
  - SOC 2 (via third-party)
- **Security score** — computed per standard as pass/fail ratio
- **Insights** — cross-finding aggregation queries (e.g., "all S3 buckets that are public AND have sensitive data")
- **Automated remediation** — EventBridge → Lambda/SSM Automation → auto-fix common misconfigurations
- **Cross-Region aggregation** — designate a single aggregation Region for multi-Region finding visibility
- **Multi-account** — enable via AWS Organizations; Central Configuration auto-enrolls new accounts

**Finding severity:**
- Informational, Low, Medium, High, Critical (each with numeric score 0.1–100)

## Manual

### Console

```bash
# https://console.aws.amazon.com/securityhub
# Summary → Compliance Score → Findings → Insights → Integrations
```

### AWS CLI

```bash
# Enable Security Hub
aws securityhub enable-security-hub \
  --enable-default-standards \
  --tags Environment=production

# Disable specific standard
aws securityhub batch-disable-standards \
  --standards-subscription-arns arn:aws:securityhub:us-east-1:...:subscription/cis-aws-foundations-benchmark/v/1.4.0

# List findings
aws securityhub get-findings \
  --filters '{"SeverityLabel": [{"Value": "CRITICAL", "Comparison": "EQUALS"}]}' \
  --max-items 50

# Get compliance summary
aws securityhub get-compliance-summary \
  --compliance-check-identifier "CIS.1.1"

# Batch import custom finding
aws securityhub batch-import-findings \
  --findings file://finding.json
```

### Automated Remediation via EventBridge

```bash
# Rule: Security Hub → EventBridge → SSM or Lambda
# Example: auto-close console password policy finding when fixed
aws events put-rule \
  --name "security-hub-remediation" \
  --event-pattern file://pattern.json

# pattern.json:
# {
#   "source": ["aws.securityhub"],
#   "detail-type": ["Security Hub Findings - Imported"],
#   "detail": {
#     "findings": {
#       "ProductName": ["Security Hub"],
#       "Compliance": { "Status": ["FAILED"] }
#     }
#   }
# }
```

### Multi-Account Setup

```bash
# 1. Enable Security Hub in management account
# 2. Designate admin account (if different from management)
aws securityhub enable-organization-admin-account \
  --admin-account-id 123456789012

# 3. Auto-enable for all current + future organization accounts
aws securityhub update-organization-configuration \
  --auto-enable
```

## Enable

```bash
# AWS Console → Security Hub → Get started
# Or via CLI (see above)
# First-time enable takes 2–5 minutes to initialize security standards
# Findings start appearing within 15 minutes
```

## Pricing

- Per account per Region per month
- Per finding ingestion event
- Per compliance check execution
- Free tier: 30-day trial, limited findings/month

## Links

| Resource | URL |
|----------|-----|
| Service page | https://aws.amazon.com/security-hub/ |
| Docs | https://docs.aws.amazon.com/securityhub/ |
| CLI reference | https://awscli.amazonaws.com/v2/documentation/api/latest/reference/securityhub/index.html |
| API reference | https://docs.aws.amazon.com/securityhub/1.0/APIReference/ |
| Partner integrations | https://docs.aws.amazon.com/securityhub/latest/userguide/securityhub-integrations-managed.html |
| Compliance standards | https://docs.aws.amazon.com/securityhub/latest/userguide/standards-reference.html |
| Pricing | https://aws.amazon.com/security-hub/pricing/ |
