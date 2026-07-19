# jadx — Dex to Java Decompiler

DEX/APK decompiler that produces readable Java source code from Dalvik bytecode. GUI and CLI tools. Maintained by Skylot.

## How It Works

jadx parses DEX bytecode and reconstructs Java source using multiple decompilation passes — type inference, control flow analysis, exception handling recovery, and generic type resolution. Supports APK, DEX, AAR, AAB, and JAR formats.

**Key features:**
- Full Java source reconstruction from DEX
- Smali fallback view for decompilation failures
- Resource viewer (AndroidManifest, strings, layouts)
- Search by class/method/field/string
- Export to Gradle project for IDE import
- Deobfuscation support (ProGuard/DexGuard mapping)

## Manual

```bash
# GUI (launches window)
jadx-gui app.apk

# CLI decompilation
jadx app.apk -d output-directory

# Show decompilation progress
jadx app.apk -d output --show-bad-code

# Decompile with deobfuscation
jadx app.apk -d output --deobf

# Export as Gradle project
jadx app.apk -d output --export-gradle

# Skip resources (code only)
jadx app.apk -d output --skip-resources

# Thread count
jadx app.apk -d output -j 8
```

### Search CLI

```bash
# Search by class name
jadx app.apk --search "ApiClient"

# Search with regex
jadx app.apk --search-regex "secret.*key"
```

## Build

```bash
git clone https://github.com/skylot/jadx.git
cd jadx
./gradlew dist
# Artifacts in build/jadx/
```

## Install

```bash
# Binary download
wget https://github.com/skylot/jadx/releases/latest/download/jadx-1.5.1.zip
unzip jadx-1.5.1.zip -d jadx
export PATH="$PWD/jadx/bin:$PATH"

# macOS
brew install jadx

# Docker
docker pull skylot/jadx
docker run --rm -v $(pwd):/data skylot/jadx jadx /data/app.apk -d /data/output

# Snap
snap install jadx
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/skylot/jadx |
| Releases | https://github.com/skylot/jadx/releases |
| Wiki | https://github.com/skylot/jadx/wiki |
