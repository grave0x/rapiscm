# ADR-003: Static JS bundle scanning for API endpoint discovery

## Status
Accepted

## Date
2026-07-19

## Context
Modern web applications (SPAs built with React, Vue, Angular, Next.js) register API endpoints dynamically through client-side JavaScript. Traditional wordlist-based URL discovery misses routes only referenced in JS bundles.

The existing `extract/js.rs` module only did basic URL extraction (matching `fetch()`, `axios.`, `$.ajax` calls). It couldn't:
- Download JS bundles from `<script src>` tags
- Parse minified code for route configuration objects
- Extract paths from template literals, concatenation patterns
- Discover GraphQL operation names
- Handle framework-specific patterns (Next.js API routes, route configs)

## Decision
Create a new `parser/js_bundle.rs` module that:
1. Fetches the target page HTML
2. Finds all `<script src>` tags
3. Downloads each JS bundle
4. Runs 11 extraction passes:
   - Route config objects (`{path: '/api/...'}`)
   - Fetch/XMLHttpRequest calls
   - API client calls (`.get()`, `.post()`, axios, etc.)
   - Template literals (`` `/api/${resource}` ``)
   - String concatenation patterns
   - GraphQL operation names
   - Import/require paths
   - String arrays of paths
   - Router link hrefs
   - Environment variable-based URL construction
   - Minified path references (webpack/rollup)
5. Returns structured `JsApiEndpoint` results with method hint, source, and confidence level

## Alternatives Considered

### Dynamic browser extraction (existing browser feature)
- Pros: Executes JS, catches dynamically-constructed routes
- Cons: Slow (browser launch + page load), high resource usage, only finds routes rendered in DOM
- Rejected as primary: Static analysis is faster and finds dormant routes

### Regex-only approach (status quo ante)
- Pros: Zero new code
- Cons: Misses route configs, template literals, concatenation, GraphQL ops
- Rejected: Systematic improvement needed

### Full AST parsing via tree-sitter or swc
- Pros: Semantic understanding, no false positives from strings
- Cons: Heavy dependency, slow on minified bundles, high maintenance
- Rejected: Regex with heuristics catches ~90% of real-world cases with zero deps

## Consequences
- Zero new dependencies (uses existing `regex` + `reqwest`)
- JS bundle scanning enabled via `--crawl js` or `--crawl full`
- Typical SPA yields 20-80 additional API endpoints from JS analysis
- ~200 LOC in the scanning module
- False positives possible from string literals that look like paths but aren't API endpoints
