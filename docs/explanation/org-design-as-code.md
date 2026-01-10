# Org Design as Code

> The pack is org design, compiled into prompts and flows.

---

## The Meta-Pattern

This pack doesn't just *use* organizational concepts as metaphors. It **literally encodes** them into agent prompts, flow definitions, and verification gates.

When we say "PM + IC model," we mean:
- The orchestrator prompt is designed like a PM job description
- Agent prompts are designed like IC role definitions
- Handoffs work like status updates in a well-run team

This isn't analogy. It's implementation.

---

## What We're Encoding

Three bodies of organizational theory, compiled into executable code.

### Teal Organization Principles

From Laloux's *Reinventing Organizations*:

**Self-management within role:**
- Each agent owns their domain completely
- No micromanagement from orchestrators
- Single responsibility enables autonomy
- The sandbox is the constraint, not per-action permissions

**Whole-person engagement:**
- Agents bring judgment, not just execution
- Real cognitive work, not template filling
- Graceful outcomes include honest uncertainty
- Partial progress with clear reporting is a valid outcome

**Evolutionary purpose:**
- Wisdom loop improves the system
- Templates evolve based on learnings
- The org gets smarter over time
- Flow 7 extracts insights; humans decide what to adopt

### Agile Principles

From the manifesto and its descendants:

**Short feedback loops:**
- Microloops iterate quickly (write -> critique -> fix -> verify)
- Critics catch issues fast (per-AC, not per-feature)
- Fix-forward maintains momentum
- Push early to get bot feedback

**Working software over documentation:**
- Receipts capture what happened, not what was planned
- Evidence over claims
- Artifacts with substance, not theater
- The diff is the audit surface; tests are the runtime truth

**Responding to change:**
- Resume from disk state
- Adapt to what exists
- Local resolution before escalation
- "Extras" are embraced, not blocked

### Modern PM/IC Dynamics

From contemporary tech org design:

**PM as context-holder:**
- Orchestrator maintains intent
- Routes based on understanding
- Doesn't parse, reads
- Coordinates without micromanaging

**IC as expert:**
- Deep work in narrow domain
- Honest reporting
- Recommends next steps
- Makes decisions within scope

**Staff as quality bar:**
- Critics maintain standards
- Cleanup compresses context
- Gates decide ship/no-ship
- Reviews produce evidence, not gatekeeping

---

## How It's Encoded

Organizational patterns appear in four places.

### In Agent Prompts

Each agent prompt is a job description:

| Job Description Element | Prompt Equivalent |
|------------------------|-------------------|
| Role title | Agent name (`code-implementer`, `test-author`) |
| One clear responsibility | "Your job is to..." statement |
| Success criteria | "You're done when..." conditions |
| What to do when stuck | Research ladder, escalation path |
| How to hand off | Result block format, recommendation requirement |

**Example:** The `code-critic` prompt says "Critics review and find issues. Never fix—they report to workers." This is the same instruction you'd give a human code reviewer on a team that separates review from implementation.

### In Flow Definitions

Flows are org processes:

| Org Process Element | Flow Equivalent |
|--------------------|-----------------|
| Workflow stages | Station order (Signal -> Plan -> Build...) |
| Iterative refinement | Microloops (author -> critic -> fix -> verify) |
| Approval processes | Gates (secrets check, merge decision) |
| Handoff protocols | Result blocks between stations |
| Status meetings | Cleanup agents summarizing progress |

**Example:** Flow 3's microloop structure encodes how a good team does code review: write, get feedback, improve, verify. The 2-pass default mirrors "review once, address feedback, final check."

### In Handoff Structure

Handoffs are status updates:

| Status Update Element | Handoff Equivalent |
|----------------------|-------------------|
| What I did | Summary of work completed |
| What I found | Key findings, blockers, open questions |
| My recommendation | Specific next step with reasoning |
| Owner for next action | Named agent or explicit uncertainty |

**Example:** A good junior engineer doesn't just say "done." They say "I implemented the login flow, tests pass, but I noticed the session table doesn't exist yet. I recommend routing to whoever handles migrations before AC-002."

### In Artifact Design

Artifacts are org memory:

| Org Memory Element | Artifact Equivalent |
|-------------------|---------------------|
| Project records | Receipts (`*_receipt.json`) |
| Documented rationale | ADRs, decision memos |
| Tracked action items | Worklists, AC matrices |
| Audit trail | Git log, run folders |
| Lessons learned | `learnings.md`, `wisdom_receipt.json` |

**Example:** `build_receipt.json` serves the same purpose as a project status report: what was attempted, what succeeded, what remains.

---

## Why This Matters

### Predictable Behavior

