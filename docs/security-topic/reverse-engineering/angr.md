# angr — Binary Analysis Platform

## How It Works

angr is a Python framework for binary analysis combining symbolic execution, concolic analysis, control-flow recovery, and binary lifting into a unified platform.

**Key architecture:**
- **CLE (CLE Loads Everything)** — binary loader: PE, ELF, Mach-O, raw; resolves imports, relocations, TLS
- **PyVEX** — lifts binary code to VEX IR (Valgrind's intermediate representation) for architecture-agnostic analysis
- **Solver engine (Claripy)** — wraps Z3, Bitwuzla, and CVC5 SMT solvers for path constraint solving
- **CFG recovery** — `CFGFast` (fast, heuristic) and `CFGEmulated` (precise, dynamic) for control flow graphs
- **SimuVEX** — emulated execution states with symbolic registers/memory, system call models (SimProcedures)
- **Exploration** — path explosion management with pruning, prioritization, and depth limits

**Capabilities:**
- Automatic exploit generation (AEG) for simple buffer overflows
- Symbolic input solving: find input that reaches target address / avoids bad path
- Deobfuscation: VM-based obfuscator analysis, constant unfolding
- Dynamic symbolic execution (DSE) for path coverage
- Binary patching and hooking at instruction level
- ROP gadget discovery and constraint solving

## Manual

### Basic Usage

```python
import angr

# Load binary
proj = angr.Project("sample")

# CFG
cfg = proj.analyses.CFGFast()
for func in cfg.functions.values():
    print(f"{func.name} @ {hex(func.addr)}")

# Find path to target address
state = proj.factory.entry_state()
simgr = proj.factory.simulation_manager(state)
target = 0x401234
simgr.explore(find=target)
if simgr.found:
    found_state = simgr.found[0]
    return found_state.solver.eval(found_state.posix.fds[0].read_from(0, 100))
```

### Symbolic Execution

```python
# Symbolic stdin
import claripy
flag_len = 32
flag = claripy.BVS("flag", flag_len * 8)
state = proj.factory.entry_state(stdin=angr.SimFile("/dev/stdin", content=flag))

simgr = proj.factory.simulation_manager(state)
simgr.explore(find=success_addr, avoid=fail_addr)
if simgr.found[0]:
    solution = simgr.found[0].solver.eval(flag, cast_to=bytes)
```

### Hooking Functions

```python
# Replace libc function with SimProcedure
proj.hook_symbol("puts", angr.SIM_PROCEDURES["libc"]["puts"]())

# Custom hook at address
@proj.hook(0x400123)
def my_hook(state):
    print("hit hook at 0x400123")
    state.regs.rip = 0x400456  # skip to next
```

### Decompilation

```python
# angr-management GUI (optional)
# pip install angr-management
# angr-management binary.elf
```

## Build

```bash
git clone https://github.com/angr/angr.git
cd angr
pip install -e .
# Also install optional components:
pip install -e angr-management  # GUI
pip install -e angrop           # ROP gadget finder
```

## Install

```bash
# Production install
pip install angr

# With optional GUI
pip install angr angr-management

# Docker
docker pull angr/angr
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/angr/angr |
| Docs | https://docs.angr.io/ |
| API reference | https://api.angr.io/ |
| Examples | https://github.com/angr/angr-doc/tree/master/examples |
| Community | https://angr.slack.com/ |
| angr-management GUI | https://docs.angr.io/angr-management |
