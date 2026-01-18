# Documentation Critique

## Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns: []
observations: []
can_further_iteration_help: no

## Inputs Used

- `docs/reference/pack-check.md` (updated with checks 52, 53)
- `docs/reference/stable-markers.md` (updated with PLN/BLD prefixes)
- `docs/reference/contracts.md` (updated with PLN/BLD prefixes)
- `tools/demoswarm-pack-check/src/checks/drift.rs` (implementation source for checks 52, 53)

## Stale / Missing Docs (worklist)

None.

## User-Visible Changes Needing Notes

### Check 52: Flow Boundary Enforcement

- **What changed**: New check validates that flow commands do NOT contain `demoswarm.sh` or skill CLI subcommands (count, ms, yaml, index, receipt, receipts, openapi, line, inv, time, openq, secrets).
- **Enforcement level**: Warnings by default; use `--strict` to elevate to errors.
- **Doc accuracy**: pack-check.md correctly documents this at lines 68-74 with accurate description, valid pattern list, and correct warning-by-default behavior.

### Check 53: OpenQ Prefix Validation

- **What changed**: New check validates QID patterns in `.runs/**/open_questions.md` use canonical flow codes (SIG, PLN, BLD, GAT, DEP, WIS).
- **Valid format**: `OQ-<FLOW>-<NNN>` where `<NNN>` is three-digit zero-padded (001-999).
- **Enforcement level**: Warnings by default for non-canonical codes (e.g., PLAN→PLN, BUILD→BLD) or invalid format.
- **Doc accuracy**: pack-check.md correctly documents this at lines 76-84 with exact mappings and enforcement behavior.

### Stable Markers: PLN/BLD Prefixes

- **What changed**: Both `stable-markers.md` and `contracts.md` document the canonical flow code abbreviations including PLN (Plan) and BLD (Build).
- **stable-markers.md**: Line 60 correctly lists `SIG, PLN, BLD, GAT, DEP, WIS` as the flow code values for open questions.
- **contracts.md**: Line 184 correctly references the same flow prefixes in the Open question markers section.
- **Doc accuracy**: Both references are consistent and match the implementation (drift.rs line 850 checks against `cx.c.openq_flow_codes`, which the config supplies as the canonical set).

## Verification Guidance Gaps

### pack-check.md Verification (lines 93-115)

- "Green run" output example (lines 95-105) is realistic and includes the two new check titles.
- "Red run" output (lines 107-114) correctly shows error/warning counts and realistic message format.
- JSON schema (lines 118-142) is accurate and correctly documents `check_id` numeric routing.
- **No gaps**: verification examples are actionable and match the reported check semantics.

### --strict flag documentation (line 74 and 84)

- pack-check.md correctly states that violations are "reported as warnings" and "Use `--strict` to elevate to errors."
- Implementation (drift.rs lines 759 and 904) uses `rep.warn()` for both checks, consistent with the documentation.
- **No gaps**: the "how to verify" guidance is accurate and actionable.

## Recommended Next

- None. Documentation is complete and accurate.

## Inventory (machine countable)

- No DOC_CRITIC items identified.

---

## Cross-Cutting Observations

1. **Documentation Completeness**: All three updated files correctly document the new checks with:
   - Exact pattern descriptions (check 52: "demoswarm.sh or skill CLI subcommands"; check 53: "canonical flow codes")
   - Valid examples (check 53: PLAN→PLN, BUILD→BLD mappings)
   - Correct enforcement semantics (warnings by default; `--strict` escalates)

2. **Consistency**: The PLN/BLD prefix references are consistent across stable-markers.md (signal markers) and contracts.md (inventory markers). Both correctly use the three-character canonical codes.

3. **Verification Realism**: Examples in pack-check.md accurately reflect what actual executions will produce. The JSON schema section is machine-readable and matches the implemented diagnostics structure.

4. **"How to Verify" Guidance**: Section "When validation fails" (lines 206-243) includes new remediation:
   - Check 52 (line 226-230): Flow boundary violation + remediation (move CLI to agent)
   - Check 53 (line 232-242): OpenQ prefix invalid + remediation (update QIDs to canonical)
     Both match the implementation semantics exactly.

---
