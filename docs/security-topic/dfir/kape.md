# KAPE — Kroll Artifact Parser and Extractor

## How It Works

KAPE is a triage-first forensic collection tool that rapidly gathers targeted artifacts from live Windows systems and processes them in-place. It is designed for speed — collecting critical evidence in minutes.

**Key architecture:**
- **Targets** — `.tkape` files defining what to collect (file paths, registry keys, event logs). Modular YAML definitions.
- **Modules** — `.mkape` files defining what to process (parse EVTX, dump registry, run EZ Tools, run Hayabusa/Chainsaw). Pluggable.
- **Binary** — single portable executable. No installation, no dependencies.
- **Batch collections** — compound collector files (`.ckape`) running targets + modules + output in sequence.

**Collection workflow:**
1. Define target scope (EVTX, MFT, Prefetch, Registry, Amcache, SRUM, Shimcache, USN Journal, etc.)
2. KAPE copies files from source to destination (preserving timestamps, attributes)
3. Module processing parses collected artifacts into CSV/JSON/timeline formats
4. Output ready for analysis in Timeline Explorer, Velociraptor, Timesketch, or SIEM

## Manual

### Launch

```bash
# GUI mode
.\kape.exe

# CLI collection
.\kape.exe --tsource C: --tdest E:\case001 --target !SANS_Triage

# CLI collection + processing
.\kape.exe --tsource C: --tdest E:\case001 --target !SANS_Triage \
           --mdest E:\case001\processed --module !EZTools

# List available targets
.\kape.exe --tlist

# List available modules
.\kape.exe --mlist
```

### Common Targets

```
!SANS_Triage          — Standard SANS collection (EVTX, Prefetch, Registry, Amcache, SRUM)
!Rapid_Collection     — Minimal set for speed (EVTX, Prefetch, Registry hives)
!!Full_MFT            — Full $MFT and $J USN journal
!!Registry_All        — All registry hives + transaction logs
COMMS_Evidence        — Browser history, emails, chat logs
FileSystem_Artifacts  — LNK files, Jump Lists, shellbags, Jumplists
```

### Common Modules

```
!EZTools              — Runs MFTECmd, EvtxECmd, PECmd, RECmd, JLECmd, LECmd, SBECmd, etc.
!Hayabusa             — Generate event log timeline
!Chainsaw             — Hunt EVTX with Sigma rules
!Plaso                — Generate super-timeline from collected artifacts
```

### Batch Collection (.ckape)

```yaml
name: Rapid IR Collection
description: Standard triage collection + processing
targets:
  - '!SANS_Triage'
  - '!!Registry_All'
modules:
  - '!EZTools'
  - '!Hayabusa'
```

## Build

Not open source. Binary distribution only.

## Install

```bash
# Download from kape.app (Eric Zimmerman's tools)
# Extract to any directory
# Run KAPE.exe — no install needed

# Target/Module updates via GUI: File → Check for Updates
# Or download Targets.zip/Modules.zip from kape.app
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.kape.app/ |
| Targets/Modules | https://www.kape.app/targets |
| Documentation | https://www.kape.app/documentation |
| Eric Zimmerman site | https://ericzimmerman.github.io/ |
