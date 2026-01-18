# Design Options

## Enumerated Requirement IDs

From `.runs/align-doc-ownership/signal/requirements.md`:

**Functional Requirements:**

- REQ-001: Flow Command Boundary Enforcement
- REQ-002: Agent Doc Consistency
- REQ-003: Skill Doc Ownership
- REQ-004: CLAUDE.md Scope Normalization
- REQ-005: Subtask Partitioning by Flow
- REQ-006: Validation Run Recording
- REQ-007: Archive-Over-Delete Pattern

**Non-Functional Requirements:**

- NFR-MAINT-001: Documentation Maintainability
- NFR-TEST-001: Validation Tooling Compliance
- NFR-REGR-001: No Functional Regression

---

## OPT-001: Strict Enforcement

### Description

This option adds comprehensive pack-check rules that fail on any boundary violation. Every agent that uses a skill must have a `## Skills` section. No inline CLI examples are permitted in agent docs - all CLI reference must live exclusively in skill docs, and agents reference skills by name only (e.g., "per runs-derive SKILL.md"). Flow commands are scanned for any mention of `demoswarm.sh`, skill names (`runs-derive`, `runs-index`, `openq-tools`, `secrets-tools`), or CLI flag syntax (`--file`, `--prefix`, `--run-id`) and fail the build if any are found.

The pack-check Rust codebase receives three new check modules: (1) `flow_boundary.rs` for flow command scanning, (2) `agent_skills.rs` for Skills section enforcement, (3) `agent_enums.rs` for status/action enum validation. These checks run as part of the standard `pack-check` invocation and produce machine-parseable JSON output for CI integration.

Agent docs are refactored to remove all inline CLI examples. Cleanup agents (which currently have extensive `demoswarm.sh` examples) are reduced to skill references only. For example, instead of:

```markdown
# Example invocation (before)

bash .claude/scripts/demoswarm.sh count pattern --file ... --regex ...
```

The agent doc becomes:

```markdown
# Skill usage (after)

Use the `runs-derive` skill per SKILL.md for all count operations.
```

This enforces maximum separation of concerns but creates indirection for operators reading agent docs.

### Requirements Fit

| Requirement   | Fit       | Notes                                                                              |
| ------------- | --------- | ---------------------------------------------------------------------------------- |
| REQ-001       | SATISFIED | Flow commands strictly scanned; any skill plumbing fails pack-check                |
| REQ-002       | SATISFIED | All 55 agents get Skills sections where applicable; no inline CLI examples allowed |
| REQ-003       | SATISFIED | Skill docs become sole CLI truth; agents reference them exclusively                |
| REQ-004       | SATISFIED | CLAUDE.md normalized to summary-level; no flag details                             |
| REQ-005       | SATISFIED | Subtask partitioning unchanged; pack-check rules added in ST-004                   |
| REQ-006       | SATISFIED | Validation run records pack-check passing with strict rules                        |
| REQ-007       | SATISFIED | Archive pattern followed for removed CLI examples                                  |
| NFR-MAINT-001 | SATISFIED | Clear ownership boundaries enforced by tooling                                     |
| NFR-TEST-001  | SATISFIED | Pack-check rules detect all boundary violations; negative tests required           |
| NFR-REGR-001  | SATISFIED | No functional changes; documentation-only refactoring                              |

### Trade-offs

| Dimension                                        | Impact | Rationale                                                              |
| ------------------------------------------------ | ------ | ---------------------------------------------------------------------- |
| Structure (coupling, components)                 | Low    | Maximum separation; agents tightly coupled to skill docs via reference |
| Velocity (time-to-first-change)                  | High   | Requires reading skill doc before understanding agent operation        |
| Governance (auditability, determinism)           | Low    | Clear boundaries; tooling-enforced; no judgment calls                  |
| Operability (on-call, monitoring, failure modes) | Medium | Operators must cross-reference skill docs to understand agent behavior |
| Cost (compute, complexity tax)                   | Medium | More pack-check rules; more Rust code to maintain; indirection cost    |

