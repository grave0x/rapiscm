# BinDiff — Binary Diffing Tool

## How It Works

BinDiff (formerly zynamics BinDiff, now Google-owned) compares two binaries at the function and basic-block level to identify similarities and differences — critical for patch analysis, malware variant comparison, and porting reverse engineering work across versions.

**Key architecture:**

- **IDA Pro plugin** — primary integration; also supports Ghidra (via `ghidra_bindiff`) and radare2/rizin
- **Matching algorithm** — multiple passes, each propagating confidence from the previous:
  1. **Primary match** — identical function bytes (hash), identical flow graphs (MD index)
  2. **Secondary match** — call graph isomorphism, topological sorting
  3. **Tertiary match** — instruction mnemonics histogram, register flow, stack frame size
  4. **Quaternary match** — address-based (thin — only when binaries are nearly identical)
- **Confidence scoring** — each match gets a confidence value (0.0–1.0); unconfident matches flagged
- **Diff view** — side-by-side assembly with colored basic blocks (green=matched, yellow=modified, red=new, grey=deleted)

**Use cases:**
- Patch diffing: what changed in a security update (Microsoft Patch Tuesday, Android security bulletin)
- Malware evolution: how a sample changed between versions (new C2, obfuscation, evasion)
- Code reuse identification: shared libraries, copy-paste code
- Porting comments/labels from old binary to new binary

## Manual

### Launch

```bash
# Standalone BinDiff GUI
bindiff

# From IDA Pro
# File → BinDiff → Open Diff
```

### Common Commands

```bash
# CLI diff (no GUI)
bindiff primary.exe secondary.exe --output-dir ./diff_output

# Export IDB/i64 to BinDiff database
idat.exe -OBindiffAutoGenerate:1 -A primary.exe
idat.exe -OBindiffAutoGenerate:1 -A secondary.exe

# Then diff the .BinExport files
bindiff primary.BinExport secondary.BinExport --output diff.BinDiff
```

### Ghidra Integration

```bash
# Install ghidra_bindiff plugin
# From BinDiff install: support/ghidra_bindiff/
# Copy to Ghidra's Extensions directory
# Then: File → BinDiff → Export / Diff
```

### Workflow

```bash
# Typical patch analysis workflow:
# 1. Export both binaries (unpatched / patched) to BinExport format
# 2. Run BinDiff → review matched, modified, new, deleted functions
# 3. Focus on:
#    - New functions (added features/patches)
#    - Modified with high confidence (actual fixes/backdoors)
#    - Deleted functions (removed functionality)
# 4. Port comments/labels from old IDB to new IDB
```

## Build

```bash
# Not open source — binary-only release from Google
# Download from https://cloud.google.com/software/signup/bindiff
```

## Install

### Linux / Windows

```bash
# Download installer from cloud.google.com/bindiff
chmod +x BinDiff_<version>_linux.sh && ./BinDiff_<version>_linux.sh

# Windows: run MSI installer
```

### IDA Pro Plugin

```bash
# Automatic during BinDiff install on Windows
# Manual on Linux: copy plugins/ directory to IDA's plugin dir
cp -r /opt/bindiff/plugins/* ~/.idapro/plugins/
```

## Links

| Resource | URL |
|----------|-----|
| Google Cloud page | https://cloud.google.com/software-signup/bindiff |
| BinDiff wiki | https://github.com/google/bindiff/wiki |
| Ghidra integration | https://github.com/google/bindiff/blob/master/ghidra_bindiff/README.md |
| zynamics (archived) | https://zynamics.com/bindiff.html |
