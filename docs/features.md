# Features

## Scan modes

| Mode | Description |
|---|---|
| **Spec scan** (`rapiscm spec`) | Parse OpenAPI 3.x JSON/YAML spec, resolve all endpoints, scan each one |
| **URL scan** (`rapiscm url`) | Probe common API paths, discover endpoints from HTML/Swagger/robots.txt, scan discovered paths |
| **Auto-detect** (`rapiscm scan`) | Detect spec file vs URL by extension, dispatch to spec or URL mode |
| **Fuzz** (`rapiscm fuzz`) | Fuzz endpoints with wordlist, match/filter by status, size, regex |
| **Session replay** (`rapiscm session`) | Replay recorded JSONL session with live probes, timing analytics |
| **Corporate discovery** (`rapiscm corp`) | Discover owned domains for an organisation via multiple OSINT sources |
| **Page capture** (`rapiscm capture`) | Download page HTML, extract JS API endpoints, take screenshot |

## Endpoint discovery

- **OpenAPI spec parsing** — full OpenAPI 3.x support (JSON/YAML)
- **HTML extraction** — parse `<a href>`, `<form action>`, `<link>`, `<script src>`, `<img src>` from page HTML
- **JavaScript extraction** — discover API paths from JS string literals
- **JSON discovery** — probe for `/api`, `/swagger.json`, `/openapi.json`, `/api/docs`
- **Sitemap parsing** — extract endpoints from `/sitemap.xml`
- **robots.txt parsing** — extract disallowed paths as potential endpoints
- **Crawl mode** (`--crawl html|js|full`) — recursive link following with optional JS bundle scanning
- **JS bundle scanning** (`--crawl js`) — download `<script src>` bundles, extract API routes from minified code
- **Ghost mode** (`--ghost`) — stealth scanning with UA rotation, request jitter, header randomization, proxy rotation
- **Browser JS eval** (`--eval <js>`) — run custom JS in headless browser, extract returned URLs as endpoints
- **Browser discovery** (`--features browser`) — headless Chrome/Firefox for JS-rendered content
- **Built-in wordlists** — common API paths, RESTful patterns, admin paths

## Security checks

### Synchronous (per-endpoint)

| Check module | What it detects |
|---|---|
| **Security headers** | CSP, HSTS, X-Frame-Options, X-Content-Type-Options, X-XSS-Protection, Referrer-Policy, Permissions-Policy |
| **Response schema** | Status code validation against expected, body presence, content-type matching |
| **Tracker detection** | Google Analytics, Facebook Pixel, Hotjar, Mixpanel, Amplitude, CrazyEgg, Clicky, FullStory, LinkedIn Insights, Twitter Pixel, HubSpot, Reddit, TikTok, Bing, Segment, Heap, Kissmetrics, Mouseflow, Smartlook, PostHog, Plausible, Fathom, Matomo, Cloudflare, New Relic, Datadog, Sentry, Rollbar, LogRocket, BugSnag |
| **Cookie analysis** | Missing Secure/HttpOnly/SameSite flags, persistent cookies without expiry |

### Asynchronous (per-endpoint, concurrent)

| Check module | What it detects |
|---|---|
| **CORS preflight** | `Access-Control-Allow-Origin` reflection, credential support, wildcard origins |
| **Auth enforcement** | Whether removing auth headers changes the response (auth bypass detection) |

### Check severity levels

- **Info** — informational observation
- **Warn** — potential misconfiguration
- **Critical** — security vulnerability

## Filters (include/exclude)

Control which endpoints appear in output:

| Filter type | By |
|---|---|
| `--paths` | Comma-separated path prefixes |
| `--tags` | OpenAPI spec tags |
| `--filter-tag` / `--exclude-tag` | Tag include/exclude |
| `--filter-path` / `--exclude-path` | Glob path patterns |
| `--filter-method` / `--exclude-method` | HTTP methods |
| `--filter-status` / `--exclude-status` | Status code ranges |
| `--filter` / `--exclude` | Expression syntax (e.g. `tag:rest AND status:2xx`) |

## Output formats