### Reversibility

- Rating: Moderate
- Switch cost: Relaxing rules is easy (remove checks); re-adding inline examples requires editing 55 agent files
- Blast radius if wrong: Reduced operator productivity; increased onboarding time; many "see skill doc" references feel unhelpful

### Risks

| Risk                                      | Likelihood | Impact | Mitigation (if chosen)                                           |
| ----------------------------------------- | ---------- | ------ | ---------------------------------------------------------------- |
| Operator confusion from indirection       | Medium     | Medium | Add hyperlinks in agent docs to specific skill doc sections      |
| False positives in pack-check rules       | Low        | Medium | Careful regex patterns; test with known-good files               |
| Skill doc gaps expose incomplete coverage | Medium     | High   | Audit skill docs for completeness before removing agent examples |

### Assumptions

- All skill docs are complete enough to serve as sole CLI reference - impact if wrong: operators have no usable reference for some commands
- Operators are comfortable navigating to skill docs - impact if wrong: reduced productivity, increased friction

### When to Choose This

Choose strict enforcement when documentation ownership clarity is paramount and the team is willing to accept indirection cost. Best for mature packs where skill docs are already comprehensive.

---

## OPT-002: Pragmatic Enforcement

### Description

This option adds pack-check rules for major violations (skill plumbing in flows, missing Skills sections in agents that use skills, enum inconsistencies) but allows brief inline examples in agent docs when the skill doc lacks coverage for a specific use case. The rule is: agents may include minimal examples only when needed for mechanical clarity, but must reference the skill doc as the authoritative source.

Flow commands are scanned strictly - any `demoswarm.sh`, skill-name invocation, or CLI flag syntax fails pack-check. This boundary is non-negotiable because flow commands are read by orchestrators who should not see implementation details.

Agent docs receive a `## Skills` section listing which skills they use. The agent may include a brief example if:

1. The skill doc does not cover the specific invocation pattern, OR
2. The example is critical for understanding the agent's core operation (e.g., cleanup agents that derive counts)

The pack-check rule detects undeclared skill usage (agent references `demoswarm.sh` without a Skills section) but does not require removing all inline examples. A new check `agent_skill_declaration` ensures consistency: if an agent uses `demoswarm.sh` or skill-specific CLI, it must declare the skill.

This approach balances ownership clarity with operator usability. Cleanup agents retain their most important examples while removing duplicates that exist in skill docs.

### Requirements Fit

| Requirement   | Fit       | Notes                                                                                          |
| ------------- | --------- | ---------------------------------------------------------------------------------------------- |
| REQ-001       | SATISFIED | Flow commands strictly scanned; skill plumbing fails pack-check                                |
| REQ-002       | SATISFIED | All agents using skills get Skills sections; enums normalized; minimal inline examples allowed |
| REQ-003       | PARTIAL   | Skill docs are primary CLI truth; agents may have supplementary examples (not duplicates)      |
| REQ-004       | SATISFIED | CLAUDE.md normalized to summary-level; no flag details                                         |
| REQ-005       | SATISFIED | Subtask partitioning unchanged; pack-check rules added in ST-004                               |
| REQ-006       | SATISFIED | Validation run records pack-check passing with pragmatic rules                                 |
| REQ-007       | SATISFIED | Archive pattern followed for removed content                                                   |
| NFR-MAINT-001 | PARTIAL   | Clear ownership for major boundaries; some flexibility in agents                               |
| NFR-TEST-001  | SATISFIED | Pack-check rules detect major violations; covers flow boundary + skill declaration             |
| NFR-REGR-001  | SATISFIED | No functional changes; documentation-only refactoring                                          |

### Trade-offs

| Dimension                                        | Impact | Rationale                                                                   |
| ------------------------------------------------ | ------ | --------------------------------------------------------------------------- |
| Structure (coupling, components)                 | Medium | Agents reference skills but may have local examples; some coupling remains  |
| Velocity (time-to-first-change)                  | Medium | Agent docs self-contained for common cases; skill docs for advanced usage   |
| Governance (auditability, determinism)           | Medium | Clear flow boundaries; agent boundaries have judgment calls                 |
| Operability (on-call, monitoring, failure modes) | Low    | Operators can understand agent behavior from agent doc alone for most cases |
| Cost (compute, complexity tax)                   | Low    | Fewer pack-check rules; less Rust code; lower indirection cost              |

