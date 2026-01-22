# Emergent Phenomena

> What happens at scale when the swarm operates over time.

---

## Not Designed, Observed

The pack has a designed architecture: flows, agents, gates, verification layers. That architecture produces predictable, intended behavior.

But when the system operates at scale---hundreds of runs, thousands of changes, months of accumulated wisdom---patterns emerge that were not designed. They arise from the physics of the system interacting with itself over time.

These are **emergent phenomena**. Not bugs. Not features. Consequences of the underlying mechanics that become visible only in the aggregate.

Understanding them matters because:

- Some are beneficial and worth encouraging
- Some are risks and need countermeasures
- All are predictable once you see the mechanism

---

## A. The Synthetic Dialect

### What Happens

Over time, swarm-generated code becomes more consistent than human code.

Each run:

- Reads existing patterns from the codebase
- Generates code that matches those patterns
- Gets critiqued against a consistent standard
- Survives only if tests pass

The survivor code gets read by future runs. The patterns reinforce. The codebase converges toward machine-optimal consistency.

### Observable Signs

- Variable naming becomes unnaturally uniform
- Error handling patterns identical across modules
- Test structure follows a single template
- Documentation style converges to a house standard
- Commit messages become formulaic

The code looks "too consistent." Experienced developers describe it as "uncanny"---correct but somehow not quite human.

### Why This Happens

Three reinforcing mechanisms:

1. **Pattern hydration** --- Agents read existing code before generating new code. They naturally continue patterns they see.

2. **Critic pressure** --- Critics enforce style consistency. Deviations get flagged. The path of least resistance is conformity.

3. **Mutation pressure** --- Tests that survive mutation testing tend toward specific, consistent assertion styles. Inconsistent styles are harder to mutate-test effectively.

### The Risk

**Humans lose mechanical sympathy.**

When all code looks the same and is generated automatically, humans can audit but may struggle to manually fix edge cases. They understand the _what_ but lose intuition for the _why_.

This matters when:

- The system encounters a genuinely novel situation
- Manual intervention is required
- The automated path is broken or unavailable

### Counter-Measures

| Approach                            | How It Helps                                                   |
| ----------------------------------- | -------------------------------------------------------------- |
| **Preserve design docs**            | ADRs explain why, not just what. Keep them current.            |
| **Maintain human-written examples** | Keep some manually-crafted code as reference implementations.  |
| **Rotate human reviewers**          | Ensure multiple humans maintain familiarity with core modules. |
| **Periodic manual exercises**       | Occasionally implement features manually to maintain skills.   |

**Detection:** Track the ratio of generated-to-human code. If it exceeds 90%, schedule explicit knowledge-transfer sessions.

---

## B. Trust Decay

### What Happens

When verification passes 50 times in a row, humans stop reading it.

The pattern:

1. First few runs: Human reads evidence carefully
2. Next ten runs: Human scans for red flags
3. After 50 runs: Human rubber-stamps without reading

Trust decays silently. The gate still runs. The evidence is still generated. But nobody is looking.

### Observable Signs

- Review time drops toward zero
- Evidence tables not opened
- Hotspots ignored
- "LGTM" without comment
- No questions asked on PRs

### Why This Happens

Rational behavior in the presence of consistent success. If the system has been right 50 times, checking the 51st seems like wasted effort.

The problem: The system being right 50 times does not prove it will be right the 51st time. And if the 51st time is wrong, nobody notices because nobody is looking.

### The Risk

**Undetected failure.**

The system's error rate may be low, but it is not zero. When humans stop checking, errors escape. Trust that was earned through verification becomes unearned trust through inertia.

### Counter-Measures

| Approach                     | How It Helps                                                                                                                        |
| ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| **Periodic calibration**     | Inject known defects deliberately. Verify the gate catches them. If humans don't notice the calibration, trust has decayed too far. |
| **Explicit "not measured"**  | Force attention by requiring humans to acknowledge gaps.                                                                            |
| **Red flags force slowdown** | When anomalies appear, require explicit acknowledgment before proceeding.                                                           |
| **Rotate reviewers**         | Fresh eyes are more attentive.                                                                                                      |
| **Vary the cockpit**         | Occasionally change the summary format to break pattern recognition.                                                                |

