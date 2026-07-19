# Capstone / Keystone / Unicorn — Disassembly, Assembly, Emulation Trinity

## How It Works

Three libraries from the same team providing the core building blocks for binary analysis tooling: **Capstone** (disassembly), **Keystone** (assembly), **Unicorn** (CPU emulation). Together they cover the full RE toolchain.

**Capstone — Disassembly Engine:**

Architecture support: x86/x64 (incl. AVX/AVX-512), ARM/Thumb/AArch64, MIPS, PPC, SPARC, SystemZ, XCore, M68K, TMS320C64x, M680X, EVM, WASM, BPF, RISC-V, SH, TriCore.

- **Lightweight** — single C header, no external deps
- **Detailed instruction info** — opcode, operands (reg/imm/mem), implicit registers read/written, groups, R/W flags
- **Customizable** — syntax (Intel/AT&T), mode (16/32/64-bit), endianness
- **Bindings** — Python, Ruby, Java, Go, Rust, Node, C#, PowerShell, OCaml

**Keystone — Assembler Engine:**

Mirror of Capstone for assembly. Same architecture coverage. Converts textual assembly to raw bytes.

**Unicorn — CPU Emulator:**

QEMU-based lightweight CPU emulation with:
- Memory hooking (read/write/exec per region)
- Code execution tracing (single-step, basic block)
- System call hooking (platform-specific)
- Snapshot/restore for state exploration
- No OS boot required — raw binary execution

## Manual

### Python Examples

```python
# Capstone: disassemble x64 shellcode
from capstone import *
md = Cs(CS_ARCH_X86, CS_MODE_64)
for insn in md.disasm(b"\x48\x31\xc0\x48\xff\xc0", 0):
    print(f"0x{insn.address:x}: {insn.mnemonic} {insn.op_str}")

# Capstone: ARM Thumb
md = Cs(CS_ARCH_ARM, CS_MODE_THUMB)
md.detail = True  # enable operand access info
```

```python
# Keystone: assemble x64
from keystone import *
ks = Ks(KS_ARCH_X86, KS_MODE_64)
encoding, count = ks.asm("mov rax, 0x1234; ret")
print(encoding)  # list of bytes
```

```python
# Unicorn: emulate shellcode
from unicorn import *
from unicorn.x86_const import *

mu = Uc(UC_ARCH_X86, UC_MODE_64)
ADDR = 0x1000000
mu.mem_map(ADDR, 4 * 1024 * 1024)
mu.mem_write(ADDR, code)
mu.reg_write(UC_X86_REG_RSP, ADDR + 0x100000)
mu.emu_start(ADDR, ADDR + len(code))
rax = mu.reg_read(UC_X86_REG_RAX)
```

## Build

### Capstone

```bash
git clone https://github.com/capstone-engine/capstone.git
cd capstone
mkdir build && cd build && cmake .. && make
sudo make install
```

### Keystone

```bash
git clone https://github.com/keystone-engine/keystone.git
cd keystone
mkdir build && cd build && cmake .. && make
sudo make install
```

### Unicorn

```bash
git clone https://github.com/unicorn-engine/unicorn.git
cd unicorn
mkdir build && cd build && cmake .. && make
sudo make install
```

## Install

### Python (pip)

```bash
pip install capstone keystone-engine unicorn
```

### Linux

```bash
# Debian/Ubuntu
sudo apt install libcapstone-dev libkeystone-dev libunicorn-dev
pip install capstone keystone-engine unicorn
```

### macOS

```bash
brew install capstone keystone unicorn
```

### Windows

```bash
pip install capstone keystone-engine unicorn
# Or download pre-built DLLs from GitHub releases
```

## Links

| Resource | URL |
|----------|-----|
| Capstone GitHub | https://github.com/capstone-engine/capstone |
| Capstone docs | https://www.capstone-engine.org/documentation.html |
| Keystone GitHub | https://github.com/keystone-engine/keystone |
| Unicorn GitHub | https://github.com/unicorn-engine/unicorn |
| Unicorn docs | https://www.unicorn-engine.org/docs/ |
| Python bindings | https://www.capstone-engine.org/lang_bindings.html |
