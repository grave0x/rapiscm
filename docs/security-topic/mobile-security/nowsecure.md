# NowSecure — Mobile App Security Testing Platform

Enterprise mobile app security testing platform. Automated static + dynamic analysis for Android and iOS. CI/CD integration, compliance mapping, and real-device dynamic testing.

## How It Works

NowSecure runs automated analysis pipelines against mobile app binaries. Static analysis inspects the decompiled code for vulnerabilities, hardcoded secrets, and cryptographic weaknesses. Dynamic analysis runs the app on a real device or emulator, monitoring network traffic, file system changes, and runtime behavior.

**Assessment types:**

| Type | What It Covers |
|------|----------------|
| **Static** | Binary analysis, manifest/profile inspection, hardcoded secrets, weak crypto, library scanning |
| **Dynamic** | Runtime analysis on device, encrypted traffic inspection, file system monitoring, IPC testing |
| **Interactive** | Manual hybrid testing combining static context with live device interaction |
| **API** | Backend API testing for mobile-specific endpoints and auth flaws |
| **Supply Chain** | Third-party library vulnerability scanning, SDK behavior analysis |

## Manual

```bash
# NowSecure Platform CLI
# Upload and start assessment
nowsecure upload app.apk
nowsecure upload --platform ios app.ipa

# Check assessment status
nowsecure status <assessment-id>

# Download report
nowsecure report <assessment-id> --format pdf
nowsecure report <assessment-id> --format json

# List assessments
nowsecure list

# Start dynamic analysis
nowsecure start-dynamic <assessment-id>
```

### CI/CD Integration (GitHub Actions)

```yaml
- name: NowSecure Scan
  uses: nowsecure/nowsecure-action@v1
  with:
    api_key: ${{ secrets.NOWSECURE_API_KEY }}
    artifact_path: app/build/outputs/apk/release/app-release.apk
    platform: android
    wait-for-completion: true
```

## Install

NowSecure is a SaaS platform. No local installation required for the platform, but a CLI tool is available.

```bash
# CLI installation (npm)
npm install -g @nowsecure/cli

# Or download binary from NowSecure Platform
nowsecure login
nowsecure configure --api-key <API_KEY>
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.nowsecure.com/ |
| Platform | https://lab.nowsecure.com/ |
| Docs | https://docs.nowsecure.com/ |
| CLI Reference | https://docs.nowsecure.com/auto-sdk/cli-reference/ |
| GitHub Action | https://github.com/marketplace/actions/nowsecure-scan |
| Blog | https://www.nowsecure.com/blog/ |
| OWASP MASVS mapping | https://docs.nowsecure.com/auto-sdk/masvs-mapping/ |
