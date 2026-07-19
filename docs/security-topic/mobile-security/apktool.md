# apktool — Android APK Reverse Engineering Tool

Reverse engineer Android APK files — decode resources to nearly original form, rebuild modified APK, and work with smali bytecode. Maintained by Connor Tumbleson & Ryszard Wiśniewski.

## How It Works

apktool reverses the Android binary XML and resource compilation process. It decodes `resources.arsc`, `AndroidManifest.xml`, and other binary XML files back to human-readable text. It also disassembles DEX to smali (human-readable Dalvik bytecode) for patching, then reassembles and re-signs.

**Key operations:**
- `d` (decode) — unpack APK to smali + decoded resources
- `b` (build) — rebuild from decoded directory back to APK
- `if` (install framework) — install framework APK for proper decoding

## Manual

```bash
# Decode APK
apktool d app.apk -o app-decompiled

# Decode with no resource decoding (smali only, faster)
apktool d app.apk -o app-decompiled --no-res

# Rebuild modified APK
apktool b app-decompiled -o app-modified.apk

# Install framework (for system apps)
apktool if framework-res.apk

# Decode with debug info
apktool d app.apk -o app-decompiled --debug
```

### Typical Workflow

```bash
# 1. Decode
apktool d app.apk -o app-decompiled

# 2. Modify smali or resources
vim app-decompiled/smali/com/example/MainActivity.smali

# 3. Rebuild
apktool b app-decompiled -o app-modified.apk

# 4. Sign (need Java keytool + jarsigner)
keytool -genkey -v -keystore debug.keystore -alias debug -keyalg RSA -keysize 2048 -validity 10000
jarsigner -sigalg SHA1withRSA -digestalg SHA1 -keystore debug.keystore app-modified.apk debug
```

## Build

```bash
git clone https://github.com/iBotPeaches/Apktool.git
cd Apktool
./gradlew build
# Artifact: brut.apktool/apktool-cli/build/libs/apktool-cli.jar
```

## Install

```bash
# Linux/macOS (manual download)
wget https://raw.githubusercontent.com/iBotPeaches/Apktool/master/scripts/linux/apktool
wget https://github.com/iBotPeaches/Apktool/releases/latest/download/apktool_2.9.3.jar
chmod +x apktool
mv apktool apktool_2.9.3.jar /usr/local/bin/

# macOS
brew install apktool

# Windows
# Download apktool.bat and apktool.jar from GitHub releases
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/iBotPeaches/Apktool |
| Docs | https://apktool.org/docs/ |
| Releases | https://github.com/iBotPeaches/Apktool/releases |
