# Claims and Evidence

> Evidence over trust. Receipts over vibes.

---

## The Problem: Confident Wrongness

LLMs don't fail by producing nonsense. They fail by being **credibly wrong**.

The failure mode isn't gibberish. It's plausible-sounding claims without backing:

| Claim | Reality |
|-------|---------|
| "Tests passed" | Tests didn't run, or ran against wrong code |
| "No security issues found" | Scanner ran on 3 files, not the codebase |
| "Implementation matches spec" | Semantic drift between code and contract |
| "Coverage is adequate" | Coverage was inferred, not measured |

This is **process confabulation**: confident assertions about what happened when the thing didn't happen, or happened differently than described.

### Why This Matters

A human reviewing "Tests passed" trusts that claim. If the claim is false, the entire review is compromised. The downstream decisions (merge, deploy, ship) are based on fiction.

The pack solves this by requiring **evidence pointers** for claims. Not trust. Evidence.

---

## The Rule

> **Claims must have evidence pointers. "Not measured" is acceptable. False certainty is not.**

This is the discipline that prevents confabulation and builds trust. It applies to:

- Receipts
- PR descriptions
- Gate decisions
- Any artifact that makes claims about what happened

---

## What Counts as Evidence

Different claims require different evidence.

### Tests Passing

**Required evidence:**
- Command that was run (`npm test`, `pytest`, etc.)
- Key output (pass count, failure count)
- Exit code or artifact path

**Example:**
```yaml
test_execution:
  command: "npm test"
  exit_code: 0
  summary: "47 passed, 0 failed"
  artifact: ".runs/feat-auth/build/test_execution.md"
```

**Not evidence:**
```yaml
tests: "all passing"  # No command, no output, no artifact
```

### Security/Quality Checks

**Required evidence:**
- Scanner that was run (tool name, version if relevant)
- Coverage of what was scanned (files, directories)
- Specific findings (or explicit "no findings")

**Example:**
```yaml
secrets_scan:
  tool: "secrets-tools"
  scope: "staged changes (12 files)"
  findings: "none"
  artifact: ".runs/feat-auth/gate/secrets_scan.md"
```

**Not evidence:**
```yaml
security: "no issues"  # Which scanner? What scope?
```

### Spec Conformance

**Required evidence:**
- File:line references
- Diff summaries
- Specific assertions with locations

**Example:**
```yaml
spec_conformance:
  requirement: "REQ-001"
  implementation: "src/auth/login.ts:142-168"
  assertion: "JWT expiration set to 24h per ADR-005"
```

**Not evidence:**
```yaml
requirements: "all met"  # No references, no mapping
```

---

## The Pointer Format

Evidence pointers look like:

| Type | Format | Example |
|------|--------|---------|
| Location | `file:line` | `src/auth.rs:142` |
| Artifact | Relative path | `.runs/feat-auth/build/test_execution.md` |
| Command | Command + result | `npm test (exit 0, 47 passed)` |
| External | URL | `https://github.com/org/repo/actions/runs/123` |

The pointer tells the reader: "If you want to verify this claim, look here."

---

## "Not Measured" Is Normal

It's perfectly acceptable to say:

- "Security scan not run (no scanner configured)"
- "Mutation testing not performed (excluded from this run)"
- "Performance not measured (out of scope)"
- "Integration tests skipped (requires external service)"

**The crime is claiming measurement that didn't happen.**

### Examples

**Honest:**
```yaml
## Proof (measured vs not measured)
- Gate: PASS (evidence: `.runs/feat-auth/gate/gate_receipt.json`)
- Unit tests: 47 passed (evidence: `.runs/feat-auth/build/test_execution.md`)
- **Not measured:** mutation testing, performance, integration tests
```

**Dishonest:**
```yaml
## Proof
- All checks passed
- Comprehensive testing performed
```

The first tells you exactly what was verified and what wasn't. The second sounds confident but says nothing.

---

## Receipts Implement This

Receipts exist to:

1. **Summarize what was actually done** (not what was intended)
2. **Point to the evidence** (not claim without support)
3. **Make audit cheap** (follow pointers, not re-run everything)

### Receipt Qualities

A good receipt is:

- **Human-readable** — Can be understood in 30 seconds
- **Worth reading in 3 months** — Explains what happened, not just what ran
- **Backed by tool outputs** — Evidence comes from execution, not narrative

A bad receipt is:

- **Vague** — "Tests ran successfully"
- **Unsupported** — Claims without artifact paths
- **Narrative-heavy** — Explanations without evidence