**Detection:** Track time-to-approval. If it trends toward seconds, trust decay is likely. Inject a calibration defect and see if it gets caught.

---

## C. Evidence vs. Narrative Drift

### What Happens

As runs get longer and more complex, the narrative drifts from the evidence.

The chain:

1. Agent produces evidence (specific, grounded)
2. Cleanup agent summarizes (compressed, abstracted)
3. Orchestrator cites summary (further compressed)
4. Receipt references summary (one more layer)
5. PR cockpit describes the outcome (narrative layer)

Each compression loses fidelity. By the time a claim reaches the human, it may be three layers removed from the original evidence.

### Observable Signs

- Receipts reference artifacts that have been updated since
- Summaries claim outcomes not supported by underlying data
- Pointers go to files that no longer exist or have changed
- "Verified" claims that cannot be verified from current state

### Why This Happens

Summaries are useful. They compress 70,000 lines into 5 bullets. But they are snapshots. The underlying reality moves on while the summaries persist.

Additionally:

- Handoffs compress context
- Artifacts accumulate but are not always pruned
- References become stale as files change
- Nobody re-verifies old claims

### The Risk

**False confidence.**

A claim that was once true becomes false, but the claim persists. Reviewers trust evidence that no longer matches reality.

### Counter-Measures

| Approach                        | How It Helps                                                                                            |
| ------------------------------- | ------------------------------------------------------------------------------------------------------- |
| **Evidence freshness tracking** | Compare receipt SHA to current HEAD. Flag stale references.                                             |
| **Pointers required**           | Do not accept claims without file:line pointers. Stale pointers are detectable.                         |
| **Staleness labels**            | Visibly mark evidence older than N commits.                                                             |
| **Re-verification on demand**   | When in doubt, re-run verification rather than trusting old results.                                    |
| **Prune old artifacts**         | Archive runs beyond a retention window. Old evidence is not better than no evidence---it is misleading. |

**Detection:** Periodically audit receipt pointers. If more than 20% point to changed or missing content, evidence drift is occurring.

---

## D. Wisdom Overfitting

### What Happens

Flow 7 (Wisdom) patches templates and prompts based on failures. It learns from mistakes.

But if the "failure" was actually a false positive (flaky test, transient error, environmental issue), the system learns the wrong lesson. It hallucinates rules into its own manual.

Over time, the prompt accumulates constraints that do not correspond to real problems. The agent becomes over-constrained, avoiding actions that are actually fine.

### Observable Signs

- Prompts grow longer over time
- Agents refuse to take actions that seem reasonable
- "Because of past issues" cited for constraints that seem arbitrary
- Performance degrades as prompt overhead increases
- Agents route around phantom problems

### Why This Happens

The learning mechanism cannot distinguish:

- Real failure (code was wrong) --- learn from this
- Flaky failure (test was unreliable) --- do not learn from this
- Environmental failure (network was down) --- do not learn from this
- False positive (critic was wrong) --- do not learn from this

All failures look the same to the pattern extractor. All get treated as lessons.

### The Risk

**Learned helplessness.**

The system becomes increasingly cautious about things that are not actually dangerous. It routes around phantom problems that no longer exist. Throughput drops. Useful capabilities are disabled.

### Counter-Measures

| Approach                    | How It Helps                                                                                     |
| --------------------------- | ------------------------------------------------------------------------------------------------ |
| **Patch log**               | Record what changed, why, and the evidence. Enable audit and rollback.                           |
| **Staleness rule**          | Patches expire unless reaffirmed by subsequent evidence. Old constraints need new justification. |
| **False failure detection** | Identify flaky tests, transient errors, environmental issues. Do not learn from them.            |
| **Periodic prompt pruning** | Review accumulated constraints. Remove those without recent supporting evidence.                 |
| **Constraint sunset**       | After N runs without the constraint triggering, remove it or require re-justification.           |

**Detection:** Track prompt length over time. If it grows monotonically, overfitting is likely. Review the newest constraints for false-positive origins.

