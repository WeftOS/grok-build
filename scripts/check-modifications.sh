#!/usr/bin/env bash
# Fails if a file changed relative to the upstream baseline is not listed in
# MODIFICATIONS.md, or if an MPL-2.0 file (docs/tapestry/mpl-files.md) is
# changed without an "(MPL-2.0)" marker on its MODIFICATIONS.md line.
set -euo pipefail

BASE="${BASELINE:-2bdd1d6a}"
cd "$(git rev-parse --show-toplevel)"

changed=$(
  {
    git diff --name-only "$BASE"...HEAD
    git diff --name-only
    git diff --name-only --cached
    git ls-files --others --exclude-standard
  } | sort -u
)

# A path is "listed" when it appears in backticks on a MODIFICATIONS.md list line.
listed() { grep -E '^- `' MODIFICATIONS.md | grep -qF "\`$1\`"; }

mpl_paths=$(sed -n '/^## In-tree MPL-2.0 files/,/^## /p' docs/tapestry/mpl-files.md \
  | grep -E '^- `' | sed -E 's/^- `([^`]+)`.*/\1/' || true)

fail=0
while IFS= read -r p; do
  [ -z "$p" ] && continue
  if ! listed "$p"; then
    echo "NOT LISTED in MODIFICATIONS.md: $p"; fail=1
  fi
  if printf '%s\n' "$mpl_paths" | grep -qxF "$p"; then
    if ! grep -E '^- `' MODIFICATIONS.md | grep -F "\`$p\`" | grep -qF '(MPL-2.0)'; then
      echo "MPL-2.0 file changed without (MPL-2.0) marker: $p"; fail=1
    fi
  fi
done <<< "$changed"

if [ "$fail" -ne 0 ]; then
  echo "check-modifications: FAIL"; exit 1
fi
echo "check-modifications: OK ($(printf '%s\n' "$changed" | grep -c .) changed paths, all listed)"
