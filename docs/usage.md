# Usage

## Synopsis

```
rapiscm [command] [target] [flags]
```

## Commands

### `scan` — auto-detect

Detects whether target is a spec file or URL:

```sh
rapiscm scan ./openapi.json
rapiscm scan https://api.example.com
```

Auto-detection logic: if the target has a `.json`, `.yaml`, or `.yml` extension → spec mode; otherwise → URL mode.

### `spec` — OpenAPI spec scan

```sh
rapiscm spec ./openapi.json
rapiscm spec ./api.yaml
```

Parses OpenAPI 3.x spec (JSON/YAML), extracts all endpoints from all paths, and scans each one.

### `url` — URL scan

```sh
rapiscm url https://api.example.com
rapiscm url https://example.com
```

Probes common API paths, discovers endpoints from HTML/Swagger/JSON, and scans them.

### `corp` — corporate domain discovery

```sh
rapiscm corp "Acme Corp"
```

Discovers domains owned by an organisation via crt.sh, RDAP, Google CSE, Shodan, favicon hashes, and ASN lookups.

Also available as `--corp` flag with `scan`/`url`/`spec`:

```sh
rapiscm scan https://example.com --corp
# auto-detects org from URL
```

### `fuzz` — endpoint fuzzing

```sh
rapiscm fuzz https://target.com/api/v1/users/FUZZ -w wordlist.txt
```

Fuzz endpoints with a wordlist. Supports match/filter on status codes, response sizes, and body regex.

| Flag | Description |
|---|---|
| `-w, --wordlist` | Wordlist file path or built-in name |
| `-e, --extensions` | Extensions to append (comma-separated) |
| `--mc` | Match status codes (e.g. `200,200-299`) |
| `--fc` | Filter status codes |
| `--ms` | Match response size range |
| `--fs` | Filter response size |
| `--mr` | Regex match on response body |
| `--fr` | Regex filter on response body |
| `--ac` | Auto-calibrate filters |

### `tasks` — scan task management

```sh
rapiscm tasks list
rapiscm tasks show <id>
rapiscm tasks delete <id>
rapiscm tasks prune --keep 10
rapiscm tasks export <id> --format md -o report.md
rapiscm tasks diff <old_id> <new_id>
```

Tasks persist scan results for later review, export, and diffing.

Save a scan as a task:

```sh
rapiscm scan https://example.com --save --task-name "weekly-scan"
```

Export formats: `md` (markdown), `sarif`, `html`.

### `session` — replay recorded session

```sh
rapiscm session session.jsonl
rapiscm session session.jsonl --timing
```

Replays a JSONL session file, re-running all requests with live probes. Optional timing analytics (bursts, gaps, rate limits).

| Flag | Description |
|---|---|
| `--timing` | Show timing analytics |
| `--max-parse-errors` | Max malformed lines before aborting (default: 10) |
| `--skip-cors` | Skip CORS preflight probes |
| `--skip-auth` | Skip auth-enforcement probes |

## Global flags

| Flag | Description | Default |
|---|---|---|
| `-H, --header` | Custom header (`Key: Value`, repeatable) | — |
| `--auth` | Auth config: `bearer:<token>`, `basic:<user:pass>`, `header:<name:value>` | — |
| `--rate-limit` | Requests per second cap | 50 |
| `--timeout` | Per-request timeout in seconds | 30 |
| `--concurrency` | Max concurrent requests | 10 |
| `-o, --output` | Output format: `table`, `json`, `md` | table |
| `--follow-redirects` | Follow 3xx redirects | false |
| `-k, --insecure` | Skip TLS verification | false |
| `--paths` | Path filter (comma-separated) | — |
| `--tags` | OpenAPI tag filter (comma-separated) | — |
| `--filter-tag` | Include endpoints matching ALL of these tags | — |
| `--exclude-tag` | Exclude endpoints matching ANY of these tags | — |
| `--proxy` | Proxy URL (e.g. `http://127.0.0.1:8080`) | — |
| `--crawl` | Enable recursive crawl for deeper discovery | false |
| `--depth` | Max crawl depth (with `--crawl`) | 2 |
| `--filter-path` | Glob path include filter | — |
| `--exclude-path` | Glob path exclude filter | — |
| `--filter-method` | Method include filter | — |
| `--exclude-method` | Method exclude filter | — |
| `--filter-status` | Status range include (e.g. `200,200-299`) | — |
| `--exclude-status` | Status range exclude | — |
| `--filter` | Expression filter (e.g. `tag:rest AND status:2xx`) | — |
| `--exclude` | Expression exclude | — |
| `--show-tags` | Show tags in report | false |
| `--no-trackers` | Disable tracker/analytics detection | false |
| `--corp` | Company name for discovery (scan + discover) | — |
| `--save` | Save scan results as a task | false |
| `--task-name` | Label for saved task | — |
| `--task-tag` | Tags for saved task (repeatable) | — |
| `--no-bodies` | Do NOT store response bodies in task | false |
| `--raw` | Store raw endpoint files in task directory | false |
| `--task-dir` | Task storage directory | `~/.local/share/rapiscm/tasks` |
| `--resume` | Task ID to resume (re-scan failed/incomplete) | — |
| `--git` | Capture git context when saving task | false |
| `--log-level` | Log level: error, warn, info, debug, trace | info |
| `--log-filter` | Module-level log filters (e.g. `rapiscm::scan=debug`) | — |
| `--log-format` | Log output format: text, json | text |

### Browser flags (requires `--features browser`)

| Flag | Description | Default |
|---|---|---|
| `--browser` | Browser engine: `chrome`, `firefox` | chrome |
| `--headed` | Show browser GUI (non-headless) | false |

## Examples

### Basic scan

```sh
rapiscm scan https://petstore.swagger.io/v2
```

### Scan with auth and custom header

```sh
rapiscm url https://api.example.com -H "X-API-Key: abc123" --auth "bearer:eyJ..."
```

### Scan with filtering

```sh
rapiscm spec api.json --filter-tag users --exclude-method DELETE
```

### Scan behind a proxy

```sh
rapiscm scan https://internal.api.corp --proxy http://127.0.0.1:8080 -k
```

### Deep crawl with browser discovery

```sh
rapiscm url https://spa.example.com --crawl --depth 3 --browser chrome
```

### Export task as SARIF

```sh
rapiscm tasks export 1 --format sarif -o results.sarif
```

### Diff two scans

```sh
rapiscm tasks diff 1 2
```

### Fuzz with filters

```sh
rapiscm fuzz https://target.com/FUZZ -w raft-large.txt --mc 200,201,403 --ac
```

### Corporate domain discovery

```sh
rapiscm corp "Example Corp"
```

### Scan with discovery + task save

```sh
rapiscm scan https://example.com --corp "Example Corp" --save --task-name "full-audit" --git
```

### Resume failed scan

```sh
rapiscm scan https://api.example.com --resume 3
```
