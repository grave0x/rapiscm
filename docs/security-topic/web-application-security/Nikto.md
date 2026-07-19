# Nikto — Web Server Scanner

Classic web server scanner — 7k+ potentially dangerous files/programs, 1.3k+ server checks, outdated version detection, CGI enumeration.

## How It Works

Nikto sends crafted HTTP requests and analyzes responses to identify:
- Dangerous files/cgi (default passwords, sample files, admin interfaces)
- Outdated server software (version fingerprinting)
- Server configuration issues (HTTP methods, headers, directory listing)
- Known vulnerabilities per server type (Apache, Nginx, IIS, Tomcat)

**Database:** libwhisker-based with regular updates. Plugins for custom functionality.

## Manual

```bash
# Basic scan
nikto -h https://target.com

# Specific port
nikto -h https://target.com -p 8443

# SSL only
nikto -h https://target.com -ssl

# Output to file
nikto -h https://target.com -o report.html -F html
nikto -h https://target.com -o report.txt -F txt
nikto -h https://target.com -o report.csv -F csv
nikto -h https://target.com -o report.json -F json

# With authentication
nikto -h https://target.com -id admin:password

# Evasion techniques
nikto -h https://target.com -e 1

# Specific CGI directories
nikto -h https://target.com -C /cgi-bin/

# Tuning (check certain vulnerability categories)
nikto -h https://target.com -T 9
# Tuning codes: 1=Interesting File, 2=Misconfiguration, 3=Info Disclosure, etc.
```

## Install

```bash
# Debian/Ubuntu
sudo apt install nikto

# macOS
brew install nikto

# Git (latest)
git clone https://github.com/sullo/nikto.git
cd nikto/program

# Docker
docker pull sullo/nikto
docker run sullo/nikto -h https://target.com

# No build needed — Perl-based
# Run directly
perl nikto.pl -h https://target.com
```

## Build

Perl-based — no compilation. Update database via:
```bash
nikto -update
# or
git pull
```

## Package

| Manager | Command |
|---------|---------|
| DEB | `apt install nikto` |
| Brew | `brew install nikto` |
| Git | Clone + run |
| Docker | `sullo/nikto` |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://cirt.net/Nikto2 |
| GitHub | https://github.com/sullo/nikto |
| Docs | https://github.com/sullo/nikto/wiki |
| Plugin list | https://github.com/sullo/nikto/tree/master/program/plugins |
| Database | https://github.com/sullo/nikto/tree/master/program/databases |
