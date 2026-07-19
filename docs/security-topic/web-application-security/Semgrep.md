# Semgrep — Static Analysis (SAST)

Fast, customizable SAST engine. Code-aware pattern matching with OWASP Top 10 coverage, CI integration, and interfile analysis.

## How It Works

Semgrep parses source code into an AST and matches user-defined patterns on the AST structure — understanding syntax, not just text.

**Key concepts:**
- **Rules** — YAML files defining pattern, message, severity, and metadata
- **Patterns** — Code snippets with metavariables (`$X`, `$ARG`, `...`) that match any expression
- **Composition** — `patterns:`, `pattern-either:`, `pattern-inside:` for complex logic
- **Taint tracking** — `pattern-sources:` → `pattern-sinks:` to track untrusted data flow
- **Interfile** — Cross-file analysis (Java, Python, Go, JavaScript, TypeScript, Ruby)
- **Pro rules** — 2,000+ pre-written rules (Semgrep Pro tier)

**Supported languages:** 30+ including Python, JavaScript, TypeScript, Go, Java, Kotlin, C#, Ruby, Rust, Solidity, Terraform, YAML, Dockerfile.

## Manual

```bash
# Scan a project
semgrep --config=auto .

# Scan with specific rule
semgrep --config path/to/rule.yaml .

# Use registry rules
semgrep --config=p/default .

# JSON output
semgrep --config=auto --json .

# Git-aware (scan only changed files)
semgrep --config=auto --baseline-commit main .

# Interfile scan
semgrep --pro --config=p/default .

# Pre-commit hook
semgrep --config=auto --error .
```

## Install

```bash
# Homebrew
brew install semgrep

# pip
pip3 install semgrep

# Docker
docker pull semgrep/semgrep
docker run --rm -v $(pwd):/src semgrep/semgrep semgrep --config=auto /src

# GitHub Action
# uses: semgrep/semgrep-action@v1
```

## Build

```bash
git clone https://github.com/semgrep/semgrep.git
cd semgrep
make install
# Requires OCaml + Python + Go
```

## Package

| Manager | Command |
|---------|---------|
| Pip | `pip3 install semgrep` |
| Brew | `brew install semgrep` |
| Docker | `semgrep/semgrep` |
| npm | `npm install -g semgrep` (unofficial) |

Free tier: 2,000+ community rules. Pro tier: interfile analysis, taint mode, 2,000+ pro rules, priority support.

## Links

| Resource | URL |
|----------|-----|
| Official site | https://semgrep.dev/ |
| GitHub | https://github.com/semgrep/semgrep |
| Docs | https://semgrep.dev/docs/ |
| Registry | https://semgrep.dev/r |
| Rule writing | https://semgrep.dev/docs/writing-rules/overview/ |
| Playground | https://semgrep.dev/playground |
