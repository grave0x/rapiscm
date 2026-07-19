# GoPhish — Open-Source Phishing Framework

Complete phishing simulation framework — template editor, SMTP configuration, landing pages, campaign analytics, and user management.

## How It Works

Web-based admin interface (default `:3333`). Admin configures:
- **Sending Profiles** — SMTP server, auth, from address, headers
- **Email Templates** — HTML rich text, import from URL, tracking pixel injection
- **Landing Pages** — Capture credentials, redirect, clone from URL
- **User Groups** — CSV import, target list management
- **Campaigns** — Launch + schedule, track open/click/credential-submit rates

GoPhish uses a tracking pixel (1x1 transparent image) for open-rate tracking. Landing pages submit credentials via POST to GoPhish server (encrypted). All data stored in SQLite (default) or MySQL.

## Manual

```bash
# Start GoPhish (Linux)
./gophish

# Start with config
./gophish --config config.json

# Access admin UI
# https://localhost:3333
# Default: admin / gophish (change immediately)

# REST API
curl -k -X POST https://localhost:3333/api/login \
  -d '{"username":"admin","password":"gophish"}' \
  -H "Content-Type: application/json"

# API: list campaigns
curl -k -H "Authorization: Bearer <token>" \
  https://localhost:3333/api/campaigns/
```

## Build

```bash
git clone https://github.com/gophish/gophish.git
cd gophish
go build -o gophish .
```

## Install

```bash
# Download binary from GitHub releases
wget https://github.com/gophish/gophish/releases/download/v0.12.1/gophish-v0.12.1-linux-64bit.zip
unzip gophish-v0.12.1-linux-64bit.zip
cd gophish-v0.12.1-linux-64bit
sudo chmod +x gophish

# Docker
docker pull gophish/gophish
docker run -d -p 3333:3333 -p 80:80 gophish/gophish
```

## Package

Prebuilt binaries for Linux, macOS, Windows on GitHub releases. Docker image `gophish/gophish`.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://getgophish.com/ |
| GitHub | https://github.com/gophish/gophish |
| Docs | https://docs.getgophish.com/ |
| API docs | https://github.com/gophish/gophish/blob/master/api/README.md |
| User Guide | https://docs.getgophish.com/user-guide/ |
