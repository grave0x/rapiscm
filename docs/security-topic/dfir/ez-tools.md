# Eric Zimmerman Tools (EZ Tools) — Windows Forensic Utilities

## How It Works

Eric Zimmerman's open-source forensic tools parse specific Windows artifacts with high precision. Each tool targets a single artifact type and outputs structured CSV/JSON for Timeline Explorer or SIEM ingestion.

**Key tools:**

| Tool | Artifact | Description |
|------|----------|-------------|
| **KAPE** | Collection | Kroll Artifact Parser and Extractor — targeted collection + parsing of forensic artifacts |
| **MFTECmd** | $MFT | Parse Master File Table ($MFT), $Boot, $J, $LogFile, $SDS |
| **PECmd** | Prefetch | Parse Windows Prefetch files (.pf) for program execution evidence |
| **LECmd** | LNK | Parse Windows shortcut (.lnk) files |
| **JLECmd** | Jumplists | Parse Windows Jump Lists (automatic destinations) |
| **SBECmd** | Shellbags | Parse Shellbag registry data (folder view settings) |
| **RECmd** | Registry | Batch registry hive querying with .REB rule files |
| **EvtxECmd** | Event Logs | Parse EVTX with event maps, reverse lookup, message template rendering |
| **AmcacheParser** | Amcache | Parse Amcache.hve for program execution evidence |
| **AppCompatParser** | ShimCache | Parse AppCompat cache (ShimCache, $DriveLetterAmCache) |
| **bstrings** | Strings | Parallelized forensic string extraction |

## Manual

### KAPE

```bash
# Collection
kape --tsource C: --tdest D:\output --tflush \
  --target !SANS_TRIAGE --gui 0

# Collection + parsing (dual mode)
kape --tsource C: --tdest D:\output \
  --target !SANS_TRIAGE \
  --module !EZ_TRIAGE \
  --gui 0
```

### MFTECmd

```bash
MFTECmd.exe -f "C:\$MFT" --csv "output.csv"
MFTECmd.exe -f "C:\$MFT" --json "output.json"
MFTECmd.exe -f "C:\$LogFile" --csv "logfile.csv"
MFTECmd.exe -f "C:\$Boot" --csv "boot.csv"  # parse NTFS boot sector
```

### PECmd

```bash
PECmd.exe -f "C:\Windows\Prefetch\CMD.EXE-*.pf"
PECmd.exe -d "C:\Windows\Prefetch" --csv "output.csv"
PECmd.exe -d "C:\Windows\Prefetch" --json "output.json"
```

### LECmd / JLECmd

```bash
LECmd.exe -f "target.lnk" --csv "output.csv"
JLECmd.exe -d "C:\Users\%user%\AppData\Roaming\Microsoft\Windows\Recent" --csv "output.csv"
```

### RECmd

```bash
# Batch registry processing with rules
RECmd.exe -d "C:\Users\%user%\NTUSER.DAT" \
  --re "C:\rules\my_rules.reb" \
  --csv "output.csv"

# Built-in rule sets for common forensic queries
RECmd.exe --nl  # list available rules
```

## Build

```bash
# Closed-source — pre-compiled binaries only from GitHub
# Tools written in .NET (C#)
```

## Install

```bash
# Download from GitHub: https://ericzimmerman.github.io/
# All tools in one download: Get-ZimmermanTools PowerShell script

# PowerShell (automated download)
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
Set-ExecutionPolicy RemoteSigned -Force
iex ((New-Object System.Net.WebClient).DownloadString('https://raw.githubusercontent.com/EricZimmerman/Get-ZimmermanTools/master/Get-ZimmermanTools.ps1'))
Get-ZimmermanTools -Dest "C:\tools\EZTools"

# Or manual download per tool
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://ericzimmerman.github.io/ |
| GitHub | https://github.com/EricZimmerman |
| KAPE | https://www.kroll.com/en/services/cyber-risk/incident-response-litigation-support/kape |
| Timeline Explorer | https://ericzimmerman.github.io/#!index.md |
| Get-ZimmermanTools | https://github.com/EricZimmerman/Get-ZimmermanTools |
