# EPSS — Exploit Prediction Scoring System

Data-driven vulnerability prioritization by FIRST. Predicts probability of exploitation within 30 days using real-world exploit data, CVE metadata, and threat intelligence.

## How It Works

EPSS scores every CVE in the NVD daily (0–1, or percentile 0–100). Unlike CVSS (static severity), EPSS is dynamic — scores update daily based on observed exploit activity, Twitter mentions, exploit-db, Metasploit, and other signals.

**Score interpretation:**

| EPSS Percentile | Meaning |
|-----------------|---------|
| > 0.9 (90th) | Very likely exploited within 30 days |
| 0.5–0.9 | Moderate exploitation probability |
| 0.1–0.5 | Low probability |
| < 0.1 | Very low probability (includes most CVEs) |

**EPSS vs CVSS:**

| Metric | EPSS | CVSS |
|--------|------|------|
| What it measures | Likelihood of exploitation | Intrinsic severity |
| Score range | 0–1 (probability) | 0–10 (severity) |
| Updates | Daily | Static per CVE |
| Data sources | Real-world exploit activity | Expert analysis |
| Use case | Prioritize which CVEs to patch | Baseline severity classification |

## Manual

### API

```bash
# Single CVE
curl -X POST https://api.first.org/data/v1/epss \
  -H "Content-Type: application/json" \
  -d '{"cves": ["CVE-2024-3094"]}'

# Multiple CVEs
curl -X POST https://api.first.org/data/v1/epss \
  -H "Content-Type: application/json" \
  -d '{"cves": ["CVE-2024-3094", "CVE-2024-27198", "CVE-2023-44487"]}'

# Query with date
curl "https://api.first.org/data/v1/epss?date=2024-04-15"

# Filter by EPSS score
curl "https://api.first.org/data/v1/epss?epss-score-gt=0.5"

# Paginate
curl "https://api.first.org/data/v1/epss?limit=100&offset=200"
```

### Python

```python
import requests

def get_epss(cve_ids):
    r = requests.post(
        "https://api.first.org/data/v1/epss",
        json={"cves": cve_ids}
    )
    data = r.json()
    results = {}
    for item in data.get("data", []):
        cve = item["cve"]
        results[cve] = {
            "epss": float(item["epss"]),
            "percentile": float(item["percentile"])
        }
    return results

# Usage
scores = get_epss(["CVE-2024-3094", "CVE-2023-44487"])
for cve, s in scores.items():
    print(f"{cve}: EPSS={s['epss']:.4f} (pct={s['percentile']:.2f})")
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.first.org/epss |
| API docs | https://www.first.org/epss/api |
| Data download | https://epss.cyentia.com/ |
| FIRST | https://www.first.org/ |
| EPSS + CVSS calculator | https://epss.dev/ |
