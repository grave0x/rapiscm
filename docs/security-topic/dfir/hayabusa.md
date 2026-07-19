# Hayabusa — Windows Event Log Forensic Tool

## How It Works

Hayabusa is a Rust-based Windows event log (EVTX) timeline generator for fast forensic triage. Uses Sigma rules with a custom rule format for detection speed.

**Key architecture:**

- **EVTX parsing** — native Rust EVTX parser (no dependencies on Windows libraries)
- **Rule engine** — YAML-based detection rules (Hayabusa format, compatible with Sigma subset)
  - Selection-based matching (event ID, channel, provider, specific fields)
  - Keyword matching across message strings
  - Multi-condition rules (AND/OR with field mappings)
- **Timeline generation** — outputs CSV/JSONL timelines of matched events with: timestamp, event ID, rule name, severity (critical/high/medium/low/informational), MITRE ATT&CK mapping
- **Pivot queries** — generate compact pivot tables (counts by host, user, event ID)

**Detection coverage:**
- 2000+ built-in detection rules covering: process injection, credential dumping, lateral movement (WMI, PSExec, DCOM), log tampering, service persistence, scheduled task creation, account manipulation
- MITRE ATT&CK technique tagging per rule

**Performance:**
- Single EVTX file: <1 second per 10 MB
- Batch processing: multi-threaded across cores
- Memory-mapped I/O for minimal RAM overhead

## Manual

### Launch

```bash
# Create timeline from EVTX files
hayabusa csv-timeline -d /path/to/evtx/ -o timeline.csv

# JSON output
hayabusa json-timeline -d /path/to/evtx/ -o timeline.jsonl

# Scan single file
hayabusa csv-timeline -f sample.evtx
```

### Common Commands

```bash
# Quick timeline (level threshold)
hayabusa csv-timeline -d ./evtx/ -o timeline.csv -L high

# With MITRE ATT&CK mapping
hayabusa csv-timeline -d ./evtx/ -o timeline.csv -M

# Pivot tables
hayabusa pivot-tables -d ./evtx/
# Produces: pivot_user.csv, pivot_computer.csv, pivot_eventid.csv

# Update rules
hayabusa update-rules

# List rules matching specific technique
hayabusa list-rules -t T1055.012  # Process Hollowing

# Summary statistics
hayabusa logon-summary -d ./evtx/
hayabusa computer-summary -d ./evtx/
```

### Output Filters

```bash
# Filter by rule severity
hayabusa csv-timeline -d ./evtx/ -l critical,high

# Filter by event ID
hayabusa csv-timeline -d ./evtx/ -e 4624,4625,4634  # logon events

# Exclude informational rules
hayabusa csv-timeline -d ./evtx/ -L medium
```

## Build

```bash
git clone https://github.com/Yamato-Security/hayabusa.git
cd hayabusa
cargo build --release
# Artifact: target/release/hayabusa

# Or from crates.io
cargo install hayabusa
```

## Install

### Linux / macOS / Windows

```bash
# Download binary from GitHub releases
wget https://github.com/Yamato-Security/hayabusa/releases/latest/download/hayabusa-x64-linux.zip
unzip hayabusa-x64-linux.zip
./hayabusa

# Or with package managers (limited)
# Homebrew (macOS/Linux)
brew tap yamato-security/tap
brew install hayabusa
```

### Docker

```bash
docker pull ghcr.io/yamato-security/hayabusa:latest
docker run --rm -v $PWD/evtx:/data ghcr.io/yamato-security/hayabusa \
  csv-timeline -d /data -o /data/timeline.csv -L high
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/Yamato-Security/hayabusa |
| Docs | https://hayabusa.readthedocs.io/ |
| Rule set | https://github.com/Yamato-Security/hayabusa-rules |
| Sigma conversion | https://github.com/Yamato-Security/sigma-to-hayabusa-converter |
| Community | https://discord.gg/hayabusa |
