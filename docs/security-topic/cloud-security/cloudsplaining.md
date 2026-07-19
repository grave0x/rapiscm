# Cloudsplaining — AWS IAM Least-Privilege Analysis

## How It Works

Cloudsplaining scans AWS IAM policies and identifies over-permissioned roles, privilege escalation paths, and violations of least-privilege principles. It generates an interactive HTML report with prioritized findings.

**Key architecture:**
- **Downloader module** — uses boto3 to enumerate all IAM policies (AWS managed, customer managed, inline, SCPs, permissions boundaries) across account(s)
- **Analysis engine** — evaluates each policy against:
  - Infrastructure modification privileges (`ec2:RunInstances`, `s3:PutBucketPolicy`)
  - Privilege escalation risk (IAM + EC2 + Lambda combinations)
  - Resource constraint (is it `"Resource": "*"` or scoped?)
  - Service-specific dangerous actions
- **Risk categories**:
  - **Privilege Escalation** — actions that enable privilege escalation (set IAM policy on self, pass role with wide trust, create access keys)
  - **Resource Exposure** — S3 GetObject/ListBucket broadcast, SQS send/receive without restriction
  - **Infrastructure Modification** — EC2/VPC/Lambda create/update/delete operations
  - **Data Exfiltration** — S3 GetObject, RDS snapshot sharing, secretsmanager retrieval

**HTML report sections:**
- Summary: total policies, high/medium/low findings
- Infamous modifier actions — sorted by blast radius
- Privilege escalation paths — chainable actions for privilege climbing
- Service-specific risk breakdown (S3, EC2, IAM, Lambda, KMS)
- Excel/CSV download for spreadsheets

## Manual

### Launch

```bash
# Analyze existing IAM policies from account
cloudsplaining download --profile production
cloudsplaining scan --input-file default.json --output-dir reports/

# Analyze a single policy file
cloudsplaining scan --input-file policies.json --output-dir reports/

# Exclusions file (false positives / approved services)
cloudsplaining scan --input-file default.json --output-dir reports/ --exclusions-file exclusions.yml
```

### Exclusion File

```yaml
# exclusions.yml
exclusions:
  accounts:
    "123456789012":
      - "AWSServiceRole*"   # skip AWS service-linked roles
  policies:
    - "AWS*"                # skip all AWS-managed policies
  actions:
    - "s3:Get*"             # allow all S3 GET (approved CDN pattern)
```

### HTML Report Features

- Interactive policy table with search, sort, filter
- Each policy shows: ARN, attached entities (users/groups/roles), resource scope
- High-risk actions link to IAM documentation
- Privilege escalation technique descriptions with TTP references
- Exportable CSV for compliance tracking

## Build

```bash
git clone https://github.com/salesforce/cloudsplaining.git
cd cloudsplaining
pip install -e .
```

## Install

```bash
# Option 1 — pip
pip install cloudsplaining

# Option 2 — Docker
docker pull salesforce/cloudsplaining:latest
docker run --rm -v $PWD:/data cloudsplaining cloudsplaining download
docker run --rm -v $PWD:/data cloudsplaining cloudsplaining scan \
  --input-file /data/default.json --output-dir /data/reports/
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/salesforce/cloudsplaining |
| Docs | https://cloudsplaining.readthedocs.io/ |
| Examples | https://cloudsplaining.readthedocs.io/en/latest/example-reports/ |
| Blog | https://engineering.salesforce.com/cloudsplaining-an-aws-iam-security-assessment-tool-c32a4e72044b/ |
