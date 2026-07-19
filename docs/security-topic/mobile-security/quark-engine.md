# Quark Engine — Android Malware Analysis Engine

Android malware scoring system. Decomposes APK behavior into crime activities and rates risk using weighted rules. By Quark-Engine team (Trend Micro).

## How It Works

Quark Engine analyzes Android APK behavior by chaining API calls into **crime activities** — sequences of methods that together constitute suspicious behavior (e.g., getDeviceId + HttpURLConnection.connect = data exfiltration). Each crime activity is scored by confidence, and results roll up into a risk graph.

**Analysis layers:**

| Layer | What It Detects |
|-------|-----------------|
| **Permission mapping** | Maps API calls to required permissions; flags permission-to-call mismatches |
| **Crime activity detection** | Rules for known malicious behavior chains (exfil, root, dynamic loading, obfuscation) |
| **Weighted scoring** | Each rule contributes to a total risk score; configurable thresholds |
| **MITRE ATT&CK mapping** | Crime activities mapped to mobile ATT&CK techniques |
| **OVAL** | OWASP MASVS level validation |

## Manual

```bash
# CLI analysis
quark -a app.apk -s

# Output formats
quark -a app.apk -s -o result.json
quark -a app.apk -s -o result.html --format html

# Specify custom rules
quark -a app.apk -r custom_rules/ -s

# Summary only
quark -a app.apk -s --summary

# List available rules
quark --list-rules

# Integration with other tools output
quark -a app.apk -o report.json
# Then feed to MobSF or other platforms
```

### Python API

```python
from quark.engine import QuarkEngine

engine = QuarkEngine("app.apk")
engine.analyze()
report = engine.get_report()

for crime in report.crime_activities:
    print(f"[{crime.score}] {crime.name} — {crime.description}")
    for call in crime.api_calls:
        print(f"  {call.class_name}.{call.method_name}")
```

## Build

```bash
git clone https://github.com/quark-engine/quark-engine.git
cd quark-engine
pip install -e .
```

## Install

```bash
# pip
pip install quark-engine
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/quark-engine/quark-engine |
| Docs | https://quark-engine.readthedocs.io/ |
| Rules | https://github.com/quark-engine/quark-rules |
| PyPI | https://pypi.org/project/quark-engine/ |
| OVAL (MASVS mapping) | https://github.com/quark-engine/oval-gui |
