# ADR-004: Auto-sync docs to GitHub Wiki

## Status
Accepted

## Date
2026-07-19

## Context

rapiscm's user-facing documentation lives in `docs/` (Markdown files for installation, usage, architecture, contributing, roadmap, features) plus `README.md` as the project homepage.

Users browse docs in three contexts:
1. **On GitHub** — view source Markdown in the repo tree
2. **Via `cargo doc`** — API docs from doc comments (separate concern)
3. **On the GitHub Wiki** — browseable, searchable, linkable, no repo clone needed

Without sync, the Wiki drifts from `docs/` — stale content, missing updates. Manual copy is error-prone and skipped under time pressure.

GitHub Wikis are a separate Git repository (`<owner>/<repo>.wiki.git`). They support Markdown, sidebars, and free-form page hierarchy — good fit for user-facing project documentation.

## Decision

Auto-sync `docs/*.md` and `README.md` (as `Home.md`) to the GitHub Wiki on every push that touches `docs/**` or `README.md`.

Implementation: a GitHub Actions workflow (`.github/workflows/sync-wiki.yml`) that:

1. Triggers on `push` to `main`/`master` when `docs/**` or `README.md` changes
2. Checks out the repo and the wiki repo (`<repo>.wiki`)
3. Overwrites wiki files with `docs/*.md` + `README.md → Home.md`
4. Commits and pushes if there are changes

## Alternatives Considered

### Manual wiki editing
- Pros: Full control, can rearrange pages differently
- Cons: Drifts immediately, no one does it
- Rejected: Drift is worse than rigid structure

### mkdocs + GitHub Pages
- Pros: Richer site (nav bars, search, themes)
- Cons: Separate build step, needs a `gh-pages` branch, more CI complexity
- Deferred: Good option if wiki becomes insufficient

### docsify / readthedocs
- Pros: Zero-build rendering, hosting flexibility
- Cons: Adds a dependency, overkill for current project size
- Deferred: Revisit if project needs grow

## Consequences

- Wiki always reflects `docs/` on `main` — single source of truth
- ADRs in `docs/decisions/` are automatically published as wiki pages
- Pages must be named by their filename (no custom wiki page hierarchy)
- The wiki's sidebar must be manually updated if the doc list changes (GitHub Wiki sidebar is not synced by this workflow)
- First push fails if the wiki repo doesn't exist yet (user must initialize wiki once via GitHub UI)
