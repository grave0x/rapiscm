# Ghidra — NSA Reverse Engineering Framework

## How It Works

Ghidra is a Java-based RE framework with a native decompiler, 50+ processor families, multi-user collaboration server, and extensible plugin system.

**Key components:**
- **Decompiler** — translates assembly to structured C-like pseudocode (SLEIGH processor specs)
- **Project Manager** — organizes binaries, scripts, bookmarks, annotations
- **Listing** — disassembly view with XREFs, operand highlighting, inline comments
- **Symbol Tree** — imports/exports, functions, labels, namespaces
- **Data Type Manager** — structures, unions, enums, function signatures
- **Script Manager** — Python 3 / Jython scripting API for automation
- **Collaboration Server** — multi-analyst concurrent editing on same project
- **Version Tracking** — automated binary diffing and patch analysis across versions

**Detection methods:**
- Auto-analysis: entry-point discovery, function signature recognition (FLIRT-like), stack variable detection, switch statement recovery
- Decompiler parameter ID: calling convention analysis, argument/return type inference
- PDB/DWARF symbol integration for source-level debugging info
- Byte pattern matching for known library functions

## Manual

### Launch

```bash
# Linux
./ghidraRun

# Headless (CLI, CI)
./analyzeHeadless /path/to/project ProjectName -import /path/to/binary -postScript AnalysisScript.java

# Docker
docker pull ghcr.io/borjafdezgauna/ghidra:latest
```

### Common Commands

```bash
# Headless analysis (no GUI, suitable for batch processing)
analyzeHeadless /tmp/ghidra_proj AutoAnalysis \
  -import sample.exe \
  -postPostProcessP perl_script.pl

# Script runner
# File → Run Script (Python 3/Java)
# Key bindings: R = rename, ; = comment, C = create function, L = set label, F = follow XREF
```

### Scripting (Python 3)

```python
# Get the current program
program = getCurrentProgram()
listing = program.getListing()

# Iterate over functions
for func in listing.getFunctions(True):
    monitor = getMonitor()
    if monitor.isCancelled():
        break
    body = func.getBody()
    print(f"{func.getName()} @ {body.getMinAddress()} — {body.getNumAddresses()} bytes")

# Get decompilation
from ghidra.app.decompiler import DecompInterface
ifc = DecompInterface()
ifc.openProgram(program)
decomp = ifc.decompileFunction(func, 30, monitor)
print(decomp.getDecompiledFunction().getC())
```

## Build

```bash
# From source (Java 17+, Gradle)
git clone https://github.com/NationalSecurityAgency/ghidra.git
cd ghidra
./gradlew buildGhidra
# Artifact: build/dist/ghidra_<version>.zip
```

## Install

### Linux / macOS / Windows

```bash
# Unzip pre-built release from github.com/NationalSecurityAgency/ghidra/releases
unzip ghidra_<version>.zip
./ghidraRun  # Java 17+ required
```

### Docker

```bash
docker pull ghidra/ghidra:latest  # community image
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/NationalSecurityAgency/ghidra |
| Docs | https://ghidra.re/online-courses/ |
| Decompiler paper | https://ghidra.re/papers/ |
| Script examples | https://github.com/NationalSecurityAgency/ghidra/tree/master/Ghidra/Features/Decompiler/ghidra_scripts |
| Plugin gallery | https://github.com/NationalSecurityAgency/ghidra/tree/master/Ghidra/Extensions |
