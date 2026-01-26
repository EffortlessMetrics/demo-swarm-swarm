#!/usr/bin/env bash
set -e

# Bootstrap script for demo-swarm
# This script validates that the repository is in a healthy state
# by installing tools and running basic checks.
#
# Platform compatibility:
#   - macOS: Works with bash 3.2+ (default system bash)
#   - Linux: Works with bash 4.0+
#   - Windows: Works with Git Bash / MSYS2
#
# Note: This script uses minimal bashisms and should work on bash 3.2+.

# -----------------------------------------------------------------------------
# Bash version check
# -----------------------------------------------------------------------------
check_bash_version() {
  local major="${BASH_VERSINFO[0]:-0}"
  local minor="${BASH_VERSINFO[1]:-0}"

  if [ "$major" -lt 3 ] || { [ "$major" -eq 3 ] && [ "$minor" -lt 2 ]; }; then
    echo "Warning: bash $BASH_VERSION detected. This script requires bash 3.2+." >&2
    echo "         Some features may not work correctly." >&2
  fi
}
check_bash_version

echo "=================================================="
echo "DemoSwarm Bootstrap"
echo "=================================================="
echo ""

# Check for required dependencies
echo "[1/5] Checking dependencies..."
if ! command -v cargo &> /dev/null; then
    echo "ERROR: cargo not found. Please install Rust toolchain from https://rustup.rs/"
    exit 1
fi
echo "  ✓ cargo found"
echo ""

# Install pack-check
echo "[2/5] Installing pack-check..."
if ! cargo install --path tools/demoswarm-pack-check --root .demoswarm; then
    echo "ERROR: Failed to install pack-check"
    exit 1
fi
echo "  ✓ pack-check installed to .demoswarm/bin/"
echo ""

# Install demoswarm CLI
echo "[3/5] Installing demoswarm CLI..."
if ! cargo install --path tools/demoswarm-runs-tools --root .demoswarm; then
    echo "ERROR: Failed to install demoswarm CLI"
    exit 1
fi
echo "  ✓ demoswarm CLI installed to .demoswarm/bin/"
echo ""

# Run pack-check validation
echo "[4/5] Running pack-check validation..."
if ! bash .claude/scripts/pack-check.sh --no-color; then
    echo "ERROR: Pack validation failed"
    exit 1
fi
echo "  ✓ Pack validation passed"
echo ""

# Run demoswarm smoke tests
echo "[5/5] Running demoswarm smoke tests..."

echo "  Testing: time now"
if ! bash .claude/scripts/demoswarm.sh time now; then
    echo "ERROR: demoswarm time now failed"
    exit 1
fi
echo "  ✓ time now passed"

echo "  Testing: count pattern (counting markdown headers in CLAUDE.md)"
if ! bash .claude/scripts/demoswarm.sh count pattern --file CLAUDE.md --regex '^#'; then
    echo "ERROR: demoswarm count pattern failed"
    exit 1
fi
echo "  ✓ count pattern passed"
echo ""

echo "=================================================="
echo "Bootstrap Complete!"
echo "=================================================="
echo ""
echo "Repository is healthy and ready for use."
echo ""
echo "Next steps:"
echo "  - Run your first flow: /flow-1-signal \"your feature idea\""
echo "  - See CLAUDE.md for full documentation"
echo ""
