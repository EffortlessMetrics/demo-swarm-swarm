# Documentation Updates for issue-37-cli-docs

## Handoff

**What I did:** Added comprehensive exit code documentation and error handling guidance to `docs/reference/demoswarm-cli.md`. The documentation now includes a dedicated Exit Codes section explaining normal vs strict mode behavior, an Error Handling section with output signals table, common error scenarios with resolution guidance, and debugging tips.

**What's left:** Nothing. All 16 commands requested in GitHub issue #37 were already documented with synopsis, options, and examples. The missing elements (exit code documentation and error message guidance) have now been added.

**Recommendation:** Route to doc-critic to review the updated documentation for staleness and accuracy before merging.

## Inputs Used
- GitHub Issue #37 requirements
- `docs/reference/demoswarm-cli.md` (existing documentation)
- `.claude/scripts/demoswarm.sh` (shim implementation)
- `.claude/skills/runs-derive/fallback/runs_tools.py` (Python fallback implementation)
- `tools/demoswarm-runs-tools/src/main.rs` (Rust implementation)

## Files Updated
| File | Change Type | Summary |
|------|-------------|---------|
| `docs/reference/demoswarm-cli.md` | updated | Added Exit Codes section (lines 17-30) explaining normal vs strict mode exit code behavior. Added Error Handling section (lines 32-84) with output signals table, common error scenarios table, and debugging tips. |

## Deferred / Not Updated (and why)
- None. All requested documentation was updated.

## Mismatches Found (if any)
- None. The implementation in both Rust and Python fallback aligns with the documented behavior.

## Assumptions Made
- Assumed the exit code value of 2 for strict mode errors is correct based on the Rust implementation (`ExitCode::from(2)` in main.rs line 127).
- Assumed all output signals documented are exhaustive based on analysis of the Python fallback implementation patterns.

## Command Coverage Verification

All 16 commands from the acceptance criteria were verified as documented:

| Command | Section | Lines | Status |
|---------|---------|-------|--------|
| count pattern | Command Reference | 115-154 | Documented with synopsis, options, semantics, example |
| count bdd | Command Reference | 157-188 | Documented with synopsis, options, semantics, example |
| ms get | Command Reference | 191-229 | Documented with synopsis, options, semantics, example |
| yaml get | Command Reference | 232-266 | Documented with synopsis, options, semantics, example |
| yaml count-items | Command Reference | 269-303 | Documented with synopsis, options, semantics, example |
| inv get | Command Reference | 306-340 | Documented with synopsis, options, semantics, example |
| line get | Command Reference | 343-378 | Documented with synopsis, options, semantics, example |
| receipts count | Command Reference | 381-413 | Documented with synopsis, options, semantics, example |
| receipt get | Command Reference | 417-444 | Documented with synopsis, options, example |
| openapi count-paths | Command Reference | 447-479 | Documented with synopsis, options, semantics, example |
| index upsert-status | Command Reference | 482-523 | Documented with synopsis, options, semantics, example |
| time now | Command Reference | 526-546 | Documented with synopsis, example |
| openq next-id | Utility Commands | 556-587 | Documented with synopsis, options, semantics, example |
| openq append | Utility Commands | 590-632 | Documented with synopsis, options, semantics, example |
| secrets scan | Utility Commands | 635-671 | Documented with synopsis, options, semantics, example |
| secrets redact | Utility Commands | 674-722 | Documented with synopsis, options, valid types, semantics, example |

## Acceptance Criteria Status

| Criterion | Status | Evidence |
|-----------|--------|----------|
| All commands documented with synopsis | DONE | See Command Coverage table above |
| All commands documented with options | DONE | Each command section includes Arguments table |
| All commands documented with examples | DONE | Each command section ends with Example block |
| Exit code documentation added | DONE | New Exit Codes section (lines 17-30) |
| Error message guidance included | DONE | New Error Handling section with Output Signals table (lines 38-50), Common Error Scenarios table (lines 54-63), and Debugging Tips (lines 65-84) |
