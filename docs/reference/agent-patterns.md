# Agent Patterns Reference

> Quick reference for Claude-native agent patterns.

---

## Communication Patterns

### Prose Handoff

```markdown
## Handoff

**What I did:** Reviewed implementation against spec.

**What I found:** Session timeout wrong (30m vs 15m spec).

**Recommendation:** code-implementer fix, then re-run me.
```

### YAML Status Block

```yaml
## Machine Summary
status: UNVERIFIED
recommended_action: RERUN
route_to_agent: code-implementer
```

**Why prose wins:** Orchestrators are Claude threads. They read prose better than they parse YAML. Machine Summary is for audit trails, not routing.

---

## Artifact Patterns

### Substantive Decision

```markdown
# Merge Decision

## Evidence Reviewed

Build produced 47 tests covering the authentication flow. All pass.
Contract check found POST /users returns 200 but spec says 201.
Security scan clean. Coverage at 82% line, 71% branch.

## Analysis

The status code mismatch is mechanical (one line fix). Tests prove the
logic works. Coverage is adequate for auth paths. The mismatch doesn't
affect clients because the response body is correct.

## Decision

**Merge** — status code fix can happen in the same PR. The implementation
is sound and the mismatch is trivial.

## Notes for Future Readers

Accepted the 201 vs 200 discrepancy because response shape is correct.
If clients start checking status codes strictly, revisit this.
```

### Stubby Gate

```json
{
  "verdict": "MERGE",
  "confidence": 0.87,
  "checks_passed": true
}
```

**Why substance wins:** Decisions need reasoning. "Why did we merge despite the contract violation?" The substantive version answers that; the stub doesn't.

---

## Prompt Patterns

### Positive Guidance

```markdown
## Tips

- Focus on correctness first, style second
- Cite specific locations for easy fixes
- Explain why the issue matters, not just where it is
- Name who should fix it when you know
```

### Constraint Lists

```markdown
## Constraints

- Do NOT modify files outside scope
- NEVER skip the summary block
- You are FORBIDDEN from committing
- You MUST NOT assume the schema exists
```

**Why positive wins:** "Do this" produces more capable behavior than "don't do that." Constraints create defensive, hedge-everything agents.

---

### Single Responsibility

```markdown
## Your Job

Review the implementation against the spec. Find issues.
```

### Kitchen Sink

```markdown
## Your Responsibilities

1. Review implementation
2. Check contracts
3. Validate security
4. Update receipt
5. Post to GitHub
6. Notify stakeholders
7. Log metrics
```

**Why single wins:** Agents with one job do it well. Agents with seven jobs do all of them badly.

---

### Graceful Failure

```markdown
## If You're Stuck

Report what you tried and what blocked you.
Partial progress with clear next steps is valuable.

**Blocked but made progress:**
> Reviewed 3 of 5 REQs before hitting missing schema. Found 2 issues
> in what I could review. Blocked on REQ-004 pending schema migration.
> Recommend continuing after schema work completes.

**Mechanical failure:**
> Cannot read impl_changes_summary.md — file doesn't exist. Need
> code-implementer to run first.
```

### Silent Failure

*(No failure handling section = agent stops and orchestrator doesn't know why)*

**Why graceful wins:** The orchestrator routes on your signals. Silent failure = no signal = stuck workflow.

---

## Routing Patterns

### Natural Language Routing

```markdown
**Recommendation:** Run code-implementer to fix the timeout value,
then back to me for verification. The issue is mechanical — no design
questions involved.
```

### Field-Based Routing

```yaml
route_to_agent: code-implementer
route_to_flow: 3
reason_code: IMPL_FIX_NEEDED
```

**Why natural wins:** The orchestrator is a Claude thread. It understands "run X to fix Y, then back to me" better than it parses routing fields.

---

## Receipt Patterns

### Audit Summary

```markdown
## Signal Receipt

**What was produced:**
- 5 functional requirements (REQ-001 through REQ-005)
- 2 non-functional requirements (NFR-PERF-001, NFR-SEC-001)
- 3 BDD scenarios covering the happy path and two error cases
- 1 open question about auth token expiry (defaulted to 15m)

**Quality notes:**
- REQ-002 may be underspecified (no error handling defined)
- All requirements have acceptance criteria

**Counts:**
- Functional: 5, Non-functional: 2, Scenarios: 3, Questions: 1
```

### Routing Input

```json
{
  "status": "VERIFIED",
  "req_count": 5,
  "nfr_count": 2,
  "scenario_count": 3,
  "question_count": 1,
  "route_to": null
}
```

**Why narrative wins:** Receipts are audit artifacts. "Why did Signal produce VERIFIED with open questions?" The narrative version explains; the JSON doesn't.

---

## Quick Reference Table

| Do This | Not This | Why |
|---------|----------|-----|
| Prose handoff | YAML status blocks | Orchestrators read prose |
| Positive tips | Constraint lists | "Do X" > "don't do Y" |
| One job | Multiple responsibilities | Focus produces quality |
| "Run X next" | `route_to_agent: X` | Natural language routing |
| Substantive artifacts | Stubby JSON gates | Decisions need reasoning |
| Graceful failure | Silent failure | No signal = stuck workflow |
| Audit narrative | Routing fields | Receipts explain, not route |

---

## See Also

- [agent-philosophy.md](../explanation/agent-philosophy.md) — Why agents work this way
- [contracts.md](contracts.md) — Control-plane blocks and schemas
- [documentation-conventions.md](documentation-conventions.md) — Writing style for agent prompts
