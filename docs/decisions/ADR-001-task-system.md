# ADR-001: File-based task persistence system

## Status
Accepted

## Date
2026-07-16

## Context
rapiscm needed a way to persist, compare, and export scan results. Requirements:
- No database dependency (keep binary self-contained)
- Survive crashes and reboots
- Support listing, showing, deleting, pruning old tasks
- Support diff between two tasks for regression detection
- Support export to multiple formats (markdown, SARIF, HTML)
- Support resume of partially-completed scans
- Support queue for batch scanning

## Decision
Use a flat-file JSON storage system at `~/.local/share/rapiscm/tasks/`.

Each task gets a directory named `{id}-{name}/` containing:
- `task.json` — full config + metadata + result summary
- `results.json` — full `Vec<ResponseResult>` array
- `index.json` — lightweight index for fast listing
- `queue.json` — persistent queue with crash recovery
- `raw/` — optional raw HTTP request/response pairs (with `--raw`)

## Alternatives Considered

### SQLite via rusqlite
- Pros: ACID, queries, indexing, less I/O
- Cons: Adds ~200KB to binary, requires build-time C lib linking
- Rejected: Overkill for ~500KB JSON files per scan

### In-memory only
- Pros: Simplest implementation
- Cons: No persistence across runs, no diff, no export
- Rejected: Task system would lose all value

## Consequences
- Zero new dependencies (JSON via existing serde_json)
- Queue mutations are atomic (write to temp file, rename over old)
- Loading full results for large scans reads entire file into memory (~500KB typical)
- Raw mode can use significant disk (~1MB+ per scan with request/response pairs)
- No concurrent write safety (single-user CLI tool)
