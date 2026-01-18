# Secrets Scan Report

## Status: CLEAN

## Scope

- Allowlist scanned: `.runs/local-alignment-audit-aba1c6/gate/`, `.runs/local-alignment-audit-aba1c6/run_meta.json`, `.runs/index.json`
- Staged files scanned: 0
- Notes: No staged files detected (git working tree clean)

## Findings (redacted)

| #      | Type | File | Line | Action              |
| ------ | ---- | ---- | ---- | ------------------- |
| (none) | -    | -    | -    | No secrets detected |

## Actions Taken

### Redacted

- None required

### Externalized

- None required

### Unstaged

- None required

## Safety Flags

- safe_to_commit: true
- safe_to_publish: true
- needs_upstream_fix: false
- recommended_action: PROCEED
- route_to_flow: null
- route_to_station: null
- route_to_agent: null

## Notes

- All 13 gate artifacts scanned (flow_plan.md, receipt_audit.md, coverage_audit.md, contract_compliance.md, security_scan.md, gate_fix_summary.md, risk_assessment.md, policy_analysis.md, traceability_audit.md, merge_decision.md, gate_receipt.json, cleanup_report.md, github_report.md)
- run_meta.json and index.json verified clean
- No high-confidence secret patterns detected (GitHub tokens, AWS keys, private keys, Stripe keys, bearer tokens, DB URLs with passwords)
- No medium-confidence patterns detected (api_key, secret, token, password value assignments)
- security_scan.md artifact independently confirms 0 secrets in changed surface

## Files Scanned

| File                                                           | Size | Result |
| -------------------------------------------------------------- | ---- | ------ |
| .runs/local-alignment-audit-aba1c6/gate/flow_plan.md           | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/receipt_audit.md       | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/coverage_audit.md      | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/contract_compliance.md | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/security_scan.md       | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/gate_fix_summary.md    | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/risk_assessment.md     | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/policy_analysis.md     | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/traceability_audit.md  | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/merge_decision.md      | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/gate_receipt.json      | json | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/cleanup_report.md      | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/gate/github_report.md       | text | CLEAN  |
| .runs/local-alignment-audit-aba1c6/run_meta.json               | json | CLEAN  |
| .runs/index.json                                               | json | CLEAN  |

---

_Scanned by secrets-sanitizer at 2025-12-20T15:45:00Z_
