# The Flywheel

> The system is a loop, not a line. Wisdom flows back to Signal.

---

## The Linear Illusion

The seven flows look like a pipeline:

```
Signal -> Plan -> Build -> Review -> Gate -> Deploy -> Wisdom
```

This is how you experience a single run: left to right, beginning to end. The work starts at Signal and finishes at Wisdom. Done.

But the system does not end at Wisdom.

---

## The Reality: A Loop

Wisdom connects back to Signal:

```
Signal -> Plan -> Build -> Review -> Gate -> Deploy -> Wisdom
   ^                                                      |
   |                                                      |
   +---------------------- learnings ---------------------+
```

Each run produces learnings. Those learnings improve the next run's starting conditions. The system is not a line that finishes; it is a loop that accelerates.

This is the flywheel.

---

## Two Paths Through the System

The flows carry information in both directions.

### The Forward Path (Traceability)

What we already have documented: requirements trace forward to code. Evidence accumulates through flows. Each flow builds on the previous.

```
Problem -> Requirement -> BDD Scenario -> Test -> Code -> Critique -> Receipt -> Merge Decision
```

This chain answers: "Why does this code exist?" Follow it forward to see how requirements became implementation. Follow it backward to see what drove a design decision.

See [traceability-spine.md](traceability-spine.md) for the full specification.

### The Backward Path (Learning)

What flows back from Wisdom to Signal:

- **Learnings** -- What worked, what did not, what to try differently
- **Regressions** -- What broke, why, how to prevent it next time
- **Patterns** -- Recurring issues across runs
- **Calibration** -- Which agents need tuning, which prompts need refinement

This path is how the system improves itself.

---

## Wisdom to Signal: The Specific Connections

| Wisdom Produces | Flows Back To | How It Gets There |
|-----------------|---------------|-------------------|
| `learnings.md` | Agent prompts | Pattern becomes prompt guidance via feedback-applier |
| `regression_report.md` | Risk assessment | Known failure mode informs `early_risks.md` in Signal |
| Quality observations | Test strategy | Quality gaps inform `test_plan.md` in Plan |
| Process friction | Flow design | Inefficiencies lead to pack updates |
| Pack observations | Agent prompts | PACK_OBS markers become diffs to agent files |

---

## The Feedback Mechanisms

### Explicit Feedback (Documented)

These agents exist to close the loop:

**learning-synthesizer** extracts patterns from run artifacts. It reads receipts, critiques, and reports to produce `learnings.md` with stable markers (PACK_OBS, ACTION). See `.claude/agents/learning-synthesizer.md`.

**feedback-applier** reads learnings and produces ready-to-apply diffs for pack improvements or issue drafts for larger work. It turns observations into edits. See `.claude/agents/feedback-applier.md`.

**regression-analyst** identifies what broke and why. It connects production failures back to their origins in earlier flows.

**pattern-analyst** and **process-analyst** look across multiple runs to find recurring themes that single-run analysis misses.

### Implicit Feedback (Human-Mediated)

Not all feedback is automated:

- Human reviews Wisdom output, updates pack manually
- Patterns across runs inform CLAUDE.md updates
- Recurring issues become entries in `anti-patterns.md`
- Team discussions lead to flow refinements

The explicit mechanisms reduce the burden. But humans remain in the loop for judgment calls.

---

## What Makes It a Flywheel

A flywheel gains momentum. Each turn makes the next easier.

**Turn 1:** Run produces learnings
- "Agent X kept missing pattern Y"

**Turn 2:** Learnings improve prompts
- feedback-applier adds instruction to Agent X

**Turn 3:** Better prompts produce better artifacts
- Agent X now catches pattern Y

**Turn 4:** Better artifacts produce richer learnings
- Wisdom notices Agent X improved; captures what worked

**Turn 5:** Richer learnings improve prompts further
- Pattern Y handling refined based on new evidence

The system gets better at its job over time. Not from magic, but from deliberately closing the loop.

---

## Flywheel Failure Modes

The loop can break. Here is how and what to do.

### Stall

**Symptom:** Wisdom runs but learnings are not applied.

Learnings are written. Feedback actions are drafted. But nobody applies the diffs or files the issues. The artifacts accumulate but the system does not improve.

**Fix:** Actually read and apply learnings. Review `pack_improvements.md` after each run. Apply the diffs or explain why not. The loop only closes if someone turns the crank.

### Drift

**Symptom:** Learnings applied inconsistently across agents.

One agent gets updated. Its sibling agent does not. Now they behave differently for similar situations. The pack becomes inconsistent.

**Fix:** Use doc governance. When a pattern applies to multiple agents, update them together. CLAUDE.md is the canonical source; agent prompts inherit from it.

### Noise

**Symptom:** Too many learnings, signal lost.

Every run produces ten learnings. Nobody can process that volume. The important patterns drown in trivial observations.

