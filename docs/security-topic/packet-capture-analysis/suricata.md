# Suricata — IDS/IPS & Network Security Monitoring

Open-source intrusion detection and prevention system. Signature-based, protocol-aware, with Lua scripting and file extraction. Developed by OISF.

## How It Works

Suricata captures packets at line rate (multi-threaded, NUMA-aware), decodes protocols via its parser engine, and matches traffic against rule signatures. It operates in three modes: IDS (alert-only), IPS (inline blocking via NFQUEUE or AF_PACKET), and PCAP (offline file analysis). The rule engine supports multi-pattern matching (Hyperscan/Automaton), packet reassembly, and application-layer detection.

**Core concepts:**

| Concept | Description |
|---------|-------------|
| **Rules** | Signatures in Emerging Threats / custom format |
| **Protocol parsers** | HTTP, TLS, DNS, SMB, Modbus, DNP3, 40+ |
| **EVE JSON** | Unified event output — alerts, metadata, stats |
| **Hyperscan** | Multi-pattern matching engine for high throughput |
| **Lua scripting** | Custom extractors and output plugins |

## Manual

### Running Suricata

```bash
# IDS mode (default)
suricata -c /etc/suricata/suricata.yaml -i eth0

# IPS mode (inline)
suricata -c /etc/suricata/suricata.yaml -q 0

# PCAP file analysis
suricata -c /etc/suricata/suricata.yaml -r capture.pcap

# Test configuration
suricata -c /etc/suricata/suricata.yaml -T
```

### Rule Management

```bash
# Rule update
suricata-update

# List enabled sources
suricata-update list-sources

# Enable additional source
suricata-update enable-source et/open

# Test rules
suricata -c /etc/suricata/suricata.yaml -S custom.rules -T
```

### EVE JSON Output

```bash
# Watch alerts
tail -f /var/log/suricata/eve.json | jq 'select(.event_type == "alert")'

# Extract HTTP logs
tail -f /var/log/suricata/eve.json | jq 'select(.event_type == "http")'

# Extract TLS metadata
tail -f /var/log/suricata/eve.json | jq 'select(.event_type == "tls")'
```

### Custom Rule Example

```
alert tcp $EXTERNAL_NET any -> $HOME_NET any (
  msg:"Suspicious User-Agent";
  content:"|0d 0a|User-Agent: MalwareAgent";
  sid:1000001;
  rev:1;
)
```

## Build

```bash
git clone https://github.com/OISF/suricata.git
cd suricata
git checkout master
./autogen.sh
./configure --enable-hyperscan
make -j$(nproc)
sudo make install
```

## Install

```bash
# Debian/Ubuntu
sudo add-apt-repository ppa:oisf/suricata-stable
sudo apt update && sudo apt install suricata

# RHEL/Fedora
sudo dnf install epel-release
sudo dnf install suricata

# Docker
docker pull jfrog/suricata
```

### Rule Sets

```bash
# Install and enable ET Open rules
suricata-update enable-source et/open
suricata-update
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://suricata.io/ |
| GitHub | https://github.com/OISF/suricata |
| Docs | https://docs.suricata.io/ |
| Rules | https://rules.emergingthreats.net/ |
| Suricata-Update | https://github.com/OISF/suricata-update |
| FAQ | https://suricata.io/faq/ |
