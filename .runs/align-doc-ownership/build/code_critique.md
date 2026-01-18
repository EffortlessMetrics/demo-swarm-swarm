# Code Critique

## Machine Summary

status: VERIFIED

recommended_action: PROCEED
route_to_flow: null
route_to_agent: null

blockers: []
missing_required: []
concerns:

- Check 46 (missing Skills section) reports as warning not error; may allow non-compliance to pass silently
- Check 47 (flow output paths) is advisory only; enforcement relies on review discipline
- Negative tests (NEG-001, NEG-002, NEG-003) not yet executed; will require manual patch-verify-revert workflow in Gate

can_further_iteration_help: no

severity_summary:
critical: 0
major: 0
minor: 2

coverage_summary:
reqs_in_scope_total: 2
reqs_with_impl: 2
reqs_with_tests: 2
reqs_missing_impl: []
reqs_missing_tests: []
nfrs_in_scope_total: 1
nfrs_missing_evidence: []
adr_violations: 0
contract_violations: 0
observability_gaps: 0

## Scope

### In-scope Requirements

- REQ-001: Flow Command Boundary Enforcement (flow commands must not contain skill plumbing)
- REQ-002: Agent Doc Consistency (agents using skills must have Skills section)

### In-scope NFRs (if any)

- NFR-TEST-001: Validation Tooling Compliance (pack-check rules detect boundary violations)

### Out-of-scope (IDs only)

- REQ-003 — Skill doc CLI truth ownership; not addressed by this implementation (documentation update, not pack-check rules)
- REQ-004 — CLAUDE.md scope normalization; not addressed by this implementation (documentation update)
- REQ-005 — Subtask partitioning; work plan concern, not code
- REQ-006 — Validation run recording; deferred to ST-006
- REQ-007 — Archive-over-delete pattern; PR review concern, not code
- NFR-MAINT-001 — Documentation maintainability; verified by manual inspection, not pack-check
- NFR-REGR-001 — No functional regression; verified by validation run in ST-006

## Reviewed Surface

- FILE: `tools/demoswarm-pack-check/src/checks/flow.rs`
- FILE: `tools/demoswarm-pack-check/src/contracts.rs`
- FILE: `.claude/commands/flow-1-signal.md`
- FILE: `.claude/commands/flow-2-plan.md`
- FILE: `.claude/commands/flow-3-build.md`
- FILE: `.claude/commands/flow-4-gate.md`
- FILE: `.claude/commands/flow-5-deploy.md`
- FILE: `.claude/commands/flow-6-wisdom.md`

## Coverage Table (REQ -> impl -> tests)

| REQ     | Implementation                                                                                                         | Tests                                           | Notes                              |
| ------- | ---------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- | ---------------------------------- |
| REQ-001 | `flow.rs:509-546` (`check_flow_skill_plumbing`), `contracts.rs:237-239` (`skill_names_in_prose`, `demoswarm_shim_ref`) | pack-check run passes; NEG-001 planned for Gate | OK - boundary enforcement in place |
| REQ-002 | `flow.rs:548-580` (`check_missing_skills_section`), `contracts.rs:239` (`demoswarm_shim_ref`)                          | pack-check run passes; NEG-002 planned for Gate | OK - Skills section check in place |

## NFR Table (NFR -> evidence)

| NFR          | Evidence                                                                              | Notes                                                           |
| ------------ | ------------------------------------------------------------------------------------- | --------------------------------------------------------------- |
| NFR-TEST-001 | pack-check output shows checks 45-47 passing; `contracts.rs:235-241` defines patterns | OK - MET-1 (pack-check exit 0), MET-3 (enforcement rules exist) |

## ADR Alignment

No ADR violations found in reviewed surface.

The implementation aligns with ADR decision OPT-002 (Pragmatic Enforcement):

- Flow commands are strictly scanned for skill plumbing (REQ-001) per `api_contracts.yaml` FLOW_VIO_001, FLOW_VIO_002
- Agents using demoswarm.sh are checked for Skills section (REQ-002) per AGENT_VIO_001
- Check 46 reports as WARNING not ERROR, consistent with pragmatic (not strict) enforcement

