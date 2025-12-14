# Gate Fix Summary for align-doc-ownership

## Scope & Evidence

Gate artifacts consulted:
- `.runs/align-doc-ownership/gate/receipt_audit.md` (status: UNVERIFIED)
- `.runs/align-doc-ownership/gate/contract_compliance.md` (status: VERIFIED)
- `.runs/align-doc-ownership/gate/security_scan.md` (status: VERIFIED)
- `.runs/align-doc-ownership/gate/coverage_audit.md` (status: VERIFIED)

Additional evidence gathered:
- `bash .claude/scripts/pack-check.sh --no-color` - All 49 checks passed
- `cargo fmt --check` in `tools/demoswarm-pack-check/` - Formatting issues detected

Missing optional inputs:
- `.runs/align-doc-ownership/gate/lint_issues.md` - not present
- `.runs/align-doc-ownership/gate/policy_analysis.md` - not present
- `.runs/align-doc-ownership/build/build_receipt.json` - permission denied (cannot verify)

---

## Mechanical Fixes (apply in Flow 3)

### MECH-001: Rust code formatting violations in pack-check

- **Evidence:** `cargo fmt --check` output showing diff across 8 Rust source files
- **Files/Paths:**
  - `tools/demoswarm-pack-check/src/checks/contracts.rs` (1 diff: trailing newline)
  - `tools/demoswarm-pack-check/src/checks/control_plane.rs` (2 diffs: line length formatting)
  - `tools/demoswarm-pack-check/src/checks/drift.rs` (9 diffs: line length, import ordering)
  - `tools/demoswarm-pack-check/src/checks/flow.rs` (6 diffs: line length formatting)
  - `tools/demoswarm-pack-check/src/checks/mod.rs` (1 diff: extra blank line)
  - `tools/demoswarm-pack-check/src/contracts.rs` (5 diffs: line length formatting)
  - `tools/demoswarm-pack-check/src/ctx.rs` (3 diffs: import ordering, line length)
  - `tools/demoswarm-pack-check/src/reporter.rs` (1 diff: whitespace alignment)
  - `tools/demoswarm-pack-check/src/util.rs` (1 diff: line length formatting)
- **Category:** format
- **Suggested Tool Hint:** run project formatter (cargo fmt)
- **Suggested Command:** `cd tools/demoswarm-pack-check && cargo fmt`
- **Why mechanical:** Formatting changes do not alter program behavior; `cargo fmt` applies deterministic, style-only transformations.

---

## Non-Mechanical Findings (for merge-decider context)

### NONMECH-001: receipt_audit reports UNVERIFIED status from self-reviewer

- **Evidence:** `.runs/align-doc-ownership/gate/receipt_audit.md` lines 4, 10-11:
  > `status: UNVERIFIED`
  > `blockers: self_reviewer status is UNVERIFIED (blocks receipt validation)`
- **Likely Target:** Flow 3 (Build)
- **Why not mechanical:** The self-reviewer returned UNVERIFIED due to content/logic concerns that require human or iterative judgment to resolve; not a simple formatting fix.

### NONMECH-002: mutation_score extraction failed

- **Evidence:** `.runs/align-doc-ownership/gate/receipt_audit.md` lines 12-15, 19:
  > `mutation_score could not be extracted from mutation_report.md`
  > `missing_required: mutation_report.md: missing or malformed "Mutation Score:" marker line`
- **Likely Target:** Flow 3 (Build)
- **Why not mechanical:** The mutation report content structure needs to be corrected or the marker added; this requires understanding of what mutation score to report.

### NONMECH-003: Pack-check Rust tooling lacks unit tests

- **Evidence:** `.runs/align-doc-ownership/gate/coverage_audit.md` lines 96-99:
  > `[MINOR] COV-MIN-001: Pack-check Rust tooling lacks unit tests for boundary checks (45-47)`
  > `Evidence: Grep for #[test] in tools/demoswarm-pack-check/**/*.rs returned no matches`
- **Likely Target:** Flow 3 (Build) or Flow 2 (Plan) - depending on whether test coverage is required
- **Why not mechanical:** Writing unit tests requires judgment about test cases, assertions, and code structure.

---

## Inventory (machine countable)

- MECH_FIX: MECH-001 category=format paths=[tools/demoswarm-pack-check/src/checks/contracts.rs, tools/demoswarm-pack-check/src/checks/control_plane.rs, tools/demoswarm-pack-check/src/checks/drift.rs, tools/demoswarm-pack-check/src/checks/flow.rs, tools/demoswarm-pack-check/src/checks/mod.rs, tools/demoswarm-pack-check/src/contracts.rs, tools/demoswarm-pack-check/src/ctx.rs, tools/demoswarm-pack-check/src/reporter.rs, tools/demoswarm-pack-check/src/util.rs] tool_hint=cargo_fmt
- NON_MECH: NONMECH-001 target_flow=3
- NON_MECH: NONMECH-002 target_flow=3
- NON_MECH: NONMECH-003 target_flow=3

---

## Machine Summary

```yaml
status: VERIFIED
recommended_action: BOUNCE
route_to_flow: 3
route_to_agent: fixer
blockers:
  - MECH-001: Rust formatting violations (9 files)
missing_required: []
concerns:
  - NONMECH-001: self-reviewer UNVERIFIED status (non-mechanical, context for merge-decider)
  - NONMECH-002: mutation_score extraction failed (non-mechanical)
  - NONMECH-003: pack-check lacks unit tests (acknowledged in coverage audit as acceptable)
  - cargo audit could not complete due to CVSS 4.0 parsing issue (external tooling limitation)
```
