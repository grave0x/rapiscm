# Velociraptor — Endpoint Visibility and Collection

## How It Works

Velociraptor is a Velocidex open-source tool for endpoint monitoring, DFIR triage, and threat hunting. It uses VQL (Velociraptor Query Language) for flexible, live data collection across thousands of endpoints.

**Key architecture:**
- **Server** — single binary (server + GUI). Manages agents, collects artifacts, serves API.
- **Client (agent)** — Go-based, lightweight, cross-platform. Deployed on endpoints, connects to server via gRPC.
- **VQL** — declarative query language over endpoint data (filesystem, registry, processes, event logs, memory, YARA scans)
- **Artifacts** — pre-built VQL query packs organized by OS and use case (triage, hunting, persistence, forensics)
- **Hunts** — distribute artifact collection to a fleet of endpoints simultaneously
- **Flows** — per-client collection jobs with live progress tracking

**Collection methods:**
- Client-side VQL execution — agent runs queries locally, returns results
- Server-side artifacts — schedule recurring collections across fleet
- Event monitoring — real-time or near-real-time endpoint event streaming
- Shell access — interactive or scripted remote PowerShell/bash via VQL

## Manual

### Launch

```bash
# Server (interactive setup)
velociraptor config generate -i
velociraptor --config server.yaml gui

# Client deploy
velociraptor --config client.config.yaml client
```

### Common VQL Queries

```sql
-- List processes
SELECT Pid, Name, Exe, CommandLine FROM pslist()

-- Network connections
SELECT Pid, Family, Type, Laddr, Raddr, Status FROM netstat()

-- YARA scan directory
SELECT * FROM yara(
    rules='rule Suspicious { strings: $a = "malware" condition: $a }',
    files=glob(globs='''C:\Users\*\AppData\Roaming\*\*''')
)

-- Registry Run keys
SELECT * FROM read_reg_key(
    globs=['''HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\Run\*''']
)

-- Event log hunting (Sigma)
SELECT * FROM foreach(
    row={
        SELECT * FROM wmi_event_log(
            query="SELECT * FROM Application WHERE EventID=4663"
        )
    },
    query={
        SELECT * FROM scope()
    }
)
```

### Pre-built Artifacts

```bash
# Search artifacts
velociraptor artifacts list | grep -i triage

# Common artifacts:
Windows.Triage.KapeFiles          — mirrors KAPE collection
Windows.Forensics.EVTX            — Windows Event Log extraction
Windows.Forensics.Prefetch        — Prefetch file parsing
Windows.Forensics.RegistryMount   — offline registry hive mounting
Windows.Detection.YARA            — YARA scanning
Windows.Hunting.FileFinder        — file search with regex
Windows.System.Pslist             — process listing
Linux.Triage.CollectFiles         — Linux triage collection
```

## Build

```bash
git clone https://github.com/Velocidex/velociraptor.git
cd velociraptor
go build -o velociraptor .
```

## Install

```bash
# Linux server
wget https://github.com/Velocidex/velociraptor/releases/latest/download/velociraptor-linux-amd64
chmod +x velociraptor-linux-amd64
sudo ./velociraptor-linux-amd64 config generate -i

# Windows client (MSI)
# Download Velociraptor.msi from releases
# Deploy via GPO/SCCM/Intune

# Docker server
docker pull velocidex/velociraptor:latest
docker run -p 8889:8889 -p 8000:8000 velocidex/velociraptor
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.velociraptor.app/ |
| GitHub | https://github.com/Velocidex/velociraptor |
| Docs | https://docs.velociraptor.app/ |
| VQL Reference | https://docs.velociraptor.app/vql_reference/ |
| Artifact Exchange | https://docs.velociraptor.app/exchange/ |
