# Cartography — Cloud Asset Graph Mapper

## How It Works

Cartography builds a Neo4j graph database of cloud infrastructure assets and their relationships across AWS, GCP, Azure, and Kubernetes — enabling attack path analysis, dependency mapping, and compliance visibility.

**Key architecture:**
- **Neo4j graph model** — every cloud resource is a node, every relationship (IAM → S3 bucket, EC2 → VPC, role → service) is an edge
- **Sync modules** — `cartography.intel.aws` / `azure` / `gcp` / `okta` — each queries provider APIs and upserts into Neo4j
- **Cypher queries** — built-in or custom queries for asset inventory, exposure analysis, privilege escalation paths
- **Updatable** — incremental syncs add/update/remove nodes without full rebuild

**Asset types (AWS):**
- EC2 instances, AMIs, security groups, VPC, subnets, ELB
- S3 buckets, policies, ACLs
- IAM users, groups, roles, policies, instance profiles
- RDS, DynamoDB, Lambda, SQS, SNS
- Organizations, accounts, SCPs

**Attack path queries:**
- IAM roles assumable from EC2 → privilege escalation to S3
- Security group overly permissive (0.0.0.0/0 on SSH/RDP) → exposed EC2
- Lambda execution role with full S3 access → data exfiltration path
- Cross-account role trust → lateral movement vector

## Manual

### Launch

```bash
# Start Neo4j (required)
docker run -p 7474:7474 -p 7687:7687 neo4j:latest

# Run Cartography (AWS)
AWS_PROFILE=production cartography --neo4j-password <password>

# Run specific modules only
cartography --neo4j-password <password> --modules aws azure

# Run with config file
cartography --config-file config.yaml
```

### Common Cypher Queries

```cypher
// All EC2 instances with their security groups
MATCH (ec2:EC2Instance)-[:MEMBER_OF_EC2_SECURITY_GROUP]->(sg:EC2SecurityGroup)
RETURN ec2.id, ec2.instanceid, sg.groupid, sg.ip_permissions[0].from_port

// IAM roles attached to EC2
MATCH (role:AWSRole)<-[:HAS_IAM_ROLE]-(profile:InstanceProfile)<-[:HAS_INSTANCE_PROFILE]-(ec2:EC2Instance)
RETURN role.name, ec2.id

// Public S3 buckets
MATCH (bucket:S3Bucket)
WHERE bucket.anonymous_access IS NOT NULL
RETURN bucket.name, bucket.anonymous_access
```

### Config File

```yaml
# config.yaml
neo4j_uri: bolt://localhost:7687
neo4j_max_connection_lifetime: 3600
update_tag: cartography-20260717
modules:
  aws:
    enabled: true
    accounts:
      - '123456789012'
      - '210987654321'
  azure:
    enabled: true
```

## Build

```bash
git clone https://github.com/lyft/cartography.git
cd cartography
pip install -e .
```

## Install

```bash
# Option 1 — pip
pip install cartography

# Option 2 — Docker
docker pull lyft/cartography:latest

# Need Neo4j running:
docker run -p 7474:7474 -p 7687:7687 -e NEO4J_AUTH=neo4j/<password> neo4j:latest
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/lyft/cartography |
| Docs | https://cartography-cncf.readthedocs.io/ |
| Query library | https://github.com/lyft/cartography/tree/master/cartography/graphs |
| Attack path queries | https://cartography-cncf.readthedocs.io/en/latest/community/attack_paths.html |
