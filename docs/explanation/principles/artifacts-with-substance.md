# Principle: Artifacts with Substance

> Artifacts exist because the content matters, not as routing gates.

## The Principle

Every artifact an agent produces should be worth reading. It should capture real information, explain reasoning, and be useful to humans reviewing later. Artifacts are not JSON blobs for machines to parse.

## Why This Matters

### Artifacts Outlive the Run

Someone will read these artifacts later:
- Reviewing a PR
- Debugging an issue
- Understanding why a decision was made
- Auditing the process

Stubby JSON routing gates are useless to them.

### Understanding Over Parsing

The orchestrator (Claude) doesn't parse artifacts for routing — it reads agent handoffs. So artifacts don't need to be machine-optimized. They should be human-optimized.

### Audit Trail

Artifacts form the audit trail of the development process. A good audit trail explains:
- What was done
- What was found
- Why decisions were made

### Knowledge Capture

A substantive artifact captures knowledge that would otherwise be lost:
- Why did we choose this approach?
- What issues did we find and address?
- What trade-offs did we make?

## How It Works

### The Artifact Test

Ask: If someone reads this artifact in 3 months, do they understand:
- What was done?
- What was found?
- Why decisions were made?
- What context matters?

If yes → purposeful artifact
If no → stubby routing gate

### Good Artifacts Explain Why

```markdown
# Merge Decision

## Evidence Reviewed
- Build: 47 tests pass, implementation complete
- Critics: 2 minor findings (naming conventions in auth module)
- Security: Clean scan
- Contracts: All endpoints match spec

## Analysis
Implementation is solid. Tests prove the behavior. The naming issues are
style-level — auth module uses camelCase where we prefer snake_case.
Not a correctness or security concern.

Coverage is 82%. The untested 18% is error logging paths that are
difficult to unit test. Acceptable.

## Decision
**Merge.** The work is done. Ship it.

## Notes for Future
The naming inconsistency should get cleaned up eventually.
Not worth blocking this PR.
```

### Bad Artifacts Are Stubby

```json
{
  "verdict": "MERGE",
  "blockers": [],
  "confidence": "high"
}
```

No one learns anything from this.

### Artifact Categories

| Artifact | Should Contain |
|----------|----------------|
| **Critique** | What's wrong, why it matters, how to fix, severity |
| **Receipt** | What happened, what was checked, key findings |
| **Decision** | Evidence reviewed, analysis, decision, reasoning |
| **Changes summary** | What changed, why, impact |

## Anti-Patterns

### Field-Only Artifacts
```json
{"status": "VERIFIED", "blockers": [], "counts": {"tests": 47}}
```

### Template Without Content
```markdown
## Findings
[None]

## Recommendation
Proceed.
```

### Machine-Optimized Format
Artifacts optimized for parsing rather than reading.

### Missing Context
Stating a decision without explaining why.

## Examples

### Good: Substantive Critique
```markdown
# Code Critique

## Critical Finding
**Session timeout mismatch.** Implementation uses 30-minute timeout
(src/auth/session.rs:47), but ADR-005 specifies 15 minutes for
security compliance. This must be fixed.

## Major Findings
1. **Missing rate limiting.** Login endpoint has no rate limiting.
   REQ-002 requires "prevent brute force." Add rate limiter middleware.

2. **Error messages leak info.** "User not found" vs "Wrong password"
   tells attackers which usernames exist. Use generic "Invalid credentials."

## Minor Findings
- Inconsistent naming: `sessionTimeout` vs `session_timeout`
- TODO comment should be tracked as issue

## Recommendation
Critical must be fixed. Major should be fixed. Minor can defer.
```

### Good: Substantive Receipt
```markdown
# Build Receipt

## Summary
Implemented health check endpoint per REQ-001. Added 47 tests.

## What Was Built
- src/api/health.rs — endpoint implementation
- src/api/health_test.rs — test suite

## Quality Checks
- Code critic: 2 minor findings (non-blocking)
- Test critic: Good coverage, added one suggested edge case

## Test Results
47 passed, 0 failed. Coverage: 94% of new code.

## Notes
Health endpoint returns 200 with `{"status": "ok"}` when DB reachable,
503 otherwise. Matches contract in api_contracts.yaml.
```

### Bad: Stubby Receipt
```json
{"status": "VERIFIED", "tests_passed": 47, "tests_failed": 0}
```

## See Also

- [Agent Philosophy](../agent-philosophy.md)
- [How to Design Agents](../../how-to/design-agents.md)