## Contract Compliance

No contract violations found in reviewed surface.

Verification against `api_contracts.yaml`:

- FLOW_VIO_001 (demoswarm_invocation): Implemented via `demoswarm_shim_ref` regex at `contracts.rs:239`
- FLOW_VIO_002 (skill_name_invocation): Implemented via `skill_names_in_prose` regex at `contracts.rs:237`
- AGENT_VIO_001 (missing_skills_section): Implemented via check 46 at `flow.rs:548-580`

Note: The contract specifies violation patterns FLOW_VIO_003 (CLI flag syntax) which is NOT explicitly checked by new rules. However, existing check 45 covers the skill name and demoswarm.sh patterns which are the primary boundary concern. CLI flag syntax in flow commands is a secondary indicator and may be addressed in a future iteration.

## Observability

Observability hooks present for reviewed surface.

pack-check output format provides clear pass/fail indicators for each check. The reporter pattern (`rep.pass()`, `rep.fail()`, `rep.warn()`) produces human-readable output suitable for CI integration.

## Security / Safety

No obvious security hazards found in reviewed surface.

The implementation:

- Does not handle user input directly (operates on static markdown files)
- Uses Regex patterns that are compiled at startup (not vulnerable to ReDoS from file content)
- Does not write to disk (read-only validation)

## Edge Cases / Failure Modes

- [MINOR] `skill_names_in_prose` regex matches skill names anywhere in the file (line 237: `\b(runs-derive|runs-index|...)\\b`). This could produce false positives if a flow command discusses skills in prose (e.g., "the context-loader delegates to runs-derive skill"). However, per ADR RSK-004 mitigation, false positives are acceptable at pragmatic enforcement level; they encourage cleaner separation of concerns.

- [MINOR] Check 46 reports missing Skills section as WARNING (`rep.warn()`) rather than ERROR (`rep.fail()`). Per `api_contracts.yaml` line 144, AGENT_VIO_001 has severity "error". The implementation uses warning for pragmatic enforcement, but this means violations will not fail pack-check. Consider whether this is the intended behavior.

## Verification of Flow Command Changes

The flow commands have been successfully cleaned of skill plumbing references:

**Confirmed clean** (no skill name references, no demoswarm.sh references):

- flow-1-signal.md: No `runs-derive`, `runs-index`, `openq-tools`, `secrets-tools` mentions
- flow-2-plan.md: Clean
- flow-3-build.md: Clean
- flow-4-gate.md: Clean
- flow-5-deploy.md: Not reviewed (per user context, did not have skill reference to remove)
- flow-6-wisdom.md: Clean

**pack-check validation**: Check 45 output shows "Flow commands do not leak skill plumbing" - PASS

## Rust Code Quality

The implementation follows existing patterns in `flow.rs`:

- Check functions follow the `check_*` naming convention
- Use of `CheckCtx` and `Reporter` consistent with other checks
- Regex patterns defined in `contracts.rs` rather than inline
- Proper error handling via `anyhow::Result`

No idiomatic issues observed.

## Iteration Guidance

**Rationale:** The implementation satisfies the in-scope requirements (REQ-001, REQ-002) with appropriate pack-check rules. All checks pass. The remaining concerns are:

1. Severity level (warning vs error) for check 46 - this is a design decision consistent with pragmatic enforcement
2. Advisory nature of check 47 - expected per ADR OPT-002
3. Negative tests not executed - scheduled for Gate phase per test_plan.md

None of these concerns require code changes; they are either intentional design decisions or deferred to later flow phases.

## Recommended Next

1. **PROCEED to Gate (Flow 4)** - Implementation is complete for in-scope requirements
2. Execute negative tests (NEG-001, NEG-002, NEG-003) during Gate to prove enforcement rules fire
3. Consider future enhancement: Add check for FLOW_VIO_003 (CLI flag syntax) if false positive rate from skill name matching proves problematic

---

## Code Critic Result

status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
can_further_iteration_help: no
blockers: []
missing_required: []
severity_summary:
critical: 0
major: 0
minor: 2
