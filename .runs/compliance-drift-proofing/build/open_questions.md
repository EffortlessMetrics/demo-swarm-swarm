# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract

- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run compliance-drift-proofing

### Questions That Would Change the Spec

#### Category: Technical

- QID: OQ-BUILD-001
  - Q: Which check IDs should be used for the new flow boundary (REQ-001) and OpenQ prefix (REQ-003) checks? The ADR and work_plan.md specify checks 50 and 51, but drift.rs already uses check 50 for `check_gh_body_hygiene`. [OPEN]
  - Suggested default: Use check IDs **52** (flow boundary) and **53** (OpenQ prefix) instead of 50 and 51.
  - Impact if different: Using 50/51 would create a collision with existing check 50 in drift.rs, causing runtime errors or silent overwrite.
  - Needs answer by: Before ST-002 implementation
  - Evidence: drift.rs lines 85-88 (check 50 = check_gh_body_hygiene), flow.rs lines 97-100 (check 50 = check_flow_agent_name_resolution)

- QID: OQ-BUILD-002
  - Q: The tests directory `tools/demoswarm-pack-check/tests/` does not exist. Should ST-008/ST-009/ST-010 create this directory and establish the fixtures directory structure from scratch? [OPEN]
  - Suggested default: Yes, create `tools/demoswarm-pack-check/tests/fixtures/` as part of ST-008.
  - Impact if different: If tests should go elsewhere, fixture paths in work_plan.md would need updating.
  - Needs answer by: Before ST-008 implementation
  - Evidence: Glob pattern `tools/demoswarm-pack-check/tests/**/*` returned no files.

- QID: OQ-BUILD-003
  - Q: Check 49 exists in BOTH drift.rs (`check_skills_section_required`) AND flow.rs (`check_inv_marker_contracts`) with completely different purposes. Is this duplicate ID intentional by design (different modules), or is one of these a bug? [OPEN]
  - Suggested default: This is intentional - pack-check allows overlapping IDs across modules (drift vs flow). The drift.rs check 49 correctly implements REQ-002 (Skills section enforcement).
  - Impact if different: If IDs must be unique globally, one check would need renumbering.
  - Needs answer by: Before ST-003 verification
  - Evidence: drift.rs line 80 (check 49 = check_skills_section_required), flow.rs lines 93-95 (check 49 = check_inv_marker_contracts)

- QID: OQ-BUILD-004
  - Q: Should the new checks (52, 53) follow the drift.rs pattern of using `find_matches_regex_recursive` or should they use simpler file iteration like `check_skills_section_required`? [OPEN]
  - Suggested default: Follow the simpler pattern from check 49 (`check_skills_section_required`) - iterate files and check content directly, since both new checks need to examine specific file patterns.
  - Impact if different: More complex regex approach adds implementation time but may be more flexible.
  - Needs answer by: Before ST-002 implementation
  - Evidence: drift.rs check 49 (lines 562-588) uses simple iteration; checks 45-48 use regex helpers.

#### Category: Integration

- QID: OQ-BUILD-005
  - Q: Check 45 and 46 in drift.rs (`check_cleanup_uses_demoswarm_shim` and `check_skill_ownership`) overlap with check 45 in flow.rs (`check_flow_skill_plumbing`). This means the same check ID produces different output depending on which module runs first. Is this acceptable? [OPEN]
  - Suggested default: Yes, this is acceptable - modules are logically separate check collections. The reported check ID includes context from the check title.
  - Impact if different: Would require global ID uniqueness enforcement and renumbering of 20+ existing checks.
  - Needs answer by: Flow 4 (Gate review)
  - Evidence: drift.rs checks 45-48 (lines 63-78), flow.rs checks 45-50 (lines 71-101)

#### Category: Testing

- QID: OQ-BUILD-006
  - Q: What is the minimum valid build_receipt.json schema for the test fixtures (ST-008)? The requirements mention "required fields" but do not enumerate them. [OPEN]
  - Suggested default: Use the existing build_receipt.json files in `.runs/` as reference. Required fields include: `status`, `recommended_action`, `counts`, `quality_gates`.
  - Impact if different: Invalid fixtures would not test the correct validation logic.
  - Needs answer by: Before ST-008 implementation
  - Evidence: requirements.md REQ-004 AC-1 references "passes receipt-checker validation" but does not define the schema.

### Assumptions Made to Proceed

- Assumption: Check IDs 52 and 53 are available and should be used for the new flow boundary and OpenQ prefix checks respectively.
  - Rationale: drift.rs already uses ID 50 for `check_gh_body_hygiene`; the highest existing ID in drift.rs is 50; flow.rs also uses up to 50; using 52/53 provides clear separation.
  - Impact if wrong: Would need to identify alternative IDs or reorganize existing checks.
  - Linked question: OQ-BUILD-001

- Assumption: The ADR and work_plan.md references to "check 50" and "check 51" should be mentally mapped to checks 52 and 53 during implementation.
  - Rationale: The ADR was written before checking existing check ID allocation; the intent is clear (add 2 new checks) even if specific IDs are wrong.
  - Impact if wrong: Literal interpretation would cause check ID collision.
  - Linked question: OQ-BUILD-001

- Assumption: Overlapping check IDs across modules (drift.rs vs flow.rs) is acceptable by design.
  - Rationale: Both modules independently define check 45-50 for different purposes; this appears to be an existing pattern, not a bug.
  - Impact if wrong: Would require significant renumbering effort before adding new checks.
  - Linked question: OQ-BUILD-003, OQ-BUILD-005

- Assumption: PLN/BLD abbreviations are definitively canonical (resolves OQ-PLAN-004 and OQ-SIG-002).
  - Rationale: openq-tools/SKILL.md lines 31 and 140-149 explicitly use PLN/BLD in the Flow Codes Reference table.
  - Impact if wrong: N/A - this assumption is now verified by reading the authoritative source.
  - Linked question: OQ-PLAN-004, OQ-SIG-002

- Assumption: The tests directory structure (`tools/demoswarm-pack-check/tests/fixtures/`) should be created fresh during ST-008.
  - Rationale: Glob returned no files; the directory does not exist; work_plan.md explicitly targets this location.
  - Impact if wrong: Fixtures would go in wrong location.
  - Linked question: OQ-BUILD-002

### Resolutions (if any)

- A: PLN/BLD is the canonical OpenQ prefix format (resolves OQ-PLAN-004 and OQ-SIG-002) [RESOLVED]
  - Evidence: openq-tools/SKILL.md lines 31, 140-149 explicitly define Flow Codes Reference as SIG/PLN/BLD/GAT/DEP/WIS
  - stable-markers.md and contracts.md should be updated to match (covered by ST-006)

### Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
output_path: .runs/compliance-drift-proofing/build/open_questions.md
questions_added: 6
assumptions_added: 5
missing_required: []
blockers: []
concerns:

- CRITICAL: ADR/work_plan check IDs (50, 51) collide with existing drift.rs check 50; must use 52, 53 instead
- Check ID overlap exists across drift.rs and flow.rs modules (IDs 45-50 used differently in each); appears intentional
- tests directory does not exist and must be created from scratch
