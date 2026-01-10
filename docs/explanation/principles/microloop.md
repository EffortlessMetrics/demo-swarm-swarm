# Principle: The Microloop

> Writer -> Critic -> Fix -> Repeat until quality.

## The Principle

Quality comes from iteration. A writer produces work, a critic finds flaws, the writer fixes them, the critic verifies. This loop continues until the critic is satisfied.

## Why This Matters

### First Drafts Aren't Final

No one writes perfect code on the first try. The microloop acknowledges this:
- Writers focus on getting something working
- Critics focus on finding what's wrong
- Iteration closes the gap

### Separation of Concerns

The writer's job: produce work
The critic's job: find flaws

These are different mindsets. Separating them improves both.

### Rigorous Verification

"Trust but verify" in action:
- We trust the writer to do good work
- We verify by having a critic review
- We trust the fix, then verify again

### Bounded Iteration

The loop doesn't run forever:
- Critic says "proceed" when satisfied
- Critic says "no further iteration helps" when stuck
- Orchestrator can cap iterations

## How It Works

### The Basic Pattern

```
Writer produces work
    |
Critic reviews
    |
Issues found? --Yes--> Writer fixes --> Critic reviews again
    |
    No
    |
Proceed to next step
```

### Roles

**Writer** (code-implementer, test-author, fixer):
- Produces or modifies artifacts
- Responds to critic feedback
- Focuses on making it work

**Critic** (code-critic, test-critic, etc.):
- Reviews against spec/requirements
- Finds issues, explains why they matter
- Recommends whether to proceed or iterate

**Orchestrator** (Claude PM):
- Routes based on critic recommendation
- Decides when to proceed despite imperfections
- Caps runaway iteration

### The Critic's Recommendation

Critics end with a clear recommendation:

**"Proceed"** - Work is good enough
- No blocking issues
- Minor issues can be deferred

**"Rerun writer"** - Fixable issues found
- Specific issues identified
- Writer can address them

**"No further iteration helps"** - Stuck
- Issues require human decision
- Or architectural change needed
- Or external dependency

### The Handoff

Critic to orchestrator:
```markdown
## Handoff

**What I found:** Session timeout wrong (30m vs 15m spec), missing null check.

**Recommendation:** Run code-implementer to fix these, then re-run me.
Both are mechanical fixes the writer can handle.
```

Orchestrator routes to writer with the feedback.

## Examples

### Test Writing Loop

```
test-author writes tests
    |
test-critic reviews
    |
"Missing edge case for empty input,
 assertion messages unclear"
    |
test-author adds edge case, improves assertions
    |
test-critic reviews again
    |
"Proceed - tests are solid"
```

### Implementation Loop

```
code-implementer writes feature
    |
code-critic reviews
    |
"Logic correct but error handling missing,
 also naming inconsistent with codebase"
    |
code-implementer adds error handling, fixes naming
    |
code-critic reviews again
    |
"Proceed - implementation is clean"
```

### Stuck Loop

```
code-implementer writes feature
    |
code-critic reviews
    |
"This approach can't meet the latency requirement.
 Need to either change the requirement or redesign."
    |
Critic recommends: "No further iteration helps.
 Route to design for architecture decision."
```

## When to Exit

### Green Path: Critic Says Proceed
Work meets quality bar. Continue to next step.

### Yellow Path: Max Iterations
Orchestrator decides good enough. Proceed with documented issues.

### Red Path: Stuck
Critic says no fix possible at this level. Bounce to design or human.

## Anti-Patterns

### Infinite Loop
No exit condition. Writer and critic bounce forever.

**Fix:** Critic explicitly recommends proceed or escalate.

### Single Pass
No iteration. Writer produces, immediately move on.

**Fix:** Always have critic review. Iterate if needed.

### Critic Fixes
Critic finds issues AND fixes them.

**Fix:** Critics find, writers fix. Separation of concerns.

### Orchestrator Ignores Critic
Critic says iterate, orchestrator proceeds anyway.

**Fix:** Route on critic recommendation. Trust their judgment.

## The Quality Ratchet

Each iteration should improve:
- Critic finds issues
- Writer fixes them
- Those issues don't recur

Quality ratchets up, never down.

If issues recur, something is wrong with the feedback loop.

## Applying to Different Domains

| Domain | Writer | Critic | Loop On |
|--------|--------|--------|---------|
| Tests | test-author | test-critic | Coverage, assertions, edge cases |
| Code | code-implementer | code-critic | Correctness, maintainability |
| Docs | doc-writer | doc-critic | Accuracy, completeness |
| Design | design-optioneer | option-critic | Distinctness, comparability |

Same pattern, different specialists.

## See Also

- [Single Responsibility](single-responsibility.md) - Why we separate writer/critic
- [Graceful Outcomes](graceful-outcomes.md) - How critics report stuck states
