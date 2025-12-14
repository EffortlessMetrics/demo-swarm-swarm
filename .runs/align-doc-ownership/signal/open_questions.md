# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract
- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run align-doc-ownership

### Questions That Would Change the Spec

#### Category: Product

- QID: OQ-SIG-001
  - Q: Should cleanup agents reference skill docs rather than embed CLI examples inline? [OPEN]
  - Suggested default: Yes - agents should reference skill docs (e.g., 'use runs-derive skill per SKILL.md') and remove duplicated CLI invocation examples
  - Impact if different: Keeps agents DRY but adds indirection; if different, agents must maintain their own CLI examples and risk drift
  - Needs answer by: Flow 2 (Plan)
  - Evidence: github_research.md -> Prior Art Pointers (notes cleanup agents have extensive demoswarm.sh examples)

- QID: OQ-SIG-002
  - Q: Is the CLAUDE.md Skills table meant to stay summary-level (one-line per command)? [OPEN]
  - Suggested default: Yes - CLAUDE.md stays summary-level; detailed flag/contract info lives only in skill docs
  - Impact if different: Expanding CLAUDE.md with flag details creates duplication; if different, CLAUDE.md becomes deep reference and skill docs become secondary
  - Needs answer by: Flow 2 (Plan)
  - Evidence: CLAUDE.md -> Skills section (currently has one-line per command)

#### Category: Technical

- QID: OQ-SIG-003
  - Q: Should flow commands ever mention specific CLI flags or skill plumbing (e.g., 'bash demoswarm.sh ...')? [OPEN]
  - Suggested default: No - flow commands own orchestration/routing only; specific CLI flags are agent/skill concerns
  - Impact if different: Flows currently avoid this pattern; if different, flows would need to be updated whenever CLI flags change
  - Needs answer by: Flow 2 (Plan)
  - Evidence: github_research.md -> Prior Art Pointers ("Minimal skill invocation detail in flow commands")

- QID: OQ-SIG-004
  - Q: How strictly should we enforce the 'no skill plumbing in flows' rule? [OPEN]
  - Suggested default: Strict enforcement - any mention of demoswarm.sh, runs-derive, or similar in flow commands is a boundary violation
  - Impact if different: Strict enforcement catches all drift; if lenient, some skill references might be acceptable for clarity
  - Needs answer by: Flow 2 (Plan)
  - Evidence: issue_normalized.md -> Constraints (flows must contain no skill plumbing)

- QID: OQ-SIG-005
  - Q: What about existing agents that mix operational and contract content (e.g., both 'what to output' and 'how to invoke CLI')? [OPEN]
  - Suggested default: Separate - operational detail (CLI invocation) belongs in agent docs; contract content (output format/schema) belongs in agent docs but references skill docs for CLI
  - Impact if different: Clear separation aids maintenance; if different, some agents may legitimately embed CLI examples when skill indirection is too costly
  - Needs answer by: Flow 2 (Plan)
  - Evidence: context_brief.md -> Agent Docs (55 files to audit for consistency)

- QID: OQ-SIG-006
  - Q: Should we add new pack-check rules for boundary enforcement (detecting skill plumbing in flows, CLI drift in agents)? [OPEN]
  - Suggested default: Yes - add pack-check rules that fail on boundary violations (skill plumbing in flows, undeclared skill usage)
  - Impact if different: Pack-check enforcement catches regressions; if no new rules, boundary enforcement relies on manual review
  - Needs answer by: Flow 3 (Build)
  - Evidence: issue_normalized.md -> Constraints (pack-check must pass including new drift checks)

#### Category: Data

(none identified)

#### Category: Ops

(none identified)

### Assumptions Made to Proceed

- Assumption: The three-tier ownership model (flow commands -> agents -> skills) is the intended architecture.
  - Rationale: Recent commits (be0c81a, 186ea53) show active alignment work toward this structure; github_research confirms pattern.
  - Impact if wrong: The entire boundary definition would need rethinking.
  - Linked question: null

- Assumption: Cleanup agents legitimately need operational detail (they execute the work, not just orchestrate).
  - Rationale: Cleanup agents perform counts, derive summaries, and write receipts - they need to know how to invoke tools.
  - Impact if wrong: Cleanup agents would become thin orchestrators, pushing more work to skill implementations.
  - Linked question: OQ-SIG-001

- Assumption: CLAUDE.md is meant to be entry-point level, not deep reference.
  - Rationale: CLAUDE.md states it is "repo-level policy + shared contracts" and defers to skills for details.
  - Impact if wrong: CLAUDE.md would need significant expansion to serve as primary reference.
  - Linked question: OQ-SIG-002

- Assumption: This run is a continuation of existing alignment work, not a new direction.
  - Rationale: Commits be0c81a and 186ea53 are recent and explicitly mention "clarity and consistency" improvements.
  - Impact if wrong: We might contradict prior decisions.
  - Linked question: null

### Resolutions (if any)

(none yet)

### Machine Summary
```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: 1
route_to_agent: null
output_path: .runs/align-doc-ownership/signal/open_questions.md
questions_added: 6
assumptions_added: 4
missing_required: []
blockers: []
concerns:
  - All questions are non-blocking (defaults provided)
  - Questions primarily affect Flow 2 (Plan) decisions
```
