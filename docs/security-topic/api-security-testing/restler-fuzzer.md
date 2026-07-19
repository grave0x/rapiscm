# RESTler — Stateful REST API Fuzzer

Automated fuzzing tool from Microsoft Research that generates and executes test sequences while tracking producer-consumer dependencies between API requests. Designed to find 500 errors, authentication bypasses, and resource leak vulnerabilities.

## How It Works

RESTler is unique among API fuzzers because it is **stateful** — it understands that creating a resource (POST) must happen before reading (GET) or modifying (PUT/DELETE) it.

### Pipeline

```
OpenAPI spec (.json/.yaml)
        │
        ▼
  Compiler ──► Grammar.py   (every endpoint + parameter + dependency as Python dict)
        │
        ▼
    Test   ──► Run dependencies, happy-path validation
        │
        ▼
    Fuzz   ──► Mutate parameters, skip auth, send garbage
        │
        ▼
    Fuzz-lean ──► Targeted re-run at bug depth on each endpoint
        │
        ▼
  Report (JSON) ──► Bug bucket, response code, request/response pair
```

### Grammar Generation

The compiler parses the OpenAPI spec and produces a **Python dictionary** that encodes:

1. **Endpoint tree** — all paths, methods, parameters, request bodies
2. **Producer-consumer dependencies** — if `POST /users` returns `{"id": 5}`, and `GET /users/{id}` needs that ID, RESTler infers the dependency
3. **Parameter constraints** — types, enums, min/max, patterns
4. **Auth schema** — where tokens/API keys must be injected

### Fuzzing Modes

| Mode | What It Does | Use Case |
|------|-------------|----------|
| **Test** | Runs happy-path sequence per dependency graph | Validates grammar, catches obvious crashes |
| **Fuzz** | Mutates every parameter independently: out-of-range integers, null strings, invalid enums, missing required fields | Maximum coverage for error handling bugs |
| **Fuzz-lean** | Fast subset: only mutates paths where bugs were found at depth | CI gate (5-10 min vs 1-2 hours for full Fuzz) |
| **Fuzz-restler-quick** | Single sequence, no dependency tracking | Sanity check |

### Bug Detection

RESTler flags any response that is:

| Flag | Meaning |
|------|---------|
| **Main driver bug** | Request returns `5xx` when spec says `2xx` |
| **Invalid dynamic object** | Request references resource ID that doesn't exist |
| **Resource leak** | Created resource (POST) never deleted — memory leak |
| **Auth bypass** | Endpoint returns data without auth when spec says auth required |
| **Unexpected success** | Request with missing/invalid params returns `200` instead of `4xx` |

## Manual

### Compile & Fuzz

```bash
# Step 1: Compile spec to grammar
restler-compile --api_spec openapi.json

# Output:
#   Compile/grammar.py
#   Compile/config.json

# Step 2: Test (happy path validation)
restler-test --grammar_file Compile/grammar.py \
  --settings Compile/config.json

# Step 3: Fuzz (full mutation)
restler-fuzz --grammar_file Compile/grammar.py \
  --settings Compile/config.json \
  --time_budget 2           # hours

# Step 4: Fuzz-lean (CI-friendly)
restler-fuzz-lean --grammar_file Compile/grammar.py \
  --settings Compile/config.json
```

### Authentication

```bash
# Token-based
restler-compile --api_spec openapi.json \
  --token_refresh_command "curl -X POST .../token" \
  --token_refresh_interval 300

# Header-based
restler-compile --api_spec openapi.json \
  --set_header "Authorization: Bearer <token>"
```

### Custom Dependencies

For complex workflows, manual dependency hints:

```python
# In grammar.py, annotate producer-consumer pairs
# POST /resources returns {"resource_id": <id>}
# GET /resources/{resourceId} consumes it
# RESTler infers this automatically if response body key matches path param name
```

### Output Analysis

```bash
# Output directory structure
RESTlerResults/
└── experiment-<timestamp>/
    ├── bug_buckets.txt       # Unique bugs found
    ├── fuzz_summary.json     # Summary per endpoint
    ├── main_driver_log.txt   # Full request/response log
    └── logs/                 # Per-bucket debug logs

# Check bug bucket
cat RESTlerResults/experiment-*/bug_buckets.txt
```

## Build

```bash
# Prerequisites: .NET SDK 6.0+
git clone https://github.com/microsoft/restler-fuzzer.git
cd restler-fuzzer

# Build
./build-restler.sh

# Output: restler-bin/ directory with compiled binaries
```

## Install

### Binary (Linux)

```bash
# Download pre-built from GitHub Releases
wget https://github.com/microsoft/restler-fuzzer/releases/latest/download/restler.tar.gz
tar xzf restler.tar.gz
sudo mv restler-bin /opt/restler
export PATH=$PATH:/opt/restler
```

### Docker

```bash
docker pull mcr.microsoft.com/restlerfuzzer/restler:latest
docker run mcr.microsoft.com/restlerfuzzer/restler restler-compile --help
```

### From Source

```bash
git clone https://github.com/microsoft/restler-fuzzer.git
cd restler-fuzzer
./build-restler.sh
# Binaries in ./restler-bin/
```

## Package

| Artifact | Distribution |
|----------|-------------|
| Pre-built binaries (Linux) | GitHub Releases |
| Docker image | MCR (`mcr.microsoft.com/restlerfuzzer/restler`) |
| Source | GitHub |

## Links

| Resource | URL |
|----------|-----|
| GitHub | https://github.com/microsoft/restler-fuzzer |
| Documentation | https://github.com/microsoft/restler-fuzzer/blob/main/docs/README.md |
| Compiler guide | https://github.com/microsoft/restler-fuzzer/blob/main/docs/compiler/Compiler.md |
| User guide | https://github.com/microsoft/restler-fuzzer/blob/main/docs/user-guide/Setup.md |
| Research paper | https://www.microsoft.com/en-us/research/publication/restler-stateful-rest-api-fuzzing/ |
| Security advisory | https://github.com/microsoft/restler-fuzzer/security |
| Releases | https://github.com/microsoft/restler-fuzzer/releases |
