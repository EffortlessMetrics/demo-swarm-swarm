# Agent Philosophy

> How agents think, act, and fail gracefully.

---

## Core Principle

**Agents are intelligent actors, not script executors.**

This pack treats agents as senior engineers who can investigate, reason, and make judgment calls. We tell them what TO DO, not what NOT to do.

---

## Agents Are Smart, Config Is Dumb

**Config contains only mechanics:**
- What command to run (`npm test`)
- Where files live (`src/`, `tests/`)
- Environment details (`github`, `windows-wsl2`)

**Policies stay in agent prompts:**
- Coverage thresholds
- Quality gates
- Review requirements
- Merge criteria
- What constitutes "good enough"

**Why?** Policies require judgment. "Is 80% coverage acceptable?" depends on context—is this a critical auth module or a CLI helper? Agents can apply judgment; config files cannot.

---

## The Autonomy Pattern

### Roles + Guardrails (Not Permissions + Handcuffs)

**The anti-pattern (handcuffs):**
- Allowlists that restrict which files an agent can touch
- Manifests that define "permitted" file paths
- "Stop and ask permission" protocols for reading context
- Denylists that forbid certain operations

**The correct pattern (roles + guardrails):**
- **Role focus:** "Your mission is to write tests for this AC"
- **Autonomy:** "You can read any file you need. You can edit files to make code testable."
- **Detective guardrails:** Critics evaluate afterward — quality, correctness, whether it solves the problem

**Why this matters:**
- Allowlists assume the planner is omniscient — they're not
- "Stop and ask" creates token-burning loops for basic exploration
- Agents are intelligent — they can determine what they need by searching and reading
- Critics evaluate quality afterward — does it work? does it solve the problem?

**Practical implications:**
- `context-loader` is an **accelerator** (optional starting point), not a gate
- Workers can explore beyond the manifest if they need more context
- Critics evaluate quality and whether the implementation solves the problem

---

## Research-First Autonomy

**If an agent can't derive an answer, it investigates first, then defaults, then escalates.**

The escalation ladder (in order):
1. **Investigate locally:** Search code, tests, configs, prior runs, existing docs
2. **Investigate remotely (if allowed):** GitHub issues/PRs, web search, library docs
3. **Derive from evidence:** Use patterns in the codebase to infer correct behavior
4. **Default if safe:** Choose a reversible default, document it, continue
5. **Escalate only when boxed in:** All of the above failed AND no safe default exists

**The bar for human escalation is high.** A timeout value? Look at existing timeouts. An error format? Look at existing error handlers. Auth approach? Look at existing auth code.

**Most questions are NOT blockers.** DEFAULTED (safe reversible default chosen) is the common case. NON_DERIVABLE is rare and requires proof-of-research.

---

## Intelligence in Action

### Early Detection Over Late Gates

Problems should be caught where the fix is cheapest:
- **Per-AC**: Catch reward hacking during the microloop (before next AC starts)
- **Per-checkpoint**: Catch CI failures during feedback harvest (before flow ends)
- **Per-flow**: Catch format/lint issues in standards-enforcer (before Gate)
- **Gate**: VERIFY earlier findings (discovery belongs in upstream flows)

Gate is a **verification checkpoint**, not a quality filter. If Gate is catching issues that should have been caught earlier, that's a signal the upstream flows need improvement.

### Fix-Forward Within Flows

Small issues should be fixed where they're found:
- Formatting drift: `standards-enforcer` fixes it, doesn't BOUNCE
- Missing imports: `code-implementer` adds them on the next pass
- Stale comments: `fixer` removes them during review worklist

**BOUNCE only when:**
- The fix requires design changes (BOUNCE to Plan)
- The fix spans multiple ACs beyond current scope (BOUNCE to Build start)
- The fix requires human judgment (BOUNCE with `reason: NEEDS_HUMAN_REVIEW`)

### Intelligent Summarization

When summarizing for reports or routing:
- Explain what the issue IS, not just where it is
- Provide your assessment of validity (is this a real issue or bot noise?)
- Route to the agent best suited to fix it
- Synthesize understanding over file path lists

**Agents are smart.** They can read context, understand intent, and make judgment calls. Trust them to summarize intelligently rather than mechanically dumping file pointers.

### Intelligent Conflict Resolution

When conflicts arise (git, semantic, or otherwise):
- **Try to resolve first** - Read both sides, understand intent, merge if possible
- **Only escalate when ambiguous** - When you genuinely cannot determine the right resolution
- **Provide context when escalating** - Explain what you tried and why you couldn't resolve it

Agents should behave like senior engineers who can solve most problems themselves and only escalate the genuinely difficult ones.

### Intelligence Everywhere

Any agent is authorized to fix an obvious, safe error it sees (typo, lint nit, missing import). We don't silo "fixing" to a specific agent.

If a researcher sees a typo in the README, they should fix it and move on.

---

## Natural Resiliency

### Success Pressure → Guessing

Agents under pressure to complete a task will **guess** to finish. The fix is giving them **multiple successful exits**.

**`PARTIAL` is a successful completion** when:
- State is written to disk (`.runs/<run-id>/…`)
- Next steps are documented
- Work is checkpointed so the flow can be rerun cleanly

A `PARTIAL` exit is not failure. It's a save point.

### Honest State Is the Primary Success Metric

Agents are rewarded for **accurate reporting**, not completion theater.

**This is a VERIFIED success:**
```yaml
status: UNVERIFIED
work_status: PARTIAL
what_completed: "Implemented 2/5 ACs"
blockers: ["Missing schema migration for AC-3"]
evidence: "Tests pass for AC-1, AC-2. AC-3 requires DB changes."
```

**This is a HIGH-RISK failure (even though it says "complete"):**
```yaml
status: VERIFIED
work_status: COMPLETED
what_completed: "All 5 ACs implemented"
assumptions: ["Assumed schema exists (didn't verify)"]
```

The first report tells the orchestrator exactly what happened and what to do next. The second report hides uncertainty behind a false completion signal, causing downstream failures.

**Agent rule:** When uncertain, report the uncertainty. A 40% completion with honest blockers is more valuable than a 100% completion with hidden assumptions.

### Write Early, Write Often

Flows are **naturally re-runnable**. Re-running a flow is not "failure recovery"—it's routine:
- Double-check work
- Tighten schema alignment
- Clean up artifacts
- Improve quality incrementally

**Always room for improvement**, even if rerunning something that was already run.

### Forensic Truth: Diff + Test Results

We trust **git diffs and test results** as forensic evidence.
- The diff is the best audit surface for what changed
- Tests are the runtime truth for what works
- Critics do forensic analysis of both

No rigid "coverage ratio" gates—use judgment to assess honesty and risk.

---

## Model Strategy

We intentionally avoid hardcoding model tiers into the pack.

- **Most agents:** `model: inherit` (lets users choose their default)
- **Some operator/librarian agents:** may default to `haiku` for fast search
- **Only force a heavier model** when the task truly needs it (rare)

**Naming rule:** Use model *names* only (Haiku, Sonnet, Opus). No version numbers—they become stale.

---

## See Also

- [architecture.md](architecture.md) — System design and patterns
- [why-ops-first.md](why-ops-first.md) — Engineering default-allow philosophy
- [trust-model.md](../reference/trust-model.md) — Evidence hierarchy
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
