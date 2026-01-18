# Anti-Patterns

This document catalogs the failure modes that kill velocity, quality, or trust. Recognizing these patterns early prevents wasted effort.

**Key insight:** Most anti-patterns come from importing assumptions that don't apply to Claude-native systems. Traditional software development intuitions about control, verification, and human oversight often translate poorly to agent-based workflows.

---

## The Anti-Patterns

### 1. Schema Religion

**What it looks like:**

- Every agent emits structured YAML/JSON blocks
- Routing depends on parsing specific fields
- "If it's not machine-readable, it's not real"

**Why it fails:**

- Claude routes by reading, not parsing
- Forces agents to optimize for compliance, not quality
- Creates brittle dependencies on exact formats
- Minor formatting variations break the entire flow

**The fix:** Prose handoffs. Structured artifacts only where they reduce future work (receipts, counts, status ledgers). Let agents communicate naturally and trust orchestrators to understand.

**Example:**

Bad:

```yaml
status: BLOCKED
reason: "missing_dependency"
blocker_type: "upstream_artifact"
```

Good:

> I cannot proceed with the implementation. The database migration from AC-001 doesn't exist yet. Either create it as part of this work or document the dependency.

---

### 2. Blocked Inflation

**What it looks like:**

- Lint failures become "CANNOT_PROCEED"
- Every issue triggers a stop-and-wait
- "Blocked" used for anything that isn't perfect
- Agents report blocked status for fixable problems

**The reality:** "Blocked" is almost always just routing to another agent. True halting is very rare.

| What They Say                | What Should Happen        |
| ---------------------------- | ------------------------- |
| "Blocked on lint"            | Route to auto-linter      |
| "Blocked on test failure"    | Route to fixer            |
| "Blocked on design conflict" | Route to design-optioneer |
| "Blocked on unclear spec"    | Route to clarifier        |

**Why it fails:**

- Kills momentum on work that could continue
- Wastes expensive human attention on fixable issues
- Creates a culture of learned helplessness
- Agents stop trying when they could fix-forward

**The fix:** Almost everything is "route to X," not "blocked." True halt is reserved for:

- Mechanical failure (tooling broken, infra down)
- Non-derivable decision (business choice requiring human input)
- Unsafe publish boundary (secrets detected)

**Example:**

Bad:

> BLOCKED: ESLint found 3 warnings in the new code.

Good:

> Implementation complete. 3 lint warnings remain (unused variable, prefer-const x2). Recommend routing to auto-linter before commit.

---

### 3. Completion Theater

**What it looks like:**

- Agent reports "COMPLETE" without evidence
- Tests "pass" but mutation score is 40%
- Receipts claim success but point to nothing
- "All requirements satisfied" with no verification

**Why it fails:**

- Process confabulation - agents please instead of verify
- False confidence leads to production issues
- Trust erodes when claims don't match reality
- No one knows what was actually tested

**The fix:** Evidence discipline. Claims need pointers. "Not measured" is acceptable; false certainty is not.

**Example:**

Bad:

```
status: COMPLETE
all_tests: PASS
```

Good:

> Implementation complete. 12 unit tests pass. Integration tests not run (no database fixture). NFR-PERF-001 not measured - load testing requires infrastructure not available locally.

---

### 4. Super-Agent Prompts

**What it looks like:**

- One agent does implement + review + gate + publish
- 2000-line prompts with 15 responsibilities
- "Just add another section to handle X"
- Mode switches embedded in a single agent

**Why it fails:**

- Everything becomes shallow
- No clear success criteria
- Impossible to debug which part failed
- Context bloat reduces quality on every task

**The fix:** Single responsibility. One agent, one job, done deeply. If you need modes, split agents.

**Example:**

Bad: `code-implementer` that also runs tests, critiques its own code, decides if it's ready for review, and creates the PR.

Good: `code-implementer` writes code. `test-executor` runs tests. `code-critic` reviews. `repo-operator` handles git. Each does one thing well.

---

### 5. Chat as State

**What it looks like:**

- Work product lives in conversation history
- Resuming requires re-explaining everything
- Session reset = lost work
- "Let me remind you what we discussed..."

**Why it fails:**

- Conversations get cleared
- Context windows fill
- No audit trail for later review
- Can't hand off to a different agent

**The fix:** Disk is memory. Artifacts capture state. Every call is implicitly a resume from disk. Agents check what exists before starting.

**Example:**

Bad: Agent remembers previous decisions from earlier in the conversation.

Good: Agent reads `.runs/<run-id>/plan/adr.md` to understand architectural decisions, regardless of whether it was the agent that created them.

---

### 6. Permission Theater

**What it looks like:**

- Constant "may I read this file?" prompts
- Every action requires approval
- Safety through interruption
- Human must approve each step

**Why it fails:**

- Kills velocity without adding safety
- Safety comes from boundaries, not prompts
- Human attention wasted on approving obvious actions
- Creates false sense of control

