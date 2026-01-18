# Principle: Evidence Over Trust

> Decisions are based on artifacts, not faith.

## The Principle

When gate agents make decisions (merge, deploy, release), they base those decisions on evidence in artifacts — not on trust that earlier steps were done correctly.

## Why This Matters

### Trust Doesn't Scale

In a complex system:

- Many agents contribute
- Context resets happen
- Memory is imperfect

"I trust that the tests passed" is fragile. "The test report shows 47 passed, 0 failed" is solid.

### Evidence Is Auditable

When something goes wrong:

- "Why did we ship this?"
- Evidence answers: "Here's what the gate saw"
- Trust can't answer: "I thought it was fine"

### Evidence Survives Context Loss

Across sessions:

- Artifacts persist
- Trust doesn't

The merge-decider next week can see the same evidence the merge-decider today saw.

## How It Works

### Evidence Chain

```
Signal produces → requirements.md, features/*.feature
    ↓
Plan references → requirements (evidence of what to build)
Plan produces → adr.md, api_contracts.yaml
    ↓
Build references → plan artifacts (evidence of how to build)
Build produces → code, tests, test_execution.md
    ↓
Gate references → build artifacts (evidence of what was built)
Gate produces → merge_decision.md (evidence of decision)
```

Each flow produces evidence. Next flow consumes it.

### Gate Evidence

Merge-decider reviews:

| Evidence               | What It Proves                   |
| ---------------------- | -------------------------------- |
| build_receipt.json     | Build completed, counts captured |
| test_execution.md      | Tests ran, results known         |
| code_critique.md       | Code was reviewed                |
| contract_compliance.md | Contracts honored                |
| security_scan.md       | Security checked                 |

Decision is based on this evidence, not assumption.

### Evidence Quality

Good evidence:

- Exists (artifact is there)
- Is recent (from this run)
- Is complete (has the data needed)
- Is honest (reflects actual state)

Questionable evidence:

- Missing artifacts
- Stale data
- Incomplete reports
- Overstated claims

### Handling Missing Evidence

When evidence is missing:

```markdown
## Decision

Cannot recommend MERGE.

**Missing evidence:**

- test_execution.md not found (tests may not have run)
- security_scan.md not found (security not checked)

**Recommendation:** Run test-executor and security-scanner, then re-run gate.
```

The gate doesn't assume tests passed. It notes evidence is missing.

## Evidence Types

### Existence Evidence

"The artifact exists" proves the step ran.

- test_execution.md exists → tests were executed
- code_critique.md exists → code was reviewed

### Content Evidence

"The artifact contains X" proves the outcome.

- test_execution.md shows 47 passed → tests passed
- code_critique.md shows 0 critical → no critical issues

### Derived Evidence

"Counting markers" proves quantities.

- 5 `### REQ-` markers → 5 requirements
- 3 `[CRITICAL]` markers → 3 critical issues

### Absence Evidence

"The artifact doesn't contain X" proves absence.

- No findings in security_scan.md → no security issues found
- Empty blockers in receipt → no blockers

## Anti-Patterns

### Assuming Success

"Build said it was done, so tests probably passed."

**Fix:** Check test_execution.md for actual results.

### Trusting Status Fields

"The status says VERIFIED so we're good."

**Fix:** Status is a summary. Check the underlying evidence.

### Ignoring Missing Artifacts

"security_scan.md doesn't exist but let's merge anyway."

**Fix:** Missing evidence = unknown state. Don't assume positive.

### Outdated Evidence

Using evidence from a previous run or different branch.

**Fix:** Evidence must be from this run, this branch.

## Evidence in Decisions

### Good Decision Document

```markdown
# Merge Decision

## Evidence Reviewed

| Artifact           | Status  | Key Data            |
| ------------------ | ------- | ------------------- |
| build_receipt.json | Present | status: VERIFIED    |
| test_execution.md  | Present | 47 passed, 0 failed |
| code_critique.md   | Present | 0 critical, 2 minor |
| security_scan.md   | Present | No findings         |

## Analysis

All evidence present and positive:

- Tests pass (evidence: test_execution.md)
- No critical issues (evidence: code_critique.md)
- Security clear (evidence: security_scan.md)

## Decision

**MERGE** — Evidence supports shipping.
```

### Poor Decision Document

```markdown
# Merge Decision

Build looks good. Merging.
```

No evidence cited. No way to audit.

## The Evidence Hierarchy

1. **Artifacts** — Primary evidence (test reports, critiques)
2. **Receipts** — Summary evidence (counts, status)
3. **Handoffs** — Narrative evidence (what agent reported)

Gates should check artifacts, not just trust receipts.

## See Also

- [Artifacts with Substance](artifacts-with-substance.md) — Good artifacts provide evidence
- [The Gate Pattern](gate-pattern.md) — Where evidence is checked
