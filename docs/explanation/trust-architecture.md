# Trust Architecture

> How trust emerges from verification, not from restrictions.

Trust isn't granted by policy. Trust is earned by architecture.

---

## The Core Insight

**Trust AND verify.** Not a trade-off—both simultaneously.

**High trust comes FROM high verification, not from restrictions.**

| Phrase | Meaning | Result |
|--------|---------|--------|
| "Trust but verify" | Trust first, check later | Gaps when verification skipped |
| "Verify then trust" | No action until proven | Too slow, too cautious |
| **"Trust AND verify"** | High autonomy + high verification | Fast AND safe |

The system gives agents high trust (autonomy, no babysitting) BECAUSE it has high verification (tests, critics, gates). These aren't in tension—they're mutually reinforcing.

Traditional AI safety asks: "How do we prevent the AI from doing dangerous things?"

AgOps asks: "How do we verify that what the AI did is correct?"

| Approach | Mechanism | Result |
|----------|-----------|--------|
| **Traditional** | Permissions, restrictions, approval dialogs | Slow iteration, false confidence, friction |
| **AgOps** | Work freely, verify thoroughly, gate at boundaries | Fast iteration, real evidence, clean boundaries |

The traditional approach tries to prevent bad outcomes by constraining actions. AgOps achieves safe outcomes by verifying results. The difference is fundamental: one is about control, the other is about evidence.

---

## Trust Is Earned, Not Granted

The system doesn't start with trust and try to prevent violations. It starts with distrust and earns trust through evidence.

Every claim must be backed. Every output must be verified. Every boundary must be checked. Trust accumulates as evidence accumulates.

### What Earns Trust

| Layer | What It Proves | Trust Gained |
|-------|---------------|--------------|
| Tests pass | Code behaves as specified | Functional trust |
| Critics approve | No obvious issues found | Quality trust |
| Mutation score high | Tests actually test the code | Verification trust |
| Evidence complete | Claims are backed by pointers | Audit trust |
| Gates clear | Boundaries are respected | Safety trust |

Each layer adds confidence. Missing any layer leaves a gap.

### The Trust Equation

```
Trust = Evidence x Verification x Boundaries
```

- **More evidence** — more trust
- **More verification** — more trust
- **Stronger boundaries** — safer trust

This isn't philosophy. It's arithmetic. Trust scales with the product of these factors. Zero any factor and trust collapses.

---

## Why Restrictions Don't Work

### The Permission Theater Problem

Consider a typical "safe" AI workflow:

```
Agent: "I'd like to read src/auth/login.ts"
System: "Approved."
Agent: "I'd like to read src/auth/middleware.ts"
System: "Approved."
Agent: "I'd like to write a test file"
System: "Approved."
[repeat 50 times]
```

What does this accomplish?

- **Not safety** — The agent will read the files anyway; the approval is theater
- **Not quality** — Approving an action says nothing about whether the result is correct
- **Not efficiency** — Tokens spent on permission negotiation instead of engineering
- **Not confidence** — The human approver has no ability to assess whether reading that file is "safe"

Permission theater creates the appearance of oversight without the substance of verification.

### The Real Risks

The risks that matter in AI-assisted development:

| Real Risk | Permission Theater | Verification Architecture |
|-----------|-------------------|--------------------------|
| Secrets in commits | "May I commit?" (useless) | Scan staged changes for secrets |
| Wrong code in production | "May I write code?" (useless) | Tests prove behavior |
| Incorrect claims believed | "May I claim X?" (useless) | Evidence backs every claim |
| Scope creep unnoticed | "May I edit this file?" (useless) | Critics verify alignment |
| Test weakening | "May I delete tests?" (useless) | Guards detect test deletion |

Permission dialogs don't address these risks. They can't. The question "May I do X?" doesn't have enough information to determine safety. The question "Did the result of X satisfy constraints Y?" does.

---

## The Trust Architecture

Trust emerges from four layers, each providing a different kind of assurance.

### Layer 1: Containment (Sandbox)

The workspace is bounded:

