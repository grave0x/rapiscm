# RustScan — Fast Port Scanner

Rust-based port scanner that finishes full scans in ~3 seconds and pipes results into Nmap for detailed enumeration.

## How It Works

RustScan uses Rust's async concurrency and adaptive port selection. It scans all 65K ports in seconds, determines which are open, then optionally feeds them to Nmap for service/OS detection.

**Key concepts:**

| Concept | Description |
|---------|-------------|
| **Adaptive scan** | Learns port distributions to optimize order |
| **Nmap integration** | Auto-pipes open ports to Nmap with user-defined flags |
| **Batch size (-b)** | Number of ports scanned per batch |
| **Timeout (-t)** | Milliseconds per port probe |
| **Greppable output** | Easy to pipe into other tools |

## Manual

### Basic Usage

```bash
# Quick scan (default)
rustscan -a target.com

# With Nmap integration
rustscan -a target.com -- -sV -sC

# Scan specific ports
rustscan -a target.com -p 80,443,8080

# Scan subnet
rustscan -a 192.168.1.0/24 -- -A

# Increase batch size for speed
rustscan -a target.com -b 3000 -t 500

# Output to file
rustscan -a target.com -- -oN scan.nmap
```

### Options

```bash
rustscan -h
# -a, --addresses  Target IPs or hostnames
# -p, --ports      Port list (default: all 65535)
# -b, --batch-size Ports per batch (default: 1500)
# -t, --timeout    Timeout ms (default: 150)
# -r, --range      Port range
# --               Pass remaining args to Nmap
```

## Build

```bash
git clone https://github.com/RustScan/RustScan.git
cd RustScan
cargo build --release
# Binary at target/release/rustscan
```

## Install

```bash
# Debian/Ubuntu (via GitHub releases)
wget https://github.com/RustScan/RustScan/releases/latest/download/rustscan_amd64.deb
sudo dpkg -i rustscan_amd64.deb

# Docker
docker pull rustscan/rustscan:latest
docker run -it --rm --name rustscan rustscan/rustscan:latest -a target.com

# Cargo
cargo install rustscan

# macOS
brew install rustscan
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/RustScan/RustScan |
| Docker Hub | https://hub.docker.com/r/rustscan/rustscan |
