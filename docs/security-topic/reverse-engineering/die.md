# Detect It Easy (DIE) — File Type & Packer Detector

## How It Works

DIE detects packers, compilers, protectors, cryptors, and file formats by matching binary signatures against a community-maintained database of detection rules.

**Key architecture:**

- **Signature database** — YAML-based `.signature` files organized by format/compiler/packer. Each signature defines byte patterns, offsets, and entropy checks.
- **Scanning engine:**
  1. Identify file format via magic bytes (PE, ELF, Mach-O, MS-DOS, etc.)
  2. Load signatures matching that format/architecture
  3. Scan sections, imports, resources, overlays against signature patterns
  4. Score matches by confidence
  5. Return detected toolchains with version hints
- **Entropy scanning** — per-section Shannon entropy visualization for encrypted/compressed data detection
- **Hash database** — optional MD5/SHA1/SHA256 lookup against known packer signatures

**Detection capability:**
- Packer: UPX, Themida, VMProtect, ASPack, Enigma, MPress, PECompact
- Obfuscator: ConfuserEx, .NET Reactor, SmartAssembly, Eazfuscator
- Compiler: Visual Studio, GCC, MinGW, Delphi, Go, Rust, Nim
- Protector: VMP, Enigma, EXECryptor, Armadillo, Obsidium
- Installers: InnoSetup, NSIS, Wise, InstallShield
- Documents: PDF, OLE2, Office Open XML, RTF

## Manual

### Launch

```bash
# GUI
die  # or die.exe on Windows

# CLI
diec sample.exe
diec sample.elf
```

### CLI Commands

```bash
# Basic detection
diec target.exe

# JSON output (for scripting)
diec --json target.exe

# Scan all files in directory recursively
diec -r ./samples/

# Deep scan (slower, more thorough)
diec --deep target.exe

# Show entropy for each section
diec --entropy target.exe
```

### GUI Features

```bash
# Drag & drop file onto DIE window
# Tab 1: Detect (packer/compiler signatures)
# Tab 2: Entropy (section-level entropy graph)
# Tab 3: Hex (binary hex editor with signature highlighting)
# Tab 4: Strings (configurable min length, encoding filter)
```

### Headless / Scripting

```bash
# JSON output for automation
diec --json -r ./samples/ > results.json

# Pipe to jq for filtering
diec --json sample.exe | jq '.detects[].name'
```

## Build

```bash
git clone https://github.com/horsicq/Detect-It-Easy.git
cd Detect-It-Easy
mkdir build && cd build
cmake ..
make
sudo make install
```

## Install

### Linux

```bash
# Download AppImage from GitHub releases
wget https://github.com/horsicq/Detect-It-Easy/releases/latest/download/die_linux_x64.AppImage
chmod +x die_linux_x64.AppImage
./die_linux_x64.AppImage

# Or build from source
```

### Windows

```bash
# Download portable zip or installer from GitHub releases
# die_win64_portable.zip → extract and run
```

### macOS

```bash
# Download dmg from GitHub releases
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/horsicq/Detect-It-Easy |
| Releases | https://github.com/horsicq/Detect-It-Easy/releases |
| Signature database | https://github.com/horsicq/Detect-It-Easy/tree/master/db |
| Wiki | https://github.com/horsicq/Detect-It-Easy/wiki |
