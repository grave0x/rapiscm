# Digital Forensics & Incident Response

Tools for memory forensics, disk forensics, artifact collection, timeline analysis, event log hunting, and endpoint visibility during incident response.

## Topics

| Topic | Description |
|-------|-------------|
| Memory Forensics | Capture and analyze RAM — process listings, injected code, network connections, registry data |
| Disk Forensics | Forensic imaging, filesystem parsing (NTFS/ext4/FAT), deleted file recovery via MFT/carving |
| Artifact Collection | Rapid triage collection — EVTX, Prefetch, MFT, Registry, Amcache, SRUM, Shimcache |
| Timeline Analysis | Build super-timelines from filesystem/registry/event logs for chronological investigation |
| Windows EVTX Hunting | Search event logs with Sigma rules — Chainsaw (fast), Hayabusa (timelines) |
| Endpoint Visibility | VQL-based remote hunts (Velociraptor), IOC scanning (Loki), fleet-wide collection |
| Network Forensics | PCAP analysis, C2 detection, DNS tunneling, exfiltration flow identification |
| Cloud Forensics | Query CloudTrail/GuardDuty logs, snapshot volumes, preserve cloud storage artifacts |

## Methods

- **Memory Acquisition:** DumpIt (Win), WinPMEM (Win), AVML (Linux), LiME (Linux) → analyze with Volatility 3
- **Artifact Collection:** KAPE targets → EVTX + MFT + Registry + Prefetch + Amcache in minutes
- **Timeline Construction:** Plaso (log2timeline) → Timesketch (collaborative web platform)
- **EVTX Hunting:** Chainsaw (Sigma rules, Rust, fast) / Hayabusa (timeline CSV/JSON) — detect lateral movement, persistence, privilege escalation
- **Remote Triage:** Velociraptor server → VQL hunts across 10k+ endpoints → collect and query in real-time
- **IOC Scanning:** Loki — YARA rules + hash sets + filename/regex patterns for live response

## Tools

| Tool | Category | Description | License |
|------|----------|-------------|---------|
| Volatility 3 | Memory Forensics | World's most widely used volatile memory extraction and analysis framework | VSL (open source) |
| Plaso | Timeline | Create super-timelines from filesystem, registry, event logs, and web artifacts | GPL 3.0 |
| Timesketch | Timeline (Web) | Collaborative forensic timeline analysis web platform (GUI for Plaso output) | Apache 2.0 |
| KAPE | Artifact Collection | Kroll Artifact Parser and Extractor — rapid target collection + module processing | Free (internal/gov) |
| Velociraptor | Endpoint Visibility | Endpoint visibility and collection via VQL — server/agent, remote hunts, live triage | GPL 2.0 |
| Chainsaw | EVTX Hunting | Rapid Sigma-based Windows event log hunting tool (Rust, fast EVTX parsing) | GPL 3.0 |
| Hayabusa | EVTX Hunting | Windows event log timeline generator with Sigma rules, CSV/JSON output | GPL 3.0 |
| EZ Tools | Artifact Parsing | Eric Zimmerman's forensic parsers — MFTECmd, EvtxECmd, PECmd, RECmd, etc. | MIT (free) |
| Autopsy / Sleuth Kit | Disk Forensics | Full-disk forensic analysis GUI (Autopsy) + CLI (TSK) for filesystem autopsy | Apache 2.0 / GPL |
| FTK Imager | Disk Imaging | Free forensic imaging tool — creates dd, E01, AFF images; preview content | Free (proprietary) |
| Loki | IOC Scanner | IOC scanner using YARA rules and hash sets for live response | GPL 3.0 |
