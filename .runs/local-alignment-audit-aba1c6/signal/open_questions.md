# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract
- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run local-alignment-audit-aba1c6

### Questions That Would Change the Spec

#### Category: Product

- QID: OQ-SIG-001
  - Q: Are the 10 flow commands implementing 6 main flows with variant re-entry paths, or 7 distinct flows? [OPEN]
  - Suggested default: Seven distinct flows (Signal, Plan, Build, Review, Gate, Deploy, Wisdom) with Flows 4-6 having variant commands for different entry points into the same flow phase
  - Impact if different: If 6 flows, documentation update is simpler (just explain variants); if 7 flows, public docs (README, DEMO_RUN, architecture.md) must change "six flows" to "seven flows" and enumerate Flow 7 distinctly
  - Needs answer by: Flow 2 (Plan)
  - Evidence: github_research.md -> "Core Finding: Six-Flow vs Seven-Flow Discrepancy" (README L67 says "six flows"; CLAUDE.md L13 says "7 flows"; 10 command files exist on disk)

- QID: OQ-SIG-002
  - Q: What is the intended use case for /flow-7-wisdom (vs /flow-6-wisdom)? [OPEN]
  - Suggested default: /flow-7-wisdom is for second-cycle or batch wisdom extraction across multiple runs (distinct from /flow-6-wisdom which follows a single deploy)
  - Impact if different: If same purpose as /flow-6-wisdom, one should be deprecated; if distinct but undocumented, need documentation explaining when to use each
  - Needs answer by: Flow 2 (Plan)
  - Evidence: github_research.md -> "Flow 7 purpose" question; .claude/commands/flow-7-wisdom.md exists but is undocumented in public guides

- QID: OQ-SIG-003
  - Q: Should Flow 7 have its own subtask (ST-007) in compliance run requirements partitioning? [OPEN]
  - Suggested default: Yes, if Flow 7 is a distinct flow (not a variant), it warrants ST-007 to ensure compliance coverage
  - Impact if different: If Flow 7 is not distinct, current ST-001 through ST-006 partitioning is correct; if distinct, compliance runs are missing coverage
  - Needs answer by: Flow 2 (Plan)
  - Evidence: github_research.md -> compliance-drift-proofing run shows "ST-005 covers Flow 5, ST-006 covers Flow 6, but no ST-007"

#### Category: Technical

- QID: OQ-SIG-004
  - Q: Is the path traversal in secrets.rs exploitable given its local-only execution context? [OPEN]
  - Suggested default: Low risk in current context (local execution, no untrusted input paths), but should be addressed for defense-in-depth
  - Impact if different: If exploitable (e.g., user-controlled paths from .runs/ content or symlinks), requires immediate fix; if not exploitable, can document as accepted risk with mitigation plan
  - Needs answer by: Flow 3 (Build)
  - Evidence: issue_normalized.md -> "secrets scanner accepts raw paths without canonicalization (no realpath/canonicalize calls)" at lines 76-124 of secrets.rs

- QID: OQ-SIG-005
  - Q: Should agent color coding (frontmatter `color:` field) be schema-validated, or remain advisory? [OPEN]
  - Suggested default: Remain advisory (no schema validation); colors are for human consumption (VS Code, documentation) not routing
  - Impact if different: If used for routing or UI logic, missing color fields would cause failures and require schema enforcement
  - Needs answer by: Flow 2 (Plan)
  - Evidence: context_brief.md -> "Agent color coding consistency" risk; agent frontmatter example in issue_normalized.md shows `color: red` in test-critic.md

#### Category: Data

- QID: OQ-SIG-006
  - Q: What is the correct test count story for DemoSwarm? (102 unit passing + 41 integration filtered) [OPEN]
  - Suggested default: Report as "102 unit tests passing, 41 integration tests require manual environment setup" with explanation of why integration tests are filtered in standard runs
  - Impact if different: If users expect 374 tests (or similar higher number), documentation must clarify what the number represents; if 102 is the full count, public claims must be updated to match
  - Needs answer by: Before merge
  - Evidence: issue_normalized.md -> "test result: ok. 102 passed; 0 failed; 0 ignored; 0 measured; 277 filtered out" from test_output.log line 109

#### Category: Ops

(none identified)

### Assumptions Made to Proceed

- Assumption: CLAUDE.md (stating "7 flows") is the source of truth for flow count; public docs (README, DEMO_RUN, architecture.md) are outdated.
  - Rationale: CLAUDE.md is described as "repo-level policy + shared contracts" and explicitly lists "7 flows: Signal -> Plan -> Build -> Review -> Gate -> Deploy -> Wisdom"
  - Impact if wrong: CLAUDE.md would need correction instead of public docs
  - Linked question: OQ-SIG-001

- Assumption: Flow variants (flow-4-gate vs flow-4-review, flow-5-gate vs flow-5-deploy, etc.) are intentional design patterns for re-entry, not bugs or duplicates.
  - Rationale: Each variant represents a different entry point into the review/gate/deploy/wisdom cycle; enables rework flows after gate rejection
  - Impact if wrong: Duplicate commands would need consolidation rather than documentation
  - Linked question: OQ-SIG-002

- Assumption: The path traversal concern in secrets.rs is a defense-in-depth improvement, not a critical vulnerability.
  - Rationale: secrets.rs is invoked locally by agents with controlled paths from .runs/ artifacts; no evidence of untrusted external input reaching this code
  - Impact if wrong: Would require immediate security fix before proceeding with other work
  - Linked question: OQ-SIG-004

- Assumption: The ReDoS claim about secrets.rs is invalid because Rust's regex crate uses finite automata (not backtracking).
  - Rationale: issue_normalized.md explicitly states "Rust regex crate used (not vulnerable to ReDoS via finite automata implementation)"
  - Impact if wrong: Would require regex pattern review and potential mitigation
  - Linked question: null

- Assumption: The "374 tests" claim (if it exists elsewhere) is aspirational or includes integration tests that are currently filtered.
  - Rationale: test_output.log shows 102 passed + 277 filtered = 379 total tests defined; actual pass count is 102
  - Impact if wrong: Test documentation may need to clarify what "passing" means vs "defined"
  - Linked question: OQ-SIG-006

### Resolutions (if any)

(none yet)

### Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 2
route_to_agent: null
output_path: .runs/local-alignment-audit-aba1c6/signal/open_questions.md
questions_added: 6
assumptions_added: 5
missing_required: []
blockers: []
concerns:
  - Six-Flow vs Seven-Flow discrepancy is the most impactful question (affects all public documentation)
  - All questions have suggested defaults allowing work to proceed
  - output_path was provided by orchestrator context (not inferred)
