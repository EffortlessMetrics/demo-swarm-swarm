#!/usr/bin/env bash
set -euo pipefail

# Doc drift guard - prevents stale references from creeping into docs
# Run from repo root: bash scripts/check-doc-drift.sh

ERRORS=0

echo "Checking for doc drift..."

# 1. Check for old skill name references outside the crate itself
echo -n "  Checking for stale 'runs-tools' skill references... "
if grep -rn "\bruns-tools\b" . \
  --include="*.md" \
  --exclude-dir=".git" \
  --exclude-dir="target" \
  --exclude-dir="node_modules" \
  --exclude-dir="tools/demoswarm-runs-tools" | \
  grep -v "demoswarm-runs-tools" | \
  grep -v "Cargo.toml" | \
  grep -v "Cargo.lock" > /dev/null 2>&1; then
  echo "FAILED"
  echo "    Found stale 'runs-tools' references (skill was split into runs-derive, runs-index, openq-tools, secrets-tools):"
  grep -rn "\bruns-tools\b" . \
    --include="*.md" \
    --exclude-dir=".git" \
    --exclude-dir="target" \
    --exclude-dir="node_modules" \
    --exclude-dir="tools/demoswarm-runs-tools" | \
    grep -v "demoswarm-runs-tools" | \
    grep -v "Cargo.toml" | \
    grep -v "Cargo.lock" | \
    head -10
  ERRORS=$((ERRORS + 1))
else
  echo "ok"
fi

# 2. Check for old openq interface references
echo -n "  Checking for old openq interface (--flow, --qid flags)... "
if grep -rn "openq.*--flow\|openq.*--qid" . \
  --include="*.md" \
  --exclude-dir=".git" \
  --exclude-dir="target" > /dev/null 2>&1; then
  echo "FAILED"
  echo "    Found old openq interface references:"
  grep -rn "openq.*--flow\|openq.*--qid" . \
    --include="*.md" \
    --exclude-dir=".git" \
    --exclude-dir="target" | head -10
  ERRORS=$((ERRORS + 1))
else
  echo "ok"
fi

# 3. Check for old secrets interface references
echo -n "  Checking for old secrets interface (--files, --dir flags)... "
if grep -rn "secrets.*--files\|secrets.*--dir" . \
  --include="*.md" \
  --exclude-dir=".git" \
  --exclude-dir="target" > /dev/null 2>&1; then
  echo "FAILED"
  echo "    Found old secrets interface references:"
  grep -rn "secrets.*--files\|secrets.*--dir" . \
    --include="*.md" \
    --exclude-dir=".git" \
    --exclude-dir="target" | head -10
  ERRORS=$((ERRORS + 1))
else
  echo "ok"
fi

# 4. Check for flag mismatch regressions
# These catch the common doc-table vs CLI drift patterns

echo -n "  Checking for 'yaml count-items --key' drift... "
if grep -rn "yaml count-items.*--key" . \
  --include="*.md" \
  --exclude-dir=".git" \
  --exclude-dir="target" > /dev/null 2>&1; then
  echo "FAILED"
  echo "    Found 'yaml count-items --key' (should be --item-regex):"
  grep -rn "yaml count-items.*--key" . \
    --include="*.md" \
    --exclude-dir=".git" \
    --exclude-dir="target" | head -5
  ERRORS=$((ERRORS + 1))
else
  echo "ok"
fi

echo -n "  Checking for 'inv get --key' drift... "
if grep -rn "inv get.*--key" . \
  --include="*.md" \
  --exclude-dir=".git" \
  --exclude-dir="target" > /dev/null 2>&1; then
  echo "FAILED"
  echo "    Found 'inv get --key' (should be --marker):"
  grep -rn "inv get.*--key" . \
    --include="*.md" \
    --exclude-dir=".git" \
    --exclude-dir="target" | head -5
  ERRORS=$((ERRORS + 1))
else
  echo "ok"
fi

echo -n "  Checking for 'line get --line' drift... "
if grep -rn "line get.*--line" . \
  --include="*.md" \
  --exclude-dir=".git" \
  --exclude-dir="target" > /dev/null 2>&1; then
  echo "FAILED"
  echo "    Found 'line get --line' (should be --prefix):"
  grep -rn "line get.*--line" . \
    --include="*.md" \
    --exclude-dir=".git" \
    --exclude-dir="target" | head -5
  ERRORS=$((ERRORS + 1))
else
  echo "ok"
fi

echo -n "  Checking for 'receipts count --dir' drift... "
if grep -rn "receipts count.*--dir[^-]" . \
  --include="*.md" \
  --exclude-dir=".git" \
  --exclude-dir="target" > /dev/null 2>&1; then
  echo "FAILED"
  echo "    Found 'receipts count --dir' (should be --run-dir):"
  grep -rn "receipts count.*--dir[^-]" . \
    --include="*.md" \
    --exclude-dir=".git" \
    --exclude-dir="target" | head -5
  ERRORS=$((ERRORS + 1))
else
  echo "ok"
fi

# 5. Check required docs exist
echo -n "  Checking required docs exist... "
MISSING_OUTPUT=""

require_one_of() {
  local label="$1"; shift
  for p in "$@"; do
    if [ -f "$p" ]; then
      return 0
    fi
  done
  MISSING_OUTPUT="${MISSING_OUTPUT}    Missing required docs for ${label}: $*\n"
  return 1
}

require_one_of "toy run tutorial" \
  "docs/tutorials/toy-run.md" \
  "docs/TOY_RUN.md" || true

require_one_of "releasing guide" \
  "docs/maintainers/release-checklist.md" \
  "docs/RELEASING.md" || true

for doc in \
  "docs/reference/demoswarm-cli.md" \
  "SECURITY.md" \
  "CODE_OF_CONDUCT.md" \
  "LICENSE"; do
  if [ ! -f "$doc" ]; then
    MISSING_OUTPUT="${MISSING_OUTPUT}    Missing required doc: ${doc}\n"
  fi
done

if [ -n "$MISSING_OUTPUT" ]; then
  echo "FAILED"
  printf "%b" "$MISSING_OUTPUT"
  ERRORS=$((ERRORS + 1))
else
  echo "ok"
fi

# 6. Check for legacy Python fallback references in main docs (allowed in skill fallbacks)
echo -n "  Checking for legacy Python fallback promoted in main docs... "
if grep -n "runs_tools\.py" README.md CLAUDE.md docs/reference/demoswarm-cli.md 2>/dev/null | \
  grep -v "fallback" > /dev/null 2>&1; then
  echo "FAILED"
  echo "    Found Python fallback promoted (should prefer Rust CLI):"
  grep -n "runs_tools\.py" README.md CLAUDE.md docs/reference/demoswarm-cli.md 2>/dev/null | \
    grep -v "fallback" | head -5
  ERRORS=$((ERRORS + 1))
else
  echo "ok"
fi

echo ""
if [ $ERRORS -gt 0 ]; then
  echo "Doc drift check: FAILED ($ERRORS error(s))"
  exit 1
else
  echo "Doc drift check: PASSED"
  exit 0
fi
