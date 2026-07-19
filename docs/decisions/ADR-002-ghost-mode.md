# ADR-002: Ghost mode — stealth scanning via UA rotation, jitter, header randomization

## Status
Accepted

## Date
2026-07-19

## Context
API scanners are easily detected and blocked by WAFs, rate limiters, and bot detection services. rapiscm needed a stealth mode that:
- Avoids fingerprinting by rotating User-Agent across real browser strings
- Randomizes request timing to avoid pattern detection
- Randomizes Accept/Accept-Language/Accept-Encoding headers
- Supports proxy rotation to distribute requests across IPs
- Maintains a cookie jar for session consistency
- Can be enabled with a single `--ghost` flag

## Decision
Create a dedicated `ghost.rs` module with `GhostConfig` and `GhostState` structs.

`GhostConfig` holds static configuration (enabled, jitter percentage, UA mode, proxy list).
`GhostState` holds runtime state (RNG, rotation counters).

Ghost state is threaded through the scan pipeline:
1. `build_client()` applies ghost proxy + UA to the reqwest Client
2. `ghost_headers()` generates randomized Accept/Accept-Language/Encoding headers per request
3. `ScanRunner` applies jitter (±jitter_pct%) to rate delay
4. Proxy rotation uses round-robin through `--proxy-rotate` list

UA rotation supports modes: `desktop` (6 UAs), `mobile` (2 UAs), `random` (any from pool).

## Alternatives Considered

### Random per-request without state
- Pros: Simpler implementation
- Cons: Can't round-robin proxies; harder to debug
- Rejected: GhostState adds minimal complexity for significant debugging benefit

### External proxy rotation via separate tool (proxychains)
- Pros: Zero code
- Cons: Not portable, adds deploy dependency, can't rotate per-request
- Rejected: Native rotation is more reliable and cross-platform

### TOR integration
- Pros: Strong anonymity
- Cons: Requires TOR daemon, slow, overkill for API scanning
- Rejected: Proxy rotation is sufficient for WAF evasion

## Consequences
- No new dependencies (uses existing `rand` crate)
- Ghost mode adds ~0.5-2ms overhead per request (RNG call)
- Proxy rotation works with HTTP, HTTPS, and SOCKS5 proxies
- Jitter applied on top of rate limit (target rate is approximate with jitter)
