# Issue Tracker

**Primary tracker:** GitHub Issues on the repo's remote.

Issues are created via `gh issue create`. The `to-spec`, `to-tickets`, `triage`,
and `qa` skills use the `gh` CLI.

**Local mirror:** `.scratch/` directory at the repo root mirrors all issues as
markdown files for offline access. The `.scratch/` store is always kept in sync
with GitHub Issues.

## Layout

```
.scratch/
├── feature-name/          # one directory per feature
│   ├── spec.md            # the spec (written by to-spec)
│   ├── tickets.md         # tracer-bullet tickets (written by to-tickets)
│   └── notes.md           # ad-hoc notes
├── bugs/                  # bug reports (written by qa)
└── chores/                # maintenance, refactors
```

## External PRs as a request surface

**No.** PRs are not treated as a triage surface. Only issues go through the
triage pipeline.
