# What Makes This Different

> This pack uses patterns that may surprise you. Here's what's different and why.

## Why This Document Exists

If you're an LLM reading this pack, or a developer working with it, you may have assumptions about how agent systems work. Many of those assumptions come from older patterns that don't apply to Claude Code.

This document explicitly names those assumptions and explains what we do instead.

---

## Assumption: Agents emit structured data for routing

**What you might expect:**
Agents output YAML or JSON blocks that get parsed by an orchestrator to determine the next step:

```yaml
status: UNVERIFIED
recommended_action: RERUN
route_to_agent: code-implementer
```

**What we actually do:**
Agents write prose handoffs. The orchestrator (Claude) reads and understands them:

```markdown
## Handoff

Found 2 issues with the implementation. Session timeout is wrong.
Recommend code-implementer fix these, then re-run me.
```

**Why:**
Claude Code's orchestrator IS Claude. It doesn't need structured data to route—it reads and understands natural language. Structured routing blocks are from systems where a Python script parsed output to make decisions. Claude doesn't need training wheels; it can read.

---

## Assumption: Receipts are routing inputs

**What you might expect:**
Receipts contain status fields that the orchestrator checks to decide what to do next. You'd grep for `status: VERIFIED` or parse a JSON field.

**What we actually do:**
Receipts are audit artifacts for humans reviewing later. The orchestrator routes on agent handoffs, not receipt contents.

**Why:**
The orchestrator gets routing information directly from the agent that just ran. The handoff says "I'm done, run the critic next." The receipt exists so a human reviewing the run in 3 months can understand what happened, what was counted, and what evidence was present. These serve different audiences—don't conflate them.

---

## Assumption: Status enums are contracts

**What you might expect:**
Agents must emit status values from a closed enum (`VERIFIED | UNVERIFIED | CANNOT_PROCEED`) and the system routes based on matching these strings.

**What we actually do:**
Agents express outcomes in prose. "Ship it" not `verdict: MERGE`. Status concepts exist for receipts (audit vocabulary) but aren't routing contracts.

**Why:**
Claude doesn't need enum values to understand "the tests pass and we should ship this." Natural language is more expressive and handles edge cases better. "Almost ready but check the timeout value" doesn't fit in an enum—but Claude understands it perfectly.

---

## Assumption: Agents need constraint lists

**What you might expect:**
Agent prompts should have extensive "Do NOT" and "NEVER" sections to prevent bad behavior:

```
DO NOT:
- Modify files outside the manifest
- Skip writing tests
- Commit directly to main
- Use deprecated APIs
```

**What we actually do:**
Positive prompting—tell agents what TO do and how to do it well. Trust them to be capable.

```
Your goal is to implement the acceptance criteria with full test coverage.

Read the ADR to understand the design constraints. Write tests alongside
your implementation. Update the AC status file when you complete each item.
```

**Why:**
Constraint lists are cognitive overhead. They teach the model to imagine the forbidden behavior. A well-trained professional doesn't need a list of forbidden actions—they need clear goals and helpful guidance. Negative prompts create defensive behavior; positive prompts create capable behavior.

---

## Assumption: More structure = more reliability

**What you might expect:**
Detailed schemas, strict field validation, and rigid output formats ensure consistent behavior. If you lock down the format, the agent can't go wrong.

**What we actually do:**
Minimal structure. Natural language handoffs. Artifacts with substance rather than schema compliance.

**Why:**
Claude is more reliable when communicating naturally than when trying to satisfy rigid schemas. Structure doesn't add reliability—it adds brittleness and constrains expression. When an agent needs to say "mostly done but there's an edge case," a rigid schema forces it to lie (pick a status that doesn't fit) or fail (can't emit valid output). Prose lets it tell the truth.

---

## Assumption: Agents should be comprehensive

**What you might expect:**
An agent should handle all aspects of its domain—a "code reviewer" should check logic AND style AND security AND contracts AND test coverage.

