# Packet Capture & Analysis Tools

Tools for capturing, inspecting, and analyzing network traffic — from live packet capture to forensic reconstruction and threat detection.

## Sub-Topics

| Topic | Description |
|-------|-------------|
| Live Packet Capture | Real-time network traffic acquisition from interfaces |
| Protocol Analysis | Deep inspection of application-layer protocols |
| Network Flow Analysis | NetFlow/IPFIX/sFlow collection and visualization |
| PCAP Forensics | Post-capture analysis for incident investigation |
| Network Detection & Response | Real-time threat detection from network telemetry |

## Methods

- **Capture**: SPAN/mirror ports, TAP devices, libpcap, eBPF, netfilter hooks
- **Flow collection**: NetFlow v5/v9, IPFIX, sFlow, jFlow, AppFlow exporters
- **Analysis**: Protocol decoders, entropy analysis, beacon detection, statistical baselining
- **Forensics**: PCAP carving, stream reassembly, file extraction, TLS key logging
- **Pipeline**: Packet capture → flow export → collector → enrichment → visualization → alerting

## Tool Comparison

| Tool | Description | License | Primary Use Case |
|------|-------------|---------|-----------------|
| **Wireshark** | Premier GUI packet analyzer, 3K+ protocol dissectors | GPL | Interactive protocol analysis |
| **tshark** | CLI version of Wireshark for scripted analysis | GPL | Automated/scripted packet analysis |
| **tcpdump** | CLI packet capture tool, libpcap-based | BSD | Lightweight packet capture |
| **Zeek** (formerly Bro) | Network security monitor, deep protocol analysis, structured logs | BSD | Network security monitoring |
| **Arkime** (formerly Moloch) | Full-packet capture, indexing, and search at scale | Apache 2 | Large-scale PCAP storage + search |
| **Suricata** | IDS/IPS/NSM engine with multi-threading and GPU acceleration | GPL | Intrusion detection + prevention |
| **ntopng** | Web-based traffic analysis, flow collection, real-time host drilldown | GPL / Enterprise | Traffic visualization + flow analysis |
| **Scapy** | Python library for packet crafting, sniffing, and injection | GPL | Packet manipulation + scripting |
| **Tcpreplay** | PCAP editing and replay for testing NIDS | GPL | Traffic replay + testing |
| **nfdump** | NetFlow processing tools (nfcapd, nfdump, nfsen) | BSD | NetFlow collection + analysis |

## Stack Recommendations

| Profile | Recommended Stack |
|---------|-----------------|
| Incident responder | tcpdump + Wireshark + Zeek |
| SOC / NOC | Zeek → Arkime/Elasticsearch + ntopng |
| Threat hunter | Zeek + Suricata + Arkime |
| Network engineer | ntopng + nfdump + tcpdump |

## References

- [Wireshark](https://www.wireshark.org/docs/)
- [Zeek](https://zeek.org/documentation/)
- [Arkime](https://arkime.com/)
- [ntopng](https://www.ntop.org/products/traffic-analysis/ntop/)
