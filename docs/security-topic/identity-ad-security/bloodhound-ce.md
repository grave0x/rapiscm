# BloodHound CE — AD Attack Path Mapping

Open-source graph-based tool that reveals hidden & unintended relationships in Active Directory and Entra ID environments. Uses graph theory to map attack paths from low-privileged user to domain admin / global admin.

## How It Works

BloodHound CE (Community Edition) replaces the legacy BloodHound. It consists of three components:

| Component | Role |
|-----------|------|
| **Collector** | SharpHound (C#) / AzureHound (Go) — enumerate AD objects, ACLs, sessions, group memberships, trust relationships and ship as JSON |
| **Database** | Neo4j graph DB — stores nodes (users, groups, computers, GPOs) and edges (relationships and attack primitives) |
| **UI** | React-based frontend — query the graph with Cypher, see pre-built attack path queries, visualize shortest paths to DA/GA |

**Data collected:**
- Group memberships (recursive), nested groups
- ACL/ACE entries — `GenericAll`, `WriteDACL`, `WriteOwner`, `ForceChangePassword`
- Computer sessions, logged-on users, local admin mappings
- Kerberos delegation — unconstrained, constrained, RBCD, RBCD
- GPO links, OU structure, trust relationships (forest, external)
- Certificates (AD CS) — ESC1–ESC13 misconfigurations
- Entra ID — roles, service principals, app roles, MFA status

**Attack path types:**
- Shortest path to Domain Admin
- Kerberoastable / AS-REP roastable users
- DCSync-privileged principals
- AD CS abuse paths (ESC1, ESC2, ESC3, ESC4, ESC8, ESC13)
- Golden Certificate / forged CA paths
- Cross-trust attacks (SIDHistory, trust ticket)

## Manual

### Collector (SharpHound)

```powershell
# Run from compromised Windows host (admin or non-admin)
SharpHound.exe -c All --zipfilename loot

# Specific collections
SharpHound.exe -c Group,Session,ACL,Trusts
SharpHound.exe -c LocalAdmin,RDP,DCOM,PSRemote
SharpHound.exe -c Container --computer OU=Servers,DC=domain,DC=com

# LDAP filter
SharpHound.exe -d domain.local --ldapfilter "(samAccountType=805306368)"
```

### Collector (BloodHound.py — Linux)

```bash
# Python collector — run from Linux
bloodhound-python -d domain.local -u user -p 'Password!' -c All -ns 10.0.0.1

# Specific collections
bloodhound-python -u user -p 'Pass!' -d domain.local -c Session,ACL,Group,Trusts
```

### Browser UI

```bash
# BloodHound CE runs as a single binary with embedded UI
bloodhound serve

# Default: https://localhost:8080
# First-run: register admin account
```

### Cypher Queries (examples)

```cypher
// Shortest path from user to Domain Admin
MATCH p=shortestPath((u:User {samaccountname: "JDOE"})-[*1..]->(g:Group {name: "DOMAIN ADMINS@DOMAIN.LOCAL"})) RETURN p

// Users with most privileges (outbound object control)
MATCH (u:User) RETURN u.name, u.outbound_control_count ORDER BY u.outbound_control_count DESC LIMIT 20

// Kerberoastable users
MATCH (u:User {hasspn:true}) RETURN u.name, u.serviceprincipalnames

// Computers with Unconstrained Delegation
MATCH (c:Computer {unconstraineddelegation:true}) RETURN c.name
```

## Build

```bash
# BloodHound CE
git clone https://github.com/SpecterOps/BloodHound.git
cd BloodHound
npm install
npm run build
```

## Install

### Linux

```bash
# Download from GitHub releases
wget https://github.com/SpecterOps/BloodHound/releases/latest/download/bloodhound-linux-x64.zip
unzip bloodhound-linux-x64.zip
./bloodhound serve
```

### macOS

```bash
brew install --cask bloodhound
# Or download from GitHub releases
```

### Windows

```powershell
# Download from GitHub releases
Invoke-WebRequest -Uri https://github.com/SpecterOps/BloodHound/releases/latest/download/bloodhound-win-x64.zip -OutFile bloodhound.zip
# Extract and run bloodhound.exe
```

### SharpHound (collector, C#)

```powershell
# Download from GitHub releases
Invoke-WebRequest -Uri https://github.com/BloodHoundAD/SharpHound/releases/latest/download/SharpHound.exe -OutFile SharpHound.exe

# Or via Chocolatey
choco install bloodhound
```

## Package

| Manager | Command |
|---------|---------|
| Homebrew (macOS) | `brew install --cask bloodhound` |
| Chocolatey | `choco install bloodhound` |
| Docker | `docker run -p 8080:8080 ghcr.io/specterops/bloodhound-ce:latest` |
| GitHub Releases | https://github.com/SpecterOps/BloodHound/releases |

## Links

| Resource | URL |
|----------|-----|
| GitHub (CE) | https://github.com/SpecterOps/BloodHound |
| Docs | https://support.bloodhoundenterprise.io/ |
| SharpHound releases | https://github.com/BloodHoundAD/SharpHound/releases |
| BloodHound.py | https://github.com/fox-it/BloodHound.py |
| Cypher cheat sheet | https://bloodhound.readthedocs.io/en/latest/ |
| Blog | https://posts.specterops.io/ |
| Attack path docs | https://bloodhound.readthedocs.io/ |
