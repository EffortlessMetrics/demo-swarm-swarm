# Documentation Updates for compliance-drift-proofing

## Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns: []
```

## Inputs Used
- `.runs/compliance-drift-proofing/plan/adr.md`
- `.runs/compliance-drift-proofing/plan/work_plan.md`
- `tools/demoswarm-pack-check/src/checks/drift.rs` (implementation reference)
- `tools/demoswarm-pack-check/src/contracts.rs` (constants reference)

## Files Updated
| File | Change Type | Summary |
|------|-------------|---------|
| `docs/reference/pack-check.md` | updated | Added documentation for checks 52 and 53, plus remediation guidance |
| `docs/reference/stable-markers.md` | updated | Normalized OpenQ flow codes to PLN/BLD (was PLAN/BUILD) |
| `docs/reference/contracts.md` | updated | Normalized OpenQ flow codes to PLN/BLD (was PLAN/BUILD) |

## What Changed

- **pack-check.md**: Added "Flow boundary enforcement (check 52)" section documenting what the check validates (no demoswarm.sh or skill CLI subcommands in flow commands), three-tier ownership model, and warning behavior with --strict flag interaction.

- **pack-check.md**: Added "OpenQ prefix validation (check 53)" section documenting the canonical QID format (OQ-<FLOW>-<NNN>), valid flow codes (SIG, PLN, BLD, GAT, DEP, WIS), and warning behavior.

- **pack-check.md**: Added remediation guidance for check 52 ("Flow boundary violation") explaining how to move CLI invocations to appropriate agents.

- **pack-check.md**: Added remediation guidance for check 53 ("OpenQ prefix invalid") with mapping from non-canonical to canonical codes.

- **stable-markers.md**: Updated line 60 from "PLAN, BUILD" to "PLN, BLD" to align with openq-tools canonical abbreviations per ADR ASM-002.

- **contracts.md**: Updated line 184 from "PLAN, BUILD" to "PLN, BLD" to align with openq-tools canonical abbreviations per ADR ASM-002.

## Deferred / Not Updated (and why)

- `impl_changes_summary.md` - Not available; doc updates based on direct code inspection of drift.rs and contracts.rs
- `subtask_context_manifest.json` - Not present in `.runs/compliance-drift-proofing/build/`
- Docstrings in `drift.rs` - Already adequate; no updates needed (functions have clear doc comments)

## Mismatches Found (if any)

None. Documentation now aligns with implemented behavior:
- Check 52 validates flow boundary enforcement (as implemented in drift.rs lines 693-773)
- Check 53 validates OpenQ prefix patterns (as implemented in drift.rs lines 775-931)
- PLN/BLD abbreviations now consistent across all reference docs and contracts.rs

## Assumptions Made

- ASM-DOC-001: Check IDs 52 and 53 are the final assigned IDs (not 50 and 51 as in original work_plan.md). Impact if wrong: Documentation references wrong check IDs.

- ASM-DOC-002: The --strict flag behavior for checks 52 and 53 follows the existing --strict_warnings pattern (warnings elevated to errors). Impact if wrong: Documentation may misrepresent flag behavior.

## Recommended Next

- Verify pack-check runs cleanly: `bash .claude/scripts/pack-check.sh --no-color`
- Run tests to confirm no regressions: `cargo test -p demoswarm-pack-check`
- Proceed to build-cleanup for receipt generation

## Inventory (machine countable)

- DOC_UPDATED: docs/reference/pack-check.md
- DOC_UPDATED: docs/reference/stable-markers.md
- DOC_UPDATED: docs/reference/contracts.md