When org patterns are explicit:
- Agents behave consistently across runs
- Expectations are clear to both agents and humans
- Failures are diagnosable (which role failed? which handoff broke?)

Compare to implicit patterns: "The AI should figure out how to collaborate." That's unpredictable. Explicit role definitions produce consistent behavior.

### Transferable Knowledge

Org design expertise transfers:

| Human Org Skill | Pack Design Application |
|-----------------|------------------------|
| Good PM practices | Good orchestrator design |
| Good IC practices | Good agent design |
| Good team dynamics | Good flow design |
| Good meeting hygiene | Good handoff design |
| Good documentation | Good artifact design |

If you know how to run a good eng team, you already know most of what makes this pack work. The patterns aren't novel—they're encoded.

### Evolvable System

As org theory evolves:
- Patterns can be updated in prompts
- New research applies directly
- System improves with organizational knowledge

When someone publishes better practices for code review, we can update `code-critic.md`. When team dynamics research suggests better handoff formats, we can update the Result block schema.

---

## The Compilation Process

Traditional software compiles source code into machine instructions.

This pack compiles org design into:

| Org Design Element | Compiled Into |
|-------------------|---------------|
| Role definitions | Agent prompts (`.claude/agents/*.md`) |
| Workflows | Flow commands (`.claude/commands/flow-*.md`) |
| Policies | Gate logic, critic criteria |
| Communication norms | Handoff structure, Result blocks |
| Quality standards | Critic severity levels, verification gates |

**The compiler is the pack author.** Reading org design literature and encoding it into prompts is compilation. The prompts are the bytecode.

---

## Concrete Examples

### PM/IC Boundary

**Org theory:** PMs scope work and remove blockers. ICs do the technical work. Mixing these roles creates confusion.

**Pack encoding:** Orchestrators call agents and route on results. They don't parse files or run commands. Agents do the work and report back. Orchestrators never reach into agent work; agents never route themselves.

### Single Responsibility

**Org theory:** Teams work best when each member has one clear job. Overlapping responsibilities create coordination overhead and diffuse accountability.

**Pack encoding:** `code-implementer` writes code. `code-critic` reviews code. `fixer` fixes issues. No overlap. If a critic spots an issue, they report it; they don't fix it themselves.

### Psychological Safety

**Org theory:** Teams perform better when members can report problems without fear of blame. Honest uncertainty beats false confidence.

**Pack encoding:** `PARTIAL` is a valid completion status. Agents are explicitly told that honest partial reports are successful outcomes. The prompt says "A report saying 'I completed 2/5 ACs, blocked on missing schema' is a VERIFIED success."

### Research Before Escalating

**Org theory:** Good ICs investigate problems themselves before asking for help. Escalation should be the last resort, not the first.

**Pack encoding:** Law 5 (Research-First Autonomy) gives agents a five-step escalation ladder. They investigate locally, derive from evidence, choose safe defaults, then escalate "only when boxed in."

### Fix-Forward Culture

**Org theory:** High-performing teams fix issues as they find them rather than blocking on approval chains.

**Pack encoding:** The default outcome is "route to fix," not "block and wait." Gates constrain publishing, not thinking. Small issues get fixed in-place; only design-level problems bounce to earlier flows.

---

## The Key Insight

Traditional AI systems try to make models "smart enough" to figure out collaboration. They assume that with enough capability, the model will discover how to work with humans.

This pack says: **encode collaboration patterns explicitly**.

Give agents the same structure that makes human teams work:
- Clear roles with defined boundaries
- Explicit handoff protocols
- Shared vocabulary for status and outcomes
- Escalation ladders for when things go wrong
- Quality gates at appropriate boundaries

It's not about making AI human. It's about giving AI the same organizational scaffolding that helps humans collaborate effectively.

The pack is a team structure, compiled into prompts and flows.

---

## Implications for Pack Authors

If you're extending or forking this pack:

**Think in org design first.** Before writing a new agent prompt, ask: "What role is this person playing on the team? What's their job description?"

**Encode the team you want.** If you want agents to pair-program, encode pair-programming practices. If you want strict code review, encode strict review criteria.

**Test with org theory.** When something isn't working, ask: "Would this team dynamic work with humans?" If the handoff is broken, the handoff is broken—regardless of whether agents or humans are doing it.

**Update the org, not just the code.** When you change agent behavior, you're changing your org design. Be intentional about it.

---

## One Sentence

> The pack is org design, compiled into prompts and flows.

---

## See Also

- [agent-philosophy.md](agent-philosophy.md) — How agents think and work
- [operating-model.md](operating-model.md) — The full operating model
- [architecture.md](architecture.md) — Technical implementation
- [why-ops-first.md](why-ops-first.md) — Default-allow engineering philosophy
