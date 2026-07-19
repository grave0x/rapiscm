# Escape — AI-Powered API Security

API security platform combining AI-driven DAST with business logic fuzzing. Distinguishes itself by **understanding API workflows** (not just endpoints) and generating context-aware test cases for logic-level vulnerabilities.

## How It Works

Escape uses a **three-engine architecture**:

### 1. Discovery & Context Engine

Builds a behavioral model of the API by:

- **Spec parsing** — OpenAPI, GraphQL, gRPC, SOAP, Postman, HAR
- **Traffic analysis** — Observes real requests to infer undocumented endpoints, hidden parameters, and parameter types
- **Workflow inference** — Connects endpoints into sequences (login → browse → add-to-cart → checkout) by tracking session state
- **Auth flow mapping** — Identifies OAuth2 flows, JWT structures, API key locations

### 2. AI Fuzzing Engine

For each endpoint, Escape generates **context-aware test cases**:

```yaml
# Example: /api/v2/orders/{orderId}
# Engine infers:
#   - orderId is an integer, range 1-99999
#   - Endpoint requires "Authorization: Bearer" header
#   - Order must belong to the authenticated user
#   - Workflow: POST /cart → POST /checkout → GET /orders/{id}

# Test cases generated:
#   1. GET /api/v2/orders/99999  (valid ID, valid token) → 200 ✓
#   2. GET /api/v2/orders/99999  (valid ID, NO token) → 401 ✓
#   3. GET /api/v2/orders/1      (another user's order) → 403 ✓
#   4. GET /api/v2/orders/-1     (negative ID) → 400 expected, got 500 ✗
#   5. POST /api/v2/orders/99999 (wrong method) → 405 expected, got 200 ✗
```

### 3. Business Logic Engine

Tests **multi-step workflows** for logic flaws:

| Logic Flaw | Test Method |
|-----------|-------------|
| Price manipulation | Submit checkout with modified price in request body |
| Coupon abuse | Apply coupon multiple times, remove after discount |
| Quantity overflow | Set quantity > stock, negative quantity, fractional quantity |
| Race condition | Send concurrent requests to create double resources |
| Step skipping | POST /checkout directly without POST /cart |
| Privilege escalation | Invite user with role=admin injected in request |
| Password reset abuse | Chain password reset tokens across users |

### Detection

| Class | Coverage |
|-------|----------|
| OWASP API Top 10 (2023) | 10/10 risks |
| Business logic | Price, coupon, cart manipulation; race conditions; privilege escalation |
| GraphQL | Introspection, query depth, batch attacks, alias-based enumeration |
| JWT | Algorithm confusion (none/HS256/RS256), weak secret brute-force, kid injection |
| Injection | SQL, NoSQL, OS command, SSTI, LDAP, XPath, XXE |

## Manual

### CLI

```bash
# Install
npm install -g @escape-cli/core

# Scan from OpenAPI spec
escape scan --spec openapi.yaml --target https://api.staging.example.com

# With API key
escape auth login --api-key ${ESCAPE_API_KEY}
escape scan --spec openapi.yaml --target https://api.staging.example.com

# Business logic tests
escape scan --spec openapi.yaml --target https://api.staging.example.com \
  --test-level business-logic
```

### GitHub Actions

```yaml
- name: Escape API Scan
  uses: escape-security/action@v1
  with:
    api_key: ${{ secrets.ESCAPE_API_KEY }}
    spec_path: openapi.yaml
    target_url: https://staging.example.com
```

### CI/CD

```bash
# Break build on critical findings
escape scan --spec openapi.yaml \
  --target https://staging.example.com \
  --fail-on critical,high \
  --exit-code
```

### Output

```bash
escape scan --report-format json --output report.json
escape scan --report-format sarif --output report.sarif
escape scan --report-format junit --output report.xml  # CI integration
```

## Build

Escape is **closed-source** (SaaS platform + CLI agent). Not buildable from source.

## Install

### CLI

```bash
# npm
npm install -g @escape-cli/core

# Or binary download
curl -L https://escape.engineering/cli/latest/linux-amd64/escape -o escape
chmod +x ./escape
sudo mv escape /usr/local/bin/
```

### Docker

```bash
docker pull escapeengineering/cli
docker run escapeengineering/cli --help
```

### CI Agents

```yaml
# Pinned Docker image for pipeline
image: escapeengineering/cli:latest
```

## Package

| Artifact | Distribution |
|----------|-------------|
| CLI (npm) | npm (`@escape-cli/core`) |
| CLI binary | `escape.engineering/cli/latest/` |
| Docker | Docker Hub (`escapeengineering/cli`) |
| GitHub Action | Marketplace (`escape-security/action`) |
| Web console | SaaS (app.escape.engineering) |

## Links

| Resource | URL |
|----------|-----|
| Official site | https://escape.engineering/ |
| Documentation | https://escape.engineering/docs |
| CLI reference | https://escape.engineering/docs/cli |
| Business logic testing | https://escape.engineering/business-logic |
| GitHub Action | https://github.com/marketplace/actions/escape-api-scan |
| Blog | https://escape.engineering/blog |
| Pricing | https://escape.engineering/pricing |
| Status | https://status.escape.engineering |
