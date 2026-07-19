# FFUF — Fuzz Faster U Fool

High-speed web fuzzer written in Go. Used for directory discovery, parameter fuzzing, vhost enumeration, and content brute-forcing.

## How It Works

FFUF takes a base URL with a `FUZZ` placeholder and replaces it with entries from a wordlist, sending concurrent HTTP requests. It filters responses by status code, size, word count, or regex to identify valid results. The Go runtime gives it exceptional speed compared to tools like DirBuster or WFuzz.

**Core concepts:**

| Concept | Description |
|---------|-------------|
| **FUZZ keyword** | Placeholder replaced by wordlist entries |
| **Matcher** | Criteria to include response (`-mc 200,301`) |
| **Filter** | Criteria to exclude response (`-fs 1234`) |
| **Multi-wordlist** | Multiple `FUZZ` placeholders with per-position wordlists |
| **Recursion** | Automatically scan discovered directories |

## Manual

### Directory Fuzzing

```bash
# Basic directory discovery
ffuf -u https://target.com/FUZZ -w /usr/share/wordlists/directory-list-2.3-medium.txt

# Filter by response code
ffuf -u https://target.com/FUZZ -w wordlist.txt -mc 200,301,403

# Filter by size (remove false positives)
ffuf -u https://target.com/FUZZ -w wordlist.txt -fs 1234
```

### Extension Fuzzing

```bash
# File extension fuzzing
ffuf -u https://target.com/FUZZ -w wordlist.txt -e .php,.asp,.txt,.bak

# Combined with directory
ffuf -u https://target.com/indexFUZZ -w wordlist.txt -e .php,.asp
```

### VHost Enumeration

```bash
ffuf -u https://target.com -H "Host: FUZZ.target.com" -w subdomains.txt -mc 200
```

### Parameter Fuzzing

```bash
# GET parameter
ffuf -u https://target.com/page?FUZZ=1 -w params.txt -mc 200

# POST parameter
ffuf -u https://target.com/login -X POST -d "username=admin&password=FUZZ" -w passwords.txt -mc 302
```

### Advanced Usage

```bash
# Multi-wordlist (dir + extension)
ffuf -u https://target.com/FUZZ1/FUZZ2 -w dirs.txt:FUZZ1 -w ext.txt:FUZZ2

# Rate limiting
ffuf -u https://target.com/FUZZ -w wordlist.txt -rate 100

# Recursion
ffuf -u https://target.com/FUZZ -w wordlist.txt -recursion -recursion-depth 2

# Proxy through Burp
ffuf -u https://target.com/FUZZ -w wordlist.txt -x http://127.0.0.1:8080
```

### Output

```bash
# JSON output for processing
ffuf -u https://target.com/FUZZ -w wordlist.txt -o results.json -of json

# HTML report
ffuf -u https://target.com/FUZZ -w wordlist.txt -o report.html -of html
```

## Build

```bash
git clone https://github.com/ffuf/ffuf.git
cd ffuf
go build
# Binary: ./ffuf
```

## Install

```bash
# Go install
go install github.com/ffuf/ffuf/v2@latest

# Debian/Ubuntu (prebuilt binary)
wget https://github.com/ffuf/ffuf/releases/latest/download/ffuf_*_linux_amd64.tar.gz
tar xzf ffuf_*.tar.gz && sudo mv ffuf /usr/local/bin/

# macOS
brew install ffuf

# Docker
docker pull ffuf/ffuf
```

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/ffuf/ffuf |
| Releases | https://github.com/ffuf/ffuf/releases |
| Wordlist repo | https://github.com/ffuf/ffuf-wordlists |
| Examples | https://github.com/ffuf/ffuf#usage |
