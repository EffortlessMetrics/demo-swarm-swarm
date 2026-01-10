# Operating Model

> Who we are, how we work, and why it matters.

---

## The Identity

We are a **fix-forward, high-momentum, high-trust, high-verification** system.

We implement **modern org design as code**: PM + IC roles, Teal-style autonomy, clear boundaries, observable evidence. This isn't a metaphor. The pack literally encodes organizational patterns into agent prompts, flow definitions, and verification gates.

**Fix-forward:** The default outcome is progress, not pause. When something breaks, we route to fix it. Blocking is rare and requires mechanical failure, true ambiguity, or unsafe publish conditions.

**High-momentum:** Machine iteration is cheap. We spend tokens to buy verification. The system grinds so humans can skim.

**High-trust:** Agents operate freely within their sandbox. We don't ask permission for every file read or command execution. Trust comes from architecture, not supervision.

**High-verification:** Everything produces evidence. Tests prove behavior. Critics find issues. Receipts summarize what happened. The human reviews evidence, not every line.

---

## The Human Role

**The developer is a senior architect reviewing the work of a team of juniors managed by a PM.**

This is architect mode, not typist mode.

### What the developer does NOT do

- Write every line of code
- Read every line of code
- Manually run every test
- Babysit the chat window
- Approve each file read or command execution
- Micromanage agent decisions

### What the developer DOES do

- **Set intent:** Define the problem statement. Write acceptance criteria. Specify what success looks like.
- **Review evidence:** Read receipts. Check decision memos. Scan the PR cockpit display.
- **Spot-check implementation:** Use hotspots to guide attention. Review high-risk areas. Ignore the mechanical parts.
- **Make ship/no-ship decisions:** The system produces evidence. The human decides.
- **Handle true ambiguity:** When agents hit non-derivable decisions, they escalate. The human resolves.

### The economic logic

**Senior attention is the scarcest resource.**

Every minute a senior developer spends reading boilerplate code or running obvious tests is a minute not spent on architecture decisions, design review, or genuine problem-solving.

The system exists to compress "work that needs doing" into "evidence that needs reviewing." The developer reviews the evidence and makes the call.

---

## The Verification Model

We build code that can be **verified without reading every line**.

### How verification works

| Layer | What It Proves | Who Checks |
|-------|---------------|------------|
| **BDD scenarios** | Intent is captured and testable | Human reviews during Signal |
| **Tests** | Code matches intent | Machine runs, human reviews summary |
| **Mutation testing** | Tests actually exercise the code | Machine runs (when configured) |
| **Critics** | Issues found before humans see them | Agents review each other's work |
| **Receipts** | What happened, with evidence | Human audits when needed |

### What the human reviews

1. **The intent:** Did we build the right thing? (Requirements, acceptance criteria, BDD scenarios)
2. **The evidence:** Did verification actually happen? (Test results, critic findings, receipts)
3. **The hotspots:** Are high-risk areas solid? (Guided spot-checking, not exhaustive reading)

### What the human does NOT review

- Every line of generated code
- Every test assertion
- Every lint fix
- Every format change

**The system produces evidence. The human evaluates evidence.**

---

## Fix-Forward Momentum

The default outcome is **route to fix**, not **block and wait**.

### The routing hierarchy

| Issue Type | Response |
|-----------|----------|
| Lint failures | Route to linter (auto-fix) |
| Test failures | Route to fixer with repro |
| Missing info | Derive from context or ask a crisp question |
| Scope issues | Try local resolution first |
| Design conflicts | Call reasoning agent, fix locally |

### "Blocked" is rare and literal

True blockers are:

1. **Mechanical failure:** Tooling broken, environment unusable, permissions missing
2. **Non-derivable human decision:** True ambiguity that cannot be resolved from codebase patterns
3. **Unsafe publish boundary:** Secrets detected, security gate failed

Everything else: fix forward.

### Why this matters

Stopping costs context. Every time the system pauses for human input, it loses momentum and accumulates overhead. The escalation ladder exists to exhaust machine options before burning human attention:

1. Investigate locally (search code, tests, configs)
2. Derive from evidence (use existing patterns)
3. Default if safe (choose reversible default, document it)
4. Escalate only when boxed in

Most questions are NOT blockers. DEFAULTED (safe reversible default) is the common case.

---

## The Org Model (Teal-ish)

Modern org design applied to agents.

### Autonomy with alignment

Each agent has:

- **Clear mission:** One job, done deeply. code-implementer writes code. code-critic reviews code. No overlap.
- **Clear boundaries:** The sandbox is the constraint, not per-action permissions.
- **Visible evidence:** Everything produces artifacts. Work is observable.
- **Decentralized execution:** Agents make decisions within their domain. They don't wait for approval.

