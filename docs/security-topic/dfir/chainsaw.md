# Chainsaw — Rapid EVTX Hunting

## How It Works

Chainsaw is a Rust-based Windows event log hunting tool that applies Sigma rules across EVTX files with millisecond performance. Built for speed — hunts millions of events in seconds.

**Key architecture:**
- **Sigma engine** — full Sigma rule support (logsource, detection, condition, fields). No external dependency.
- **EVTX parser** — custom Rust parser for Windows EVTX format (fast, memory-mapped)
- **Mapping engine** — maps field names across sources (EVTX, Sysmon, PowerShell, etc.)
- **JSON rules** — lightweight, simplified rule format (`chainsaw-rules`) for quick creation
- **Output** — terminal table (colorized), JSON, CSV, HTML, Chromium

**Rule types:**
- **Sigma** — full Sigma 1.0 rules (YAML)
- **Chainsaw rules** — simplified JSON rules for quick hunting
- **Hunter** — specialized sigma handlers for known attack techniques

## Manual

### Launch

```bash
# Hunt directory of EVTX files with Sigma rules
chainsaw hunt /path/to/evtx/ -r /path/to/sigma/rules/ --mapping mapping.yml

# JSON output
chainsaw hunt C:\logs\ -r .\sigma\rules\ -j > results.json

# CSV output
chainsaw hunt C:\logs\ -r .\sigma\rules\ --csv > results.csv

# HTML report
chainsaw hunt C:\logs\ -r .\sigma\rules\ -o report.html
```

### Common Commands

```bash
# Search for specific event IDs
chainsaw search C:\logs\ -e 4625,4624  # logon failures/success

# Search with keyword
chainsaw search C:\logs\ -k "net.exe"

# View sigma rule coverage
chainsaw coverage --sigma-dir /rules/

# List all available hunts
chainsaw lists
```

### Pre-built Hunts

```bash
# Chainsaw includes pre-built hunt packs:
chainsaw hunt C:\logs\ -r .\chainsaw_rules\ -m .\mappings\sigma\ --full
```

## Build

```bash
git clone https://github.com/WithSecureLabs/chainsaw.git
cd chainsaw
cargo build --release
# Artifact: target/release/chainsaw
```

## Install

```bash
# Option 1 — pre-built binary
# Download from github.com/WithSecureLabs/chainsaw/releases
# Windows/Linux/macOS binaries available

# Option 2 — cargo
cargo install chainsaw

# Option 3 — KAPE
# Included as a KAPE module
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/WithSecureLabs/chainsaw |
| Release page | https://github.com/WithSecureLabs/chainsaw/releases |
| Blog intro | https://www.withsecure.com/en/expertise/blog/chainsaw-rapidly-search-and-hunt-through-windows-event-logs |
