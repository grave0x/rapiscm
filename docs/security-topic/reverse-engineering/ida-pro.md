# IDA Pro — Interactive Disassembler

## How It Works

IDA Pro is the industry-standard disassembler and debugger. Its Hex-Rays decompiler produces C-like pseudocode from binaries across 50+ architectures.

**Key components:**
- **Disassembly engine** — auto-analyzes entry points, library function recognition (FLIRT), stack variables, switch tables
- **Hex-Rays decompiler** — licensed separately, converts assembly to typed C pseudocode
- **IDB database** — persistent analysis state with comments, names, types, cross-references
- **IDC / IDAPython** — two scripting APIs for automation and custom analysis
- **Debugger** — local/remote/Android/WinDbg/GDB backends, attach to processes, step through code
- **FLIRT** — Fast Library Identification and Recognition Technology for known function signatures
- **F.L.I.R.T.** — decompiler-level library recognition for standard libraries

**Analysis workflow:**
1. Auto-analysis: entry point → section parsing → function detection → FLIRT matching → tail call detection
2. Manual refinement: rename variables, set types, add comments, define structs
3. Decompilation: F5 in any function to view C pseudocode
4. Cross-reference navigation: XREF graph for data/code flows

## Manual

### Launch

```bash
# GUI
ida64   # 64-bit binary
ida     # 32-bit binary

# Text-only (batch)
idal64 -A -Llog.txt -P+ -Sauto.py malware.exe

# Remote debugger server
# ida_android_server runs on device; connect from IDA via Debugger → Process options
```

### Common Operations

- `F5` — decompile current function (Hex-Rays)
- `N` — rename symbol/label
- `` ` — comment
- `D/W/D/Q` — toggle data size byte/word/dword/qword
- `C` — mark as code
- `Alt+M` — add bookmark
- `Ctrl+X` — cross-reference list
- `G` — jump to address
- `Shift+F12` — strings window
- `View → Open subviews → Signatures` — apply FLIRT sigs

### Scripting (IDAPython)

```python
import idautils
import idc

for func_addr in idautils.Functions():
    name = idc.get_func_name(func_addr)
    if "Decrypt" in name or "decrypt" in name:
        print(f"[+] Found {name} @ 0x{func_addr:x}")

# Get decompiled output
import ida_hexrays
func = idaapi.get_func(func_addr)
cfunc = ida_hexrays.decompile(func)
print(str(cfunc))
```

## Build

Closed source. Not buildable from source. Hex-Rays distributes binaries.

## Install

### Linux / macOS / Windows

```bash
# Download installer from hex-rays.com
chmod +x ida-pro-<version>.run
./ida-pro-<version>.run  # GUI installer
```

Permanent license or named/float subscription required. Hex-Rays decompiler is a separate purchase.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://hex-rays.com/ |
| IDAPython docs | https://hex-rays.com/products/ida/support/idapython_docs/ |
| Community wiki | https://www.hex-rays.com/products/ida/support/ |
| Hex-Rays blog | https://hex-rays.com/blog/ |