### Reversibility

- Rating: Easy
- Switch cost: Can tighten to strict enforcement by adding rules; can loosen by removing rules
- Blast radius if wrong: Minimal; some inline examples remain that might drift (acceptable risk)

### Risks

| Risk                                         | Likelihood | Impact | Mitigation (if chosen)                                             |
| -------------------------------------------- | ---------- | ------ | ------------------------------------------------------------------ |
| Drift between agent examples and skill docs  | Medium     | Low    | doc-drift script catches duplicates; manual review for consistency |
| Judgment calls on "when examples are needed" | Medium     | Low    | Define guideline: examples allowed only if not in skill doc        |
| Incomplete skill doc coverage                | Medium     | Medium | Track gaps; backfill skill docs over time                          |

### Assumptions

- Pack maintainers can exercise judgment on "minimal examples" - impact if wrong: inconsistent application of rule
- Skill docs will be backfilled for gaps over time - impact if wrong: agents retain examples indefinitely
- User input strongly suggests this option ("agents may include minimal examples only when needed for mechanical clarity")

### When to Choose This

Choose pragmatic enforcement when balancing documentation ownership with operator usability. Best for packs in active development where skill docs may have gaps and operators need self-contained agent docs.

---

## OPT-003: Minimal Enforcement (Manual Fix)

### Description

This option fixes known violations manually without adding new pack-check rules. The 5 flow commands with skill references are edited to remove the skill plumbing. The 8 agents using `demoswarm.sh` without Skills sections get Skills sections added. CLAUDE.md is normalized to summary-level. No new pack-check drift checks are implemented.

Boundary enforcement relies on PR review discipline rather than automated tooling. The `doc-drift` script (`scripts/check-doc-drift.sh`) may be expanded with shell-based checks for the most critical violations (e.g., `grep -l "demoswarm.sh" .claude/commands/*.md` should return empty), but these are advisory rather than blocking.

This option has the lowest implementation cost but provides no regression prevention. Future changes could reintroduce skill plumbing in flow commands without detection. The validation run (Toy Run A/B) still occurs but only validates current state, not ongoing compliance.

### Requirements Fit

| Requirement   | Fit       | Notes                                                              |
| ------------- | --------- | ------------------------------------------------------------------ |
| REQ-001       | TRADE_OFF | Flow commands cleaned manually; no automated regression prevention |
| REQ-002       | TRADE_OFF | Agents fixed manually; no automated Skills section enforcement     |
| REQ-003       | SATISFIED | Skill docs updated; agents reference them                          |
| REQ-004       | SATISFIED | CLAUDE.md normalized manually                                      |
| REQ-005       | SATISFIED | Subtask partitioning unchanged                                     |
| REQ-006       | SATISFIED | Validation run records manual cleanup success                      |
| REQ-007       | SATISFIED | Archive pattern followed                                           |
| NFR-MAINT-001 | PARTIAL   | Boundaries established but not enforced by tooling                 |
| NFR-TEST-001  | TRADE_OFF | No new pack-check rules; relies on doc-drift shell script          |
| NFR-REGR-001  | SATISFIED | No functional changes                                              |

### Trade-offs

| Dimension                                        | Impact | Rationale                                                |
| ------------------------------------------------ | ------ | -------------------------------------------------------- |
| Structure (coupling, components)                 | Low    | Boundaries fixed but not enforced; drift possible        |
| Velocity (time-to-first-change)                  | Low    | No new tooling to learn; minimal process change          |
| Governance (auditability, determinism)           | High   | No automated enforcement; relies on PR review discipline |
| Operability (on-call, monitoring, failure modes) | Low    | Agent docs remain self-contained                         |
| Cost (compute, complexity tax)                   | Low    | No Rust development; no new infrastructure               |