### Short feedback loops

- **Microloops:** author -> critic -> fix -> verify (per AC, not per feature)
- **Checkpoint frequently:** Write to disk early and often. State survives context exhaustion.
- **Iterate cheaply:** Try multiple approaches before escalating.

### Self-management within role

- Each agent owns their domain completely
- Single responsibility enables depth
- Handoffs communicate what happened and what's next
- No committee approval for routine work

### What this looks like in practice

The orchestrator (PM) says: "Implement AC-001."

The code-implementer (IC) reads context, explores the codebase, makes implementation decisions, writes code, runs tests, and reports back: "Implemented login flow. Tests pass. Ready for review."

The PM didn't approve file reads. The PM didn't approve the approach. The PM receives a summary and routes based on it.

---

## The Economic Logic

**Tokens are cheap. Senior attention is expensive.**

### What we spend (cheap)

- Machine iteration (try multiple approaches)
- Evidence generation (tests, critiques, receipts)
- Verification loops (run tests repeatedly)
- Context exploration (read files, search code)

### What we buy (expensive)

- Reduced review time (evidence instead of exhaustive reading)
- Higher confidence (verification before human sees it)
- Faster trust (architecture instead of supervision)
- Fewer surprises (critics catch issues before merge)

### The metric: DevLT

**DevLT (Developer Lead Time):** Minutes of human attention per trusted change.

This is what we optimize. Not lines of code. Not token efficiency. Not agent count.

A change that costs 100K tokens but requires 5 minutes of human review beats a change that costs 10K tokens but requires 30 minutes of human review.

**Optimizing DevLT means:** Generate more evidence, require less reading.

---

## The Trust Architecture

High trust comes from architecture, not faith.

### Four pillars of trust

| Pillar | How It Works |
|--------|-------------|
| **Containment** | Sandbox limits blast radius. Mistakes happen locally. Git is the safety net. |
| **Verification** | Critics review work. Tests prove behavior. Evidence accumulates. |
| **Boundaries** | Publish gates can say no. Secrets scanning. Merge review. Deploy verification. |
| **Observability** | Everything is auditable. Receipts exist. Diffs are visible. Nothing hides. |

### Trust is earned, not assumed

We don't trust agents because they're "AI." We trust them because:

- The sandbox constrains damage
- Critics catch mistakes before humans see them
- Gates verify at boundaries
- Everything leaves a trail

This is the same trust model you'd apply to a capable junior engineer with repo access. Not blind trust. Architectural trust.

### Recovery over prevention

We don't try to prevent all possible mistakes. Instead:

- State is on disk (recoverable)
- Git is the safety net (revertible)
- Human review at boundaries (catchable)

A mistake in the sandbox is cheap. A mistake in production is expensive. Gate accordingly.

---

## The Work/Publish Split

| Plane | Posture | Purpose |
|-------|---------|---------|
| **Work Plane** | Default-allow | Explore, implement, iterate freely |
| **Publish Plane** | Gated | Control what leaves the workspace |

### Work Plane (freedom)

Everything up to staging runs without friction:
- Read any file
- Run any command
- Try any approach
- Iterate as needed

### Publish Plane (gates)

Gates engage only at boundaries:
- **Commit:** Secrets scanning
- **Push:** Repo hygiene check
- **Merge:** Human review
- **Deploy:** Verification gate

This separation prevents security theater. Agents don't spend tokens proving they're allowed to act. They act, and gates verify at boundaries.

---

## The Core Insight

> The system doesn't try to replace senior judgment. It replaces senior toil. Architects architect; the system grinds.

### What this means

The developer's expertise is in:
- Knowing what to build
- Evaluating whether it's right
- Making ship decisions
- Handling genuine ambiguity

The developer's time shouldn't go to:
- Writing boilerplate
- Running obvious tests
- Reading routine code
- Approving mechanical operations

### The analogy

A senior architect doesn't lay bricks. They design the building, review the plans, inspect critical joints, and approve the final structure.

The bricklayers (agents) do the construction. The foreman (orchestrator) coordinates the work. The architect (developer) sets intent and evaluates results.

This pack encodes that division of labor into executable workflows.

---

## One Sentence

> We're a PM + junior team that produces verified work for a senior architect to review and ship.

---

## See Also

- [Architecture](architecture.md) — Technical implementation
- [Agent Philosophy](agent-philosophy.md) — How agents think and work
- [Operational Philosophy](operational-philosophy.md) — Trust and freedom
- [What Makes This Different](what-makes-this-different.md) — Breaking assumptions
- [Why Ops First](why-ops-first.md) — Default-allow engineering
