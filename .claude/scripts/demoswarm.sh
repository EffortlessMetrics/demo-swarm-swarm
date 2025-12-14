#!/usr/bin/env bash
# DemoSwarm Runs-Tools Shim
# Thin wrapper that prefers the Rust demoswarm binary over fallbacks.
# Run from repo root: bash .claude/scripts/demoswarm.sh <command> [OPTIONS]
#
# Resolution order:
#   1. .demoswarm/bin/demoswarm (installed Rust binary)
#   2. demoswarm on PATH (global install)
#   3. cargo run fallback (dev: only if tools/demoswarm-runs-tools exists)
#   4. Python fallback (runs_tools.py with argparse subcommands)
#
# Agents should always invoke via this shim for stable behavior.

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
  "$REPO_ROOT/.demoswarm/bin/demoswarm"
  "$REPO_ROOT/.demoswarm/bin/demoswarm.exe"
)

if command -v demoswarm >/dev/null 2>&1; then
  TOOL_CANDIDATES+=("$(command -v demoswarm)")
fi

# 1-2. Try Rust binary candidates
for tool in "${TOOL_CANDIDATES[@]}"; do
  if [[ -n "$tool" && -x "$tool" ]]; then
    exec "$tool" "$@"
  fi
done

# 3. Cargo run fallback (only in pack dev repo with tools/ present)
CARGO_MANIFEST="$REPO_ROOT/tools/demoswarm-runs-tools/Cargo.toml"
if [[ -f "$CARGO_MANIFEST" ]] && command -v cargo >/dev/null 2>&1; then
  exec cargo run --quiet --manifest-path "$CARGO_MANIFEST" -- "$@"
fi

# 4. Python fallback - single consolidated CLI (lives in runs-derive skill)
FALLBACK="$REPO_ROOT/.claude/skills/runs-derive/fallback/runs_tools.py"
if [[ -f "$FALLBACK" ]]; then
  if command -v python3 >/dev/null 2>&1; then
    exec python3 "$FALLBACK" "$@"
  elif command -v python >/dev/null 2>&1; then
    exec python "$FALLBACK" "$@"
  fi
fi

# No implementation found
echo "demoswarm: no implementation found" >&2
echo "" >&2
echo "Install (repo-local):" >&2
echo "  cargo install --path tools/demoswarm-runs-tools --root .demoswarm" >&2
echo "" >&2
echo "Usage:" >&2
echo "  bash .claude/scripts/demoswarm.sh <command> [options]" >&2
echo "" >&2
echo "Commands:" >&2
echo "  count pattern    Count lines matching regex" >&2
echo "  count bdd        Count BDD scenarios" >&2
echo "  ms get           Extract Machine Summary field" >&2
echo "  yaml get         Extract YAML block field" >&2
echo "  index upsert-status  Update index.json" >&2
echo "  time now         Get ISO8601 timestamp" >&2
exit 1
