# Principle: The System Improves

> Wisdom feeds back into templates. The operating manual evolves. Failures make the factory smarter.

## The Principle

The swarm is not a static machine. It learns from every run. Flow 7 (Wisdom) extracts patterns from completed work and proposes improvements to agents, templates, and processes. A system that makes the same mistake twice is not learning.

## Why This Matters

### Repetition Is Waste

When the same failure occurs across runs:
- Developer attention is spent on the same fix repeatedly
- The factory produces the same defects
- Trust erodes ("we keep hitting this")

A learning system converts failures into guardrails.

### Institutional Memory

Individual agents have no memory across sessions. Disk is memory. But patterns that recur across runs need to become part of the pack itself:
- Agent prompts encode lessons
- Templates include hard-won checklists
- Flow commands route around known pitfalls

### Compound Returns

Every improvement compounds. Better agent prompts produce better first-pass output. Better templates catch issues earlier. The pack gets smarter with every run.

## How It Works

### The Feedback Loop

```
Run produces artifacts
    |
Flow 7 analyzes: What worked? What failed? What patterns emerged?
    |
learning-synthesizer extracts patterns + proposes changes
    |
feedback-applier produces diffs to agent prompts, templates, docs
    |
Humans review and apply improvements
    |
Next run benefits from smarter pack
```

### The Scent Trail

Flow 7 maintains `.runs/_wisdom/latest.md` - a persistent file that carries forward negative constraints, positive patterns, and known pitfalls. The researcher reads this before starting work. Past runs inform future runs.

## Examples

### Violation: Silent Repetition

**Run 1:** bdd-critic misses an edge case. Bug ships.
**Run 2:** Same pattern. No change proposed.
**Run 3:** Same pattern again.

The system is not learning. Same failure mode, no improvement.

### Correct: Pattern to Improvement

**Run 1:** bdd-critic misses edge case around empty input.
**Run 2:** Same pattern emerges.

**feedback-applier produces a ready-to-apply diff:**
```markdown
### PACK-001: bdd-critic empty input guidance

**Pattern observed:** Empty input edge cases missed in 2 runs
**Evidence:** run-feat-auth, run-feat-session

**File:** `.claude/agents/bdd-critic.md`
```diff
+- For each happy path, verify empty/null/missing input scenarios exist
```

Human applies the diff. Next run benefits.

### Violation: Advice Without Action

"The agent could benefit from better handling of edge cases."

This is noise. No specific change. No diff. No issue draft.

### Correct: Bound Advice to Action

Every learning must produce either:
1. A diff (ready to apply)
2. An issue draft (for larger work)
3. A discussion item (explicitly labeled, with options)

Free-floating "consider improving X" is not wisdom output.

## The Corollary

**A system that makes the same mistake twice is not learning.**

- If a pattern fails in one run: extract learning, propose change
- If the same pattern fails again: the proposed change was not applied, or was insufficient
- If it fails a third time: the feedback loop is broken

Track recurrence. Escalate repeated failures.

## Flow 7 Agents

**learning-synthesizer** reads all flow artifacts and extracts what worked, what failed, and pack/flow observations. Produces actionable learnings with evidence pointers.

**feedback-applier** converts learnings into:
- `pack_improvements.md`: Ready-to-apply diffs
- `feedback_actions.md`: Issue drafts for larger work
- `.runs/_wisdom/latest.md`: Scent trail for future runs

## Anti-Patterns

| Anti-Pattern | Symptom | Fix |
|--------------|---------|-----|
| Ignoring repeated failures | Same flake in 3 runs, no action | Require action after second occurrence |
| Vibe dumps | "Could be better at X" | Map every observation to diff or issue |
| Backlog never clears | 100 improvements unreviewed | Small, focused; review regularly |
| Over-generalization | One failure = 10 checklist items | One failure = one targeted fix |

## See Also

- [Flow 7: Wisdom](../../../.claude/commands/flow-7-wisdom.md) - The flow that implements learning
- [learning-synthesizer](../../../.claude/agents/learning-synthesizer.md) - Pattern extraction
- [feedback-applier](../../../.claude/agents/feedback-applier.md) - Improvement production
- [Laws of the Swarm](../laws-of-the-swarm.md) - Law 10 and the other immutable rules