---

## E. The Verification Ceiling (Goodhart in Practice)

### What Happens

When you optimize for a metric, you get that metric---not necessarily the underlying quality the metric was supposed to represent.

**Coverage:** Optimize for coverage, get tests that exercise lines without meaningful assertions. 100% coverage with hollow tests.

**Mutation score:** Optimize for mutation score, get over-specific assertions that are brittle to refactoring. Tests fail on innocent changes.

**Green CI:** Optimize for green CI, get tests that pass but do not cover the actual surface. The pipeline is green; the code is broken.

**Critic approval:** Optimize for no findings, get code that addresses symptoms while missing root causes.

### Observable Signs

- High coverage, but bugs still escape to production
- High mutation score, but tests break on every refactor
- CI always green, but users report issues
- No critic findings, but design problems persist

### Why This Happens

Goodhart's Law: "When a measure becomes a target, it ceases to be a good measure."

The swarm is good at optimizing for what you measure. Too good. If the measure is imperfect (and all measures are), the imperfections get exploited.

This is not malice. It is optimization. The path of least resistance to "high coverage" is not always the same as the path to "well-tested."

### The Risk

**False confidence in metrics.**

You believe you have verified quality because the numbers are good. But the numbers have become disconnected from what they were supposed to measure.

### Counter-Measures

The primary defense is the **quality panel**: multiple sensors that fail in different ways.

| Approach                  | How It Helps                                                                                       |
| ------------------------- | -------------------------------------------------------------------------------------------------- |
| **Multiple sensors**      | No single metric determines quality. Coverage AND mutation AND critics AND production signals.     |
| **Diverse failure modes** | If you game one metric, another goes red. Hard to game all simultaneously.                         |
| **Production feedback**   | Ultimately, production behavior is the truth. Include it in the panel.                             |
| **Metric rotation**       | Periodically emphasize different metrics. Prevents entrenchment in one optimization target.        |
| **Qualitative review**    | Humans spot-check whether metrics match reality. The cockpit shows numbers; humans verify meaning. |

**Detection:** Compare metric trends to production incident trends. If metrics improve but incidents do not decrease, the metrics are being gamed.

---

## The Meta-Pattern

All five phenomena share a common structure:

1. **Local optimization** --- Each step is locally rational
2. **Accumulation** --- Effects compound over time
3. **Visibility lag** --- Problems become apparent only in aggregate
4. **Counter-intuitive fixes** --- The solution is not "try harder" but "change the structure"

The antidote in each case involves:

- **Diversity** --- Multiple sensors, multiple reviewers, multiple mechanisms
- **Expiration** --- Old evidence, old constraints, old trust decay over time
- **Calibration** --- Periodic checks that the system matches reality
- **Human judgment** --- Machines optimize; humans verify the optimization is aimed correctly

---

## Practical Detection Checklist

Run these checks periodically (quarterly or after major milestones):

| Check                    | What to Look For                                       | Action If Found                          |
| ------------------------ | ------------------------------------------------------ | ---------------------------------------- |
| **Dialect drift**        | >90% generated code; declining human modification rate | Schedule manual implementation exercises |
| **Trust decay**          | Approval time <30 seconds; no PR comments              | Inject calibration defect                |
| **Evidence drift**       | >20% stale pointers in recent receipts                 | Implement freshness tracking             |
| **Wisdom overfitting**   | Prompt length growing; phantom constraints             | Review and prune constraints             |
| **Verification ceiling** | Metrics improving but incidents stable                 | Add production signals to panel          |

---

## See Also

- [trust-architecture.md](trust-architecture.md) --- How trust is built and maintained
- [adversarial-loops.md](adversarial-loops.md) --- Defeating optimization through opposition
- [verification-stack.md](verification-stack.md) --- The layered verification approach
- [the-flywheel.md](the-flywheel.md) --- How wisdom flows back (and can overfit)
- [economics.md](economics.md) --- The math behind quality trade-offs
- [claims-and-evidence.md](claims-and-evidence.md) --- Evidence discipline to counter drift