**What we actually do:**
Single responsibility. One agent, one job. Chain specialists instead of creating generalists.

- `code-critic`: Reviews code quality and correctness
- `test-critic`: Reviews test coverage and validity
- `standards-enforcer`: Checks formatting and lint
- `secrets-sanitizer`: Scans for secrets

**Why:**
Depth beats breadth. An agent with one job does it well. An agent with seven jobs does all of them poorly. Specialists also produce cleaner handoffs—"code quality is good, route to test-critic" is clearer than "code quality is good but test coverage is weak and there might be a security issue."

---

## Assumption: Failure is bad

**What you might expect:**
Agents should always complete their assigned task. Partial results indicate something went wrong. The system should retry until the agent succeeds.

**What we actually do:**
Graceful outcomes. An honest report of what happened—even if partial—is a successful agent run. Only silence is failure.

```markdown
## Handoff

**What I did:** Implemented 2 of 5 ACs. Tests pass for login and logout.

**What's blocked:** AC-3 requires a database migration that doesn't exist.

**Recommendation:** Create the migration first, then rerun me.
```

**Why:**
Real work rarely completes perfectly on the first try. The orchestrator needs to know what happened to adapt. Punishing partial results encourages dishonest reporting—agents will claim completion and hide blockers. `PARTIAL` is a win. It's a save point. The flow can rerun and pick up where it left off.

---

## Assumption: Artifacts are for machines

**What you might expect:**
Artifacts like receipts and decisions are structured data for downstream processing. They exist so the next agent can parse them.

**What we actually do:**
Artifacts are for humans. They should be worth reading, explain reasoning, and be useful months later.

**Stubby artifact (bad):**

```markdown
# Requirements Critique

Status: VERIFIED
Issues: 0
```

**Substantive artifact (good):**

```markdown
# Requirements Critique

All five requirements pass the testability check.

REQ-001 through REQ-003 have clear acceptance criteria with measurable outcomes.
REQ-004 references "appropriate security measures" which I interpret as the
OWASP guidelines mentioned in NFR-SEC-001. REQ-005's timeout value aligns with
the ADR's session management constraints.

No blocking issues. Recommend proceeding to implementation.
```

**Why:**
The machine (Claude) gets what it needs from handoffs. Artifacts exist for the humans who will review, audit, and learn from the run. An artifact that just says "PASSED" wastes disk space. An artifact that explains what was checked and why helps the human understand what happened.

---

## Assumption: The orchestrator is dumb

**What you might expect:**
The orchestrator is a simple state machine or script that can only do if/else on structured data. It needs explicit routing tables and enum matching.

**What we actually do:**
The orchestrator is Claude. It reads, understands context, makes judgment calls, and adapts.

**Why:**
Claude Code's architecture puts Claude in the driver's seat. It's not a parser—it's a capable PM directing a team. It can read "the implementation looks good but I'm worried about the edge case in session expiry" and decide to route to code-critic for a second opinion. A state machine can't do that.

---

## Assumption: Agents are functions

**What you might expect:**
Agents are like pure functions: input goes in, deterministic output comes out, next step follows. They're replaceable components in a pipeline.

**What we actually do:**
Agents are actors with judgment. They read context, do work, make decisions, and recommend next steps. They're intelligent participants, not mechanical transformers.

**Why:**
The value of agents is their judgment, not just their execution. They should think about what they're doing, not just do it. An agent that notices "this AC contradicts the ADR" and raises it is more valuable than one that blindly implements the contradiction.

---

## Assumption: Allowlists prevent mistakes

**What you might expect:**
You should define exactly which files an agent can touch. A manifest lists permitted paths. Anything outside the manifest is forbidden.

**What we actually do:**
Role focus, not path restrictions. "Your job is to implement this AC" rather than "You may only touch these 5 files."

**Why:**
Allowlists assume omniscient planning—that someone can predict every file the agent might need. They can't. An agent implementing authentication might discover it needs to touch a utility file, a config, or a migration. Blocking exploration forces it to either fail or work around the restriction.

