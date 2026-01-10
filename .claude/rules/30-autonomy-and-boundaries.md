# Autonomy and Boundaries

> Default-allow inside the workspace. Strict gates at publish boundaries.

---

## The Core Principle

**Engineering is default-allow. Publishing is gated.**

The pack is a build pipeline with guardrails, not a guardrail pipeline that sometimes builds. Agents explore, implement, and iterate freely. Gates engage only at publish boundaries.

---

## Default-Allow Work Plane

Inside the sandbox / run directories:
- Move fast
- Fix forward
- Retry freely
- Do not stop mid-flow to ask permission

Agents can:
- Read any file
- Write any code
- Run any test
- Iterate any number of times

No permission checks inside the workspace. Permission theater kills throughput.

**Violation:** "Before reading this file, let me check if I have permission..."
**Correct:** Read the file. Gates engage only when publishing.

---

## Strict Boundaries (The Real Gates)

When crossing a boundary (commit, push, publish, release):
- Require evidence
- Prefer deterministic checks
- Do not "trust the agent's claim"

### The Boundaries

| Boundary | Gate | What It Checks |
|----------|------|----------------|
| **Commit** | secrets-sanitizer | No credentials in staged changes |
| **Push** | repo-operator | Clean state, no anomalies |
| **GitHub post** | content restrictions | Safe to publish |
| **Release** | deploy-decider | All gates passed |

### Canonical Boundary Order (Save Game)

**Stage → Sanitize → Persist**

1. **Stage** — Define the surface (what would be published)
2. **Sanitize** — Scan the staged surface (secrets, anomalies)
3. **Persist** — Commit/push only if safe

This order is mandatory. You cannot sanitize what you have not staged. You cannot persist what you have not sanitized.

---

## Questions Don't Stop Flows

Agents can record questions anytime:
- OpenQ register for tracked questions
- Assumptions in handoffs
- Notes in artifacts

But flows continue with best-effort assumptions:
1. Make the safest reasonable choice
2. Record it as an assumption or defaulted question
3. Proceed
4. Surface to humans at flow boundary

### The Flow Boundary Ask

At the end of a flow, surface:
- What was done
- What was found
- What assumptions were made (with reasoning)
- What decisions are requested (with options)

Humans decide at boundaries, not mid-flow.

---

## Merge Conflicts and Rebases

Merge conflicts are high-cognitive-load / low-value work.

Handle them by:
1. Route to an agent to reconcile
2. Verify with compiler/tests
3. Continue

Do not treat merge conflicts as human-required decisions unless they involve actual semantic choices.

---

## True Halting (Very Rare)

Most "blocked" is just routing. True halt requires:

1. **Mechanical failure** — File system permissions, network unavailable, tooling broken
2. **Non-derivable decision** — Two valid designs, human must choose business direction
3. **Unsafe boundary** — Would expose secrets, requires remediation before any publish

Even when halted:
- Work often continues in parallel
- The halt is scoped to the specific boundary
- Other flows may proceed

---

## What This Enables

| Without This Principle | With This Principle |
|------------------------|---------------------|
| Agents ask permission for every action | Agents work freely, gates check at boundaries |
| Flows stop constantly for approval | Flows complete, humans decide at boundaries |
| Security theater everywhere | Real security at real boundaries |
| Slow, frustrating, permission-obsessed | Fast, productive, appropriately controlled |

---

## See Also

- [why-ops-first.md](../../docs/explanation/why-ops-first.md) — The philosophy behind default-allow
- [boundary-physics.md](../../docs/explanation/boundary-physics.md) — How boundaries work
- [why-two-gates.md](../../docs/explanation/why-two-gates.md) — The two-gate model
