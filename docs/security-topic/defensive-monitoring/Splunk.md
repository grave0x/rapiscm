# Splunk Enterprise — SIEM Platform

Industry-leading SIEM platform — SPL correlation, dashboards, MLTK anomaly detection, alerting, RBAC. Scales from single server to distributed clusters.

## How It Works

Splunk indexes machine data (logs, metrics, traces) into searchable events. Architecture:

- **Forwarder** — Lightweight agent collects + forwards data to indexer
- **Indexer** — Parses, indexes, and stores data in buckets
- **Search Head** — Query layer, dashboards, alerts, RBAC
- **Deployment Server** — Centralized forwarder configuration management
- **License Master** — Volume tracking

**SPL (Search Processing Language):** Pipe-based query language. Series of commands chained with `|`.

```spl
index=windows EventCode=4625 | stats count by src_ip | where count > 10
```

**Key features:** Correlation rules, data models, pivot, MLTK (machine learning toolkit), KV store lookups, scheduled reports/alerts, REST API.

## Manual

```bash
# Search
splunk search 'index=main error | stats count by sourcetype'

# Add data
splunk add monitor /var/log/syslog

# Create index
splunk add index my_index

# Export data
splunk search 'index=main earliest=-1d' -output csv > export.csv
```

## Install

```bash
# Linux (DEB)
wget -O splunk.deb https://download.splunk.com/products/splunk/releases/9.3.1/linux/splunk-9.3.1-amd64.deb
sudo dpkg -i splunk.deb
sudo /opt/splunk/bin/splunk start --accept-license

# Linux (tarball)
wget https://download.splunk.com/products/splunk/releases/9.3.1/linux/splunk-9.3.1-linux-amd64.tgz
sudo tar xzf splunk-*.tgz -C /opt/
sudo /opt/splunk/bin/splunk start --accept-license

# macOS
brew install --cask splunk

# Docker
docker pull splunk/splunk:latest
docker run -e SPLUNK_START_ARGS=--accept-license -e SPLUNK_PASSWORD=changeme1 -p 8000:8000 splunk/splunk:latest

# Windows
# Download MSI from splunk.com, run installer
```

## Build

Splunk is closed-source. Apps and add-ons built via SDK. Splunkbase for community apps.

## Package

| Manager | Command |
|---------|---------|
| DEB | `dpkg -i splunk*.deb` |
| RPM | `rpm -i splunk*.rpm` |
| Tarball | extract to `/opt/splunk` |
| Docker | `docker pull splunk/splunk` |

Free dev license (500 MB/day). Paid licenses by ingest volume.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://www.splunk.com/ |
| Docs | https://docs.splunk.com/ |
| SPL reference | https://docs.splunk.com/Documentation/Splunk/latest/SearchReference/ |
| Splunkbase | https://splunkbase.splunk.com/ |
| Free dev license | https://www.splunk.com/en_us/download/splunk-enterprise.html |
| GitHub | https://github.com/splunk/ |
