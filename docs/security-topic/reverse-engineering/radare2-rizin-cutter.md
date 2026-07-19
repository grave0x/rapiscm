# radare2 / Rizin / Cutter — Reverse Engineering Framework

## How It Works

radare2 is a portable reversing framework with disassembly, debugging, patching, scripting, and forensics capabilities. Rizin is a community fork. Cutter provides a Qt GUI backend for Rizin.

**Key architecture:**
- **Core** — analysis engine with ESIL (Evaluable Strings Intermediate Language) for emulation
- **RAsm** — assembler/disassembler across 50+ architectures
- **RBin** — binary format parser (PE, ELF, Mach-O, DEX, class, etc.)
- **RCore** — command interface, visualization, script engine
- **RHash** — hash calculation engine
- **Custom scripting** — r2pipe for Python/Node/Rust/Go/C bindings
- **Rizin** — fork with cleaner codebase, modernized plugin system, Cutter GUI included

**Analysis workflow:**
1. `rizin ./binary` → automatic analysis (`aaa` / `aaaa`)
2. Navigate with `s` (seek), `V` (visual mode), PDF (`p`, `pd`)
3. Rename (`afn`), annotate (`CC`), define structs (`t`)
4. Debug with `dc` (continue), `ds` (step), `dr` (registers)
5. ESIL emulation for symbolic execution / code path coverage

## Manual

### Launch

```bash
# radare2
r2 ./binary
r2 -d ./binary        # debug mode
r2 -

# Rizin
rizin ./binary
rizin -d ./binary

# Cutter GUI
cutter ./binary
```

### Common Commands

```bash
# Basic analysis
aaa                 # auto analysis (functions, XREFs)
aaaa                # more thorough analysis
afl                 # list functions
afl~...             # filter function list (grep)

# Navigation
s main              # seek to main
s 0x401000           # seek to address
s-                  # undo seek

# Disassembly
pdf                 # disassemble current function
pdc                 # decompile to pseudo-C
V                   # visual mode (q to quit, p to cycle views)

# Data types
t struct_name      # define structure
afn new_name        # rename function
CC comment text     # add comment

# Debugger
dc                  # continue execution
ds                  # step instruction
dr                  # print registers
dm                  # memory map

# ESIL emulation
aei                 # init ESIL VM
aesu 0x401000       # step until address
```

### Scripting (r2pipe)

```python
import r2pipe
r2 = r2pipe.open("/bin/ls")
r2.cmd("aaa")
functions = r2.cmd("afl").splitlines()
print(f"Functions: {len(functions)}")
r2.quit()
```

## Build

```bash
# radare2
git clone https://github.com/radareorg/radare2.git
cd radare2 && ./sys/install.sh

# Rizin
git clone https://github.com/rizinorg/rizin.git
cd rizin && meson setup build && ninja -C build && ninja -C build install
```

## Install

### radare2

```bash
git clone https://github.com/radareorg/radare2.git && cd radare2 && ./sys/install.sh
# or
brew install radare2
# or
apt install radare2
```

### Rizin + Cutter

```bash
# Homebrew
brew install rizin
brew install --cask cutter

# Ubuntu
apt install rizin

# Docker
docker pull ghcr.io/rizinorg/rizin
docker pull ghcr.io/rizinorg/cutter

# Windows
# Download installer from github.com/rizinorg/cutter/releases
```

## Links

| Resource | URL |
|----------|-----|
| radare2 GitHub | https://github.com/radareorg/radare2 |
| radare2 book | https://book.rada.re/ |
| Rizin site | https://rizin.re/ |
| Rizin GitHub | https://github.com/rizinorg/rizin |
| Cutter | https://cutter.re/ |
| r2pipe bindings | https://github.com/radareorg/radare2-r2pipe |
