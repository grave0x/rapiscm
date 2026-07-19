# Wiki

## Documentation index

| Document | Description |
|---|---|
| [README](../README.md) | Project overview, quick start, conventions |
| [Installation](installation.md) | Install from source, with browser features, with cargo |
| [Build](build.md) | Build modes, features, cross-compilation, CI, troubleshooting |
| [Usage](usage.md) | Full CLI reference with all commands, flags, examples |
| [Features](features.md) | Feature catalog with module map and capabilities |
| [Architecture](architecture.md) | Internal design, data flow, module dependencies |
| [Contributing](contributing.md) | Coding standards, PR process, testing, release |
| [Roadmap](roadmap.md) | Short/medium/long-term plans, known gaps |
| [Security Topics](security-topic.md) | Reference material on 18 security disciplines |

## External links

- **Source**: [github.com/grave0x/rapiscm](https://github.com/grave0x/rapiscm)
- **Wiki**: [github.com/grave0x/rapiscm/wiki](https://github.com/grave0x/rapiscm/wiki)
- **Issues**: [github.com/grave0x/rapiscm/issues](https://github.com/grave0x/rapiscm/issues)
- **Releases**: [github.com/grave0x/rapiscm/releases](https://github.com/grave0x/rapiscm/releases)
- **CI status**: [github.com/grave0x/rapiscm/actions](https://github.com/grave0x/rapiscm/actions)
- **crates.io**: [crates.io/crates/rapiscm](https://crates.io/crates/rapiscm) *(pending publish)*

## Release notes

See [Releases](https://github.com/your-org/rapiscm/releases) for changelog.

## Frequently Asked Questions

### What is rapiscm?

Rust-based API security scanner. Point it at an OpenAPI spec or a URL and it discovers endpoints, runs security checks, and outputs findings in table/JSON/markdown format.

### How is this different from other scanners?

- Rust performance — fast concurrent scanning with low resource usage
- Multiple scan modes — spec, URL, fuzz, session replay, corporate discovery
- Built-in security checks — headers, CORS, auth enforcement, tracker detection
- Task system — persist, compare, and export scan results
- Browser discovery — optional headless Chrome/Firefox for JS-rendered SPAs
- Minimal dependencies — compiled binary with no runtime or database

### Does it require API keys?

Only for `rapiscm corp` with Google CSE and Shodan sources. crt.sh, RDAP, favicon, and ASN sources work without keys.

### Can it scan GraphQL?

Not yet. See [roadmap](roadmap.md).

### Can it authenticate?

Yes. `--auth` supports `bearer:<token>`, `basic:<user:pass>`, and `header:<name:value>`.
