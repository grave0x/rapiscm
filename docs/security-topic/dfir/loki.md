# Loki — Host-Based IOC Scanner

## How It Works

Loki (by Florian Roth / Nextron Systems) is a free host-based IOC scanner that detects malware, suspicious indicators, and threat actor artifacts using YARA rules, hash lists, and filename/path patterns.

**Key architecture:**

- **Detection methods:**
  1. **YARA rules** — compile and scan process memory, file contents, and MBR
  2. **Hash lookup** — MD5/SHA1/SHA256 against known malicious hashes (in iocs/ directory)
  3. **Filename/path matching** — regex against known malicious filenames and directory patterns
  4. **Registry key matching** — scan for known malicious autoruns, service keys, and event log names
  5. **C2 backconnect check** — test live processes for connections to known C2 IPs
  6. **Process memory strings** — extract and match strings from running processes
  7. **Sysinternals Autoruns integration** — optional scanning of all autostart extensibility points

- **Rule sets:**
  - `iocs/` — YARA rules and hash lists updated via `loki --update`
  - `signatures/` — custom Python signature modules (advanced detection logic)
  - `config/` — database whitelist, exclusions, proxy settings

**Performance considerations:**
- YARA scanning on process memory is CPU-intensive — use `--onlyrelevant` to skip signed Microsoft processes
- Default scan: file system + processes + registry + services
- Scan time: 5-30 minutes depending on endpoint size

## Manual

### Launch

```bash
# Basic scan
loki.exe --update
loki.exe --noindel   # skip INDEL (Indicator Delete) — run first
loki.exe             # full scan after ensuring no false positives
```

### Common Commands

```bash
# Update signatures
loki.exe --update

# Quick scan (processes + memory only, skip full filesystem)
loki.exe --quick

# Scan specific directory path
loki.exe -p C:\Users\%user%\AppData

# Exclude paths
loki.exe -x C:\Windows\System32

# Only relevant (skip signed Microsoft processes — much faster)
loki.exe --onlyrelevant

# Deep scan (all files + process memory + MBR)
loki.exe --all

# CSV output for SIEM ingestion
loki.exe --csv -o D:\logs\loki_results.csv

# Alert log (syslog compatible)
loki.exe --syslog
```

### Configuration

```ini
# loki.cfg (optional)
[Loki]
whitelist_path = C:\tools\loki\config\whitelist.txt
max_filesize = 268435456  ; max 256 MB per file
scan_alternate_datastreams = 0  ; skip NTFS ADS by default (slow)
allow_config_auto_override = 1
```

### Post-Processing

```bash
# Convert .nman (Notepad++ markdown) report to readable summary
# Loki outputs: loki-report-YYMMDD-HHMMSS.nman
# Also produces CSV if --csv flag was used
```

## Build

```bash
# Closed-source (freeware) — binary download only
# Written primarily in Python + compiled to EXE with PyInstaller
```

## Install

```bash
# Download from GitHub releases
wget https://github.com/Neo23x0/Loki/releases/latest/download/loki.zip
unzip loki.zip -d C:\tools\loki

# Python version (alternative)
git clone https://github.com/Neo23x0/Loki.git
cd Loki
pip install -r requirements.txt
python loki.py --update
python loki.py

# Requires: Python 3.x, YARA + yara-python
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/Neo23x0/Loki |
| Releases | https://github.com/Neo23x0/Loki/releases |
| IOC database | https://github.com/Neo23x0/Loki/tree/master/iocs |
| YARA rules | https://github.com/Neo23x0/signature-base |
| Florian Roth | https://twitter.com/cyb3rops |
