# Identity & Active Directory Security

Tools and techniques for attacking, auditing, and defending identity infrastructure — Active Directory, Entra ID (Azure AD), Kerberos, ADCS, and cloud identity platforms.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| Attack Path Mapping | BloodHound CE graph-based AD/Entra ID analysis |
| Kerberos Abuse | Kerberoasting, AS-REP Roasting, Golden/Silver Tickets |
| Entra ID / O365 | AzureHound, AADInternals, ROADtools, GraphRunner |
| ADCS Attacks | Certipy: ESC1–ESC16, Golden Certificate, NTLM relay |
| Credential Extraction | Mimikatz, LaZagne: LSASS, SAM, NTDS.dit |
| DCSync & Replication | secretsdump DRSUAPI replication abuse |
| Delegation Abuse | Unconstrained/constrained/RBCD |
| NTLM Relay | ntlmrelayx + Coercer/PetitPotam for ADCS/LDAP relay |
| Forest Trust Attacks | SID history, inter-realm Kerberos, trust key extraction |
| PAM & Tiered Admin | ESAE model, PAWs, JIT access |

## Methods

- **Graph-based enumeration** — collect AD/Entra with SharpHound/AzureHound; ingest into Neo4j; Cypher queries to Tier 0 paths
- **Kerberos attacks** — request TGS for SPN accounts (Kerberoasting); request TGT without preauth (AS-REP Roasting); forge TGT with krbtgt hash (Golden); forge service ticket (Silver)
- **ADCS abuse** — `certipy find` to enumerate templates; exploit ESC1 (EnrolleeSuppliesSubject), ESC8 (NTLM relay), ESC13 (group membership); forge Golden Certificate
- **Credential extraction** — LSASS dump (Mimikatz sekurlsa::logonpasswords); SAM/NTDS.dit extraction; Kerberos ticket dumps
- **NTLM relay** — coerce auth via PetitPotam/Coercer; relay to ADCS (ESC8), LDAP (Shadow Credentials), SMB
- **Entra ID enumeration** — Graph API queries for tenant config, CA policies, service principals, role assignments, OAuth grants

## Tool Comparison

| Tool | Category | Key Strength |
|------|----------|--------------|
| **BloodHound CE** | Attack Paths | Graph-based AD/Entra/AWS/GitHub analysis |
| **Certipy** | ADCS | ESC1–ESC16 enumeration and exploitation |
| **Mimikatz** | Credentials | LSASS dump, pass-the-hash, Golden/Silver tickets |
| **Impacket** | Protocol Kit | DCSync, NTLM relay, lateral movement |
| **AzureHound** | Entra ID | BloodHound collector for Entra ID |
| **AADInternals** | Entra ID | PowerShell recon, token manipulation |
| **ROADtools** | Entra ID | Token acquisition, tenant recon |
| **GraphRunner** | Entra ID | Post-exploitation Graph API toolkit |
| **PingCastle** | AD Audit | Risk assessment by tier, 60+ rules |
| **LaZagne** | Credentials | Multi-application credential recovery |
| **Rubeus** | Kerberos | Full Kerberos interaction suite |