### Receipt Structure

```yaml
# Build Receipt
generated_at: "2025-01-10T14:30:00Z"
evidence_sha: "abc123"

## Summary
implementation_status: COMPLETE
test_status: PASS

## Evidence
test_execution:
  command: "npm test"
  exit_code: 0
  summary: "47 passed, 0 failed"
  artifact: ".runs/feat-auth/build/test_execution.md"

lint_execution:
  command: "npm run lint"
  exit_code: 0
  artifact: ".runs/feat-auth/build/lint_output.md"

## Not Executed
- mutation_testing: "not configured"
- integration_tests: "requires external service"
```

---

## The PR Cockpit

The ultimate evidence surface is the PR description. It's where claims meet reviewers.

### Required Sections

```markdown
### Proof (measured vs not measured)
- Gate: PASS | BOUNCE (evidence: `.runs/.../gate_receipt.json`)
- Tests: <summary> (evidence: `.runs/.../test_execution.md`)
- Mutation/fuzz: <ran | not run + reason>
- **Not measured:** <explicit list>
```

### Why This Works

Reviewers can trust the cockpit because:

1. **Claims have pointers** — "Tests passed" points to evidence
2. **Gaps are explicit** — "Not measured" is honest about unknowns
3. **Verification is cheap** — Follow pointers to verify claims
4. **No false certainty** — Unknown remains unknown

---

## Enforcement

This isn't bureaucracy. It's discipline that enables trust.

### Where Enforcement Applies

| Context | Requirement |
|---------|-------------|
| Gate decisions | Must cite evidence for PASS/BOUNCE |
| PR Brief | Must have "Proof" section with pointers |
| Receipts | Must summarize tool outputs, not opinions |
| Merge decisions | Must reference executed verification |

### What Enforcement Looks Like

**Pack-check can warn on:**
- Claims without pointers in receipts
- Gate decisions without evidence_sha
- PR briefs missing "Not measured" section
- Narrative-only artifacts (no tool outputs referenced)

**Human reviewers should ask:**
- "Where's the evidence for this claim?"
- "What was actually measured?"
- "Is this inference or execution?"

---

## The Confabulation Ladder

Understanding how claims degrade helps catch confabulation:

| Level | Example | Trust Level |
|-------|---------|-------------|
| **Executed evidence** | `npm test` exit 0, output captured | High |
| **Artifact pointer** | "See test_execution.md" (exists, recent) | Medium-high |
| **Stale artifact** | "See test_execution.md" (wrong SHA) | Low |
| **Narrative claim** | "Tests passed" (no pointer) | Very low |
| **Implicit claim** | No mention of tests at all | None |

The pack is designed to keep everything at level 1-2. Levels 3-5 are failure modes.

---

## Anti-Patterns

### "Assumed Passing"

```yaml
# BAD
tests: "assumed passing based on previous run"
```

Previous runs don't verify current code. Run the tests.

### "Verified by Review"

```yaml
# BAD
security: "verified by code review"
```

Code review is valuable but not mechanical verification. If you mean "no scanner ran," say that.

### "Should Be Fine"

```yaml
# BAD
integration: "should work with the service"
```

"Should" is not evidence. If you didn't test it, say "not measured."

### "Comprehensive Testing"

```yaml
# BAD
coverage: "comprehensive testing performed"
```

What is comprehensive? Which tests? What coverage? Point to evidence.

---

## Why This Matters for the Pack

The pack exists to enable trusted autonomous operation. That trust is built on:

1. **Gates verify before publish** — Real execution, not claimed execution
2. **Receipts document what happened** — Evidence, not narrative
3. **PR briefs summarize honestly** — Including what wasn't measured
4. **Downstream decisions inherit trust** — Because upstream evidence exists

Without claims-and-evidence discipline, the entire trust model collapses. A single "tests passed" without evidence can compromise a deployment.

---

## The Mantra

**Evidence over trust. Receipts over vibes.**

When you write a receipt, ask: "Can someone verify this claim by following a pointer?"

When you read a receipt, ask: "Is this claim backed by evidence, or is it narrative?"

When you make a gate decision, ask: "What executed evidence supports this?"

The discipline is simple. The value is enormous.

---

## See Also

- [trust-model.md](../reference/trust-model.md) — Evidence hierarchy and verification boundaries
- [pr-review-interface.md](../reference/pr-review-interface.md) — PR Brief template and requirements
- [why-ops-first.md](why-ops-first.md) — Engineering default-allow, publishing gated
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
