# Prisma Cloud — Cloud Security (Palo Alto Networks)

## How It Works

Prisma Cloud (formerly Twistlock + RedLock + Aporeto) is Palo Alto Networks' CNAPP (Cloud-Native Application Protection Platform) covering cloud security posture management, workload protection, IaC scanning, and runtime security.

**Key architecture:**

- **Cloud Security Posture Management (CSPM)** — compliance benchmarks across AWS, Azure, GCP, OCI, Alibaba Cloud: CIS, NIST 800-53, PCI DSS, HIPAA, SOC 2, ISO 27001, FedRAMP
- **Cloud Workload Protection Platform (CWPP)** — agent-based and agentless scanning:
  - Hosts: vulnerability scanning + malware detection
  - Containers: image scan at registry (ECR, Docker Hub, ACR, GCR) + runtime threat detection
  - Serverless: Lambda/Azure Function/Cloud Function code analysis
  - Kubernetes: admission controller (Prisma Cloud Defender) for pod security + network policy enforcement
- **Data Security Posture Management (DSPM)** — sensitive data discovery in S3, RDS, Azure Blob, GCS via content inspection
- **CI/CD scanning** — IaC template scans (Terraform, CloudFormation, Azure RM, Helm) in pipeline; image scanning in CodeBuild, Jenkins, GitHub Actions
- **Runtime defense** — Waas (Web Application and API Security) layer: OWASP Top 10 WAF, bot management, API discovery + threat detection
- **Attack path analysis** — graph-based visualization of toxic combinations

**Deployment options:**
- SaaS (multi-tenant)
- Self-hosted (on-premises)
- Air-gapped (no outbound connectivity)

## Manual

### Web UI

```bash
# https://app.prismacloud.io
# Dashboard → Compliance → Alerts → Defender → Compute
```

### CLI (twistcli)

```bash
# Image scanning
twistcli images scan --registry-url https://registry.example.com --user $USER --password $PASS nginx:latest

# IaC scanning
twistcli iac scan --repository ./terraform/

# Serverless scan
twistcli serverless scan --function-path ./myfunc

# Vulnerability scanning (host)
twistcli scans list --host
```

### Prisma Cloud API

```bash
# Get alerts
curl -H "x-redlock-auth: $JWT" \
  https://api.prismacloud.io/alert

# Compliance postures
curl -H "x-redlock-auth: $JWT" \
  https://api.prismacloud.io/compliance/posture

# Asset inventory
curl -H "x-redlock-auth: $JWT" \
  https://api.prismacloud.io/inventory

# Scan results
curl -H "x-redlock-auth: $JWT" \
  https://api.prismacloud.io/scan/results
```

### Defender (Kubernetes)

```bash
# Deploy Prisma Cloud Defender as DaemonSet
kubectl create secret generic twistlock-secret \
  --from-literal=password=$DEFENDER_PASSWORD
curl -k -X POST --header "Authorization: Bearer $JWT" \
  https://console.example.com/api/v1/defenders/daemonset.yaml \
  --data '{"consoleAddr":"console.example.com", "orchestration":"kubernetes"}' \
  | kubectl apply -f -
```

## Install

### twistcli

```bash
# Download from Prisma Cloud Console
curl -L -o twistcli https://<console-url>/api/v1/util/twistcli
chmod +x twistcli
sudo mv twistcli /usr/local/bin/
```

### Connectors

```bash
# Cloud account connectors configured via web UI
# AWS: CloudFormation or manual IAM role
# Azure: app registration + reader role
# GCP: service account with project viewer

# On-premise: Prisma Cloud Compute Console deployed as Docker image
docker run --rm -d -p 8083:8083 \
  -e INTERNAL_PORT=8083 \
  -e DATA_FOLDER=/data \
  -v /var/run/docker.sock:/var/run/docker.sock \
  registry.twistlock.com/twistlock/console:latest
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.paloaltonetworks.com/prisma/cloud |
| Docs | https://docs.prismacloud.io/ |
| Compute docs | https://docs.paloaltonetworks.com/prisma/prisma-cloud/prisma-cloud-admin-compute |
| API reference | https://prisma.pan.dev/api/cloud/cspm/ |
| twistcli guide | https://docs.prismacloud.io/en/classic/twistlock-cli |
| Compliance | https://docs.prismacloud.io/en/classic/compliance |
| Blog | https://www.paloaltonetworks.com/blog/cloud-security/ |
