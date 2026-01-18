# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract

- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run compliance-drift-proofing

### Questions That Would Change the Spec

#### Category: Product

- QID: OQ-SIG-001
  - Q: Should new pack-check rules be warnings first or immediate failures? [OPEN]
  - Suggested default: Warnings first with --strict flag for failures (incremental adoption)
  - Impact if different: Immediate failures would require careful rollout and migration path for existing artifacts
  - Needs answer by: Flow 2
  - Evidence: problem_statement.md -> Constraints (backward compatibility)

- QID: OQ-SIG-003
  - Q: Should cross-agent handshake validation extend beyond Build-to-Gate (e.g., Signal-to-Plan, Plan-to-Build)? [OPEN]
  - Suggested default: No, focus on Build-to-Gate as the most critical handoff first
  - Impact if different: Broader scope would require more test fixtures and validation rules per flow boundary
  - Needs answer by: Flow 2
  - Evidence: problem_statement.md -> Success Looks Like (mentions only Build-to-Gate)

- QID: OQ-SIG-008
  - Q: What is the relationship between this work and the bounced align-doc-ownership (issue #49)? Should this block on #49 resolution? [OPEN]
  - Suggested default: Proceed independently; this work complements but does not depend on #49 completion
  - Impact if different: If #49 resolves first, some validation rules here may need adjustment
  - Needs answer by: Flow 2
  - Evidence: problem_statement.md -> Prior Art (issue #49 bounced at Gate)

#### Category: Technical

- QID: OQ-SIG-002
  - Q: Is PLN vs PLAN discrepancy intentional? stable-markers.md line 60 says PLAN/BUILD but openq-tools/SKILL.md uses PLN/BLD [OPEN]
  - Suggested default: PLN/BLD is canonical (matches openq-tools which is the implementation)
  - Impact if different: Need to update stable-markers.md line 60 to use PLN/BLD instead of PLAN/BUILD for consistency
  - Needs answer by: Flow 3
  - Evidence: stable-markers.md line 60 vs openq-tools/SKILL.md Flow Codes Reference

- QID: OQ-SIG-005
  - Q: Should flow boundary enforcement (no demoswarm.sh in flow commands) be a pack-check rule (Rust) or check-doc-drift.sh rule (Bash)? [OPEN]
  - Suggested default: pack-check (Rust) for consistency with other structural validation
  - Impact if different: Bash implementation would be faster but creates maintenance divergence
  - Needs answer by: Flow 3
  - Evidence: problem_statement.md -> Constraints (pack-check is Rust-based)

- QID: OQ-SIG-006
  - Q: Should the skill list in flow commands be derived dynamically from .claude/skills/ directory or hardcoded? [OPEN]
  - Suggested default: Hardcoded list is acceptable; dynamic discovery adds complexity without clear benefit
  - Impact if different: Dynamic discovery would auto-detect new skills but require parsing infrastructure
  - Needs answer by: Flow 2
  - Evidence: context_brief.md -> Flow Commands (boundary enforcement targets)

- QID: OQ-SIG-007
  - Q: Should check-doc-drift.sh functionality be consolidated into pack-check Rust CLI or remain separate? [OPEN]
  - Suggested default: Keep separate; consolidation is explicitly a non-goal per problem statement
  - Impact if different: Consolidation would reduce tool count but requires significant Rust work
  - Needs answer by: Flow 2
  - Evidence: problem_statement.md -> Non-Goals (explicitly states not merging tools)

- QID: OQ-SIG-009
  - Q: Should receipt-to-contract validation (e.g., build_receipt.json schema) be enforced at receipt creation time or only at Gate? [OPEN]
  - Suggested default: Gate validation only (receipt-checker already does this)
  - Impact if different: Creation-time validation would catch errors earlier but requires changes to cleanup agents
  - Needs answer by: Flow 3
  - Evidence: context_brief.md -> Receipt Contract Auditing (receipt-checker validates at Gate)

#### Category: Data

(none identified)

#### Category: Ops

- QID: OQ-SIG-004
  - Q: Which agents using demoswarm.sh are intentionally exempt from Skills section requirements (if any)? [OPEN]
  - Suggested default: None; all 14 agents using demoswarm.sh should have Skills sections
  - Impact if different: If exemptions exist, need to document them rather than adding Skills sections
  - Needs answer by: Flow 3
  - Evidence: context_brief.md -> Skills Section Usage Pattern (10/14 have Skills, 4 may be missing)

### Assumptions Made to Proceed

- Assumption: The three-tier ownership model (flow commands -> agent docs -> skill docs) is authoritative and stable.
  - Rationale: problem_statement.md explicitly references this as a constraint from issue #49
  - Impact if wrong: Ownership boundary enforcement rules would need redesign
  - Linked question: OQ-SIG-008

- Assumption: PLN/BLD abbreviations in openq-tools are canonical over PLAN/BUILD in stable-markers.md.
  - Rationale: openq-tools is the implementation; stable-markers.md appears to have a documentation error
  - Impact if wrong: Would need to update openq-tools Rust code instead of documentation
  - Linked question: OQ-SIG-002

- Assumption: Warning-before-failure is acceptable for new validation rules.
  - Rationale: Enables incremental adoption without breaking existing valid artifacts
  - Impact if wrong: All new checks would need immediate enforcement
  - Linked question: OQ-SIG-001

- Assumption: The 4 agents using demoswarm.sh without Skills sections are gaps to fix, not intentional exceptions.
  - Rationale: No documented exemption policy exists; consistency is the default expectation
  - Impact if wrong: Would need to document exemption criteria rather than add missing sections
  - Linked question: OQ-SIG-004

- Assumption: Build-to-Gate is the most critical handoff and should be validated first before expanding to other boundaries.
  - Rationale: Problem statement success criteria focus on Build-to-Gate specifically
  - Impact if wrong: Would need broader test fixtures from the start
  - Linked question: OQ-SIG-003

### Resolutions (if any)

(none yet)

### Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: 1
route_to_agent: requirements-author
output_path: .runs/compliance-drift-proofing/signal/open_questions.md
questions_added: 9
assumptions_added: 5
missing_required: []
blockers: []
concerns:

- stable-markers.md line 60 uses PLAN/BUILD but openq-tools uses PLN/BLD (documentation inconsistency detected)
- output_path inferred from invocation context (signal flow)

## Update: run compliance-drift-proofing (iteration 2)

### Questions That Would Change the Spec

#### Category: Technical

- QID: OQ-SIG-010
  - Q: Should test fixtures (REQ-004) be committed to the repo or generated dynamically during CI? [OPEN]
  - Suggested default: Committed to `tools/demoswarm-pack-check/tests/fixtures/` for stability and reviewability
  - Impact if different: Dynamic generation would reduce repo size but add CI complexity and make fixtures harder to review/debug
  - Needs answer by: Flow 3
  - Evidence: problem_statement.md -> Questions / Clarifications Needed (item 6)

### Assumptions Made to Proceed

- Assumption: Test fixtures for Build-to-Gate handshake validation should be committed files, not dynamically generated.
  - Rationale: Committed fixtures are reviewable, version-controlled, and deterministic; dynamic generation adds CI complexity
  - Impact if wrong: Would need CI infrastructure to generate fixtures on the fly
  - Linked question: OQ-SIG-010

- Assumption: Iteration 2 did not invalidate any iteration 1 questions or assumptions.
  - Rationale: Problem statement iteration 2 contains same 6 questions as identified in context; no questions resolved by GitHub research
  - Impact if wrong: Some questions may need to be marked resolved or updated
  - Linked question: null

### Resolutions (if any)

(none in iteration 2)

### Machine Summary

status: VERIFIED
recommended_action: PROCEED
route_to_flow: 1
route_to_agent: requirements-author
output_path: .runs/compliance-drift-proofing/signal/open_questions.md
questions_added: 1
assumptions_added: 2
missing_required: []
blockers: []
concerns:

- All 10 questions (OQ-SIG-001 through OQ-SIG-010) remain OPEN; none resolved in iteration 2
