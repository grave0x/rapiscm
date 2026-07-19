# Gobuster — Directory/File/DNS Busting Tool

Multi-purpose brute-force tool written in Go. Directory/file enumeration, DNS subdomain discovery, vhost enumeration, and S3 bucket scanning.

## How It Works

Gobuster sends HTTP requests substituting the wordlist entry into the target URL. Also supports DNS resolution for subdomain enumeration.

**Modes:**

| Mode | Flag | Description |
|------|------|-------------|
| **dir** | `dir` | Directory/file enumeration on web servers |
| **dns** | `dns` | DNS subdomain lookup |
| **vhost** | `vhost` | Virtual host enumeration via Host header |
| **s3** | `s3` | AWS S3 bucket name enumeration |
| **fuzz** | `fuzz` | Generic fuzzing with placeholder substitution |

**Key options:** `-w` (wordlist), `-t` (threads), `-o` (output), `-x` (extensions), `-s` (status codes), `-k` (skip TLS verify), `-n` (no redirect).

## Manual

```bash
# Directory enumeration
gobuster dir -u https://target.com -w /usr/share/wordlists/dirb/common.txt

# With extensions
gobuster dir -u https://target.com -w wordlist.txt -x php,asp,html,txt

# Filter by status codes (show 200,301,302 only)
gobuster dir -u https://target.com -w wordlist.txt -s "200,301,302"

# DNS subdomain enumeration
gobuster dns -d target.com -w subdomains.txt

# DNS with wildcard detection
gobuster dns -d target.com -w subdomains.txt --wildcard

# Virtual host enumeration
gobuster vhost -u https://target.com -w vhosts.txt

# S3 bucket enumeration
gobuster s3 -w bucket-names.txt -o buckets.txt

# Fuzz mode (placeholder = FUZZ)
gobuster fuzz -u "https://target.com/FUZZ?id=1" -w wordlist.txt
```

## Build

```bash
git clone https://github.com/OJ/gobuster.git
cd gobuster
go build
```

## Install

```bash
# Go install
go install github.com/OJ/gobuster/v3@latest

# Binary download
wget https://github.com/OJ/gobuster/releases/download/v3.6.0/gobuster_Linux_x86_64.tar.gz
tar xzf gobuster_Linux_x86_64.tar.gz
sudo mv gobuster /usr/local/bin/

# Kali
sudo apt install gobuster

# Docker
docker pull ghcr.io/oj/gobuster:latest
```

## Package

| Manager | Command |
|---------|---------|
| apt/Kali | `sudo apt install gobuster` |
| Homebrew | `brew install gobuster` |
| Go | `go install github.com/OJ/gobuster/v3@latest` |
| Docker | `docker pull ghcr.io/oj/gobuster` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/OJ/gobuster |
| Docs | https://github.com/OJ/gobuster#gobuster |
| Wordlists | https://github.com/danielmiessler/SecLists |
| Kali tools | https://www.kali.org/tools/gobuster/ |
