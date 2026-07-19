#!/bin/bash
# sync-wiki.sh — sync docs/ → GitHub Wiki
# Usage: ./scripts/sync-wiki.sh [--push]
# Without --push: copies docs/ to wiki clone
# With --push: also commits and pushes

set -e
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WIKI_TMP="/tmp/rapiscm-wiki-$$"

cleanup() { rm -rf "$WIKI_TMP"; }
trap cleanup EXIT

REPO="grave0x/rapiscm"

echo "==> Cloning wiki..."
git clone "https://github.com/${REPO}.wiki.git" "$WIKI_TMP"

echo "==> Copying docs..."
# Core docs
cp "$ROOT/README.md" "$WIKI_TMP/Home.md"
cp "$ROOT/CHANGELOG.md" "$WIKI_TMP/Changelog.md"
cp "$ROOT/docs/installation.md" "$WIKI_TMP/Installation.md"
cp "$ROOT/docs/usage.md" "$WIKI_TMP/Usage.md"
cp "$ROOT/docs/features.md" "$WIKI_TMP/Features.md"
cp "$ROOT/docs/architecture.md" "$WIKI_TMP/Architecture.md"
cp "$ROOT/docs/contributing.md" "$WIKI_TMP/Contributing.md"
cp "$ROOT/docs/roadmap.md" "$WIKI_TMP/Roadmap.md"

# ADRs
mkdir -p "$WIKI_TMP/decisions"
for adr in "$ROOT"/docs/decisions/ADR-*.md; do
  cp "$adr" "$WIKI_TMP/decisions/"
done

# Create _Sidebar
cat > "$WIKI_TMP/_Sidebar.md" << 'SIDEBAR'
## rapiscm

- [Home](Home)
- [Installation](Installation)
- [Usage](Usage)
- [Features](Features)
- [Architecture](Architecture)
- [Contributing](Contributing)
- [Roadmap](Roadmap)
- [Changelog](Changelog)

### Architecture decisions
- [ADR-001: Task system](decisions/ADR-001-task-system)
- [ADR-002: Ghost mode](decisions/ADR-002-ghost-mode)
- [ADR-003: JS bundle scanning](decisions/ADR-003-js-bundle-scanning)
SIDEBAR

# Create _Footer
echo "Powered by [rapiscm](https://github.com/${REPO})" > "$WIKI_TMP/_Footer.md"

echo "===> Files staged in $WIKI_TMP:"
ls -la "$WIKI_TMP/"*.md

if [ "${1:-}" = "--push" ]; then
  cd "$WIKI_TMP"
  git add -A
  git commit -m "sync wiki from docs/ [$(date -u +%Y-%m-%d)]"
  git push origin master
  echo "==> Wiki pushed successfully!"
else
  echo "==> Dry run. Use --push to commit and push."
  echo "    Files staged at $WIKI_TMP"
fi
