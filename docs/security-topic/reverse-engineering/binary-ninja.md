# Binary Ninja — Reverse Engineering Platform

## How It Works

Binary Ninja is a modern RE platform by Vector 35 with a Python API-first design, multi-architecture decompiler, and collaborative analysis.

**Key architecture:**
- **Binary View** — abstracted file representation (PE, ELF, Mach-O, raw) with layout-aware parsing
- **Linear sweep + recursive descent** — hybrid disassembler combining FLIRT-like signatures and precise analysis
- **Medium Level IL (MLIL)** — SSA-form intermediate language with type propagation and constant folding
- **High Level IL (HLIL)** — structured control flow (loops, if/else) built from MLIL for decompiler output
- **Python API** — full headless scripting, plugin system, and UI extension via Qt/PySide
- **Collaboration** — shared project format with merge support for team analysis

**Analysis features:**
- Auto-type reconstruction from usage patterns
- Cross-reference (XREF) database with data/code/offset references
- Call graph, control flow graph (CFG), and low-level IL (LLIL) views
- Architecture support: x86/x64, ARM/Thumb/AArch64, MIPS, PPC, 6502, 8051, WASM, and 30+ more
- Native debugger integration (LLDB, GDB, WinDbg)

## Manual

### Launch

```bash
# GUI
binaryninja

# Headless (CLI)
binaryninja -c -execute script.py binary
```

### Common Commands

```python
# Load binary
from binaryninja import *
bv = BinaryViewType.get_view_of_file("sample.exe")

# Functions
for func in bv.functions:
    print(f"{func.name} @ {func.start} — {len(func.basic_blocks)} blocks")

# Decompiled output
for func in bv.functions:
    print(func.hlil if func.hlil else "no decomp")

# Cross-references
xrefs = bv.get_code_refs(addr)
for ref in xrefs:
    print(f"{hex(ref.address)} in {ref.function.name}")

# Patch bytes
bv.write(addr, b"\x90\x90")  # NOP sled
```

### Headless Scripting

```bash
# Run a plugin / script from CLI
binaryninja -c -execute analyze.py target.bin
```

## Build

```bash
# No public source — closed-source commercial product.
# API/plugin development: pip install binaryninja (requires license)
pip install binaryninja
```

## Install

### Linux / macOS / Windows

```bash
# Download from https://binary.ninja/
chmod +x binaryninja_*.sh && ./binaryninja_*.sh

# macOS (Homebrew)
brew install --cask binary-ninja
```

### Docker

```bash
docker pull vector35/binaryninja:latest  # requires license
```

## Package Managers

| Manager | Command |
|---------|---------|
| Homebrew | `brew install --cask binary-ninja` |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://binary.ninja/ |
| Docs | https://docs.binary.ninja/ |
| API reference | https://api.binary.ninja/ |
| GitHub (plugins) | https://github.com/Vector35/snippets |
| Community forum | https://forum.binary.ninja/ |
