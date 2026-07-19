# MobSF — Mobile Security Framework

Automated mobile application security testing framework. Static + dynamic analysis for Android, iOS, and Windows apps. Frida integration, malware detection, OWASP MASVS mapping.

## How It Works

MobSF ingests APK, IPA, or APPX files and performs:
- **Static analysis** — decompile; inspect manifest, permissions, hardcoded secrets, weak crypto, exported components, obfuscation
- **Dynamic analysis** — run app on Android emulator or iOS device; Frida-based runtime hooking; traffic capture; file system monitoring
- **API testing** — test mobile backend APIs via integrated REST client
- **Malware scanning** — YARA rules, Quark Engine integration, domain/IP reputation
- **MASVS mapping** — map findings to OWASP Mobile Application Security Verification Standard

## Manual

```bash
# Run MobSF Docker container
docker run -it --rm -p 8000:8000 opensecurity/mobile-security-framework-mobsf:latest

# Access web UI at http://localhost:8000
# Upload APK/IPA/APPX for analysis

# CLI analysis
docker run -it --rm -v $(pwd):/home/mobsf/MobSF/opensecurity/mobile-security-framework-mobsf:latest \
  mobsf -a /home/mobsf/MobSF/app.apk -o /home/mobsf/MobSF/report.json
```

### REST API

```bash
# Upload and scan
curl -F "file=@app.apk" http://localhost:8000/api/v1/upload

# View JSON report
curl http://localhost:8000/api/v1/report_json?hash=<HASH>
```

### CI/CD Integration

```yaml
- name: MobSF Scan
  uses: MobSF/mobsf-action@v1
  with:
    apk_path: app/build/outputs/apk/release/app-release.apk
```

## Build

```bash
git clone https://github.com/MobSF/Mobile-Security-Framework-MobSF.git
cd Mobile-Security-Framework-MobSF
./setup.sh
# Run with: ./run.sh
```

## Install

```bash
# Docker (recommended)
docker pull opensecurity/mobile-security-framework-mobsf:latest

# Manual (Python 3.10+)
git clone https://github.com/MobSF/Mobile-Security-Framework-MobSF.git
cd Mobile-Security-Framework-MobSF
python3 -m venv venv
source venv/bin/activate
pip install --no-cache-dir -r requirements.txt
./setup.sh
./run.sh
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/MobSF/Mobile-Security-Framework-MobSF |
| Docs | https://mobsf.github.io/docs/ |
| Docker images | https://hub.docker.com/r/opensecurity/mobile-security-framework-mobsf |
| MASVS | https://mas.owasp.org/ |
