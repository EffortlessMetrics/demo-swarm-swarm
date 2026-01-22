# Secrets Scan Report

## Status: CLEAN

## Scope

- Allowlist scanned: `.runs/local-alignment-audit-aba1c6/wisdom/`, `.runs/local-alignment-audit-aba1c6/run_meta.json`, `.runs/index.json`
- Staged files scanned: 0 (no staged changes)
- Notes: No binaries or large files skipped

## Files Scanned

| #   | File                                                              | Size    | Result |
| --- | ----------------------------------------------------------------- | ------- | ------ |
| 1   | `.runs/local-alignment-audit-aba1c6/wisdom/flow_history.json`     | 30 KB   | CLEAN  |
| 2   | `.runs/local-alignment-audit-aba1c6/wisdom/regression_report.md`  | 6.3 KB  | CLEAN  |
| 3   | `.runs/local-alignment-audit-aba1c6/wisdom/artifact_audit.md`     | 8 KB    | CLEAN  |
| 4   | `.runs/local-alignment-audit-aba1c6/wisdom/feedback_actions.md`   | 6 KB    | CLEAN  |
| 5   | `.runs/local-alignment-audit-aba1c6/wisdom/traceability_audit.md` | 14.8 KB | CLEAN  |
| 6   | `.runs/local-alignment-audit-aba1c6/wisdom/learnings.md`          | 8 KB    | CLEAN  |
| 7   | `.runs/local-alignment-audit-aba1c6/wisdom/risk_assessment.md`    | 17 KB   | CLEAN  |
| 8   | `.runs/local-alignment-audit-aba1c6/wisdom/wisdom_receipt.json`   | 1.5 KB  | CLEAN  |
| 9   | `.runs/local-alignment-audit-aba1c6/wisdom/cleanup_report.md`     | 8 KB    | CLEAN  |
| 10  | `.runs/local-alignment-audit-aba1c6/wisdom/github_report.md`      | 4 KB    | CLEAN  |
| 11  | `.runs/local-alignment-audit-aba1c6/wisdom/flow_plan.md`          | 1.5 KB  | CLEAN  |
| 12  | `.runs/local-alignment-audit-aba1c6/run_meta.json`                | 1 KB    | CLEAN  |
| 13  | `.runs/index.json`                                                | 1 KB    | CLEAN  |

## Findings (redacted)

| #      | Type | File | Line | Action |
| ------ | ---- | ---- | ---- | ------ |
| (none) | -    | -    | -    | -      |

No secrets detected on the publish surface.

## Actions Taken

### Redacted

None required.

### Externalized

None required.

### Unstaged

None required.

## Safety Flags

- safe_to_commit: true
- safe_to_publish: true
- needs_upstream_fix: false
- recommended_action: PROCEED
- route_to_flow: null
- route_to_station: null
- route_to_agent: null

## Notes

- The secrets-tools scan returned CLEAN status with zero findings
- Manual verification confirmed no high-confidence patterns (GitHub tokens, AWS keys, private keys, Stripe keys, Bearer tokens, DB URLs with passwords)
- Manual verification confirmed no medium-confidence patterns (api_key, secret, token, credential, password assignments)
- All files contain standard documentation, metadata, and audit trail information
- No staged files in git; publish surface is limited to wisdom flow artifacts and run metadata

---

_Scan completed: 2025-12-21T22:25:00Z_
_Sanitizer: secrets-sanitizer_
_Run ID: local-alignment-audit-aba1c6_
_Flow: wisdom_
