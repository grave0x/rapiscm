# JADX — Dex to Java Decompiler

## How It Works

JADX is a command-line + GUI DEX/APK decompiler that produces readable Java source code from Android applications.

**Key architecture:**
- **DEX input** — `.dex`, `.apk`, `.jar`, `.aar`, `.class`, `.zip` containing DEX
- **DEX → Java decompilation pipeline:**
  1. Parse DEX into AST (dexlib2 fork)
  2. Convert dalvik instructions to Jimple-like intermediate representation (IR)
  3. Apply passes: type inference, exception handler cleanup, switch recovery, inlining
  4. Generate Java source with mapped variable names (renamed or original)
- **APK unpacking** — automatic ZIP extraction, AndroidManifest.xml binary XML decoding (AXML)
- **Resources** — decoded resources via AAPT2 (res/*) + binary XML in `resources/`
- **Gradle/AGP integration** — direct import of `.apk`/`.aab` from build outputs

**Analysis features:**
- Full source tree output (packages → files)
- Cross-reference search (find usage/usages across codebase)
- Deobfuscation with mapping file (ProGuard/R8 `mapping.txt`)
- Bytecode view alongside decompiled Java
- Smali intermediate output option

## Manual

### Launch

```bash
# GUI
jadx-gui target.apk

# CLI
jadx target.apk
```

### Common Commands

```bash
# Basic decompilation
jadx app.apk -d output_dir

# Show decompilation progress
jadx --show-bad-code app.apk

# Deobfuscate with ProGuard mapping
jadx --deobf --deobf-mapping mapping.txt app.apk

# Export only source (no resources)
jadx --no-res app.apk

# Export Gradle project structure
jadx --output-dir src --export-gradle app.apk

# Use fewer threads (for low-RAM environments)
jadx -j 2 app.apk

# Skip classes and resources
jadx --skip-classes --skip-resources app.apk
```

### GUI Features

```bash
# Key shortcuts
# Ctrl+N — search class/method/field (fuzzy)
# Ctrl+Shift+F — search text across all sources
# Ctrl+E — go to Eclipse outline
# Ctrl+G — go to line
# Double-click identifier — jump to declaration
```

## Build

```bash
git clone https://github.com/skylot/jadx.git
cd jadx
./gradlew dist
# Artifact: build/jadx-<version>.zip
```

## Install

### Linux / macOS / Windows

```bash
# Download from GitHub releases
wget https://github.com/skylot/jadx/releases/download/v1.5.1/jadx-1.5.1.zip
unzip jadx-*.zip
./bin/jadx  # or jadx-gui
```

### Package Managers

```bash
# Homebrew
brew install jadx

# Scoop
scoop install jadx

# Snap
snap install jadx
```

### Docker

```bash
docker pull skylot/jadx:latest
docker run --rm -v $PWD:/data skylot/jadx /data/app.apk
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/skylot/jadx |
| Releases | https://github.com/skylot/jadx/releases |
| Wiki | https://github.com/skylot/jadx/wiki |
