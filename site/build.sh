#!/usr/bin/env bash
# Build the vault in docs/ as a website with MathStruct's Quartz
# (https://github.com/MathStruct/quartz: Quartz v4 plus a shared preset). Needs git and Node >= 22.
#
#   site/build.sh            # -> site/public/
#   site/build.sh --serve    # live preview on http://localhost:8080, rebuilt as you edit
#
# Quartz is cloned into site/.quartz/ (gitignored) and moved to QUARTZ_REF on every build.
# To hold the site at a version, set QUARTZ_REF to a tag or a commit.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONTENT="$HERE/../docs"
QUARTZ_REF="${QUARTZ_REF:-main}"

ENGINE="${QUARTZ_DIR:-$HERE/.quartz}"
if [[ -z "${QUARTZ_DIR:-}" ]]; then
  [[ -d "$ENGINE/.git" ]] || git clone -q --depth 1 https://github.com/MathStruct/quartz.git "$ENGINE"
  if git -C "$ENGINE" fetch -q --depth 1 origin "$QUARTZ_REF"; then
    git -C "$ENGINE" checkout -q -f --detach FETCH_HEAD
  else
    echo ">> offline: building with site/.quartz as it is ($(git -C "$ENGINE" rev-parse --short HEAD))"
  fi
fi
exec "$ENGINE/mathstruct/site.sh" "$HERE" "$CONTENT" "$@"