**Fix:** learning-synthesizer filters for actionable items. Learnings without actions are noise. The Advice-to-Action binding rule requires every learning to produce a marker (PACK_OBS or ACTION). If it cannot, it is not a learning---it is a vibe dump.

### Regression

**Symptom:** An "improvement" makes things worse.

A learning led to a prompt change. The change introduced a new problem. The flywheel is now spinning backward.

**Fix:** Track calibration signals. Notice when agents degrade. Revert changes that do not help. The scent trail (`.runs/_wisdom/latest.md`) should include negative constraints: "Do not do X because it failed in run Y."

---

## The Flywheel in Practice

After each significant run:

1. **Wisdom extracts learnings**
   - learning-synthesizer reads artifacts and produces `learnings.md`
   - PACK_OBS and ACTION markers identify actionable items

2. **Human or feedback-applier reviews**
   - Pack improvements become diffs in `pack_improvements.md`
   - Larger work becomes issue drafts
   - Scent trail updates for future runs

3. **Worthwhile learnings become changes**
   - Prompt updates (agent behavior)
   - Doc updates (patterns, anti-patterns)
   - Pack updates (structural changes)

4. **Next run benefits**
   - Improved agents produce better artifacts
   - Better artifacts produce richer learnings
   - The loop continues

This is not ceremony. Skip it when there is nothing to learn. But do not skip it habitually or the flywheel stalls.

---

## Cross-Run Learning

Single runs teach specific lessons. Multiple runs reveal patterns.

**Examples:**

- "This agent keeps making the same mistake" -- prompt needs work
- "This flow always bounces here" -- routing needs adjustment
- "This verification never catches issues" -- verification needs strengthening
- "This file changes in every run" -- it is a hotspot

pattern-analyst and process-analyst exist for this cross-run analysis. They look at `.runs/` history to find what single-run Wisdom misses.

### The Scent Trail

`.runs/_wisdom/latest.md` persists across runs. It contains the top learnings that should inform the next run:

- Negative constraints (things that failed)
- Positive patterns (things that worked)
- Known pitfalls (areas that need care)
- Active wisdom (learnings that carry forward)

gh-researcher reads this file before starting research. The wisdom from past runs informs the next run's Signal.

---

## The Flywheel vs The Treadmill

**Flywheel:** Each turn adds momentum. The system improves.

- Run 1 teaches something
- Run 2 is slightly better
- Run 10 is noticeably better
- Run 100 is dramatically better

**Treadmill:** Lots of motion, no progress. Same mistakes repeated.

- Run 1 hits a problem
- Run 2 hits the same problem
- Run 10 still hits the same problem
- Run 100 still hits the same problem

The difference is whether learnings are actually applied.

A team running hundreds of flows but never reading Wisdom output is on a treadmill. A team running fewer flows but closing the loop each time is building a flywheel.

---

## The Flywheel Visual

```
+-------------------------------------------------------------+
|                        THE FLYWHEEL                          |
+-------------------------------------------------------------+

        +----------+
        |  Signal  | <-----------------------------+
        +----+-----+                               |
             |                                     |
             v                                     |
        +----------+                               |
        |   Plan   |                               |
        +----+-----+                               |
             |                                     |
             v                                     |
        +----------+                         learnings
        |  Build   |                         patterns
        +----+-----+                         calibration
             |                                     |
             v                                     |
        +----------+                               |
        |  Review  |                               |
        +----+-----+                               |
             |                                     |
             v                                     |
        +----------+                               |
        |   Gate   |                               |
        +----+-----+                               |
             |                                     |
             v                                     |
        +----------+                               |
        |  Deploy  |                               |
        +----+-----+                               |
             |                                     |
             v                                     |
        +----------+                               |
        |  Wisdom  | ------------------------------+
        +----------+

        learning-synthesizer extracts
        feedback-applier applies
        pattern-analyst finds themes
        the loop closes
```

---

## Why This Matters

Without the backward path, you have a deployment pipeline. Work flows from idea to production and stops. Each run is isolated. The system does not learn.

With the backward path, you have a learning system. Each run teaches the next. Mistakes become constraints. Successes become patterns. The system improves itself.

The forward path (traceability) answers: "Why does this code exist?"

The backward path (learning) answers: "How do we get better at building code?"

Both paths matter. But the backward path is what makes the difference between a pipeline and a flywheel.

---

## See Also

- [traceability-spine.md](traceability-spine.md) -- How requirements trace forward through flows
- [why-seven-flows.md](why-seven-flows.md) -- Why these seven, not fewer or more
- [anti-patterns.md](anti-patterns.md) -- What we learned not to do
- [learning-synthesizer.md](../../.claude/agents/learning-synthesizer.md) -- The agent that extracts learnings
- [feedback-applier.md](../../.claude/agents/feedback-applier.md) -- The agent that applies learnings
