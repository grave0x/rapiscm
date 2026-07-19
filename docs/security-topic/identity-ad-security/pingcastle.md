# PingCastle — AD Security Audit and Health Check

Windows-based Active Directory security assessment tool that evaluates AD configuration against 60+ security rules, scores risk, and produces detailed HTML reports. Designed for blue teams, auditors, and AD administrators.

## How It Works

PingCastle runs in two modes — **Health Check** (scoring) and **Graph** (attack path analysis). It connects to the domain via LDAP with a domain-joined machine and authenticated user account.

**Health Check:**
- Scans AD objects (users, groups, computers, GPOs, trusts, ACLs) against 60+ rules organized in categories
- Each rule is weighted by risk impact
- Output: 0–100 risk score (lower = better), categorized findings with severity (Critical, High, Medium, Low)
- Compares score against previous baselines for trend analysis

**Graph:**
- Analyzes ACL relationships to find privilege escalation paths (similar to BloodHound but local-only and PC-based)
- Generates graphical representation of attack paths
- Scoped to reachable objects within a configurable number of hops (default 10)

**Rule categories:**

| Category | Examples |
|----------|---------|
| **Account** | Stale admin accounts, inactive users, disabled accounts not cleaned up, privileged groups |
| **ACL** | Dangerous permissions on domain root, adminSDHolder, delegated OUs |
| **ANR** | Ambiguous Name Resolution, anonymous LDAP bind |
| **Computer** | LAPS status, OS versions, stale computer accounts, delegation settings |
| **Delegation** | Unconstrained delegation, constrained delegation anomalies, RBCD |
| **GPO** | Weak GPO permissions, unused GPOs, blocked inheritance, GPO links |
| **Kerberos** | RC4 encryption enabled, weak service ticket encryption, kerberoastable users |
| **Objects** | Orphaned objects, duplicate SPNs, administrative accounts in user groups |
| **PKI** | AD CS misconfigurations, CA server health, certificate template vulns |
| **Privesc** | Escalation paths from workstation to domain admin |
| **Recon** | Users with SPNs, AS-REP roastable accounts, LDAP signing not enforced |
| **SID** | SID History abuse paths, sid filtering on trusts |
| **Trust** | Intra-forest and inter-forest trust misconfigurations |
| **Vulnerability** | Known CVEs mapped to current AD state |

## Manual

### Prerequisites

```powershell
# Domain-joined machine + domain user account
# Run as domain user (no special privilege required for most checks)
# Some checks (adminSDHolder, LAPS, PSO) need delegated or admin rights
```

### Health Check

```powershell
# Launch GUI
PingCastle.exe

# Command-line — health check
PingCastle.exe --health-check --server dc.domain.local --user domain\user --password 'Pass!' --output-directory C:\reports

# Health check with current session (no creds needed if domain-joined)
PingCastle.exe --health-check

# Generate report only (from saved data)
PingCastle.exe --health-check --report-only
```

### Graph Mode

```powershell
# Generate graph relationship data
PingCastle.exe --graph --server dc.domain.local

# Limit traversal depth
PingCastle.exe --graph --max-hop 15

# Output to specific directory
PingCastle.exe --graph --output-directory C:\reports
```

### Cartography (AD topology map)

```powershell
# Map AD site/subnet topology
PingCastle.exe --carto --server dc.domain.local
```

### Reports

```powershell
# Output formats (automatic with health check)
# - HTML report with embedded score badge
# - XML data file (for comparison/baselining)
# - JSON export for SIEM integration
# - CSV tables for individual rule results

# Compare against previous baseline
PingCastle.exe --health-check --baseline C:\reports\baseline.xml
```

### CI/CD (Scheduled/Unattended)

```powershell
# Silent mode
PingCastle.exe --health-check --no-scan --no-ui --output-directory C:\reports

# Scheduled task
schtasks /create /tn "PingCastle Health Check" /tr "C:\Tools\PingCastle.exe --health-check --output-directory C:\reports" /daily /st 02:00
```

## Build

```powershell
# No build required — precompiled binary
# Source available on GitHub
git clone https://github.com/vletoux/PingCastle.git
# Open PingCastle.sln in Visual Studio and build
```

## Install

### Windows

```powershell
# Download from GitHub releases
Invoke-WebRequest -Uri https://github.com/vletoux/PingCastle/releases/latest/download/PingCastle.zip -OutFile PingCastle.zip
Expand-Archive -Path PingCastle.zip -DestinationPath C:\Tools\PingCastle

# Run
C:\Tools\PingCastle\PingCastle.exe
```

### No installation required

```powershell
# Self-contained executable — no .NET runtime dependency (included)
# Copy to any Windows machine and run
```

## Package

| Manager | Command |
|---------|---------|
| GitHub Releases | https://github.com/vletoux/PingCastle/releases |
| Chocolatey | `choco install pingcastle` |
| Scoop | `scoop install pingcastle` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/vletoux/PingCastle |
| Official site | https://www.pingcastle.com/ |
| Documentation | https://www.pingcastle.com/documentation/ |
| Report samples | https://www.pingcastle.com/documentation/sample-reports/ |
| Risk rules list | https://www.pingcastle.com/documentation/risk-rules/ |
| Comparison guide | https://www.pingcastle.com/documentation/healthcheck-vs-graph/ |
| Blog | https://www.pingcastle.com/blog/ |
