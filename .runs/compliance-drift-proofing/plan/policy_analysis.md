# Policy Analysis

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_flow: null
route_to_agent: null

blockers: []

missing_required: []

concerns:
  - No dedicated policy documents found in standard roots (policies/, docs/policies/, .policies/); using CLAUDE.md as authoritative pack policy

compliance_summary:
  policies_found: 1
  policies_checked: 1
  compliant: 8
  non_compliant: 0
  not_applicable: 1
  unknown: 0
  waivers_needed: 0

## Context
- flow: plan
- run_id: compliance-drift-proofing
- policy_roots_searched:
  - policies/
  - docs/policies/
  - .policies/
  - CLAUDE.md (pack-level policy, authoritative)
- inputs_used:
  - .runs/compliance-drift-proofing/run_meta.json
  - .runs/index.json
  - .runs/compliance-drift-proofing/plan/adr.md
  - .runs/compliance-drift-proofing/plan/work_plan.md
  - .runs/compliance-drift-proofing/plan/api_contracts.yaml
  - .runs/compliance-drift-proofing/plan/observability_spec.md
  - .runs/compliance-drift-proofing/plan/test_plan.md
  - .runs/compliance-drift-proofing/plan/schema.md
  - .runs/compliance-drift-proofing/signal/requirements.md
  - CLAUDE.md

## Policies Reviewed
- CLAUDE.md (pack-level policy) -- version: current HEAD

## Compliance Register

| ID | Policy | Section | Requirement | Status | Severity | Evidence |
|----|--------|---------|-------------|--------|----------|----------|
| POL-001 | CLAUDE.md | Non-Negotiables #1 | Repo root only - All commands run from repo root; all paths are repo-root-relative | COMPLIANT | HIGH | adr.md:Section "What we are doing"; work_plan.md:Section "Scope Snapshot" |
| POL-002 | CLAUDE.md | Non-Negotiables #2 | No raw git in flow commands or agent prompts | COMPLIANT | HIGH | adr.md:Section "What we are NOT doing" |
| POL-003 | CLAUDE.md | Non-Negotiables #3 | Control plane vs audit plane - Orchestrators route on returned result blocks | COMPLIANT | MEDIUM | api_contracts.yaml:L229-310 (CheckDiagnostic/RunReport schemas) |
| POL-004 | CLAUDE.md | Non-Negotiables #5 | run_id folders never rename | NOT-APPLICABLE | LOW | Change does not touch run identity or folder structure |
| POL-005 | CLAUDE.md | Machine Summary Contract | Status enum is VERIFIED/UNVERIFIED/CANNOT_PROCEED only | COMPLIANT | HIGH | api_contracts.yaml:L342-344 (status enum definition) |
| POL-006 | CLAUDE.md | Machine Summary Contract | recommended_action enum is PROCEED/RERUN/BOUNCE/FIX_ENV only | COMPLIANT | HIGH | api_contracts.yaml:L345-347 (recommended_action enum) |
| POL-007 | CLAUDE.md | CLI Tooling Surface | Agents always invoke via shims (pack-check.sh, demoswarm.sh) | COMPLIANT | MEDIUM | adr.md:L43-49; observability_spec.md:L9 |
| POL-008 | CLAUDE.md | Receipts | counts are mechanical (grep/wc/parse), never estimated | COMPLIANT | MEDIUM | requirements.md:Section NFR-REL-001; api_contracts.yaml:L422-439 (counts schema) |
| POL-009 | CLAUDE.md | Canonical Status + Verdict Domains | Do not conflate status domains across contexts | COMPLIANT | HIGH | api_contracts.yaml:L229-234 (Level enum); schema.md:L45-57 (CheckResult entity) |

## Compliance Details

### POL-001: Repo root only
- Policy: CLAUDE.md, Section "Non-Negotiables #1"
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - adr.md:Section "What we are doing" - specifies repo-root-relative paths (tools/demoswarm-pack-check/...)
  - work_plan.md:Section "Scope hints" - all subtasks specify absolute paths from repo root
- Notes: All touchpoints in work_plan.md use repo-root-relative paths (e.g., tools/demoswarm-pack-check/src/checks/drift.rs)

### POL-002: No raw git in flow commands
- Policy: CLAUDE.md, Section "Non-Negotiables #2"
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - adr.md:Section "What we are NOT doing" - no git operations mentioned in implementation
  - work_plan.md - no subtasks involve raw git commands
- Notes: This change is to pack-check validation tooling; it does not modify flow commands or add git operations

### POL-003: Control plane vs audit plane separation
- Policy: CLAUDE.md, Section "Non-Negotiables #3"
- Status: COMPLIANT
- Severity: MEDIUM
- Evidence:
  - api_contracts.yaml:L229-310 - defines CheckDiagnostic and RunReport as structured output
  - observability_spec.md:Section "Diagnostic Output Format" - specifies severity levels and output structure
- Notes: The planned changes maintain separation by outputting structured diagnostics (audit) separate from exit codes (control)

