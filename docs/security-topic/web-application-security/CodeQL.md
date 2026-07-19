# CodeQL — Semantic Code Analysis Engine

GitHub's semantic code analysis engine. Finds vulnerabilities and code quality issues across languages. Used in GitHub Advanced Security for SAST.

## How It Works

CodeQL treats code as data — compiles source into a queryable relational database. Security researchers write QL (declarative logic language) queries to find patterns representing vulnerabilities.

**Pipeline:**
1. **Extraction** — CodeQL extractor parses source code into a database (AST, data flow, control flow, semantic relationships)
2. **Database** — Relational representation of the codebase (functions, variables, expressions, types, data flow edges)
3. **Query execution** — QL queries run against the database. Built-in query suites for CWE categories
4. **Results** — Locations, severity, paths (source-to-sink for data flow queries)

**Built-in query suites:**
- `code-scanning` — 1500+ queries for security + quality
- `security-and-quality` — Extended including ML-powered queries
- `security-extended` — Wider coverage with higher false positive rate
- `security-experimental` — Research-grade queries

**Supported languages:** C/C++, C#, Go, Java/Kotlin, JavaScript/TypeScript, Python, Ruby, Swift.

## Manual

```bash
# Install CodeQL CLI
# Download from github.com/github/codeql-cli-binaries/releases

# Create database
codeql database create ./codeqldb --language=python \
  --source-root=/path/to/repo

# Run query suite
codeql database analyze ./codeqldb \
  --format=sarif-latest --output=results.sarif \
  codeql/python-queries:code-scanning

# Run specific query
codeql query run path/to/query.ql \
  --database=./codeqldb --output=results.bqrs

# Convert results
codeql bqrs decode results.bqrs --format=json

# Upgrade database (for newer CodeQL)
codeql database upgrade ./codeqldb
```

## Build

```bash
# From source (OCaml)
git clone --recursive https://github.com/github/codeql-cli.git
cd codeql-cli
make
# Output: target/<...>/codeql

# Standard query packs
git clone https://github.com/github/codeql.git
```

## Install

```bash
# Download CLI binary
wget https://github.com/github/codeql-cli-binaries/releases/download/v2.17.0/codeql-linux64.zip
unzip codeql-linux64.zip
sudo mv codeql /opt/codeql
export PATH=$PATH:/opt/codeql

# Download query packs
git clone https://github.com/github/codeql.git
# Point to queries via --search-path or CODEQL_SUITES_PATH

# GitHub Actions (recommended)
# Uses github/codeql-action/analyze@v3
```

## Package

| Manager | Command |
|---------|---------|
| Binary | Download from GitHub releases |
| Homebrew | `brew install codeql` |
| GitHub Actions | `github/codeql-action` |
| Docker | `docker pull ghcr.io/github/codeql-cli` |

No package manager (standalone binary).

## Links

| Resource | URL |
|----------|-----|
| Official site | https://codeql.github.com/ |
| CLI docs | https://docs.github.com/en/code-security/codeql-cli |
| CodeQL queries | https://github.com/github/codeql |
| QL language | https://codeql.github.com/docs/ql-language-guide/ |
| Query console | https://lgtm.com/query (deprecated, use GitHub) |
| Blog | https://github.blog/tag/codeql/ |
| CVE research | https://github.com/github/securitylab/tree/main/CodeQL_Queries |
