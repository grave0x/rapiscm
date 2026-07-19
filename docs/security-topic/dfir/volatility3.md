# Volatility 3 — Memory Forensics Framework

## How It Works

Volatility 3 is the successor to Volatility 2, rewritten in Python 3 with a plugin-based architecture for extracting digital artifacts from RAM dumps.

**Key architecture:**
- **Layer system** — abstracted memory layers (raw, crash dump, HPAK, VMSS, VMDK) with automatic format detection
- **Symbol tables** — pre-compiled JSON symbol tables for Windows/Linux/macOS kernel structures (previously profile-dependent in v2)
- **Plugin framework** — `volatility3.framework.plugins` — each plugin registers requirements (layer, symbol table, address)
- **Tree nodes** — `intermediate_symbol_table`, `object_factory`, `base_types` for kernel object traversal
- **Auto-detection** — OS, architecture, symbol table selection without manual `--profile` flag

**Plugin categories:**
- Process scanning: `windows.pslist`, `windows.psscan`, `windows.pstree`
- Network artifacts: `windows.netscan`, `windows.netstat`
- Code injection: `windows.malfind`, `windows.modscan`, `windows.dlllist`
- Registry: `windows.registry.hivelist`, `windows.registry.printkey`
- Kernel: `windows.modules`, `windows.driverscan`, `windows.callbacks`
- Linux: `linux.pslist`, `linux.bash`, `linux.malfind`, `linux.check_afinfo`
- macOS: `mac.pslist`, `mac.pswalk`, `mac.malfind`

## Manual

### Launch

```bash
python3 vol.py -f memory.dmp windows.pslist
python3 vol.py -f memory.dmp -r json windows.pslist
```

### Common Commands

```bash
# Process listing
python3 vol.py -f memory.dmp windows.pslist

# Process tree (parent-child hierarchy)
python3 vol.py -f memory.dmp windows.pstree

# Suspicious process detection
python3 vol.py -f memory.dmp windows.malfind

# Network connections
python3 vol.py -f memory.dmp windows.netscan

# Loaded DLLs
python3 vol.py -f memory.dmp windows.dlllist --pid 1234

# Command line arguments of processes
python3 vol.py -f memory.dmp windows.cmdline

# Registry hives
python3 vol.py -f memory.dmp windows.registry.hivelist

# Print registry key
python3 vol.py -f memory.dmp windows.registry.printkey --key "Software\Microsoft\Windows\CurrentVersion\Run"

# Dump process memory
python3 vol.py -f memory.dmp windows.dumpfiles --pid 1234

# Scan for hidden processes (direct pool scanning, not linked list)
python3 vol.py -f memory.dmp windows.psscan

# Linux memory analysis
python3 vol.py -f linux.dmp linux.pslist
python3 vol.py -f linux.dmp linux.bash
python3 vol.py -f linux.dmp linux.check_modules  # rootkit detection
```

### Output Formats

```bash
# Default — text table
# -r json — JSON lines
# -r csv — CSV
# -r renderer — custom renderer
python3 vol.py -f memory.dmp -r json windows.pslist
```

## Build

```bash
git clone https://github.com/volatilityfoundation/volatility3.git
cd volatility3
pip install -e .
# Download symbol tables
python3 vol.py --symbol-dir ./symbols  # auto-downloads on first use
```

## Install

```bash
# Option 1 — pip
pip install volatility3

# Option 2 — git
git clone https://github.com/volatilityfoundation/volatility3.git
cd volatility3 && pip install -e .

# Docker
docker pull volatilityfoundation/volatility:latest
docker run --rm -v $PWD:/data volatilityfoundation/volatility \
  -f /data/memory.dmp windows.pslist
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/volatilityfoundation/volatility3 |
| Docs | https://volatility3.readthedocs.io/ |
| Plugin development | https://volatility3.readthedocs.io/en/latest/plugin-development.html |
| Symbol tables | https://github.com/volatilityfoundation/volatility3/tree/develop/volatility3/symbols |