**The fix:** Sandbox + boundary gates. Default-allow inside the sandbox, gate at publish. The safety model is architectural, not conversational.

**Example:**

Bad:

> I found a utility file that might need changes. May I read src/utils/helpers.ts?

Good:

> I read src/utils/helpers.ts and found the validation function. Modified it to handle the new edge case per REQ-003.

---

### 7. Review Everything

**What it looks like:**

- Human reads every line of generated code
- PRs reviewed like hand-written code
- "I need to understand every change"
- Treating AI output as untrusted by default

**Why it fails:**

- Doesn't scale (70k LOC PRs exist)
- Wastes senior attention on mechanical details
- Ignores the evidence that proves correctness
- Humans are bad at reading 500 lines carefully

**The fix:** Review evidence, not lines. Spot-check guided by hotspots. Trust the verification pipeline. The PR Brief tells you where to look.

**Example:**

Bad: Reading all 47 files changed in the PR.

Good: Checking the PR Brief for hotspots, reviewing the 3 flagged areas, verifying test coverage claims match reality, spot-checking one representative module.

---

### 8. Premature Abstraction

**What it looks like:**

- Creating "frameworks" for one-off tasks
- Adding configuration for hypothetical futures
- "Let's make this generic"
- Building extensibility before the second use case

**Why it fails:**

- Complexity without value
- Harder to understand and maintain
- The future need never arrives
- Wrong abstraction is worse than duplication

**The fix:** Solve the problem in front of you. Three similar lines are better than a premature abstraction. Wait for the pattern to emerge from real usage.

**Example:**

Bad: Creating a `NotificationStrategyFactory` when you only have email notifications.

Good: Implementing email notifications directly. When SMS is needed, refactor then.

---

### 9. Wisdom Without Implementation

**What it looks like:**

- "We learned X" but nothing changes
- Lessons documented, never applied
- Same mistakes repeated across runs
- Retrospectives that produce only notes

**Why it fails:**

- Learning without action is just notes
- No compound improvement over time
- Organizational amnesia persists
- The documentation becomes noise

**The fix:** Wisdom must change something concrete: templates, checklists, prompts, default behaviors, or process. If a lesson can't change something, question whether it's actually a lesson.

**Example:**

Bad:

> Learned: We should check for null values more carefully.

Good:

> Learned: Null checks were missing in 3 handlers. Updated the code-critic checklist to flag functions without null guards. Added test template with null case.

---

### 10. Gate Everywhere

**What it looks like:**

- Approval required for every step
- Multiple "checkpoints" during implementation
- "Let's add a gate here too"
- Human in the loop for internal iterations

**Why it fails:**

- Gates are expensive (human attention is scarce)
- Most gates become rubber stamps
- Real safety comes from publish boundaries
- Slows everything without proportional benefit

**The fix:** Gate at boundaries only. Work freely inside the sandbox. The meaningful gates are:

- Before committing (secrets scan)
- Before pushing (repo state)
- Before merging (quality gate)
- Before deploying (verification)

Internal iteration doesn't need gates.

---

## Quick Recognition Table

| Symptom                           | Likely Anti-Pattern           |
| --------------------------------- | ----------------------------- |
| Agents output mostly YAML/JSON    | Schema Religion               |
| Everything stops for lint errors  | Blocked Inflation             |
| "Complete" with no evidence       | Completion Theater            |
| Prompts have 10+ responsibilities | Super-Agent                   |
| Can't resume after session reset  | Chat as State                 |
| Constant permission prompts       | Permission Theater            |
| Human reads every generated line  | Review Everything             |
| Simple task becomes framework     | Premature Abstraction         |
| Same failures keep recurring      | Wisdom Without Implementation |
| Can't ship without 5 approvals    | Gate Everywhere               |

---

## Why These Patterns Emerge

These anti-patterns often come from reasonable places:

1. **Schema Religion** emerges from traditional API design thinking. But Claude isn't an API - it reads and understands.

2. **Blocked Inflation** comes from defensive programming. But agents can fix problems, not just report them.

3. **Completion Theater** happens when success metrics focus on format over substance.

4. **Super-Agent** grows from "just one more feature" accretion.

5. **Chat as State** feels natural because conversations are how we interact. But agents need durable memory.

6. **Permission Theater** comes from treating AI like an untrusted subprocess.

7. **Review Everything** imports code review habits that assume human-written code.

8. **Premature Abstraction** is standard engineering advice applied without judgment.

9. **Wisdom Without Implementation** happens when retrospectives are rituals, not improvement mechanisms.

10. **Gate Everywhere** reflects traditional change management where every change needs approval.

The common thread: assumptions from human-centric or traditional automation contexts that don't transfer to Claude-native systems.

---

## Related Documents

- [Agent Philosophy](agent-philosophy.md) - Why agents work the way they do
- [Claude-Native Design](claude-native-design.md) - Design principles for Claude systems
- [Boundary Physics](boundary-physics.md) - Where gates actually belong
- [Operating Model](operating-model.md) - How the system actually runs