- Dedicated directory (not your home folder)
- Limited credentials (no production secrets)
- Scoped permissions (can't affect systems outside the sandbox)
- Git is the only path out

**Trust earned:** Blast radius is limited. A mistake in the sandbox affects only the sandbox. Revert and retry is cheap.

**Why this works:** Containment is infrastructure, not policy. The agent can't access production because production credentials don't exist in the sandbox—not because a rule says "don't access production." Physics is more reliable than rules.

### Layer 2: Verification (Pipeline)

Every output is challenged:

- **Tests** prove code behaves as specified
- **Critics** find issues that tests miss
- **Evidence** backs claims with pointers to proof

**Trust earned:** Quality is demonstrated, not assumed. An agent claiming "I implemented the feature" means nothing. An agent with green tests and a clean critique means something.

**Why this works:** Verification is adversarial. Critics don't want to approve bad code—they want to find problems. Tests don't want to pass incorrectly—they want to catch regressions. The system has built-in skepticism.

### Layer 3: Boundaries (Gates)

Publish operations are gated:

- **Secrets scanning** — No credentials cross the boundary
- **Anomaly detection** — Unexpected changes are flagged
- **Two-gate rule** — Both secrets and hygiene must pass

**Trust earned:** Nothing escapes without inspection. The boundary is the enforcement point, not the interior.

**Why this works:** Boundaries are choke points. Everything that leaves the sandbox must pass through a small number of well-guarded exits. Securing those exits is tractable. Securing every interior action is not.

### Layer 4: Observability (Evidence)

Everything is auditable:

- **Receipts** capture what happened
- **Artifacts** preserve decisions
- **History** is recoverable

**Trust earned:** Problems can be diagnosed and fixed. When something goes wrong, you can trace exactly what happened and why.

**Why this works:** Observability makes trust empirical. You don't have to believe the system works—you can verify it worked for this specific run. Every run produces its own evidence.

---

## High Trust in Practice

### What High Trust Enables

- **bypassPermissions: true** — No constant interruptions asking for approval
- **Large autonomous runs** — Fire and forget; check results when done
- **Minimal babysitting** — Review outcomes, not process
- **Fast iteration** — Machine grinds through cycles; human reviews summary

The agent works at machine speed because verification happens at machine speed. Human attention focuses on decisions, not supervision.

### What High Trust Requires

High trust is not free trust. It requires:

- **Real sandbox** — Containment is infrastructure, not just policy
- **Real verification** — Tests that run, critics that critique, evidence that exists
- **Real boundaries** — Gates that can and will say "no"
- **Real observability** — Artifacts worth reading, history worth tracing

Skip any of these and trust collapses. An agent without tests is unverified. A boundary without scanning is open. A system without observability is opaque. High trust requires high investment in the verification infrastructure.

---

## The Paradox

**More autonomy requires more verification.**

This seems counterintuitive. If you trust the agent, why verify so much?

The answer: you don't trust the agent. You trust the architecture. The agent is a stochastic system that sometimes makes mistakes. The architecture catches those mistakes before they matter.

| Configuration | Outcome |
|---------------|---------|
| High autonomy, low verification | Recklessness — mistakes escape |
| Low autonomy, low verification | Bureaucracy — everything is slow |
| Low autonomy, high verification | Redundant — verification catches what permissions already blocked |
| **High autonomy, high verification** | **The sweet spot — fast iteration with real safety** |

You can't have high autonomy with low verification—that's hoping nothing goes wrong.

You can't have high verification with low autonomy—that's paying verification costs for work you don't let the agent do.

High trust = high autonomy + high verification + clear boundaries.

---

## Trust vs Control

The fundamental choice: do you achieve safety through control or through verification?

| Approach | Autonomy | Verification | Result |
|----------|----------|--------------|--------|
| Low trust, high control | Low | Low | Slow and still risky |
| Low trust, high verification | Low | High | Slow but safe |
| High trust, low verification | High | Low | Fast but dangerous |
| **High trust, high verification** | **High** | **High** | **Fast and safe** |

The goal is the bottom-right quadrant. Get there by:

1. **Building verification infrastructure** — Tests, critics, gates, observability
2. **Removing permission theater** — Approvals that don't add information
3. **Maintaining boundaries** — Real enforcement at publish points

Control is a tax. You pay it whether or not it provides safety. Verification is an investment. It provides safety proportional to its quality.

---

## Implementation

Trust architecture is implemented through:

### Sandbox Configuration (Containment)

```yaml
# The workspace is bounded by infrastructure
workspace: dedicated directory
credentials: none for production
network: no production access
exit: git only
```

### Flow Design (Verification at Each Stage)

| Flow | Verification |
|------|-------------|
| Signal | Requirements traced to features |
| Plan | ADR reviewed, contracts defined |
| Build | Tests pass, critics approve |
| Review | PR feedback addressed |
| Gate | All evidence verified |
| Deploy | Production behavior confirmed |
| Wisdom | Learnings extracted |

### Gate Agents (Boundary Enforcement)

```
secrets-sanitizer → safe_to_publish: true/false
repo-operator → proceed_to_github_ops: true/false

Both must pass for boundary crossing.
```

### Artifact Design (Observability)

```
.runs/<run-id>/
  signal/
    requirements.md        # What was asked for
    signal_receipt.json    # Evidence of shaping work
  build/
    impl_changes_summary.md # What was built
    test_execution.md      # Verification results
    build_receipt.json     # Evidence of build work
  gate/
    merge_decision.md      # Final verification
    gate_receipt.json      # Evidence of gate work
```

Trust architecture isn't a feature. It's the entire system design. Every component contributes to the trust equation.

---

## The Alternative

Without trust architecture, you get one of two failure modes:

### Failure Mode 1: Permission Hell

Every action requires approval. Agents spend more time asking permission than working. Humans spend more time approving than reviewing. The system is "safe" in the sense that nothing bad happens because nothing happens.

### Failure Mode 2: YOLO Mode

No verification, no boundaries. The agent works fast and produces output. Sometimes the output is correct. Sometimes it isn't. You find out in production.

Trust architecture is the third option: work fast, verify thoroughly, gate at boundaries. Neither permission hell nor YOLO mode. Velocity with evidence.

---

## The Key Insight

> Trust isn't granted by policy. Trust is earned by architecture.

You don't make a system trustworthy by writing rules about what it can't do. You make it trustworthy by building infrastructure that verifies what it did.

Rules can be bypassed, misunderstood, or forgotten. Architecture is load-bearing. Tests either pass or they don't. Gates either open or they don't. Evidence either exists or it doesn't.

**The trust architecture is why bypassPermissions works.** Not because we trust Claude to be safe. Because we trust the verification pipeline to catch mistakes before they matter.

Trust the architecture. Verify everything. Gate the boundaries.

---

## See Also

- [boundary-physics.md](boundary-physics.md) — The mechanics of publish boundaries
- [why-ops-first.md](why-ops-first.md) — Engineering default-allow, publishing gated
- [trust-model.md](../reference/trust-model.md) — Evidence hierarchy and verification boundaries
- [claims-and-evidence.md](claims-and-evidence.md) — How evidence prevents false claims
- [architecture.md](architecture.md) — Overall pack design
