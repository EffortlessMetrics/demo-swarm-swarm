# ADR: Documentation Ownership Boundaries

## Status
Swarm-Proposed (run-scoped; pending human review at Flow 2 boundary)

## Context
- Problem: The DemoSwarm pack documentation suffers from blurred ownership boundaries between its three documentation tiers (flow commands, agent docs, skill docs). Flow commands contain skill plumbing that should live in agent or skill docs, agent docs inconsistently apply status/action enums, and CLAUDE.md duplicates deep reference material that belongs in skill docs. This structural coupling makes the pack harder to maintain and obscures where authoritative information lives.
- Constraints:
  - `pack-check` must pass after changes, including new boundary-enforcement drift checks
  - `doc-drift` (`scripts/check-doc-drift.sh`) must pass
  - All 55 agent docs must be consistent on status/recommended_action enums, Skills sections, and file-write rules
  - Flow commands must contain no skill plumbing (`demoswarm.sh`, skill-name invocations, CLI flags)
  - Validation run (Toy Run A/B through flows 1-4) must be recorded
  - Archive-over-delete pattern (PR #48) must be followed for moved/removed content
- Non-goals:
  - Changing functional behavior of any flow, agent, or skill
  - Redesigning the three-tier documentation architecture (only enforcing it)
  - Adding new features to the pack
  - Upstream repo export or synchronization

## Decision Drivers (bound, machine-countable)
Each driver MUST include a stable marker line, then a short explanation.

- DRIVER: DR-001 req=[REQ-001] nfr=[NFR-MAINT-001] option_ref="OPT-002"
  - Why it matters: Flow commands must contain only orchestration and routing; skill plumbing creates coupling that increases drift risk and maintenance burden.

- DRIVER: DR-002 req=[REQ-002] nfr=[NFR-TEST-001] option_ref="OPT-002"
  - Why it matters: Agent doc consistency (enums, Skills sections, file-write rules) enables automated validation and reduces cognitive load during reviews.

- DRIVER: DR-003 req=[REQ-003] nfr=[NFR-MAINT-001] option_ref="OPT-002"
  - Why it matters: Skill docs as CLI truth source eliminates duplication and establishes a single authoritative location for command reference.

- DRIVER: DR-004 req=[REQ-004] nfr=[] option_ref="OPT-002"
  - Why it matters: CLAUDE.md as entry point (not deep reference) keeps the pack approachable and avoids duplicating skill-level detail.

- DRIVER: DR-005 req=[] nfr=[NFR-TEST-001] option_ref="OPT-002"
  - Why it matters: Pragmatic enforcement allows minimal inline examples in agents when skill docs have gaps, balancing ownership clarity with operator usability.

- DRIVER: DR-006 req=[] nfr=[NFR-REGR-001] option_ref="OPT-002"
  - Why it matters: Easy reversibility minimizes blast radius if the enforcement rules prove too strict or too lenient.

## Decision
We choose **OPT-002: Pragmatic Enforcement**.

### What we are doing
- Adding pack-check rules for major boundary violations:
  - Flow commands containing `demoswarm.sh`, skill-name invocations, or CLI flag syntax
  - Agent docs using skills without a `## Skills` section
  - Agent docs with non-canonical status/action enum values
- Allowing brief inline CLI examples in agent docs only when:
  - The skill doc does not cover the specific invocation pattern, OR
  - The example is critical for understanding the agent's core operation (e.g., cleanup agents)
- Normalizing CLAUDE.md to summary-level Skills table (no flag details)
- Auditing 55 agent docs for consistency: enums, Skills sections, file-write rules
- Cleaning 6 flow commands of any skill plumbing
- Recording validation run (Toy Run A/B) in `docs/maintainers/validation-log.md`

### What we are NOT doing
- Removing all inline examples from agent docs (strict enforcement would require this)
- Relying solely on manual PR review without automated enforcement (minimal enforcement)
- Changing the three-tier ownership model itself
- Modifying functional behavior of flows, agents, or skills
- Expanding CLAUDE.md to serve as deep CLI reference

### Requirements & NFR Traceability
- **Satisfied by this decision**
  - REQ-001: Flow commands strictly scanned; any skill plumbing fails pack-check
  - REQ-002: All agents using skills get Skills sections; enums normalized; minimal inline examples allowed where needed
  - REQ-004: CLAUDE.md normalized to summary-level; no flag details
  - REQ-005: Subtask partitioning (ST-001 through ST-006) preserved
  - REQ-006: Validation run recorded after alignment complete
  - REQ-007: Archive-over-delete pattern followed for removed content
  - NFR-TEST-001: Pack-check rules detect major violations (flow skill plumbing, undeclared skill usage, enum inconsistencies)
  - NFR-REGR-001: No functional changes; documentation-only refactoring; validation run confirms no regressions
- **Trade-offs / partial support**
  - REQ-003: PARTIAL - Skill docs are primary CLI truth; agents may have supplementary examples (not duplicates) when skill coverage gaps exist
  - NFR-MAINT-001: PARTIAL - Clear ownership for major boundaries (flows, CLAUDE.md); agent-level examples have judgment calls on "when needed"

## Alternatives Considered
- ALT: OPT-001 (Strict Enforcement) -- Rejected because: Maximum separation creates operator confusion through excessive indirection; requires skill docs to be comprehensively complete before agents can remove examples; higher implementation cost with limited additional benefit; operator productivity reduced by constant cross-referencing.
- ALT: OPT-003 (Minimal Enforcement / Manual Fix) -- Rejected because: No automated regression prevention; boundary violations would reintroduce silently; relies on PR review discipline which does not scale; does not satisfy NFR-TEST-001 MET-3 (negative tests for pack-check).

## Consequences

### Positive
- Clear ownership boundaries enforced by tooling for major violations (flow commands, skill usage declaration)
- Reduced cognitive load for pack maintainers: authoritative location for each concern is well-defined
- Balanced approach: operators can understand agent behavior from agent doc alone for most cases
- Easy reversibility: can tighten to strict enforcement or loosen rules as needed
- Validation run provides confidence that alignment changes cause no regressions

### Negative
- Judgment calls required on "when are inline examples needed" in agent docs
- Potential for drift between agent examples and skill docs (mitigated by doc-drift script)
- Skill docs may have coverage gaps initially; requires backfilling over time
- Pack-check rules require Rust development (acceptable per ASM-004)

## Risks and Mitigations
Use stable markers:

- RISK: RSK-001 Drift between agent examples and skill docs over time. -> Mitigation: doc-drift script catches duplicates; manual review for consistency; clear guideline that examples allowed only if not in skill doc.
- RISK: RSK-002 Judgment calls on "when examples are needed" applied inconsistently. -> Mitigation: Define guideline: inline examples allowed only if skill doc lacks coverage for that specific pattern.
- RISK: RSK-003 Incomplete skill doc coverage delays agent example removal. -> Mitigation: Track gaps; backfill skill docs incrementally; accept that some agent examples persist until coverage complete.
- RISK: RSK-004 pack-check false positives on boundary detection (regex too aggressive). -> Mitigation: Careful regex patterns; test with known-good files before deployment; pragmatic rules less prone to false positives than strict rules.

## Assumptions Made to Proceed
Use stable markers:

- ASM: ASM-001 The three-tier ownership model (flow commands -> agents -> skills) is the intended architecture. (impact if wrong: Entire boundary definition and REQ-001 through REQ-004 would need rethinking)
- ASM: ASM-002 Cleanup agents legitimately need operational detail; they execute work, not just orchestrate. (impact if wrong: Cleanup agents would need to become thin orchestrators)
- ASM: ASM-003 CLAUDE.md is entry-point level; detailed CLI reference belongs in skill docs. (impact if wrong: CLAUDE.md would expand to serve as primary reference)
- ASM: ASM-004 pack-check can be extended with boundary-enforcement checks without major refactoring. (impact if wrong: May need to fall back to shell scripts for enforcement)
- ASM: ASM-005 Pack maintainers can exercise judgment on "minimal examples" consistently. (impact if wrong: Inconsistent application of rules; may need to tighten to strict enforcement)

## Questions / Clarifications Needed
Use stable markers and include suggested defaults:

- Q: Are skill docs complete enough to serve as sole CLI reference today? Suggested default: No, retain minimal examples in agents where skill coverage has gaps. Impact: If yes, could tighten to OPT-001 strict enforcement.
- Q: Should we add pack-check rules for enum consistency in agent docs? Suggested default: Yes, detect non-canonical status/action values. Impact: If no, enum enforcement relies on manual review.
- Q: Should the validation run (Toy Run A/B) block Flow 3 or be part of ST-006? Suggested default: Include as final step in ST-006. Impact: If separate, adds coordination overhead.

## Next Steps (Flow 2 binding)
- Interface/contracts -> `.runs/align-doc-ownership/plan/api_contracts.yaml` + `.runs/align-doc-ownership/plan/schema.md`
- Observability -> `.runs/align-doc-ownership/plan/observability_spec.md`
- Tests -> `.runs/align-doc-ownership/plan/test_plan.md` (map to BDD + verification_notes if present)
- Work breakdown -> `.runs/align-doc-ownership/plan/work_plan.md`

## Pointers
- Options: `.runs/align-doc-ownership/plan/design_options.md`
- Requirements: `.runs/align-doc-ownership/signal/requirements.md`
- Problem statement: `.runs/align-doc-ownership/signal/problem_statement.md`
- Impact: `.runs/align-doc-ownership/plan/impact_map.json`
- Open questions: `.runs/align-doc-ownership/signal/open_questions.md`

## Inventory (machine countable)
(Only the following prefixed lines; do not rename prefixes)

- ADR_CHOSEN_OPTION: OPT-002
- ADR_DRIVER: DR-001
- ADR_DRIVER: DR-002
- ADR_DRIVER: DR-003
- ADR_DRIVER: DR-004
- ADR_DRIVER: DR-005
- ADR_DRIVER: DR-006
- ADR_ALT: OPT-001
- ADR_ALT: OPT-003
- ADR_RISK: RSK-001
- ADR_RISK: RSK-002
- ADR_RISK: RSK-003
- ADR_RISK: RSK-004
- ADR_ASM: ASM-001
- ADR_ASM: ASM-002
- ADR_ASM: ASM-003
- ADR_ASM: ASM-004
- ADR_ASM: ASM-005
- ADR_Q: skill-doc-completeness
- ADR_Q: enum-consistency-rules
- ADR_Q: validation-run-timing

## Machine Summary Block

```yaml
## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - Judgment calls on "minimal examples" may drift without clear guidelines
  - Skill doc coverage gaps may delay full agent example removal
  - ST-004 (cross-cutting enforcement) carries heavier scope than other subtasks

chosen_option: OPT-002 Pragmatic Enforcement
drivers_total: 6
```
