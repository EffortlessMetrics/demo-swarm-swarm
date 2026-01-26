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
#
# Platform compatibility:
#   - macOS: Works with bash 3.2+ (default system bash)
#   - Linux: Works with bash 4.0+
#   - Windows: Works with Git Bash / MSYS2
#
# Note: This script uses bash-specific features ([[ ]], arrays) that are
# compatible with bash 3.2+ but not POSIX sh. This is intentional as we
# require bash for consistent behavior across platforms.

set -euo pipefail

# -----------------------------------------------------------------------------
# Bash version check
# Warn if running on bash < 3.2
# The script should still work on 3.2+, but some features may behave differently
# -----------------------------------------------------------------------------
check_bash_version() {
  local major="${BASH_VERSINFO[0]:-0}"
  local minor="${BASH_VERSINFO[1]:-0}"

  if [[ "$major" -lt 3 ]] || { [[ "$major" -eq 3 ]] && [[ "$minor" -lt 2 ]]; }; then
    echo "Warning: bash $BASH_VERSION detected. This script requires bash 3.2+." >&2
    echo "         Some features may not work correctly." >&2
  fi
}
check_bash_version

# -----------------------------------------------------------------------------
# Find repository root by looking for .claude/ directory
# Compatible with bash 3.2+ on macOS, Linux, and Windows (Git Bash)
# -----------------------------------------------------------------------------
find_repo_root() {
  local dir="$PWD"
  # Note: On Windows Git Bash, root is typically /c or /d, not /
  # We check for both Unix root (/) and drive letter patterns
  while [[ "$dir" != "/" ]] && [[ ! "$dir" =~ ^/[a-zA-Z]$ ]]; do
    if [[ -d "$dir/.claude" ]]; then
      echo "$dir"
      return 0
    fi
    dir="$(cd "$dir/.." && pwd)"
  done
  # Check the root directory itself (edge case)
  if [[ -d "$dir/.claude" ]]; then
    echo "$dir"
    return 0
  fi
  echo "Error: Could not find repo root (missing .claude/)" >&2
  return 1
}

REPO_ROOT="$(find_repo_root)"

# Unset DEMOSWARM_STRICT so inherited parent env can't change agent tool semantics.
unset DEMOSWARM_STRICT

# -----------------------------------------------------------------------------
# Tool resolution
# Build candidate list and try each in order
# Note: Arrays work in bash 3.2+ but the += syntax requires bash 3.1+
# -----------------------------------------------------------------------------

# Build candidate list (check both unix and windows binary names)
# Using indexed array (bash 3.0+) - not associative array (bash 4.0+ only)
TOOL_CANDIDATES=(
  "$REPO_ROOT/.demoswarm/bin/demoswarm"
  "$REPO_ROOT/.demoswarm/bin/demoswarm.exe"
)

if command -v demoswarm >/dev/null 2>&1; then
  TOOL_CANDIDATES+=("$(command -v demoswarm)")
fi

# 1-2. Try Rust binary candidates
for tool in "${TOOL_CANDIDATES[@]}"; do
  # Note: -n and -x tests work in bash 3.2+
  if [[ -n "$tool" && -x "$tool" ]]; then
    exec "$tool" "$@"
  fi
done

# -----------------------------------------------------------------------------
# Cargo run fallback (development only)
# Only used when tools/ directory is present (pack development repo)
# -----------------------------------------------------------------------------
CARGO_MANIFEST="$REPO_ROOT/tools/demoswarm-runs-tools/Cargo.toml"
if [[ -f "$CARGO_MANIFEST" ]] && command -v cargo >/dev/null 2>&1; then
  exec cargo run --quiet --manifest-path "$CARGO_MANIFEST" -- "$@"
fi

# -----------------------------------------------------------------------------
# Python fallback
# Single consolidated CLI (lives in runs-derive skill)
# Works on all platforms with Python 3.x installed
# -----------------------------------------------------------------------------
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
