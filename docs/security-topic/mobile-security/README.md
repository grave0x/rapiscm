# Mobile Security

Tools and techniques for assessing the security of Android and iOS mobile applications — static analysis, dynamic instrumentation, traffic interception, malware classification, and privacy compliance.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| Static Analysis | Decompile APK/IPA; inspect manifest, permissions, secrets, crypto, exported components |
| Dynamic Analysis | Run in rooted/jailbroken environment; monitor runtime behavior, file I/O, crypto calls |
| Runtime Instrumentation | Inject hooks for function tracing, argument dumping, bypassing protections |
| Certificate Pinning Bypass | Intercept HTTPS traffic by disabling pinning via Frida/Corellium |
| Traffic Interception | Proxy through Burp Suite/mitmproxy; capture API calls and data flows |
| Malware Classification | Pattern-match C2 callbacks, SMS interception, overlay attacks, accessibility abuse |
| Obfuscation Detection | Identify ProGuard/DexGuard/OLLVM; deobfuscate |
| Privacy Compliance | Check permissions, data collection, tracking SDKs, GDPR/CCPA |
| MDM/MAM Assessment | Enterprise app management security posture |
| Binary Analysis | Mach-O/ELF/DEX; check stack canaries, ARC, PIE, RELRO |

## Methods

- **Static analysis** — decompile APK/IPA to Java/Kotlin/Swift/ObjC source; inspect AndroidManifest.xml/Info.plist; search for hardcoded secrets, weak crypto, insecure SSL, exported components
- **Dynamic analysis** — run app in rooted emulator or jailbroken device; Frida hooks on crypto, file, network APIs; detect runtime permission abuse
- **Traffic interception** — proxy through Burp/mitmproxy; Frida script to bypass certificate pinning; capture all API requests/responses
- **Malware detection** — YARA signatures; Quark Engine behavior scoring; identify C2 callbacks, SMS stealer, overlay phishing
- **Frida scripting** — inject JavaScript; trace functions, dump arguments, modify return values, bypass root/jailbreak detection

## Tool Comparison

| Tool | Category | Platform | Key Strength |
|------|----------|----------|--------------|
| **MobSF** | All-in-one | Android, iOS, Windows | Static + dynamic + Frida + MASVS mapping |
| **apktool** | Reverse Engineering | Android | Decode/repackage APK resources |
| **jadx** | Decompiler | Android | Dex → Java source, GUI + CLI |
| **Frida** | Instrumentation | Android, iOS, Win, macOS, Linux | Cross-platform dynamic hooks |
| **Objection** | Runtime Exploration | Android, iOS | Frida-powered patch, FS explore, pinning bypass |
| **Drozer** | Assessment | Android | IPC, component testing, intent fuzzing |
| **mitmproxy** | Proxy | All platforms | TLS proxy, Python scripting |
| **Quark Engine** | Malware Analysis | Android | Behavior scoring, rule matching |
| **nowsecure** | CI/CD | Android, iOS | SAST + DAST + API in pipeline |
