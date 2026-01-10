# Calibration Signals

> System health over time. Are we getting better?

---

## Purpose

The [PR Quality Scorecard](pr-quality-scorecard.md) tells you if a **single PR** is good.

Calibration signals tell you if the **system** is good.

Questions this document answers:

- Is the swarm getting better over time?
- Is the swarm getting worse?
- Which parts need tuning?
- When should we update the pack?

This is system-level health, not PR-level quality.

---

## Health Signals

### Positive Signals (System Is Working)

| Signal | How to Measure | What It Means |
|--------|----------------|---------------|
| First-pass Gate rate | % of runs that pass Gate without bouncing | High = quality work upstream |
| Critic finding rate | Findings per run, trending down | Agents learning from feedback |
| Reviewer time | Minutes to approve, trending down | Better explanations, less archaeology |
| Fix-forward vs bounce | % routed to fixer vs bounced to Plan | High fix-forward = minor issues only |
| "Not measured" shrinking | Fewer unknowns over time | Verification coverage growing |

### Negative Signals (System Needs Attention)

| Signal | How to Measure | What It Means |
|--------|----------------|---------------|
| Same critic findings | Same issues appearing across runs | Prompt not learning |
| Bounce loops | Work bouncing A to B to A to B | Routing problem or unclear spec |
| Growing review time | Reviewer spending more time | Quality declining or complexity growing |
| Stale learnings | Wisdom produces learnings, nothing changes | Flywheel stalled |
| Gate failures increasing | More runs failing Gate | Upstream quality declining |

---

## What to Measure (and How)

### Per-Run Metrics (Track in Receipts)

- Flows executed
- Bounces/iterations per flow
- Critic findings by severity
- Time in each flow (if measurable)
- Human interventions required

### Cross-Run Metrics (Track in Wisdom)

- Trends in above metrics
- Recurring patterns
- Agent-specific performance
- Prompt effectiveness

### Pack-Level Metrics

- Agent prompt update frequency
- Doc update frequency
- Pack-check warnings over time

---

## Calibration Checkpoints

### After Every Run

- Did Gate pass first try?
- Any CRITICAL findings?
- Reviewer feedback (if available)

### Weekly/Monthly (For Active Repos)

- Trend of first-pass Gate rate
- Most common critic findings
- Agents requiring most fixes

### Before Pack Updates

- What is the signal prompting this change?
- What metric should improve?
- How will we know it worked?

---

## Agent-Level Calibration

Signs an agent needs tuning:

| Agent Type | Needs Tuning When |
|------------|-------------------|
| Authors | Output frequently rejected by critics |
| Critics | Findings ignored as noise OR missing real issues |
| Implementers | Code requires extensive fixes |
| Cleanup | Missing important summary points |
| Gate | Passing things that fail in prod OR blocking good work |

See [Agents Index](agents-index.md) for the full agent listing by role.

---

## Prompt Tuning Signals

### When to Update an Agent Prompt

| Trigger | Action |
|---------|--------|
| Same mistake 3+ times | Add explicit guidance |
| False positive pattern | Add "don't do X" guidance |
| Missing context | Add relevant background |
| Wrong handoff target | Update handoff guidance |
| Verbosity issues | Add length/focus guidance |

### When NOT to Update

- **One-off issues** - Noise, not signal
- **Issues better solved upstream** - Fix the source, not the symptom
- **Preference differences** - Not quality issues

---

## Pack-Level Calibration

Signs the pack structure needs work:

| Signal | Possible Cause | Potential Fix |
|--------|----------------|---------------|
| Flows always run out-of-order | Flow boundaries wrong | Restructure flows |
| Agent keeps being skipped | Agent not useful | Remove or merge |
| Constant manual intervention | Automation gaps | Add missing agent |
| Routing confusion | Too many similar agents | Consolidate |

---

## The Anti-Metrics

What NOT to optimize for:

| Anti-Metric | Why It Fails |
|-------------|--------------|
| Speed at cost of quality | Fast garbage is still garbage |
| Metric gaming | "100% Gate pass" by lowering standards |
| Activity over outcome | Lines of code, number of runs |
| Process compliance | Following steps vs delivering value |

The goal is quality software delivered efficiently. Metrics are proxies, not goals.

---

## Calibration Hygiene

### Do

- Track signals consistently
- Look for trends, not snapshots
- Update prompts based on patterns
- Document what you changed and why

### Don't

- React to single data points
- Optimize one metric at expense of others
- Update prompts without clear signal
- Forget to check if changes helped

---

## Minimum Viable Tracking

If you track nothing else, track these three:

| Signal | Question It Answers |
|--------|---------------------|
| **First-pass Gate rate** | Are we producing quality work? |
| **Bounce rate** | How much rework? |
| **Reviewer feedback** | Human ground truth |

Everything else is refinement.

---

## The Flywheel Connection

Calibration signals feed into the improvement flywheel described in [Economics](../explanation/economics.md):

```
Measure signals
    |
    v
Identify patterns
    |
    v
Update prompts/pack
    |
    v
Measure again
    |
    v
(repeat)
```

Machine workflows are code. Profile them, optimize them, cache them. Every calibration cycle makes the next one faster.

---

## See Also

- [PR Quality Scorecard](pr-quality-scorecard.md) - Single-PR quality surfaces
- [Agents Index](agents-index.md) - Agent listing by role family
- [Economics](../explanation/economics.md) - The math behind the flywheel
- [Contracts](contracts.md) - Receipt schemas for metric extraction
