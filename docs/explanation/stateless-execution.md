# Stateless Execution Model

> Why each flow is a fresh context window and how state transfers between flows.

---

## The Core Principle

**Each flow is a fresh context window.**

When you run `/flow-2-plan`, the orchestrator doesn't remember what happened in `/flow-1-signal`. It starts with zero context and reads from disk.

This is not a limitation—it's the foundation of the pack's reliability.

---

## Why Stateless?

### 1. Token Economics

**The Problem:** Carrying context forward is expensive.

If Flow 2 inherited Flow 1's context:
- Flow 1: 50K tokens (requirements analysis)
- Flow 2: 50K (Flow 1) + 80K (plan generation) = 130K tokens
- Flow 3: 130K (Flow 1+2) + 200K (build) = 330K tokens
- Flow 4: 330K (Flow 1+2+3) + 150K (review) = 480K tokens

By Flow 4, you're paying for Flow 1's requirement analysis tokens **again**, even though that work is complete.

**The Solution:** Each flow starts fresh.

- Flow 1: 50K tokens → writes artifacts
- Flow 2: 80K tokens (reads artifacts from Flow 1)
- Flow 3: 200K tokens (reads artifacts from Flow 1+2)
- Flow 4: 150K tokens (reads artifacts from Flow 1+2+3)

Total: 480K tokens (same output, no compound tax).

But more importantly: **no degradation**. Flow 4 sees the same quality inputs as if you'd run it standalone.

### 2. Reliability (No Telephone Game)

**The Problem:** Context drift compounds.

```
User: "Add OAuth2 login"
  ↓
Flow 1: "User wants OAuth2 with Google and GitHub providers"
  ↓
Flow 2: "Requirements specify OAuth2, Google, GitHub, and Apple"
  ↓
Flow 3: "Implementing OAuth2 for all major providers including Microsoft"
```

Each flow adds details that weren't there. By Flow 3, you're building something the user didn't ask for.

**The Solution:** Sealed stations.

```
User: "Add OAuth2 login"
  ↓
Flow 1: Writes requirements.md: "OAuth2 login (provider TBD)"
  ↓
Flow 2: Reads requirements.md fresh → "OAuth2 login (provider TBD)"
        Writes adr.md: "Use Google as first provider"
  ↓
Flow 3: Reads requirements.md + adr.md fresh → "OAuth2 with Google"
```

Each flow reads the **canonical artifact**, not a degraded summary from 10 turns ago.

### 3. Auditability

**The Problem:** Hidden state is unverifiable.

If decisions live only in context history:
- What requirements drove this design?
- Why did we choose this approach?
- What tests were planned?

You have to trust the LLM's "memory" of 200 turns ago.

**The Solution:** State is durable and inspectable.

```
.runs/feat-auth/
  signal/
    requirements.md          ← What we agreed to build
    signal_receipt.json      ← Counts, gates, evidence
  plan/
    adr.md                   ← Why we chose this design
    test_plan.md             ← What tests we committed to
    plan_receipt.json        ← Counts, gates, evidence
  build/
    impl_changes_summary.md  ← What we actually built
    build_receipt.json       ← Counts, gates, evidence
```

Every decision is on disk. You can:
- **Review it** (open the file)
- **Audit it** (git blame, git log)
- **Reproduce it** (rerun from the same inputs)

---

## Two State Machines

The pack uses **two complementary state machines** for flow execution:

### 1. TodoWrite (Ephemeral, Session Navigation)

```yaml
todos:
  - content: "Ensure run directory exists"
    status: completed
    activeForm: "Ensuring run directory exists"
  - content: "Write requirements document"
    status: in_progress
    activeForm: "Writing requirements document"
  - content: "Generate signal receipt"
    status: pending
    activeForm: "Generating signal receipt"
```

**Properties:**
- Lives only in the current session
- Updates in real-time as work progresses
- Helps the orchestrator track "where am I in the flow?"
- Dies when the flow ends

**Purpose:** Navigation. "What's the next station?"

### 2. flow_plan.md (Durable, On-Disk State)

```markdown
# Flow 2 Plan

## Stations
- [x] run-prep
- [x] adr-author
- [ ] contracts-author
- [ ] test-planner
- [ ] plan-cleanup

## Status: in_progress
## Last Station: adr-author
## Next Station: contracts-author
```

**Properties:**
- Written to `.runs/<run-id>/plan/flow_plan.md`
- Committed to git
- Survives session death
- Used for reruns and debugging

**Purpose:** Persistence. "If this flow crashes, where do I resume?"

### Why Both?

- **TodoWrite** = fast, ephemeral, good for UX ("here's what's happening now")
- **flow_plan.md** = durable, auditable, good for resilience ("here's where we were")

They complement each other. TodoWrite is the real-time dashboard; flow_plan.md is the black box recorder.

---

## State Transfer Between Flows

Flows communicate via **artifacts on disk**.

### The Contract

Each flow has:
- **Input expectations** (what artifacts it needs to read)
- **Output commitments** (what artifacts it will write)

