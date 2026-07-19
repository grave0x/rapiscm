# Objection — Runtime Mobile Exploration

Frida-powered runtime mobile exploration toolkit. Patch, explore filesystem, bypass certificate pinning, dump keychain, and more on Android and iOS.

## How It Works

Objection wraps Frida into an easy-to-use CLI. It automatically patches APK/IPA binaries with Frida Gadget for dynamic instrumentation without needing root/jailbreak (on device with the patched app). Commands are organized by category: `android`, `ios`, `env`, `storage`, `keystore`, and plain `frida` passthrough.

**Key features:**
- Auto-patch APK/IPA with Frida Gadget
- File system exploration
- Certificate pinning bypass (Android TrustManager, iOS NSURLSession)
- Keychain/Keystore dump
- SQLite database querying
- NSUserDefaults/SharedPreferences browsing
- Screenshot disabling
- Root/jailbreak detection bypass
- Method hooking by search

## Manual

```bash
# Patch APK with Frida Gadget
objection patchapk -s app.apk

# Patch IPA
objection patchipa --source app.ipa

# Explore patched app (USB device)
objection explore -g com.example.app

# Explore on device by PID
objection explore -P 12345
```

### Explore Commands

```
# Android environment
android hooking list classes
android hooking search classes ApiClient
android hooking list class_methods com.example.ApiClient
android hooking set method_return com.example.ApiClient.getToken "fake-token"

# iOS environment
ios hooking list classes
ios hooking search classes Keychain
ios hooking set return_value SSLPinningManager verifyServerTrust true

# Storage
android keystore list
android keystore dump --all

# Certificate pinning bypass
android sslpinning disable

# File system
env
ls /data/data/com.example.app/
cat /data/data/com.example.app/shared_prefs/config.xml
```

### Non-Patched Mode (with separate frida-server)

```bash
objection explore -h 127.0.0.1 -p 27042 -g com.example.app
```

## Build

```bash
git clone https://github.com/sensepost/objection.git
cd objection
pip install -e .
```

## Install

```bash
pip install objection
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/sensepost/objection |
| Wiki | https://github.com/sensepost/objection/wiki |
| Frida integration | https://frida.re/ |
