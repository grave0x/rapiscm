# Steampipe — Infrastructure Query Engine

## How It Works

Steampipe exposes cloud provider APIs as PostgreSQL tables — query AWS, Azure, GCP, Kubernetes, GitHub, and 100+ other services with SQL.

**Key architecture:**
- **Foreign Data Wrapper (FDW)** — implements PostgreSQL foreign data wrapper protocol. Each table maps to an API call.
- **Plugins** — Go-based plugins for each service (`steampipe-plugin-aws`, `steampipe-plugin-azure`, etc.). Each plugin defines tables and implements API call → row mapping.
- **Connection config** — one config entry per account/region. Multiple connections can be queried together via aggregators.
- **PostgreSQL interface** — standard SQL (with JSONB for nested API responses). Works with any PostgreSQL client (psql, Tableau, Metabase, Grafana).

**Table patterns:**
- `aws_iam_role` — maps to ListRoles API
- `aws_s3_bucket` — maps to ListBuckets + GetBucketLocation + GetBucketAcl + GetBucketPolicy
- `aws_ec2_instance` — maps to DescribeInstances
- `gcp_storage_bucket` — maps to GCP Storage ListBuckets
- `azure_compute_virtual_machine` — maps to Azure VM ListAll
- `kubernetes_pod` — maps to k8s ListPods API

## Manual

### Launch

```bash
# Start Steampipe service
steampipe service start

# Or run in interactive shell
steampipe query

# Run a SQL query directly
steampipe query "SELECT name, region, create_date FROM aws_iam_role;"

# Output as JSON
steampipe query --output json \
  "SELECT count(*), region FROM aws_iam_role GROUP BY region"
```

### Common Security Queries

```sql
-- Public S3 buckets
SELECT name, region, account_id
FROM aws_s3_bucket
WHERE s3_bucket_is_public;

-- IAM users with access keys older than 90 days
SELECT name, create_date, access_key_id
FROM aws_iam_access_key
WHERE create_date < now() - interval '90 days';

-- Security groups with unrestricted SSH
SELECT group_id, vpc_id, ip, from_port, to_port
FROM aws_vpc_security_group_rule
WHERE ip = '0.0.0.0/0' AND from_port = 22;

-- EC2 instances without encryption
SELECT instance_id, state, region
FROM aws_ec2_instance
WHERE root_block_device_encrypt = false;

-- Cross-account role trusts (exfiltration risk)
SELECT role_name, trust_policy
FROM aws_iam_role
WHERE trust_policy::jsonb @> '{"Statement": [{"Effect": "Allow", "Principal": {"AWS": "*"}}]}';
```

### Configuration

```hcl
# ~/.steampipe/config/aws.spc
connection "aws_prod" {
  plugin  = "aws"
  regions = ["us-east-1", "us-west-2", "eu-west-1"]
}

connection "aws_dev" {
  plugin  = "aws"
  regions = ["us-east-1"]
}

# Query both accounts simultaneously
connection "aws_all" {
  plugin      = "aws"
  type        = "aggregator"
  connections = ["aws_prod", "aws_dev"]
}
```

## Build

```bash
git clone https://github.com/turbot/steampipe.git
cd steampipe
go build -o steampipe .
```

## Install

```bash
# Option 1 — automatically with plugin install
sudo /bin/sh -c "$(curl -fsSL https://steampipe.io/install/steampipe.sh)"

# Option 2 — Homebrew
brew install steampipe

# Option 3 — Docker
docker pull turbot/steampipe:latest

# Install plugins
steampipe plugin install aws
steampipe plugin install azure
steampipe plugin install gcp
steampipe plugin install kubernetes
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://steampipe.io/ |
| GitHub | https://github.com/turbot/steampipe |
| Docs | https://steampipe.io/docs |
| Plugin hub | https://hub.steampipe.io/ |
| Security mod | https://hub.steampipe.io/mods/turbot/aws_compliance |
