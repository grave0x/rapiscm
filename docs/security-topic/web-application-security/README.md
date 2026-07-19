# Web Application Security

DAST, SAST, SCA, WAF, API security, injection testing, CMS scanning, and TLS assessment tools for comprehensive web application security testing.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| **DAST (Dynamic Analysis)** | Automated web scanning — crawling, fuzzing, vulnerability detection against running apps |
| **SAST (Static Analysis)** | Source/binary code scanning — pattern matching, taint tracking, control-flow analysis |
| **SCA (Software Composition)** | Dependency scanning — CVE lookup, license compliance, transitive vulnerability detection |
| **WAF (Web Application Firewall)** | HTTP traffic inspection, CRS rules, bot detection, rate limiting, virtual patching |
| **API Security** | REST/GraphQL/gRPC endpoint discovery, schema validation, auth testing, rate-limit bypass |
| **SQL Injection (SQLi)** | Error-based, union, blind boolean, time-based, second-order, NoSQL injection |
| **XSS (Cross-Site Scripting)** | Reflected, stored, DOM-based, mXSS, XSS via CSP bypass |
| **SSRF** | Blind SSRF, cloud metadata access, internal port scanning, DNS rebinding, protocol smuggling |
| **HTTP Request Smuggling** | CL.TE, TE.CL, TE.TE, H2.CL, H2.TE, CL.0, WebSocket smuggling, cache poisoning |
| **SSTI (Server-Side Template Injection)** | Jinja2/Twig/Freemarker/Velocity/etc. blind detection, engine fingerprinting, RCE escalation |
| **XXE (XML External Entity)** | In-band, blind/out-of-band, error-based XXE, SSRF via XXE, XInclude |
| **Authentication Flaws** | JWT none/alg confusion, weak session tokens, credential stuffing, password reset poisoning |
| **Access Control** | IDOR/BOLA, BFLA, mass assignment, privilege escalation, role/scope bypass |
| **CORS / CSP Bypass** | Origin reflection, wildcard origin, ACAO: null bypass, CSP policy eval bypass |
| **CMS Security** | WordPress/Joomla/Drupal — version fingerprint, plugin scanning, credential brute-force, config disclosure |

## Methods

1. **Reconnaissance** — Subdomain enumeration, URL discovery, CMS fingerprint, technology stack ID, API endpoint discovery
2. **Scanning** — Crawl + audit, parameter fuzzing, form auto-fill, authenticated scan, session management
3. **Exploitation** — Payload delivery, blind confirmation (OOB), privilege escalation, data exfiltration
4. **Validation** — True-positive verification, impact assessment, CVSS calculation, proof-of-concept generation
5. **Reporting** — Risk rating, remediation guidance, retesting, SLA tracking

## Tool Comparison

| Tool | Category | License | Key Strength |
|------|----------|---------|--------------|
| **Burp Suite Professional** | Proxy + DAST | Commercial | Repeater, Intruder, Scanner, Extender API, BCheck rules |
| **OWASP ZAP** | DAST | Apache 2.0 | Automated scanner, HUD, API scan, context-based auth, scriptable |
| **Nikto** | Web server scanner | GPLv2 | 7k+ dangerous files, 1.3k+ server checks, CGI enumeration |
| **Wapiti** | DAST | Apache 2.0 | Black-box scanner, SQLi/XSS/LFI/XXE detection, authenticated crawl |
| **SQLMap** | SQLi automation | GPLv2 | Full DB takeover, 6+ injection types, all major DBs, tamper scripts |
| **NoSQLMap** | NoSQL injection | GPLv3 | MongoDB/CouchDB/Redis, auth bypass, data cloning, blind injection |
| **Nuclei** | Vuln scanner | MIT | YAML templates, protocol-based, multi-step, 10k+ templates |
| **FFUF** | Web fuzzer | MIT | Directory/parameter/subdomain fuzzing, recursion, filtering |
| **Gobuster** | Directory brute-force | Apache 2.0 | Directory/file/DNS/vhost brute-force, wordlist-based |
| **ModSecurity + CRS** | WAF | Apache 2.0 | SecLang rules engine, OWASP Top 10 protection, paranoia levels |
| **testssl.sh** | TLS/SSL assessment | GPLv2 | Cipher suite, protocol support, certificate chain, vuln scan |
| **WPScan** | WordPress scanner | GPLv3 | Plugin/theme detection, vuln DB, user enum, brute-force |
| **Semgrep** | SAST | LGPLv2.1 | Custom rules, OWASP Top 10 coverage, CI-native, interfile analysis |
| **CodeQL** | SAST | MIT | Query-based, semantic analysis, Code Scanning integration |
| **Gitleaks** | Secrets scanner | MIT | Git pre-commit/CI, 150+ rules, regex/path configurable |
| **truffleHog** | Secrets scanner | GPLv2 | Entropy + regex matching, historical scan, GitHub integration |
| **Dependency-Check** | SCA | Apache 2.0 | Java/.NET/Python/Ruby/NPM, NVD/CVE lookup, multi-format reports |

## Tool Docs

| File | Tool |
|------|------|
| [Burp-Suite.md](Burp-Suite.md) | Burp Suite Professional |
| [SQLMap.md](SQLMap.md) | SQLMap |
| [Nuclei.md](Nuclei.md) | Nuclei |
| [Semgrep.md](Semgrep.md) | Semgrep |
| [Nikto.md](Nikto.md) | Nikto |

> **Note:** OWASP ZAP is documented under `../api-security-testing/owasp-zap.md`.
