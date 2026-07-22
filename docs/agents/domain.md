# Domain Docs

**Layout:** Single-context. One `CONTEXT.md` at the repo root, one `docs/adr/` directory
for architecture decision records.

## Consumer rules

Skills that read domain context (`improve-codebase-architecture`, `diagnosing-bugs`,
`tdd`, `to-spec`, `to-tickets`, `planning-and-task-breakdown`) obey these rules:

1. **Read `CONTEXT.md` first** — it defines the project's ubiquitous language, domain
   model, and key invariants. Use its vocabulary throughout specs, tickets, and ADRs.
2. **Consult `docs/adr/`** before proposing architectural changes — existing ADRs may
   already constrain the design space.
3. **If `CONTEXT.md` is missing**, infer domain language from the codebase and flag
   gaps. The `ubiquitous-language` skill can generate a draft.
4. **If `docs/adr/` is missing**, create it and write the first ADR for any
   non-trivial architectural decision made during implementation.

## Files

```
<repo-root>/
├── CONTEXT.md             # domain glossary, invariants, entity relationships
├── docs/
│   └── adr/
│       ├── 001-use-sqlite-for-metadata.md
│       ├── 002-plugin-system-extism.md
│       └── ...
└── docs/agents/           # this config directory
    ├── issue-tracker.md
    ├── triage-labels.md
    └── domain.md
```
