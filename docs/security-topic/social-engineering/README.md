# Social Engineering Testing

Simulate human-focused attacks: phishing, vishing, smishing, pretexting, and physical SE. Measure organizational susceptibility and improve security awareness.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| **Phishing Campaigns** | Bulk email phishing, spear-phishing, whaling — credential harvesting, malware delivery |
| **AiTM (Adversary-in-the-Middle)** | Reverse-proxy phishing — session token capture, MFA bypass via real-time relay |
| **Vishing / Smishing** | Voice phishing pretext calls, SMS-based phishing, SIM swapping assessment |
| **QR Code Phishing (Quishing)** | Embedded QR codes in emails/documents redirecting to credential harvesters |
| **Pretexting** | Impersonation via phone/in-person — help-desk pretext, vendor impersonation, tailgating |
| **Physical Social Engineering** | Badge cloning, tailgating/piggybacking, lock bypass, dumpster diving |
| **Security Awareness** | Training platforms, simulated campaigns, phish-rate benchmarking |
| **OSINT for Targeting** | LinkedIn/X/Facebook profile gathering, corporate org chart recon, email format discovery |

## Methods

1. **Reconnaissance** — OSINT gathering on targets: email formats, org charts, personal interests, tech stack
2. **Infrastructure Setup** — SMTP relay config, landing page hosting, SSL certificates, reverse-proxy deployment
3. **Campaign Execution** — Template design, target list building, mail merge, launch scheduling, tracking pixel monitoring
4. **Exploitation** — Credential capture, session token hijack (AiTM), 2FA pass-through, payload drop
5. **Reporting** — Open rate / click rate / cred-submit rate, user training assignment, campaign comparison

## Tool Comparison

| Tool | Category | License | Key Strength |
|------|----------|---------|--------------|
| **GoPhish** | Phishing framework | GPLv3 | Self-hosted, template editor, campaign analytics |
| **Evilginx2/3** | AiTM proxy | MIT | Session token capture, MFA bypass, phishlets |
| **SET** | Multi-vector SE | GPLv3 | Spear-phish, credential harvester, webpage clone, infectious media |
| **Modlishka** | AiTM proxy | Apache 2.0 | Multi-domain, auto 2FA pass-through, credential harvesting |
| **KnowBe4** | Awareness platform | Commercial | Phishing simulation, training modules, compliance tracking |
| **Cofense** | Phishing defense | Commercial | Automated simulation, Triage SOC, threat intelligence |
| **Wifiphisher** | Captive portal phishing | MIT | Association-based evil twin + captive portal |

## Tool Docs

| File | Tool |
|------|------|
| [GoPhish.md](GoPhish.md) | GoPhish |
| [Evilginx.md](Evilginx.md) | Evilginx2/3 |
| [SET.md](SET.md) | Social Engineering Toolkit |
| [Modlishka.md](Modlishka.md) | Modlishka |
| [Wifiphisher.md](Wifiphisher.md) | Wifiphisher |
