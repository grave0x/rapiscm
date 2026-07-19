# Frida — Dynamic Instrumentation Toolkit

Cross-platform dynamic instrumentation toolkit. Inject JavaScript into running processes on Android, iOS, Windows, macOS, and Linux. Used for security testing, reverse engineering, and runtime analysis.

## How It Works

Frida injects a small JavaScript engine (Duktape or V8) into target processes. It allows hooking functions at runtime — intercept calls, read/modify arguments, change return values, and call functions. Communication happens via a local or remote protocol (frida-server on mobile, frida-core on desktop).

**Key components:**
- **frida** — CLI for injecting scripts, listing processes, tracing
- **frida-server** — daemon running on Android/iOS device
- **frida-tools** — Python CLI utilities (frida-ps, frida-trace, frida-ls-devices)
- **frida-gadget** — embed Frida into unmodified apps (for jailbroken/rooted devices)

## Manual

```bash
# List processes on USB device
frida-ps -U

# List USB devices
frida-ls-devices

# Trace specific function
frida-trace -U -i "open" -i "read" com.example.app

# Spawn app and inject script
frida -U -f com.example.app -l script.js

# Attach to running process
frida -U com.example.app -l script.js
```

### Frida Script Example

```javascript
// Hook into a method
Java.perform(function() {
  var MainActivity = Java.use('com.example.MainActivity');
  MainActivity.secretMethod.implementation = function() {
    console.log('secretMethod called');
    return 'hooked!';
  };
});

// Hook ObjC (iOS)
if (ObjC.available) {
  var NSUserDefaults = ObjC.classes.NSUserDefaults;
  var alloc = NSUserDefaults.alloc();
  var instance = alloc.init();
  console.log(instance.synchronize());
}
```

### Frida Server on Android

```bash
# Push server to device
adb push frida-server-16.0.0-android-arm64 /data/local/tmp/frida-server
adb shell chmod 755 /data/local/tmp/frida-server

# Run as root
adb shell su -c /data/local/tmp/frida-server

# Port forwarding for USB
adb forward tcp:27042 tcp:27042
```

## Build

```bash
git clone --recurse-submodules https://github.com/frida/frida.git
cd frida
make build
# Artifact: build/frida-*
# Build requirements: Python 3, Node.js, Meson, Ninja
```

## Install

```bash
# Install frida-tools (Python)
pip install frida-tools

# Frida-server binary download
# Get from https://github.com/frida/frida/releases
# Select architecture: frida-server-*-android-arm64.xz / frida-server-*-ios-arm64.xz

# macOS
brew install frida

# Pre-built packages at https://github.com/frida/frida/releases
```

## Package

- PyPI: `frida-tools`
- GitHub releases: frida-server (mobile), frida-core (desktop)
- npm: `frida` (JS bindings)

## Links

| Resource | URL |
|----------|-----|
| Official site | https://frida.re/ |
| GitHub | https://github.com/frida/frida |
| Docs | https://frida.re/docs/ |
| JS API docs | https://frida.re/docs/javascript-api/ |
| Frida CodeShare | https://codeshare.frida.re/ |
