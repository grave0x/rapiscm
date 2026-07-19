# Cloud Security

Tools for CSPM, CIEM, cloud penetration testing, IaC scanning, runtime security, attack path visualization, and compliance automation across AWS, Azure, and GCP.

## Topics

| Topic | Description |
|-------|-------------|
| CSPM | Cloud Security Posture Management — detect misconfigurations across cloud APIs (public buckets, open security groups, missing encryption) |
| CIEM | Cloud Infrastructure Entitlement Management — IAM graph analysis for privilege escalation paths, over-permissioned roles |
| Cloud Pen Testing | Exploitation frameworks for post-exploitation, privilege escalation, persistence, data exfiltration |
| IaC Scanning | Scan Terraform/CloudFormation/K8s manifests at PR time to block misconfigurations pre-deploy |
| Attack Path Visualization | Correlate CSPM + CWPP + CIEM + DSPM to show toxic combinations (agentless CNAPP tools) |
| Compliance Automation | Map cloud posture to multiple frameworks simultaneously — CIS, NIST 800-53, PCI DSS, HIPAA, SOC 2 |
| Runtime Security | eBPF-based syscall monitoring for container escapes, privilege escalation (Falco, Tetragon) |

## Methods

- **CSPM Scanning:** Query cloud APIs to benchmark against CIS/NIST/PCI → prioritized remediation list
- **CIEM Analysis:** Build permission graph → find escalation paths → reduce to least-privilege (Cloudsplaining, Principal Mapper)
- **Cloud Pentest:** Pacu modules → enumerate IAM → escalate → persist → exfiltrate data
- **IaC Scanning:** Checkov / tfsec — 1000+ policies across Terraform, CloudFormation, K8s, Helm, Docker
- **Attack Graph:** Cartography → Neo4j asset graph — correlation across AWS/Azure/GCP relationships
- **Infrastructure Query:** Steampipe — SQL-based real-time queries against cloud provider APIs
- **Commercial CNAPP:** Wiz / Orca / Prisma Cloud — agentless scanning, graph-based attack paths, compliance reporting

## Tools

| Tool | Category | Description | License |
|------|----------|-------------|---------|
| Prowler | CSPM | Open-source cloud security platform — 615 AWS, 190 Azure, 109 GCP checks, 47 compliance frameworks | Apache 2.0 |
| Pacu | Cloud Pentest | AWS exploitation framework — 30+ privilege escalation modules, persistence, exfiltration | BSD 3-Clause |
| Cloudsplaining | CIEM | AWS IAM least-privilege analysis — finds over-permissioned roles and policy violations | BSD 3-Clause |
| CloudFox | Attack Path | Cloud attack path discovery — outbound connections, secret hunting, permission mapping | Apache 2.0 |
| AzureHound | CIEM | BloodHound data collector for Microsoft Entra ID (Azure AD) attack paths | Apache 2.0 |
| Cartography | Asset Graph | Neo4j-based cloud asset graph and relationship mapper (AWS, GCP, Azure) | Apache 2.0 |
| Steampipe | Infrastructure Query | Query cloud infrastructure APIs with SQL — AWS/Azure/GCP/K8s/Vault | Apache 2.0 |
| Checkov | IaC Scanning | IaC scanning for Terraform/CloudFormation/K8s/Helm — 1000+ built-in policies | Apache 2.0 |
| Wiz | CNAPP | Market-leading CNAPP — agentless, graph-based attack path analysis, CIEM, DSPM | Commercial |
| Orca Security | CNAPP | Agentless CNAPP — SideScanning, attack path analysis, CIEM, DSPM, KSPM | Commercial |
| Prisma Cloud | CNAPP | Full CNAPP — CSPM, CWPP, CIEM, IaC, runtime, WAAS, Cortex integration | Commercial |
| AWS Security Hub | CSPM (AWS) | Native CSPM — aggregated findings from GuardDuty/Inspector/Macie/Config | AWS native (pay-per-event) |
