# API Security Testing Tools

Tools that systematically evaluate APIs (REST, GraphQL, gRPC, SOAP) for security vulnerabilities. Covers the full lifecycle from spec auditing to dynamic testing to runtime monitoring.

## Categories

### Specification / Contract Auditing
Analyze OpenAPI, Swagger, or GraphQL schemas for security misconfigurations *without sending requests*.

| Tool | Type | Coverage | Cost |
|------|------|----------|------|
| **42Crunch** | Spec audit + DAST | OWASP API Top 10 (8/10) | Custom |
| **APIClarity** | Spec diff + discovery | Shadow API detection | Free (OSS) |
| **Spectral** | Lint rules for API specs | Config/deprecation/security | Free (OSS) |

**Key checks:** missing auth definitions, permissive schemas, excessive field exposure, insecure defaults, undefined `4xx`/`5xx` responses.

### Dynamic Application Security Testing (DAST)
Run automated attack payloads against a live API endpoint. Simulates real-world exploitation.

| Tool | Type | OWASP API Top 10 | CI/CD | Cost |
|------|------|------------------|-------|------|
| **OWASP ZAP** | DAST | 6/10 | Docker + GA native | Free |
| **Burp Suite Professional** | DAST + manual | 9/10 | Enterprise plugin | $449/yr |
| **StackHawk** | DAST (ZAP-based) | 7/10 | Native GH/GitLab/Jenkins | $400/mo |
| **Nuclei** | Template scanner | 5/10 | CLI + Docker | Free (OSS) |
| **Escape** | AI + business logic DAST | Full | Native CI/CD | SaaS |
| **Invicti** | DAST (proof-based) | Full | CI/CD | Custom |

**Key checks:** BOLA/IDOR by parameter fuzzing, injection (SQL, NoSQL, OS, LDAP), SSRF, mass assignment, auth bypass, rate-limit bypass.

### Schema-Based / Property-Based Fuzzing
Generate test sequences from API schema definitions. Catches edge cases and implementation gaps.

| Tool | Type | Coverage |
|------|------|----------|
| **Schemathesis** | OpenAPI/GraphQL fuzzer | Property-based, finds 40-60% of API Top 10 |
| **restler-fuzzer** (Microsoft) | Stateful REST fuzzer | Producer-consumer dependency aware |
| **openapi3-fuzzer** | Lightweight spec fuzzer | Basic injection + status code testing |

### API Discovery & Inventory
Detect undocumented, shadow, zombie, and deprecated APIs.

| Tool | Approach | Cost |
|------|----------|------|
| **Akto** | Traffic capture + spec analysis | Freemium |
| **APIClarity** | eBPF-based traffic reflection | Free (OSS) |
| **Noname/Akamai** | Network-level traffic observation | $150K+/yr |
| **Salt Security** | Traffic-based behavioral baseline | Enterprise |

### Runtime API Security Posture Management (ASPM)
Monitor production API traffic for drift, abuse, and active attacks.

| Tool | Approach |
|------|----------|
| **AccuKnox** | eBPF + workload identity + zero-trust CNAPP |
| **Salt Security** | Behavioral analytics + governance |
| **Traceable AI** | Distributed tracing + threat detection |
| **Wallarm** | Inline + out-of-band with cloud-native connectors |
| **Cequence** | Traffic cataloging + posture assessment |

## Common Vulnerability Classes

| OWASP API ID | Risk | Typical Detection Method |
|-------------|------|--------------------------|
| API1:2023 | Broken Object Level Authorization (BOLA) | IDOR parameter fuzzing, multi-role sequence testing |
| API2:2023 | Broken Authentication | Token manipulation, credential stuffing, JWT audit |
| API3:2023 | Broken Object Property Level Authorization | Response field analysis, mass-assignment probes |
| API4:2023 | Unrestricted Resource Consumption | Rate-limit bypass, pagination abuse |
| API5:2023 | Broken Function Level Authorization | Role escalation, admin endpoint discovery |
| API6:2023 | Unrestricted Access to Sensitive Business Flows | Automated workflow abuse (checkout, signup, password reset) |
| API7:2023 | Server-Side Request Forgery | URL parameter fuzzing, cloud metadata probes |
| API8:2023 | Security Misconfiguration | Header audit, CORS analysis, verbose error probing |
| API9:2023 | Improper Inventory Management | Endpoint brute-force, spec-to-runtime diff |
| API10:2023 | Unsafe Consumption of APIs | Third-party data injection, webhook fuzzing |

## Methodology

1. **Shift-left with spec auditing** — lint OpenAPI before code is written (42Crunch, Spectral)
2. **Automated DAST in CI/CD** — run ZAP or StackHawk on every PR against a staging environment
3. **Manual deep testing quarterly** — Burp Suite for business logic flows, OAuth flows, multi-step auth
4. **Runtime posture monitoring** — detect shadow endpoints and behavioral drift in production
5. **Periodic full-scope pentest** — combine all layers with manual adversarial review

## Stack Recommendations by Team Size

| Team Profile | Recommended Stack |
|-------------|------------------|
| Startup / solo | ZAP (free) + Postman scripts + Nuclei |
| Small security team | StackHawk + Schemathesis + Akto (free tier) |
| Enterprise AppSec | Burp Suite Pro + 42Crunch + runtime monitor (Salt/Noname/Traceable) |
| Compliance-driven (PCI/SOC2) | Burp Suite + Invicti + quarterly manual pentest |

## References

- [OWASP API Security Top 10 (2023)](https://owasp.org/API-Security)
- [OWASP API Security Tools List](https://owasp.org/www-community/api_security_tools)
- [NIST SP 800-204: Security Strategies for Microservices](https://csrc.nist.gov/pubs/sp/800/204/final)
- [OWASP ZAP API Scan Documentation](https://www.zaproxy.org/docs/docker/api-scan/)
