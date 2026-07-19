# mitmproxy — HTTPS Intercepting Proxy

Interactive HTTPS proxy for traffic inspection, modification, replay, and scripting. Used for mobile app testing, API debugging, and protocol analysis.

## How It Works

mitmproxy acts as a man-in-the-middle proxy. It generates a CA certificate, installs it on the target device, and re-signs TLS connections on-the-fly to decrypt and inspect HTTPS traffic.

**Three interfaces:**

| Interface | Description |
|-----------|-------------|
| **mitmproxy** | Console UI (vim-like keybindings) |
| **mitmweb** | Web UI (localhost:8081) |
| **mitmdump** | Headless, CLI-only (scripting, automation) |

## Manual

```bash
# Start proxy
mitmproxy --listen-port 8080

# Start web UI
mitmweb --listen-port 8080

# Headless with script
mitmdump -s modify_request.py

# Record traffic to file
mitmdump -w traffic.flow

# Replay traffic
mitmdump -r traffic.flow --server-replay

# Transparent proxy mode
mitmproxy --mode transparent

# Reverse proxy mode
mitmproxy --mode reverse:https://api.example.com
```

### Scripting (Python)

```python
# modify_request.py
from mitmproxy import http

def request(flow: http.HTTPFlow) -> None:
    # Modify request
    flow.request.headers["X-Debug"] = "true"
    
    # Block specific domains
    if "analytics" in flow.request.pretty_host:
        flow.response = http.Response.make(403)
    
    # Log URLs
    print(f"{flow.request.method} {flow.request.pretty_url}")

def response(flow: http.HTTPFlow) -> None:
    # Modify response body
    if "api/endpoint" in flow.request.pretty_url:
        flow.response.text = flow.response.text.replace("secret", "[REDACTED]")
```

### CI/CD

```bash
# Headless traffic capture during tests
mitmdump --listen-port 8888 --mode regular -w captured.flow &
TEST_PROXY=localhost:8888 pytest
```

### Mobile Device Setup

```bash
# 1. Find device IP
# 2. Set proxy on device to <workstation-ip>:8080
# 3. Install CA cert
#   Android: Settings > Security > Install certificate
#   iOS: Safari to http://mitm.it, install profile
#   Or copy cert
mitmproxy --listen-port 8080
# Then visit http://mitm.it on device
```

## Build

```bash
git clone https://github.com/mitmproxy/mitmproxy.git
cd mitmproxy
pip install -e ".[dev]"
# Run: mitmproxy
```

## Install

```bash
# macOS
brew install mitmproxy

# Linux (pip)
pip install mitmproxy

# Docker
docker pull mitmproxy/mitmproxy
docker run --rm -it -p 8080:8080 mitmproxy/mitmproxy mitmproxy

# Windows (pip)
pip install mitmproxy
```

## Links

| Resource | URL |
|----------|-----|
| Official site | https://mitmproxy.org/ |
| GitHub | https://github.com/mitmproxy/mitmproxy |
| Docs | https://docs.mitmproxy.org/ |
| Addons | https://docs.mitmproxy.org/stable/addons-overview/ |
| mitm.it (cert install) | http://mitm.it |
