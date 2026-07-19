# Drozer — Android Security Assessment Framework

Android app and device security assessment framework. Discover attack surface via IPC, exported components, and content providers. By MWR Labs (now F-Secure).

## How It Works

Drozer deploys an agent APK on the target device/emulator and connects via USB/network. The agent exposes the Android IPC layer — intents, activities, services, content providers, broadcast receivers — for systematic security testing from a workstation.

**Attack surface modules:**

| Module | Targets |
|--------|---------|
| `app.activity.forintent` | Activities matching intent filters |
| `app.activity.start` | Launch activities without permissions |
| `app.service.info` | Exported service enumeration |
| `app.service.send` | Direct service communication |
| `app.broadcast.info` | Broadcast receiver enumeration |
| `app.broadcast.send` | Send crafted broadcasts |
| `app.content.provider.query` | Content provider SQL injection |
| `app.provider.finduri` | Provider URI discovery |
| `scanner.provider.finduri` | Automated URL path discovery |

## Manual

```bash
# Start session
drozer console connect

# Basic enumeration
dz> run app.package.list -f browser
dz> run app.package.info -a com.example.app

# Activity enumeration
dz> run app.activity.info -a com.example.app

# Start exported activity
dz> run app.activity.start --component com.example.app com.example.app.SettingsActivity

# Content provider discovery
dz> run app.provider.finduri com.example.app

# Query content provider
dz> run app.provider.query content://com.example.provider/users/

# SQL injection on provider
dz> run app.provider.query content://com.example.provider/users/ --selection "1=1"

# Broadcast injection
dz> run app.broadcast.send --action com.example.ACTION_SECRET --extra string msg "pwned"
```

### Agent Deployment

```bash
# Install agent on emulator/device
adb install drozer-agent-2.4.4.apk

# Port forward
adb forward tcp:31415 tcp:31415

# Start agent (on device) and connect
drozer console connect
```

## Build

```bash
git clone https://github.com/WithSecureLabs/drozer.git
cd drozer
make apk   # Build the agent APK
make deb   # Build Debian package
```

## Install

```bash
# pip
pip install drozer

# Debian/Ubuntu
wget https://github.com/WithSecureLabs/drozer/releases/download/2.4.4/drozer_2.4.4.deb
sudo dpkg -i drozer_2.4.4.deb

# Agent APK from GitHub releases
# https://github.com/WithSecureLabs/drozer/releases
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/WithSecureLabs/drozer |
| Wiki | https://github.com/WithSecureLabs/drozer/wiki |
| Agent APK | https://github.com/WithSecureLabs/drozer/releases |
| Tutorial | https://resources.infosecinstitute.com/topic/drozer-tutorial/ |
