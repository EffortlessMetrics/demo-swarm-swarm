#!/usr/bin/env bash
# Pack Check Shim
# Thin wrapper that prefers the Rust pack-check binary over bash.
# Run from repo root: bash .claude/scripts/pack-check.sh [OPTIONS]
#
# Resolution order:
#   1. .demoswarm/bin/pack-check (installed binary)
#   2. pack-check on PATH (global install)
#   3. cargo run fallback (dev: only if tools/demoswarm-pack-check exists)
#
# This shim ensures backward-compatible invocations while delegating
# all actual validation to the Rust implementation.

set -euo pipefail

find_repo_root() {
  local dir="$PWD"
  while [[ "$dir" != "/" ]]; do
    if [[ -d "$dir/.claude" ]]; then
      echo "$dir"
      return 0
    fi
    dir="$(cd "$dir/.." && pwd)"
  done
  echo "Error: Could not find repo root (missing .claude/)" >&2
  return 1
}

REPO_ROOT="$(find_repo_root)"

# Build candidate list (check both unix and windows binary names)
TOOL_CANDIDATES=(
  "$REPO_ROOT/.demoswarm/bin/pack-check"
  "$REPO_ROOT/.demoswarm/bin/pack-check.exe"
)

if command -v pack-check >/dev/null 2>&1; then
  TOOL_CANDIDATES+=("$(command -v pack-check)")
fi

# Try each candidate
for tool in "${TOOL_CANDIDATES[@]}"; do
  if [[ -n "$tool" && -x "$tool" ]]; then
    # Convert Unix path to Windows path when calling Windows executable
    if [[ "$tool" == *.exe ]]; then
      # Convert /mnt/c/path to C:\path format for Windows executables
      REPO_ROOT_WIN=$(echo "$REPO_ROOT" | sed 's|^/mnt/\([a-z]\)|\1:|' | sed 's|/|\\|g')
      exec "$tool" --repo-root "$REPO_ROOT_WIN" "$@"
    else
      exec "$tool" --repo-root "$REPO_ROOT" "$@"
    fi
  fi
done

# 3. Cargo run fallback (only in pack dev repo with tools/ present)
CARGO_MANIFEST="$REPO_ROOT/tools/demoswarm-pack-check/Cargo.toml"
if [[ -f "$CARGO_MANIFEST" ]] && command -v cargo >/dev/null 2>&1; then
  # Convert Unix path to Windows path when running on Windows with cargo
  if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    REPO_ROOT_WIN=$(echo "$REPO_ROOT" | sed 's|^/mnt/\([a-z]\)|\1:|' | sed 's|/|\\|g')
    CARGO_MANIFEST_WIN=$(echo "$CARGO_MANIFEST" | sed 's|^/mnt/\([a-z]\)|\1:|' | sed 's|/|\\|g')
    exec cargo run --quiet --manifest-path "$CARGO_MANIFEST_WIN" -- --repo-root "$REPO_ROOT_WIN" "$@"
  else
    exec cargo run --quiet --manifest-path "$CARGO_MANIFEST" -- --repo-root "$REPO_ROOT" "$@"
  fi
fi

# No implementation found
echo "pack-check not installed." >&2
echo "" >&2
echo "Install (repo-local):" >&2
echo "  cargo install --path tools/demoswarm-pack-check --root .demoswarm" >&2
echo "" >&2
echo "Then run:" >&2
echo "  bash .claude/scripts/pack-check.sh" >&2
exit 1