| Flow | Reads | Writes |
|------|-------|--------|
| Signal | User input | `requirements.md`, `features/*.feature`, `signal_receipt.json` |
| Plan | Signal outputs | `adr.md`, `test_plan.md`, `plan_receipt.json` |
| Build | Plan outputs | Code/tests, `build_receipt.json`, Draft PR |
| Review | Build outputs + PR | `pr_feedback.md`, `review_receipt.json` |
| Gate | Review outputs | `merge_decision.md`, `gate_receipt.json` |
| Deploy | Gate outputs | `deployment_log.md`, `deploy_receipt.json` |
| Wisdom | All outputs | `learnings.md`, `wisdom_receipt.json` |

### Out-of-Order Execution

Flows **can** run out of order. The pack adapts:

```bash
# Skip Signal, go straight to Plan
/flow-2-plan "build user auth"
```

Result:
- Plan sees: No `signal_receipt.json`, no `requirements.md`
- Plan does: Makes assumptions, writes `plan_receipt.json` with `status: UNVERIFIED`
- Plan documents: `missing_required: ["requirements.md"]` in receipt

The flow **proceeds best-effort** and documents what's missing. This is intentional: sometimes you want to prototype a design without full requirements.

### The Handoff Protocol

When Flow N completes:

1. **Cleanup agent** writes receipt (mechanical counts, quality gates)
2. **Receipt** includes `status: VERIFIED | UNVERIFIED | PARTIAL`
3. **Repo-operator** commits artifacts to `.runs/<run-id>/<flow>/`
4. **Flow ends**

When Flow N+1 starts:

1. **Read** artifacts from `.runs/<run-id>/` (prior flows)
2. **Validate** inputs (check for required files)
3. **Document** gaps (`missing_required` in receipt if inputs missing)
4. **Proceed** (best-effort if incomplete, VERIFIED if complete)

---

## Sealed Station Pattern

**Rule:** Agents do not talk to each other via chat history. They read from disk and write to disk.

Each station is **hermetically sealed**:

```
Read Fresh → Do Work → Write State → Die
```

- **No shared memory** with other stations
- **No context inheritance** from prior stations
- **File system is the only communication channel**

**Benefits:**
- Zero context entropy (no degradation through stations)
- Reproducibility (rerun by reading same inputs)
- Testability (mock inputs are just files)

---

## Manual Handoff Workflow

The pack is designed for **human-in-the-loop** execution:

### 1. Run Flow

```bash
/flow-1-signal "add user authentication"
```

Wait for completion.

### 2. Review Outputs

```bash
.runs/feat-auth/signal/
  requirements.md          ← Read this
  signal_receipt.json      ← Check status, counts, gates
  verification_notes.md    ← Assumptions and gaps
```

Human decision:
- **Good?** Proceed to next flow
- **Needs changes?** Edit the artifacts, rerun
- **Wrong direction?** Delete `.runs/feat-auth/`, start over

### 3. Run Next Flow

```bash
/flow-2-plan
```

The orchestrator:
- Finds the run (from `run_id` or current branch)
- Reads Signal outputs from disk
- Proceeds

### 4. Iterate

Repeat for Flow 3, 4, 5, 6, 7.

### Why Manual Handoffs?

1. **Human judgment points**: After each flow, you can steer
2. **Inspection opportunities**: Review artifacts before proceeding
3. **Error correction**: Fix issues early (cheaper than late)
4. **Trust building**: You see what the pack produces at each stage

**Alternative (Auto-Cascade):** You could script `/flow-1-signal && /flow-2-plan && /flow-3-build`, but you'd lose the inspection points. The pack optimizes for **visibility**, not automation.

---

## What This Enables

| Capability | How |
|------------|-----|
| **Resilience** | Flow crashes? Rerun it. Inputs are on disk. |
| **Cherry-picking** | Run Flow 1+2 only, review plan, implement manually. |
| **Human collaboration** | Edit `requirements.md` after Flow 1? Flow 2 sees the fix. |
| **Debugging** | Read `.runs/` artifacts to see exactly what each flow saw and produced. |

The entire state machine is **inspectable**.

---

## The Trade-Offs

### What We Accept

1. **Manual flow transitions**: You run `/flow-2-plan` after reviewing Flow 1 outputs
2. **File system as state**: Artifacts must exist on disk for handoffs to work
3. **No "undo"**: Once a flow commits, you edit files or rerun—no rollback magic

### What We Prevent

1. **Context drift**: Stateless = no telephone game
2. **Token waste**: No compound context tax
3. **Hidden decisions**: Every choice is in a file you can read

---

## Rules for Maintainers

1. **Flows must not assume prior context**
   - If Flow 2 needs `requirements.md`, it reads it from disk
   - Never rely on "the orchestrator knows X from Flow 1"

2. **Agents must not pass state via chat**
   - Write to disk, return a control block, die
   - Next agent reads from disk

3. **Receipts are the handoff contract**
   - `signal_receipt.json` tells Flow 2 what Flow 1 produced
   - Counts, gates, status—all mechanical, all durable

4. **State-first verification**
   - Verify against current repo state (HEAD + working tree)
   - Receipts are evidence logs, not the source of truth

---

## See Also

- [ai-physics.md](ai-physics.md) — Sealed stations, compressors, context affinity
- [why-ops-first.md](why-ops-first.md) — Work Plane vs Publish Plane
- [architecture.md](architecture.md) — Overall pack design
- [why-two-planes.md](why-two-planes.md) — Control plane vs audit plane
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
