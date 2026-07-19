# KnowBe4 — Security Awareness Training & Phishing Simulation

Industry-leading security awareness platform. Combines training modules with simulated phishing campaigns. Over 70,000 organizations use it.

## How It Works

KnowBe4 is a SaaS platform. Admin console manages users, training assignments, phishing templates, and reporting.

**Core modules:**

| Module | Description |
|--------|-------------|
| **Security Awareness Training** | Interactive courses (compliance, ransomware, social engineering, password security, etc.) |
| **Phishing Simulation** | Built-in templates, landing pages, attachment-based campaigns. Automated scheduling |
| **Vishing & Smishing** | Voice (vishing) and SMS (smishing) simulation campaigns |
| **Phish Alert Button (PAB)** | Outlook/Gmail plugin for users to report suspected phishing |
| **Risk Scoring** | Individual and organizational risk scores weighted by training compliance, phish click rate, and user attributes |
| **Compliance Tracking** | Pre-built content for GDPR, HIPAA, PCI, SOX, NIST, ISO 27001, CMMC |

**Phish-prone percentage (PPP):** Baseline measure of organizational susceptibility. Tracked over time — typical reduction from ~30% baseline to <5% after 12 months of sustained training.

## Manual

```bash
# No local binary — web-based platform
# Admin portal: https://<org>.knowbe4.com

# REST API (v1)
curl -H "Authorization: Bearer <api_key>" \
  https://<org>.knowbe4.com/v1/users

# List training campaigns
curl -H "Authorization: Bearer <api_key>" \
  https://<org>.knowbe4.com/v1/training/campaigns

# Trigger phishing campaign
curl -X POST -H "Authorization: Bearer <api_key>" \
  https://<org>.knowbe4.com/v1/phishing/campaigns \
  -d '{"name":"Q3 Assessment","template_ids":[42],"group_ids":[7]}'
```

## Install

No self-hosted option. SaaS-only. Integrations:

| Integration | Method |
|-------------|--------|
| SSO | SAML 2.0 (Okta, Azure AD, Google Workspace, OneLogin) |
| HR Sync | SCIM (user provisioning/deprovisioning) |
| SIEM | Splunk, Elastic, Sentinel, QRadar |
| MDR/XDR | CrowdStrike, SentinelOne, Defender for Endpoint |
| Email | Mimecast, Proofpoint, Exchange, Google Workspace |

## Build

Closed-source SaaS platform. API and SDK for custom integrations. SSI (Security Skills Index) via API for custom assessment scoring.

## Package

Subscription-based. Plans: Silver (training only), Gold (training + phishing), Diamond (full suite including vishing/smishing, compliance, PAB). Per-user/per-month pricing.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.knowbe4.com/ |
| Product overview | https://www.knowbe4.com/platform/ |
| Developer portal | https://developer.knowbe4.com/ |
| API docs | https://developer.knowbe4.com/ |
| Security awareness | https://www.knowbe4.com/products/security-awareness-training/ |
| Phishing | https://www.knowbe4.com/products/phishing-platform/ |
| Compliance | https://www.knowbe4.com/compliance-ready/ |
