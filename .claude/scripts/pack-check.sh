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
# Warn if running on bash < 4.0 (macOS default is 3.2)
# The script should still work on 3.2+, but some features may behave differently
# -----------------------------------------------------------------------------
check_bash_version() {
  local major="${BASH_VERSINFO[0]:-0}"
  local minor="${BASH_VERSINFO[1]:-0}"

  if [[ "$major" -lt 3 ]] || { [[ "$major" -eq 3 ]] && [[ "$minor" -lt 2 ]]; }; then
    echo "Warning: bash $BASH_VERSION detected. This script requires bash 3.2+." >&2
    echo "         Some features may not work correctly." >&2
  fi

  # Informational: note if running on older bash (3.x)
  # The script is designed to work on 3.2+ but 4.0+ is recommended
  if [[ "$major" -lt 4 ]] && [[ "${DEMOSWARM_QUIET:-}" != "1" ]]; then
    : # Silently continue - bash 3.2 is supported
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

# -----------------------------------------------------------------------------
# Path conversion for Windows (Git Bash / MSYS / Cygwin)
# Converts Unix-style paths (/c/foo) to Windows-style (C:/foo) for cargo/native tools
# On non-Windows systems, returns the path unchanged
# -----------------------------------------------------------------------------
to_cargo_path() {
  local p="$1"
  # Git Bash / MSYS / Cygwin: convert /c/... to C:/...
  if command -v cygpath >/dev/null 2>&1; then
    cygpath -m "$p"
    return
  fi
  # Default: leave it alone (Unix systems)
  printf "%s" "$p"
}

# -----------------------------------------------------------------------------
# Tool resolution
# Build candidate list and try each in order
# Note: Arrays work in bash 3.2+ but the += syntax requires bash 3.1+
# -----------------------------------------------------------------------------

# Build candidate list (check both unix and windows binary names)
# Using indexed array (bash 3.0+) - not associative array (bash 4.0+ only)
TOOL_CANDIDATES=(
  "$REPO_ROOT/.demoswarm/bin/pack-check"
  "$REPO_ROOT/.demoswarm/bin/pack-check.exe"
)

if command -v pack-check >/dev/null 2>&1; then
  TOOL_CANDIDATES+=("$(command -v pack-check)")
fi

# Try each candidate
for tool in "${TOOL_CANDIDATES[@]}"; do
  # Note: -n and -x tests work in bash 3.2+
  if [[ -n "$tool" && -x "$tool" ]]; then
    # Convert Unix path to Windows path when calling Windows executable
    if [[ "$tool" == *.exe ]]; then
      REPO_ROOT_WIN=$(to_cargo_path "$REPO_ROOT")
      exec "$tool" --repo-root "$REPO_ROOT_WIN" "$@"
    else
      exec "$tool" --repo-root "$REPO_ROOT" "$@"
    fi
  fi
done

# -----------------------------------------------------------------------------
# Cargo run fallback (development only)
# Only used when tools/ directory is present (pack development repo)
# -----------------------------------------------------------------------------
CARGO_MANIFEST="$REPO_ROOT/tools/demoswarm-pack-check/Cargo.toml"
if [[ -f "$CARGO_MANIFEST" ]] && command -v cargo >/dev/null 2>&1; then
  # Platform detection: OSTYPE is set by bash on all platforms
  # msys = Git Bash on Windows, cygwin = Cygwin on Windows
  if [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]]; then
    REPO_ROOT_WIN=$(to_cargo_path "$REPO_ROOT")
    CARGO_MANIFEST_WIN=$(to_cargo_path "$CARGO_MANIFEST")
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
