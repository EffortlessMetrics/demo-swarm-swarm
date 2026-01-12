# Principle: Artifacts Reduce Future Work

> Artifacts exist because they are worth reading later. If an artifact has no future reader, do not create it.

## The Principle

Every artifact should reduce future work for someone: the next agent, a reviewer, a future developer, or the system itself. Artifacts that serve only process compliance — with no genuine reader — are waste.

**Corollary:** Artifacts for parsers are smell. Artifacts for humans are product.

---

## Why This Matters

### Artifact Bloat Drowns Signal

When every step produces artifacts "because the process says so," the result is noise:
- Important findings get lost in templates
- Reviewers skim instead of read
- Disk fills with unread compliance logs

The PR cockpit should be scannable in 30 seconds. That is impossible when buried under process artifacts no one reads.

### Artifacts Are Part of the Product

The system produces trust, not code. Artifacts are how trust is communicated:
- Receipts prove what happened
- ADRs explain why decisions were made
- Critiques identify what was found

If an artifact does not contribute to trust, it does not contribute to the product.

### Future Readers Are Real

Someone will read these artifacts:

| Reader | What They Need |
|--------|----------------|
| **Next agent** | Context to continue work |
| **Reviewer** | Evidence to make merge decision |
| **Future developer** | Reasoning behind decisions |
| **Auditor** | Proof that process was followed |

Write for them, not for a compliance checkbox.

---

## Who Reads What

### The Next Agent

Agents read artifacts from previous steps to understand context:

```
code-critic reads impl_changes_summary.md → knows what changed
test-author reads requirements.md → knows what to test
cleanup reads all artifacts → knows what to summarize
```

If an artifact does not help the next agent, it is probably not needed.

### The Reviewer

Reviewers scan the PR cockpit to decide: merge or not merge.

Useful for reviewers:
- `merge_decision.md` — summary with evidence pointers
- `build_receipt.json` — mechanical counts and quality gates
- Hotspot list — where to spot-check

Not useful for reviewers:
- Raw logs nobody will read
- Process compliance timestamps
- Intermediate state dumps

### The Future Developer

Months later, someone asks: "Why did we build it this way?"

Useful for future developers:
- `adr.md` — explains the design decision and trade-offs
- `requirements.md` — explains what problem we were solving
- Comments in code — explain non-obvious choices

Not useful for future developers:
- `process_step_completed.json` — no reasoning, just timestamps

---

## Good Artifacts vs Bad Artifacts

### Receipts Summarize Evidence (Good)

```markdown
# Build Receipt

## Summary
Implemented OAuth2 login per REQ-001. 23 tests pass.

## Quality Events
- Code critic: 1 minor (naming convention in auth module)
- Test critic: Good coverage

## Evidence Pointers
- impl_changes_summary.md — what changed
- test_execution.md — test results
- code_critique.md — review findings
```

This reduces work: the reviewer knows what happened without digging.

### ADRs Explain Decisions (Good)

```markdown
# ADR-005: Session Timeout Policy

## Context
We need to balance security (shorter sessions) against UX (fewer re-logins).

## Decision
15-minute timeout with sliding window.

## Consequences
- Users re-authenticate if idle 15+ minutes
- Session extends on activity
- Complies with security policy SEC-003
```

This reduces work: future developers know why, not just what.

### Process Compliance Logs (Bad)

```json
{
  "step": "code_review",
  "completed_at": "2025-12-11T14:32:00Z",
  "status": "DONE"
}
```

No one reads this. It proves nothing. It explains nothing.

---

## Violation vs Correct

### Violation: Artifact for the Parser

```json
{
  "gate_status": "PASS",
  "checks": ["lint", "test", "security"],
  "ready_for_next_step": true
}
```

This is routing data dressed as an artifact. If routing happens via prose handoffs (Law 2), this serves no purpose.

### Correct: Artifact for the Human

```markdown
# Gate Decision

## Checks Passed
- Lint: 0 issues (auto-linter ran)
- Tests: 47 passed, 0 failed
- Security: Clean scan

## Decision
PASS — all quality gates met. Ready for merge review.
```

Same information, but readable. A human can understand and audit.

---

### Violation: Template Without Content

```markdown
## Findings
[None]

## Blockers
[None]

## Recommendation
Proceed.
```

The template was filled in, but nothing was communicated.

### Correct: Substance Even When Empty

```markdown
## Findings

No issues found. Code follows established patterns in the auth module.
Session handling matches the approach in `src/auth/existing_session.rs`.

## Recommendation

Proceed — implementation is straightforward and consistent with codebase.
```

Even "no findings" can communicate something useful.

---

### Violation: Creating Artifacts "Because Process Says So"

```
- process_started.json
- process_step_1_complete.json
- process_step_2_complete.json
- process_finished.json
```

Four artifacts, zero information. The git log already shows what happened when.

### Correct: One Artifact That Matters

```markdown
# Build Summary

Implemented 3 endpoints. 47 tests pass. Code critic found 2 minor issues
(addressed). Ready for gate review.
```

One artifact with actual content beats four empty ones.

---

## The Artifact Test

Before creating an artifact, ask:

1. **Who will read this?** (Name the reader: next agent, reviewer, future dev)
2. **What will they learn?** (Specific information, not "that the step ran")
3. **Could they get this elsewhere?** (git log, other artifacts, obvious from code)

If you cannot answer these questions, the artifact might not be needed.

---

## Summary

| Artifact Type | Purpose | Future Reader |
|---------------|---------|---------------|
| Receipts | Summarize evidence | Reviewer, auditor |
| ADRs | Explain decisions | Future developer |
| Critiques | Identify issues | Implementer, reviewer |
| Change summaries | Describe what changed | Next agent, reviewer |
| Process compliance logs | (None) | (None — avoid creating) |

**The rule:** Artifacts reduce work for real readers. If no one will read it, do not create it.

---

## See Also

- [Artifacts with Substance](artifacts-with-substance.md) — Good artifacts have real content
- [Evidence Over Trust](evidence-over-trust.md) — Artifacts provide evidence
- [Laws of the Swarm](../laws-of-the-swarm.md) — Law 9 in context
- [Run State](../../reference/run-state.md) — Where artifacts live