| Format | Command | Use case |
|---|---|---|
| **Table** (`-o table`) | Terminal output with ANSI colours | Interactive use |
| **JSON** (`-o json`) | Structured JSON lines | Machine processing, CI |
| **Markdown** (`-o md`) | Markdown tables | Reports, documentation |
| **Doc** (`-o doc`) | Structured API documentation (llm-api-style) | API docs generation |
| **HTML site** (`--report <name>`) | Full static HTML report site with navigation | Visual sharing |

## Task system

Persist, manage, and compare scan results:

| Action | Description |
|---|---|
| `--save` | Save scan results as a task |
| `tasks list` | List saved tasks |
| `tasks show <id>` | Show task details |
| `tasks delete <id>` | Delete a task |
| `tasks prune --keep N` | Keep N newest, delete rest |
| `tasks export <id> --format md|sarif|html` | Export to file |
| `tasks diff <old> <new>` | Diff two scans (status, time, checks, body size changes) |
| `--git` | Capture git SHA/branch/message with task |
| `--task-tag` | Tag tasks for organisation |
| `--resume <id>` | Re-scan failed/incomplete endpoints |

## Domain discovery (OSINT)

| Source | Data | Requires API key |
|---|---|---|
| **crt.sh** | TLS certificate subject names (Certificate Transparency logs) | No |
| **RDAP** | Organisation registration data | No |
| **Google CSE** | Web search results | Yes |
| **Shodan** | IP/hostname matches | Yes |
| **Favicon hash** | Icon-based domain correlation | No |
| **ASN query** | IP range → organisation mapping | No |

## Authentication modes

| Mode | Example |
|---|---|
| Bearer token | `--auth bearer:eyJhbGci...` |
| Basic auth | `--auth basic:admin:hunter2` |
| Custom header | `--auth header:X-API-Key:secret123` |

## HTTP client capabilities

- Configurable rate limiting (`--rate-limit`)
- Configurable concurrency (`--concurrency`)
- Configurable per-request timeout (`--timeout`)
- Proxy support (`--proxy`)
- Redirect following (`--follow-redirects`)
- TLS verification bypass (`--insecure`)
- Custom headers (`-H`)

## Project module map

| Module | Public API | Purpose |
|---|---|---|---|
| `analytics` | `detect.rs`, `sigdb.rs` | Tracker/analytics signature detection |
| `check` | `mod.rs`, `auth.rs`, `cors.rs`, `schema.rs`, `security.rs`, `trackers.rs` | Security checks (sync + async) |
| `cli` | `cli.rs` | Clap CLI definition |
| `config` | `config.rs` | ScanConfig builder |
| `discover` | `asn.rs`, `crtsh.rs`, `favicon.rs`, `gaid.rs`, `rdap.rs`, `search.rs` | OSINT domain discovery |
| `error` | `error.rs` | Error types (thiserror) |
| `extract` | `headers.rs`, `html.rs`, `js.rs`, `json.rs`, `sitemap.rs` | Endpoint extraction from various sources |
| `filter` | `mod.rs` | Endpoint filter expression parsing |
| `fuzz` | `runner.rs`, `wordlist.rs`, `matcher.rs` | Fuzzing engine |
| `ghost` | `ghost.rs` | Stealth scanning (UA rotation, jitter, header randomization, proxy rotation) |
| `parser` | `spec.rs`, `url.rs`, `js_bundle.rs` | OpenAPI / URL / JS bundle endpoint parsing |
| `report` | `table.rs`, `json.rs`, `summary.rs`, `doc.rs`, `site.rs` | Output formatters (table, JSON, markdown, doc, HTML site) |
| `scan` | `runner.rs`, `spec.rs`, `url.rs`, `browser.rs` | Scan pipeline orchestration |
| `session` | `parse.rs`, `timing.rs` | Session replay engine |
| `tag` | `mod.rs` | Tag management |
| `task` | `store.rs`, `index.rs`, `export.rs`, `diff.rs`, `resume.rs`, `rebuild.rs`, `queue.rs` | Persistence, export, diffing |
| `types` | `types.rs` | Core data types |
| `util` | `util.rs` | ISO timestamps, git info |
