# Cofense — Phishing Detection & Response Platform

Enterprise-grade phishing defense platform. Combines user-reported phishing intelligence with automated analysis, IOC extraction, and SIEM integration. Formerly PhishMe.

## How It Works

Cofense uses a human-augmented detection model — users report suspicious emails via PhishAlarm/Reporter button, platform analyzes and automates response.

**Product suite:**

| Product | Description |
|---------|-------------|
| **PhishMe** | Phishing simulation. Templates, landing pages, scheduled campaigns, user scoring |
| **PhishAlarm** | Outlook/Gmail/Exchange plugin. One-click phishing reporting |
| **Cofense Triage** | Analysis engine. Auto-prioritizes reported emails. Extracts IOCs (URLs, attachments, headers, sender patterns). Integrates with sandboxes (VirusTotal, Joe Sandbox, CrowdStrike) |
| **Cofense Intelligence** | Curated threat intelligence. Real-time phishing indicators. Manual analysis reports from Cofense SOC |
| **Vision** | Automated response. Quarantine emails across all recipients. Block IOCs in email gateways |

**Triage workflow:**
1. User reports email → PhishAlarm event
2. Cofense Triage analyzes: header analysis, URL extraction, attachment sandboxing, sender reputation
3. Auto-classify: clean, spam, suspicious, malicious
4. Extract IOCs: SHA-256, URLs, domains, sender IPs
5. Publish to SIEM/SOAR via API or Syslog
6. Automated containment (Vision) — remove from all inboxes

## Manual

```bash
# REST API (Triage)
# Authenticate
curl -X POST https://api.cofense.com/v2/auth \
  -d '{"access_token":"<api_key>","expires_in":3600}'

# List reported emails
curl -H "Authorization: Bearer <token>" \
  https://api.cofense.com/v2/reported_messages

# Get IOC payloads for a report
curl -H "Authorization: Bearer <token>" \
  https://api.cofense.com/v2/reported_messages/<id>/payloads

# Update report category
curl -X PUT -H "Authorization: Bearer <token>" \
  https://api.cofense.com/v2/reported_messages/<id> \
  -d '{"category_id":3,"response":"blocked sender"}'
```

## Install

SaaS deployment. No self-hosted option. Integrations:

| Integration | Method |
|-------------|--------|
| Email clients | PhishAlarm plugin (Outlook, Gmail, Exchange Web) |
| Email gateways | Mimecast, Proofpoint, Exchange Online, Google Workspace |
| SIEM | Splunk, Elastic, Sentinel, QRadar, Sumo Logic |
| SOAR | Splunk SOAR (Phantom), Palo Alto XSOAR |
| Sandboxes | VirusTotal, Joe Sandbox, CrowdStrike Falcon Sandbox, Any.Run |
| SSO | SAML 2.0 (Okta, Azure AD, OneLogin) |

## Build

Closed-source SaaS. REST API v2 for integration. Webhook events for automated playbooks.

## Package

Subscription-based. Tiered by mailboxes, simulated users, and intelligence level. Add-on modules: Triage Automated, Vision, Cofense Intelligence.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.cofense.com/ |
| Platform overview | https://www.cofense.com/platform/ |
| Developer portal | https://developer.cofense.com/ |
| API docs | https://developer.cofense.com/apis/ |
| Triage guide | https://docs.cofense.com/ |
| Cofense Intelligence | https://www.cofense.com/products/cofense-intelligence/ |
