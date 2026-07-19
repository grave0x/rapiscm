# Frida — Dynamic Instrumentation Toolkit

## How It Works

Frida injects a JavaScript engine (QuickJS/V8) into running processes, enabling runtime hooking, tracing, and modification of function behavior across platforms.

**Key architecture:**
- **Agent** — JavaScript/TypeScript code injected into target process
- **Frida Core** — C library that handles injection, memory management, communication
- **frida-server** — daemon running on device (Android/iOS/Linux/Windows/macOS)
- **frida-gadget** — embedded dynamic library for app-bundled instrumentation
- **frida-tools** — CLI utilities: frida, frida-ps, frida-trace, frida-discover

**Instrumentation methods:**
- `Interceptor.attach()` — hook function entry/exit, modify args/return values
- `Interceptor.replace()` — replace entire function implementation
- `Stalker` — thread-level code tracing (every instruction)
- `Memory.readByteArray()` / `Memory.writeByteArray()` — arbitrary memory access
- `Process.enumerateModules()` / `Process.enumerateThreads()` — runtime introspection
- `Socket` / `File` APIs — I/O interception

## Manual

### Launch

```bash
# Attach to running process
frida -p <PID> -l script.js

# Spawn and instrument
frida -f com.android.app -l script.js --no-pause

# USB device (Android/iOS)
frida -U -f com.android.app -l script.js

# frida-server on remote device
frida -H 192.168.1.10:27042 -l script.js
```

### Common Commands

```bash
# List processes on device
frida-ps -U

# Trace API calls
frida-trace -U -i "recv*" com.android.app
frida-trace -U -i "*crypto*" com.android.app

# Discover running libraries
frida-discover -p <PID>
```

### Script Example

```javascript
// Hook a Java method (Android)
Java.perform(() => {
    const cls = Java.use('com.example.Crypto');
    cls.decrypt.implementation = function(data) {
        console.log('[+] decrypt called with:', data);
        const result = this.decrypt(data);
        console.log('[+] result:', result);
        return result;
    };
});

// Hook a native function
const decrypt = Module.findExportByName('libcrypto.so', 'AES_decrypt');
Interceptor.attach(decrypt, {
    onEnter(args) {
        console.log('[+] AES_decrypt called');
    },
    onLeave(retval) {
        console.log('[+] AES_decrypt returned');
    }
});
```

## Build

```bash
git clone --recurse-submodules https://github.com/frida/frida.git
cd frida
make  # builds for all platforms, takes significant time/resources
```

## Install

```bash
# Via pip
pip install frida-tools

# Download frida-server from github.com/frida/frida/releases
# Android (ARM64)
adb push frida-server-<version>-android-arm64 /data/local/tmp/
adb shell chmod 755 /data/local/tmp/frida-server-<version>-android-arm64
adb shell /data/local/tmp/frida-server-<version>-android-arm64 &

# iOS (jailbroken)
scp frida-server-<version>-ios-arm64 root@device:/tmp/
ssh root@device /tmp/frida-server-<version>-ios-arm64 &

# macOS
brew install frida

# Windows
# Download frida-gadget-<version>-win64.dll / frida-server from releases
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://frida.re/ |
| GitHub | https://github.com/frida/frida |
| Docs | https://frida.re/docs/home/ |
| JS API reference | https://frida.re/docs/javascript-api/ |
| Frida code samples | https://github.com/frida/frida-snippets |