### Reversibility

- Rating: Easy
- Switch cost: Can add pack-check rules later as separate effort
- Blast radius if wrong: Drift reoccurs over time; requires periodic manual audits

### Risks

| Risk                                                 | Likelihood | Impact | Mitigation (if chosen)                                     |
| ---------------------------------------------------- | ---------- | ------ | ---------------------------------------------------------- |
| Regression without detection                         | High       | Medium | Periodic manual audits; doc-drift script as advisory check |
| Boundary violations reintroduced                     | High       | Medium | PR review guidelines; CODEOWNERS on flow commands          |
| No negative test for pack-check (NFR-TEST-001 MET-3) | High       | Medium | Document deviation from NFR; accept known gap              |

### Assumptions

- PR reviewers will catch boundary violations - impact if wrong: silent drift accumulates
- Manual audits will occur periodically - impact if wrong: boundaries erode
- Rust development capacity is limited or timeline is constrained - impact if wrong: OPT-002 would be preferable

### When to Choose This

Choose minimal enforcement when timeline pressure prevents pack-check development, or when the team prefers to treat boundary enforcement as a future effort. Acceptable for small teams with strong review culture.

---

## Comparison Matrix

| Dimension             | OPT-001            | OPT-002        | OPT-003           |
| --------------------- | ------------------ | -------------- | ----------------- |
| REQ coverage (count)  | 7/7                | 7/7            | 5/7               |
| NFR coverage (count)  | 3/3                | 3/3            | 1/3               |
| Implementation effort | High               | Medium         | Low               |
| Reversibility         | Moderate           | Easy           | Easy              |
| Ops burden            | Medium             | Low            | Low               |
| Primary risk          | Operator confusion | Judgment drift | Silent regression |

## Suggested Default (non-binding)

suggested_default: OPT-002
confidence: High

Rationale (tie to IDs):

- REQ-001 is strictly satisfied (flow boundary enforcement is non-negotiable per user input)
- REQ-002 allows pragmatic flexibility (user explicitly stated "minimal examples only when needed for mechanical clarity")
- REQ-003 partial fit is acceptable because skill docs remain authoritative; supplementary examples reduce operator friction
- NFR-TEST-001 is satisfied by detecting major violations (flow skill plumbing, undeclared skill usage)
- Risk RSK-004 (false positives) is lower with pragmatic rules than strict enforcement
- User's target end state explicitly calls for "minimal inline examples only when necessary"

What would change this:

- If skill docs are already comprehensive and operators are comfortable with indirection, prefer OPT-001
- If timeline pressure is severe or Rust development capacity is constrained, prefer OPT-003 as interim step
- If pack-check cannot be extended without major refactoring (ASM-004 invalid), prefer OPT-003

## Open Questions Affecting Choice

- Q: Are skill docs complete enough to remove all inline examples from agents? (OQ-SIG-001) - default if unanswered: No, retain minimal examples in agents
- Q: Is Rust development acceptable for this run? (Questions for Humans Q3 in requirements.md) - default if unanswered: Yes, Rust is acceptable if straightforward
- Q: Should we add pack-check rules for boundary enforcement? (OQ-SIG-006) - default if unanswered: Yes, add pack-check rules
- Q: How strictly should we enforce "no skill plumbing in flows"? (OQ-SIG-004) - default if unanswered: Strict enforcement for flows

## Shared Assumptions

- ASM-001: The three-tier ownership model (flow commands -> agents -> skills) is the intended architecture
- ASM-002: Cleanup agents legitimately need operational detail; they execute work, not just orchestrate
- ASM-003: CLAUDE.md is entry-point level; detailed CLI reference belongs in skill docs
- ASM-004: pack-check can be extended with boundary checks without major refactoring
- ASM-005: Subtask partitioning (ST-001 through ST-006 by flow) is the intended decomposition

---

## Machine Summary

status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

missing_required: []

blockers: []

options_proposed: 3
suggested_default: OPT-002
confidence: High
