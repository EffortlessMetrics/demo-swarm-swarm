# Maintaining the Pack

> The checklist before you commit. For maintainers adding or modifying agents, flows, or skills.

---

## Before Adding or Changing Anything

Ask these seven questions. If you cannot answer them clearly, reconsider the change.

### 1. What job does it do?

Agents exist for exactly two reasons:

- **Work** — Implementation, critique, planning, or synthesis
- **Compression** — Summarizing context for handoff

If your addition does neither, you are adding ceremony, not capability. Delete it.

### 2. What does it produce that is worth reading?

Every artifact should be useful to the next reader (human or agent). If the artifact exists only to satisfy a checklist, nobody will read it. If nobody reads it, it should not exist.

Ask: Would a reviewer learn something from this? Would the next agent use it?

### 3. How does it report back?

Every agent ends with a handoff:

```
What I did: <summary>
What's left: <remaining work or blockers>
Recommendation: <specific next step with reasoning>
```

Natural language, not parsed fields. The orchestrator reads prose and routes on it.

### 4. What is the evidence lane?

How do we know it worked?

| Evidence Type | Examples |
|---------------|----------|
| Tests | Unit tests pass, integration tests pass |
| Artifacts | Receipt written, summary produced |
| Checks | Lint passes, policy passes |
| Diff | Code changes match intent |

If there is no way to verify success, the change is not observable and may not be doing anything.

### 5. What is the graceful outcome if blocked?

PARTIAL is a valid exit. Honest partial reports are successful outcomes.

Define what the agent should do when it cannot complete:
- Document what was done
- Explain what blocked further progress
- Recommend next steps

Hiding uncertainty is the actual failure mode.

### 6. Does this add handcuffs or add inspection?

| Approach | Effect |
|----------|--------|
| **Handcuffs** | Restrict what agents can do. Slow velocity. Create workarounds. |
| **Inspection** | Verify afterward. Catch issues. Allow iteration. |

Prefer inspection over prevention. Gates engage at publish boundaries, not during internal iteration.

### 7. Does this reduce reviewer attention?

Human review time is the real bottleneck. Every change should:
- Reduce what reviewers must check, OR
- Make their checks faster, OR
- Catch issues before they reach review

If a change adds reviewer burden without reducing it elsewhere, reconsider.

---

## Agent Checklist

Before committing a new agent, verify:

- [ ] **Single clear job** — One sentence describes what it does; one says what it does not do
- [ ] **Positive framing** — Tips section uses "do X" not "don't do Y"
- [ ] **Specific input paths** — Exact file paths, not abstractions
- [ ] **Specific output paths** — Exact file paths for what it produces
- [ ] **Graceful failure section** — "If You're Stuck" with hierarchy of responses
- [ ] **Clear handoff template** — "What I did / What's left / Recommendation"
- [ ] **Outcome-focused success** — Describes value delivered, not schema compliance
- [ ] **Factual voice** — No adjectives, no persona, no theatrical language
- [ ] **Category color matches type** — Red for critics, green for implementation, etc.

See [design-agents.md](design-agents.md) for prompt structure and [add-an-agent.md](add-an-agent.md) for integration.

---

## Flow Checklist

Before adding a flow, verify:

- [ ] **Clear entry conditions** — What must exist before the flow can run
- [ ] **Clear exit conditions** — What constitutes VERIFIED, UNVERIFIED, PARTIAL
- [ ] **Defined artifacts** — Table of outputs with source agents
- [ ] **Documented dependencies** — Which upstream flows must complete first
- [ ] **Graceful degradation** — Behavior when upstream artifacts are missing
- [ ] **Receipt produced** — Cleanup agent writes `<flow>_receipt.json`
- [ ] **Updated CLAUDE.md** — Flow in "The Seven Flows" table

See [create-a-flow.md](create-a-flow.md) for flow structure.

---

## Skill Checklist

Before adding a skill, verify:

- [ ] **Deterministic** — Same input produces same output
- [ ] **No judgment required** — Mechanical work only
- [ ] **Fast execution** — Completes quickly without blocking
- [ ] **Clear invocation syntax** — Documented command and parameters
- [ ] **Returns data, not decisions** — Skill provides information; agent decides

Skills exist for work that requires no reasoning. If judgment is needed, use an agent.

---

## Anti-Patterns to Avoid

| Anti-Pattern | Why It Fails | What to Do Instead |
|--------------|--------------|-------------------|
| **God agent** | Too many responsibilities. Hard to debug. Brittle. | Split into focused agents with single responsibilities |
| **Parsed routing** | Fragile. Encourages gaming structured fields. | Route on prose. Orchestrators read and understand. |
| **Approval-per-action** | Kills velocity. Creates token-burning loops. | Gate at publish boundaries only |
| **Schema-first output** | Mechanical compliance. Misses actual quality. | Outcome-first success criteria |
| **Constraint lists** | "DO NOT" prompts teach bad behavior. Hard to follow. | Positive guidance. Describe what TO DO. |
| **Kitchen sink agent** | Does everything shallowly. No accountability. | Single responsibility, done deeply |
| **"While you're there..."** | Couples unrelated work. Muddies ownership. | Separate agents for separate jobs |

---

## When to Say No

Reject changes that:

- **Add ceremony without capability** — If it does not do work or compress context, delete it
- **Slow down the happy path** — Gates during internal iteration kill velocity
- **Require parsing structured blocks for routing** — Natural language is the contract
- **Add permissions that don't protect boundaries** — Handcuffs, not guardrails
- **Create artifacts nobody will read** — If it is not useful to the next reader, do not require it
- **Conflate multiple responsibilities** — One agent, one job

---

## The Smell Test

Good changes pass these checks:

**For agents:**
- Can you describe the job in one sentence?
- Would you trust a well-trained junior to do this with these instructions?
- Is there a clear way to verify it worked?

**For flows:**
- Is there a meaningful phase boundary here, or is this just adding steps?
- Does the cleanup agent have something real to measure?
- Can the flow recover gracefully from partial completion?

**For skills:**
- Is this truly mechanical, or does it require judgment?
- Could an agent do this inline, or does it need isolation?
- Is the output stable and predictable?

---

## Contributing Flow

When contributing to the pack:

1. **Read first** — Understand existing patterns before adding new ones
2. **Check alignment** — Does this fit the philosophy in CLAUDE.md?
3. **Test locally** — Run the change through a real flow
4. **Pack-check passes** — `bash .claude/scripts/pack-check.sh`
5. **Update docs** — If behavior changes, docs change too

---

## See Also

- [design-agents.md](design-agents.md) — How to write agent prompts
- [add-an-agent.md](add-an-agent.md) — Mechanics of adding an agent
- [create-a-flow.md](create-a-flow.md) — How to create a new flow
- [agent-philosophy.md](../explanation/agent-philosophy.md) — How agents think and act
- [single-responsibility.md](../explanation/principles/single-responsibility.md) — Why one job matters
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
