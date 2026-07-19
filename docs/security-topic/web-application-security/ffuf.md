# ffuf — Fast Web Fuzzer

High-performance web fuzzer written in Go. Used for directory/file discovery, parameter fuzzing, subdomain enumeration, and value brute-forcing.

## How It Works

ffuf sends HTTP requests with payload substitutions at marked positions (FUZZ keyword). Multi-threaded, highly configurable.

**Fuzzing modes:**

| Mode | Use Case | Example |
|------|----------|---------|
| **Directory** | Discover hidden paths | `ffuf -u https://target.com/FUZZ -w wordlist.txt` |
| **Extension** | Find file types | `ffuf -u https://target.com/indexFUZZ -w .ext` |
| **Parameter** | Enumerate GET parameters | `ffuf -u "https://target.com/page?FUZZ=1"` |
| **POST data** | Brute-force POST fields | `ffuf -u https://target.com/login -d "user=admin&pass=FUZZ"` |
| **Header** | Fuzz custom headers | `ffuf -u https://target.com -H "X-Forwarded-For: FUZZ"` |
| **Value** | Fuzz a known parameter | `ffuf -u "https://target.com/page?id=FUZZ"` |
| **Recursion** | Directory fuzzing with depth | `ffuf -u https://target.com/FUZZ -w list.txt -recursion` |
| **Multi-wordlist** | Multiple substitution points | `ffuf -u "https://target.com/FUZZ1?x=FUZZ2" -w w1.txt:FUZZ1 -w w2.txt:FUZZ2` |

**Key options:** `-c` (color), `-fc` (filter status), `-fs` (filter size), `-fw` (filter words), `-t` (threads), `-p` (proxy), `-e` (extensions), `-o` (output JSON).

## Manual

```bash
# Directory discovery
ffuf -u https://target.com/FUZZ -w /usr/share/wordlists/dirb/common.txt

# With extensions
ffuf -u https://target.com/FUZZ -w wordlist.txt -e .php,.asp,.jsp,.txt,.bak

# Parameter fuzzing
ffuf -u "https://target.com/api/users?id=FUZZ" -w ids.txt -fs 0

# POST data brute-force
ffuf -u https://target.com/login -d "username=admin&password=FUZZ" \
  -w passwords.txt -fc 401

# Header fuzzing (virtual hosts)
ffuf -u https://target.com -w subdomains.txt -H "Host: FUZZ.target.com"

# Recursive directory fuzzing
ffuf -u https://target.com/FUZZ -w wordlist.txt -recursion -recursion-depth 2

# Match only 200 responses
ffuf -u https://target.com/FUZZ -w wordlist.txt -mc 200

# Output to JSON for processing
ffuf -u https://target.com/FUZZ -w wordlist.txt -o results.json -of json
```

## Build

```bash
git clone https://github.com/ffuf/ffuf.git
cd ffuf
go build
```

## Install

```bash
# Go install
go install github.com/ffuf/ffuf/v2@latest

# Binary download
wget https://github.com/ffuf/ffuf/releases/download/v2.1.0/ffuf_2.1.0_linux_amd64.tar.gz
tar xzf ffuf_2.1.0_linux_amd64.tar.gz
sudo mv ffuf /usr/local/bin/

# Docker
docker pull ghcr.io/ffuf/ffuf
docker run --rm -v $(pwd)/wordlist.txt:/wordlist.txt ghcr.io/ffuf/ffuf \
  -u https://target.com/FUZZ -w /wordlist.txt
```

## Package

| Manager | Command |
|---------|---------|
| Homebrew | `brew install ffuf` |
| apt/Kali | `sudo apt install ffuf` |
| Go | `go install github.com/ffuf/ffuf/v2@latest` |
| Docker | `docker pull ghcr.io/ffuf/ffuf` |
| Snap | `snap install ffuf` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/ffuf/ffuf |
| Docs | https://github.com/ffuf/ffuf#usage |
| Wordlists | https://github.com/danielmiessler/SecLists |
| Ffuf tips | https://codingo.io/tools/ffuf/ |
