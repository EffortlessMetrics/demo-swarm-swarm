# Secrets Scan Report

## Status: CLEAN

## Scope

- Allowlist scanned: `.runs/local-alignment-audit-aba1c6/signal/`, `.runs/local-alignment-audit-aba1c6/run_meta.json`, `.runs/index.json`
- Staged files scanned: 0
- Notes: No staged files to scan (git diff --cached returned empty)

## Findings (redacted)

| #   | Type | File | Line | Action |
| --- | ---- | ---- | ---- | ------ |

No secrets detected on the publish surface.

## Actions Taken

### Redacted

(none)

### Externalized

(none)

### Unstaged

(none)

## Safety Flags

- safe_to_commit: true
- safe_to_publish: true
- needs_upstream_fix: false
- recommended_action: PROCEED
- route_to_flow: null
- route_to_station: null
- route_to_agent: null

## Notes

- Scanned 25 allowlist files in the signal flow directory
- Scanned run_meta.json and index.json
- No secrets patterns detected (GitHub tokens, AWS keys, private keys, Stripe keys, JWT tokens, passwords, API keys)
