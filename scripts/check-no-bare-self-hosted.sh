#!/usr/bin/env bash
set -euo pipefail

bad=0
workflow_dir=".github/workflows"

echo "Checking for bare self-hosted runner usage..."

if [ ! -d "$workflow_dir" ]; then
  echo "No GitHub workflow directory found; skipping self-hosted runner routing guard."
  exit 0
fi

if ! command -v rg >/dev/null 2>&1; then
  echo "ripgrep (rg) is required for the self-hosted runner routing guard." >&2
  exit 2
fi

inline_matches="$(rg -n --glob '*.yml' --glob '*.yaml' 'runs-on:[[:space:]]*\[[^]]*self-hosted[^]]*linux[^]]*x64[^]]*\]' "$workflow_dir" || true)"
if [ -n "$inline_matches" ]; then
  printf '%s\n' "$inline_matches"
  echo "Bare inline self-hosted/linux/x64 runs-on is forbidden." >&2
  bad=1
fi

while IFS=: read -r file line _; do
  [ -n "${file:-}" ] || continue
  window="$(sed -n "${line},$((line + 16))p" "$file")"

  if printf '%s\n' "$window" | rg -q '^[[:space:]]*-[[:space:]]*linux[[:space:]]*$' &&
     printf '%s\n' "$window" | rg -q '^[[:space:]]*-[[:space:]]*x64[[:space:]]*$' &&
     ! printf '%s\n' "$window" | rg -q 'group:[[:space:]]*em-ci-' &&
     ! printf '%s\n' "$window" | rg -q '^[[:space:]]*-[[:space:]]*(em-ci|ci-nano|policy-nano|workflow-nano|rust-tiny|rust-medium|rust-large|rust-16gb|cx23|cx33|cx43|cx53|cpx42)[[:space:]]*$'; then
    echo "$file:$line: bare self-hosted block lacks group/capacity labels" >&2
    bad=1
  fi
done < <(rg -n --glob '*.yml' --glob '*.yaml' '^[[:space:]]*-[[:space:]]*self-hosted[[:space:]]*$' "$workflow_dir" || true)

exit "$bad"
