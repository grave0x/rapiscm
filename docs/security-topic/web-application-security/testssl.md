# testssl.sh — TLS/SSL Security Testing

Command-line tool to check TLS/SSL configuration. Tests protocols, ciphers, certificate chain, vulnerabilities, and security headers. No dependencies beyond OpenSSL and bash.

## How It Works

testssl.sh connects like a client and probes the server for supported TLS versions, cipher suites, certificate details, and known vulnerabilities. Single bash script — no install required.

**Test categories:**
- **Protocols** — SSLv2, SSLv3, TLS 1.0–1.3, STARTTLS
- **Ciphers** — All supported cipher suites per protocol, including export-grade, NULL, anonymous
- **Certificate** — Chain validation, expiration, key size, signature algorithm, SAN/DNS names, CA issuance
- **Vulnerabilities** — Heartbleed, CCS injection, POODLE, DROWN, LOGJAM, FREAK, ROBOT, BEAST, CRIME, BREACH, LUCKY13, RC4, Ticketbleed, Padding Oracle, Renegotiation
- **Security headers** — HSTS, HPKP (deprecated), CSP, Expect-CT
- **Perfect Forward Secrecy** — Key exchange algorithms and PFS coverage
- **Server defaults** — Cipher order, TLS version preference, named groups

## Manual

```bash
# Quick check (default tests)
./testssl.sh https://target.com

# Check single protocol
./testssl.sh --ssl https://target.com
./testssl.sh --tls https://target.com

# Check specific vulnerability
./testssl.sh --heartbleed https://target.com
./testssl.sh --poodle https://target.com
./testssl.sh --logjam https://target.com
./testssl.sh --robot https://target.com

# Full check (everything, slow)
./testssl.sh --full https://target.com

# HTML report
./testssl.sh --htmlfile report.html https://target.com

# JSON output for automation
./testssl.sh --jsonfile report.json https://target.com

# Check multiple hosts
./testssl.sh --file hosts.txt --csvfile results.csv

# Check on custom port
./testssl.sh https://target.com:8443

# STARTTLS (SMTP, IMAP, XMPP, etc.)
./testssl.sh --starttls smtp target.com:25
```

## Build

Single bash script (no compilation). Updates via git.

```bash
git clone --depth 1 https://github.com/drwetter/testssl.sh.git
cd testssl.sh
```

## Install

```bash
# No install needed — just clone and run
git clone --depth 1 https://github.com/drwetter/testssl.sh.git
cd testssl.sh

# Or symlink to PATH
sudo ln -s $(pwd)/testssl.sh /usr/local/bin/testssl

# Kali
sudo apt install testssl.sh

# macOS
brew install testssl

# Docker
docker pull drwetter/testssl.sh
docker run --rm drwetter/testssl.sh https://target.com
```

## Package

| Manager | Command |
|---------|---------|
| apt/Kali | `sudo apt install testssl.sh` |
| Homebrew | `brew install testssl` |
| Docker | `docker pull drwetter/testssl.sh` |
| Git | `git clone https://github.com/drwetter/testssl.sh.git` |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/drwetter/testssl.sh |
| Docs | https://github.com/drwetter/testssl.sh/wiki |
| ChangeLog | https://github.com/drwetter/testssl.sh/blob/master/CHANGELOG |
| Docker Hub | https://hub.docker.com/r/drwetter/testssl.sh |
| TLS basics | https://github.com/drwetter/testssl.sh/wiki/TLS-basics |