We use critics as guardrails instead. The agent explores freely, does its work, then a critic reviews whether it stayed focused on the task. This catches scope creep without preventing legitimate exploration.

---

## Assumption: Agents should ask before acting

**What you might expect:**
When uncertain, agents should stop and request clarification. "I'm not sure which timeout value to use—should I proceed?"

**What we actually do:**
Research-first autonomy. Agents investigate, derive from context, apply safe defaults, and only escalate when truly blocked.

The escalation ladder:

1. **Investigate locally:** Search code, tests, configs for the answer
2. **Derive from evidence:** Use patterns in the codebase to infer correct behavior
3. **Default if safe:** Choose a reversible default, document it, continue
4. **Escalate only when boxed in:** All above failed AND no safe default exists

**Why:**
"Stop and ask" creates token-burning loops for basic exploration. Most questions can be answered by reading existing code. What timeout value? Look at other timeouts in the codebase. What error format? Look at existing error handlers. The agent has the full codebase available—it should use it.

---

## Assumption: Gates should be comprehensive

**What you might expect:**
Gate checks should be thorough and catch everything before merge. Security, quality, coverage, performance—validate it all at the gate.

**What we actually do:**
Gates VERIFY, they don't DISCOVER. Discovery belongs in upstream flows. Gate confirms that earlier findings were addressed, not that new issues don't exist.

**Why:**
Problems should be caught where the fix is cheapest. Catching a design issue at Gate is expensive—the code is already written. Catching it in Plan is cheap—you just update the ADR. If Gate is discovering issues, that's a signal the upstream flows need improvement, not that Gate needs more checks.

---

## The Key Insight

**Old model:** Machine orchestrator + agent functions + structured data contracts

**Claude Code model:** Claude as PM + agents as capable juniors + natural language communication

Everything in this pack follows from that insight. When something seems counterintuitive, ask: "Does this make sense if Claude is the orchestrator?"

If the orchestrator is a Python script, you need structured data because Python can't understand prose. But if the orchestrator is Claude, prose is actually better—more expressive, more nuanced, easier to audit.

If agents are functions, you need strict I/O contracts. But if agents are intelligent actors, you want them to exercise judgment and report honestly, even when the result is "I'm stuck."

If you're fighting the pack's patterns, you might be applying old-model assumptions. Step back and ask what Claude (the orchestrator) actually needs.

---

## Summary Table

| Old Assumption            | What We Do Instead           | Key Benefit                          |
| ------------------------- | ---------------------------- | ------------------------------------ |
| Structured routing blocks | Prose handoffs               | Claude understands; humans can audit |
| Receipts for routing      | Receipts for audit only      | Separation of concerns               |
| Status enums              | Natural language outcomes    | Handles edge cases                   |
| Constraint lists          | Positive prompting           | Capable behavior, not defensive      |
| Rigid schemas             | Minimal structure            | Honest reporting, flexibility        |
| Comprehensive agents      | Single responsibility        | Depth over breadth                   |
| Failure is bad            | Graceful outcomes            | Honest partial reports               |
| Artifacts for machines    | Artifacts for humans         | Worth reading in 3 months            |
| Dumb orchestrator         | Claude as PM                 | Judgment and adaptation              |
| Agents as functions       | Agents as actors             | Value from judgment                  |
| Allowlists                | Role focus + critics         | Exploration without scope creep      |
| Stop and ask              | Research-first autonomy      | Fewer interruptions                  |
| Comprehensive gates       | Gates verify, don't discover | Fix where it's cheapest              |

---

## See Also

- [Claude-Native Design](claude-native-design.md) — Full philosophy
- [Agent Philosophy](agent-philosophy.md) — How agents work
- [Principles](principles/) — Deep dives on each principle
- [AI Physics](ai-physics.md) — LLM failure modes this design addresses