### POL-004: run_id folders never rename
- Policy: CLAUDE.md, Section "Non-Negotiables #5"
- Status: NOT-APPLICABLE
- Severity: LOW
- Evidence:
  - requirements.md - no requirements touch run identity
  - work_plan.md - no subtasks modify .runs/ folder structure
- Notes: This change adds validation checks to pack-check; does not affect run identity mechanics

### POL-005: Status enum frozen
- Policy: CLAUDE.md, Section "Machine Summary Contract"
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - api_contracts.yaml:L342-344 - BuildReceipt schema uses exact enum: VERIFIED, UNVERIFIED, CANNOT_PROCEED
  - adr.md:L9 - explicitly states "Canonical status and action enums are frozen"
- Notes: ADR acknowledges constraint and new validation does not introduce non-canonical status values

### POL-006: recommended_action enum frozen
- Policy: CLAUDE.md, Section "Machine Summary Contract"
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - api_contracts.yaml:L345-347 - BuildReceipt schema uses exact enum: PROCEED, RERUN, BOUNCE, FIX_ENV
  - adr.md:L9 - explicitly states "Canonical status and action enums are frozen"
- Notes: New validation checks produce warn/fail diagnostics; they do not introduce new action enums

### POL-007: Invoke tools via shims
- Policy: CLAUDE.md, Section "CLI Tooling Surface"
- Status: COMPLIANT
- Severity: MEDIUM
- Evidence:
  - adr.md:L43-49 - references pack-check.sh shim for validation
  - observability_spec.md:L9 - "Invocation: bash .claude/scripts/pack-check.sh [OPTIONS]"
  - work_plan.md:ST-011 - uses "bash .claude/scripts/pack-check.sh --no-color"
- Notes: All documentation and subtasks correctly reference shim invocation

### POL-008: Mechanical counts only
- Policy: CLAUDE.md, Section "Receipts"
- Status: COMPLIANT
- Severity: MEDIUM
- Evidence:
  - requirements.md:NFR-REL-001 - "MET-1: Running pack-check twice on identical input produces byte-identical output"
  - api_contracts.yaml:L422-439 - counts object schema with integer types and minimum: 0
- Notes: BuildReceipt schema enforces integer counts; determinism requirement (NFR-REL-001) ensures mechanical counting

### POL-009: Status domain separation
- Policy: CLAUDE.md, Section "Canonical Status + Verdict Domains"
- Status: COMPLIANT
- Severity: HIGH
- Evidence:
  - api_contracts.yaml:L229-234 - Level enum {pass, warn, fail} for diagnostics
  - schema.md:L45-57 - CheckResult entity defines level as {Pass, Warn, Fail}
- Notes: Check diagnostics use their own Level enum; do not conflate with Machine Summary status or Gate verdicts

## Violations Summary
| ID | Policy | Section | Severity | Remediation | Owner |
|----|--------|---------|----------|------------|-------|
| (none) | | | | | |

No violations identified.

## Waivers Needed
- None

## Security Analysis

The planned changes have been reviewed for security concerns:

1. **NFR-SEC-001 (No secrets in output)**: The plan explicitly addresses this:
   - requirements.md:NFR-SEC-001 - "pack-check does not print file contents containing potential secrets (only file paths and rule violations)"
   - observability_spec.md:Section "PII/Secrets Guidance" - "NEVER print file contents in diagnostic output"
   - Test fixtures must use obviously synthetic values

2. **No secret exposure vectors**: The validation checks scan for patterns (demoswarm.sh, skill subcommands, QID patterns) - none of these involve reading or exposing sensitive data

3. **CLI tool scope**: pack-check is a local validation tool with no network operations or credential handling

## Backward Compatibility Analysis

The planned changes maintain backward compatibility per NFR-COMP-001:

1. **Exit codes preserved**: api_contracts.yaml:L69-82 defines 0/1/2 semantics unchanged
2. **JSON schema version stable**: api_contracts.yaml:L288-291 maintains schema_version: 1
3. **Warning-first mode**: New checks 50-51 default to warnings, not errors (REQ-005)
4. **Existing check IDs stable**: Check IDs 1-49 unchanged; new checks use 50+
5. **--strict flag behavior**: Existing flag elevated warnings to errors; behavior preserved

## Recommended Next
- Proceed to Flow 3 (Build) with confidence in policy compliance
- Resolve OQ-PLN-004 (PLN vs PLAN prefix) before implementing check 51 regex
- Enumerate the 4 agents potentially missing Skills sections during ST-003
- Establish validation baseline (ST-011) before introducing new checks
- Document any edge cases discovered during implementation as known exceptions

## Inventory (machine countable)

- POL-001: Repo root only
- POL-002: No raw git in flow commands
- POL-003: Control plane vs audit plane
- POL-004: run_id folders never rename
- POL-005: Status enum frozen
- POL-006: recommended_action enum frozen
- POL-007: Invoke tools via shims
- POL-008: Mechanical counts only
- POL-009: Status domain separation
