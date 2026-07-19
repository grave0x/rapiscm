# Network Reconnaissance Tools

Tools for discovering, enumerating, and mapping network assets and attack surfaces — from passive OSINT to active port scanning.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| Passive Recon | Collect intel without touching target — DNS, certificates, WHOIS, search engines |
| Active Recon | Direct probing — port scanning, service fingerprinting, banner grab |
| Subdomain Enumeration | Discover subdomains via passive sources + brute-force |
| Attack Surface Management | Continuous asset discovery and exposure monitoring |
| OSINT Frameworks | Multi-source intelligence gathering automation |

## Methods

- **Passive**: Certificate Transparency (crt.sh), DNS enumeration, search engine dorking, WHOIS/RDAP lookups, social media scraping
- **Active**: Port scanning (TCP SYN/UDP), service version detection, OS fingerprinting, firewall rule inference
- **Brute-force**: Subdomain dictionary attacks, DNS wordlists, permutations
- **Automated pipelines**: Subfinder → httpx → nuclei → triage → report
- **AI-assisted**: LLM triage of recon findings, auto-prioritization

## Tool Comparison

| Tool | Description | License | Primary Use Case |
|------|-------------|---------|-----------------|
| **Nmap** | Industry-standard port scanner, service detection, NSE scripting | GPL | Port scanning + service fingerprinting |
| **Masscan** | Ultra-fast port scanner, transmits at line rate | AGPL | Large-scale internet scanning |
| **RustScan** | Rust-based port scanner with Nmap integration, ~3s full scan | GPL | Rapid port discovery |
| **Amass** (OWASP) | In-depth attack surface mapping, network mapping | Apache 2 | Attack surface enumeration |
| **Shodan** | Search engine for internet-connected devices | Commercial / freemium | Device discovery + exposure checking |
| **Subfinder** (ProjectDiscovery) | Fast passive subdomain enumeration via 45+ sources | MIT | Subdomain discovery |
| **httpx** (ProjectDiscovery) | Multi-purpose HTTP prober, tech detection, status codes | MIT | HTTP service probing |
| **SpiderFoot** | Automated OSINT with 200+ modules, web UI + CLI | MIT | Multi-source OSINT automation |
| **Maltego** | Graph-based link analysis for OSINT | Commercial / freemium | Relationship mapping |
| **theHarvester** | Email, subdomain, and name harvesting from public sources | GPL | Email + domain intelligence |
| **ffuf** | Fast web fuzzer for content discovery | MIT | Web directory/parameter fuzzing |
| **gau** (ProjectDiscovery) | Get all URLs from Wayback, AlienVault, CommonCrawl | MIT | URL harvesting |

## Stack Recommendations

| Profile | Recommended Stack |
|---------|-----------------|
| Bug bounty hunter | Subfinder → httpx → Nuclei + Shodan |
| Pentester | Nmap + Masscan + ffuf + Amass |
| SOC / Blue team | Shodan + Amass + SpiderFoot continuous monitoring |
| ASM program | ProjectDiscovery stack + Censys + custom pipelines |

## References

- [OWASP Amass](https://owasp.org/www-project-amass/)
- [ProjectDiscovery](https://github.com/projectdiscovery)
- [Shodan](https://www.shodan.io/)
- [Nmap Reference Guide](https://nmap.org/docs.html)
