# Schemathesis — Property-Based API Fuzzer

Automated API testing tool that reads OpenAPI/Swagger/GraphQL schemas and generates test cases using **property-based testing** (Hypothesis framework). Catches server errors, validation gaps, and logic flaws that traditional DAST misses.

## How It Works

Schemathesis does not use attack payloads. Instead, it **generates random-but-valid inputs within the schema constraints** and checks that the server responds correctly.

### Core Loop

```
OpenAPI spec → Parse types/constraints → Hypothesis generates random inputs
                                            ↓
    Feedback ←─── Compare against checks ←── Send request
                          ↓
                  PASS / FAIL / ERROR
```

### Generation Strategy

For each endpoint + method + parameter combination, Schemathesis:

1. **Parses schema types** — string, integer, boolean, array, object, enum, pattern, minLength, maxLength, etc.
2. **Builds a Hypothesis strategy** — a composable generator that produces valid inputs from the schema
3. **Generates minimal test case first** (simplest valid value)
4. **Shrinks on failure** — when a test fails, Hypothesis automatically reduces the input to the smallest reproducing case
5. **Reports the minimal failing input**

### Checks (Built-in)

| Check | What It Verifies |
|-------|-----------------|
| **Not a server error** | Response status is not `5xx` (default, configurable) |
| **Status code conformity** | Response matches one of the `responses` codes in the spec |
| **Content type conformity** | `Content-Type` header matches spec definition |
| **Response schema conformity** | Response body matches the JSON Schema defined in the spec |
| **Headers conformity** | Response headers match spec definitions |
| **Negative tests** | Sends *invalid* types (string for int, out-of-range values) and checks for proper `4xx` rejection |

### Modes

| Mode | Description |
|------|-------------|
| **CLI** | `schemathesis run openapi.yaml` — single run, stdout output |
| **Python API** | Import as library, embed in pytest suite |
| **Docker** | `schemathesis/engine` for CI environments |
| **Service Mode** | Long-running HTTP server, accepts trigger requests |
| **Standalone Binary** | Pre-built binary, no Python runtime needed |

### Stateful Testing

For API workflows (create resource → use resource → delete resource), Schemathesis can chain operations:

```python
# Python example
schema = schemathesis.from_path("openapi.yaml")

@schema.parametrize()
def test_api(case):
    # First request
    response = case.call()
    # Use response in next request
    case.form_data = {"resource_id": response.json()["id"]}
    response2 = case.call()
    assert response2.status_code < 500
```

This catches **producer-consumer dependency bugs** where an operation assumes a resource exists but doesn't.

## Manual

### CLI

```bash
# Basic API scan
schemathesis run https://api.example.com/openapi.json

# From local file
schemathesis run openapi.yaml

# GraphQL
schemathesis run https://api.example.com/graphql

# Authentication
schemathesis run openapi.yaml \
  --header "Authorization: Bearer <token>"

# Custom checks
schemathesis run openapi.yaml \
  --checks all                    # All built-in checks
  --checks not_a_server_error     # Custom check set

# Rate limiting
schemathesis run openapi.yaml \
  --rate-limit 100                # Requests per second

# Output
schemathesis run openapi.yaml \
  --report report.json            # JSON report
  --junit-xml results.xml         # JUnit for CI

# Verbose
schemathesis run openapi.yaml -v  # Info level
schemathesis run openapi.yaml -vv # Debug level
```

### Python / pytest Integration

```python
import schemathesis

schema = schemathesis.from_path("openapi.yaml")

# Auto-generate pytest tests from schema
@schema.parametrize()
def test_no_server_errors(case):
    response = case.call()
    assert response.status_code < 500

# With authentication hooks
@schema.parametrize()
def test_authenticated(case):
    case.headers["Authorization"] = "Bearer <token>"
    response = case.call()
    assert response.status_code < 500
```

### CI/CD

```yaml
# GitHub Actions
- name: Schemathesis scan
  uses: schemathesis/action@v1
  with:
    spec: https://staging.example.com/openapi.json
    checks: all
    report: schemathesis-report.json
```

## Build

```bash
# Prerequisites: Python 3.8+, pip
git clone https://github.com/schemathesis/schemathesis.git
cd schemathesis
pip install -e .[dev]
schemathesis --version
```

## Install

```bash
# pip
pip install schemathesis

# Binary (no Python needed)
curl -L https://github.com/schemathesis/schemathesis/releases/latest/download/schemathesis-$(uname -s)-$(uname -m) -o schemathesis
chmod +x schemathesis
sudo mv schemathesis /usr/local/bin/

# Docker
docker pull schemathesis/engine
docker run schemathesis/engine --help

# Homebrew
brew install schemathesis
```

### Package Managers

| Manager | Command |
|---------|---------|
| pip | `pip install schemathesis` |
| Homebrew | `brew install schemathesis` |
| Docker | `schemathesis/engine` |
| Binary | GitHub Releases |

## Package

| Artifact | Distribution |
|----------|-------------|
| pip package | PyPI (`schemathesis`) |
| Binary (Linux/macOS) | GitHub Releases |
| Docker | Docker Hub (`schemathesis/engine`) |
| GitHub Action | Marketplace (`schemathesis/action`) |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://schemathesis.io/ |
| GitHub | https://github.com/schemathesis/schemathesis |
| Documentation | https://schemathesis.readthedocs.io/ |
| Python API docs | https://schemathesis.readthedocs.io/en/stable/python.html |
| CLI reference | https://schemathesis.readthedocs.io/en/stable/cli.html |
| Checks reference | https://schemathesis.readthedocs.io/en/stable/checks.html |
| GitHub Action | https://github.com/marketplace/actions/schemathesis-action |
| Docker Hub | https://hub.docker.com/r/schemathesis/engine |
| Blog | https://blog.schemathesis.io/ |
| Changelog | https://github.com/schemathesis/schemathesis/releases |
