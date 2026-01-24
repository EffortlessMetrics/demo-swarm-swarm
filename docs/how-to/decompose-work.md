# How to Decompose Work into Agent Tasks

> Breaking feature requests into single-responsibility agent work.

---

## Before You Start

### The Core Question

When you have a feature request, ask: **"What discrete jobs need doing, and who does each one?"**

This guide helps you slice work into agent-sized pieces that are:

- **Focused** — one responsibility per agent
- **Bounded** — clear success criteria
- **Evidence-backed** — tests or checks to verify completion
- **Fail-graceful** — legitimate exits when blocked

### Prerequisites

- You understand the seven flows (Signal → Plan → Build → Review → Gate → Deploy → Wisdom)
- You know the difference between orchestrators and agents
- You've read [agent-philosophy.md](../explanation/agent-philosophy.md)

---

## The Org Chart Mapping

Think of the swarm as an org structure:

| Role                  | Maps To           | Responsibility             | Decision Authority     |
| --------------------- | ----------------- | -------------------------- | ---------------------- |
| **PM / Dispatcher**   | Orchestrator      | Scope, sequence, route     | What to do next        |
| **ICs (Workers)**     | Worker agents     | Execute discrete work      | How to implement       |
| **Staff Review**      | Critics + Cleanup | Quality gates, compression | Whether work meets bar |
| **Change Management** | Gate agents       | Boundary control           | Whether to ship        |

### Orchestrators (PMs)

Orchestrators **route and sequence**. They:

- Read artifacts and understand context
- Decide which agent handles what
- Route based on agent Result blocks
- Manage the TodoWrite checklist

Orchestrators **do not**:

- Write code or tests
- Parse files for routing decisions
- Do work that should be delegated

### Worker Agents (ICs)

Worker agents **do focused work**. They:

- Execute one specific type of task
- Report back honestly (including partial progress)
- Recommend next steps
- Update their own tracking artifacts

Workers **do not**:

- Route to other agents
- Make cross-cutting decisions
- Do work outside their specialty

### Critics and Cleanup (Staff Review)

Critics and cleanup agents **compress and verify**. They:

- Review work harshly but fairly
- Summarize large contexts into small signals
- Verify claims match evidence
- Produce receipts and worklists

Critics **never fix**. They report issues for workers to address.

### Gate Agents (Change Management)

Gate agents **control boundaries**. They:

- Read evidence and decide go/no-go
- Enforce publish-time constraints (secrets, hygiene)
- Produce decision documents

Gates engage at **publish boundaries**, not during internal iteration.

---

## Two Reasons to Spawn an Agent

Before spawning an agent, ask: **Why?**

### Reason 1: Work Needs Doing

The task requires focused expertise:

- `code-implementer` writes code that satisfies ACs
- `test-author` writes tests that verify behavior
- `requirements-author` shapes vague ideas into testable specs

**Spawn a worker when:** Implementation work is required and would benefit from focused attention.

### Reason 2: Context Needs Compressing

The conversation has accumulated too much context:

- `build-cleanup` compresses flow artifacts into a receipt
- `code-critic` distills a codebase into "what's wrong and where"
- `pr-feedback-harvester` compresses GitHub API firehose into actionable blockers

**Spawn a compressor when:** The orchestrator needs a summary to continue routing effectively.

### That's It

No other reasons. If a task doesn't fit either category:

- Maybe the orchestrator can handle it directly
- Maybe it's mechanical work for a skill (not an agent)
- Maybe it's not a task at all (just routing)

---

## The SRP Decomposition Pattern

Start with a feature request. Slice it into jobs by asking: **"What distinct types of work need doing?"**

### A. Contract Shaping (Signal)

**Job:** Turn vague ideas into testable contracts.

| Agent                 | Work                                 |
| --------------------- | ------------------------------------ |
| `requirements-author` | Write requirements with atomic ACs   |
| `requirements-critic` | Review for testability, completeness |
| `bdd-author`          | Write Gherkin scenarios              |
| `bdd-critic`          | Review scenarios for coverage        |

**Output:** `requirements.md`, `features/*.feature`, `open_questions.md`

**Evidence lane:** Can a stranger write deterministic tests from these requirements without asking follow-ups?

### B. Design Constraint Definition (Plan)

**Job:** Make architectural decisions and define boundaries.

| Agent                | Work                                       |
| -------------------- | ------------------------------------------ |
| `design-optioneer`   | Propose design options with trade-offs     |
| `option-critic`      | Review options for feasibility             |
| `adr-author`         | Capture decisions in ADR format            |
| `interface-designer` | Define API contracts                       |
| `work-planner`       | Break work into subtasks with dependencies |

**Output:** `adr.md`, `api_contracts.yaml`, `work_plan.md`

**Evidence lane:** Does the ADR answer the key architectural questions? Are contracts machine-parseable?

### C. Implementation per Acceptance Chunk (Build Microloop)

**Job:** Implement small slices with test coverage.

| Agent              | Work                            |
| ------------------ | ------------------------------- |
| `test-author`      | Write tests for the AC          |
| `test-critic`      | Review tests for coverage       |
| `code-implementer` | Write code that satisfies tests |
| `code-critic`      | Review code for correctness     |
| `test-executor`    | Run tests, report results       |

**Output:** Code changes, tests, `impl_changes_summary.md`

**Evidence lane:** Do tests pass? Does code satisfy the AC?

**Loop pattern:**

```
test-author → test-critic → code-implementer → code-critic → test-executor
                  ↑__________________________________|
                              (if issues found)
```

### D. Critique (Specialist Review)

**Job:** Find issues in specific domains.

| Critic Type       | Focus                                  |
| ----------------- | -------------------------------------- |
| `code-critic`     | Logic, correctness, ADR compliance     |
| `test-critic`     | Coverage, validity, maintainability    |
| `contract-critic` | Interface compliance, breaking changes |
| `security-critic` | Vulnerabilities, credential exposure   |
| `doc-critic`      | Accuracy, completeness                 |

**Output:** `*_critique.md` with worklist

**Evidence lane:** Are issues specific enough for workers to fix without asking follow-ups?

### E. Cleanup (Context Compression)

**Job:** Summarize what happened for downstream consumers.

| Agent            | Work                                                     |
| ---------------- | -------------------------------------------------------- |
| `signal-cleanup` | Compress Signal artifacts into receipt                   |
| `plan-cleanup`   | Compress Plan artifacts into receipt                     |
| `build-cleanup`  | Compress Build artifacts into receipt + PR cockpit draft |
| `review-cleanup` | Compress Review artifacts into receipt                   |
| `gate-cleanup`   | Compress Gate artifacts into receipt                     |

**Output:** `*_receipt.json`, summary documents

**Evidence lane:** Can the next flow start with just the receipt? Does the PR cockpit give reviewers what they need?

### F. Gate (Decision)

**Job:** Decide go/no-go at boundaries.

| Agent               | Work                                     |
| ------------------- | ---------------------------------------- |
| `merge-decider`     | Read evidence, decide merge/bounce       |
| `deploy-decider`    | Read verification, decide deploy/hold    |
| `secrets-sanitizer` | Scan for secrets, decide safe-to-publish |

**Output:** `merge_decision.md`, `gate_receipt.json`

**Evidence lane:** Is the decision traceable to evidence? Are blockers documented?

---

## What Makes an SRP Task "Agent-Safe"

A task is safe for an agent when it has:

### 1. Crisp Success Definition

**Agent-safe:**

> "Write tests for AC-001 (user login). Cover happy path, invalid credentials, and rate limiting."

**Not agent-safe:**

> "Make sure the tests are good."

### 2. Bounded Surface Area

**Agent-safe:**

> "Implement the `/users` POST endpoint per the API contract."

**Not agent-safe:**

> "Clean up the API layer."

### 3. Evidence Lane

**Agent-safe:**

> "Tests in `tests/auth/` should pass. Contract compliance checked by `contract-enforcer`."

**Not agent-safe:**

> "Code should be high quality." (No measurable check)

### 4. Graceful Failure Mode

**Agent-safe:**

> "If the database schema doesn't exist, log the blocker in `open_questions.md` and continue with remaining ACs."

**Not agent-safe:**

> "Complete all work or fail." (No partial completion allowed)

### The Four-Part Checklist

Before assigning a task to an agent, verify:

| Criterion         | Question                             | Bad Sign         |
| ----------------- | ------------------------------------ | ---------------- |
| **Crisp**         | Can I write a test for "done"?       | "Make it better" |
| **Bounded**       | Can I list the files likely touched? | "Fix everything" |
| **Evidenced**     | What check proves completion?        | "Trust me"       |
| **Fail-graceful** | What if this blocks?                 | "Must complete"  |

---

## Example: Feature Decomposition

### Feature Request

> "Add user authentication with JWT tokens"

### Step 1: Signal Jobs

| Task                    | Agent                 | Success Criteria                             |
| ----------------------- | --------------------- | -------------------------------------------- |
| Write auth requirements | `requirements-author` | REQs have atomic ACs; no vague terms         |
| Review requirements     | `requirements-critic` | All REQs are testable                        |
| Write auth scenarios    | `bdd-author`          | Scenarios cover login, logout, token refresh |
| Review scenarios        | `bdd-critic`          | No missing edge cases                        |

### Step 2: Plan Jobs

| Task                           | Agent                | Success Criteria         |
| ------------------------------ | -------------------- | ------------------------ |
| Propose JWT vs session options | `design-optioneer`   | Trade-offs documented    |
| Review options                 | `option-critic`      | Feasibility confirmed    |
| Write auth ADR                 | `adr-author`         | Decision rationale clear |
| Define auth API contract       | `interface-designer` | OpenAPI spec valid       |
| Break into subtasks            | `work-planner`       | Dependency graph valid   |

### Step 3: Build Jobs (per AC)

#### AC-001: User can login with credentials

| Task                  | Agent              | Evidence                                  |
| --------------------- | ------------------ | ----------------------------------------- |
| Write login tests     | `test-author`      | Tests exist in `tests/auth/login.test.ts` |
| Review login tests    | `test-critic`      | Coverage includes happy + error paths     |
| Implement login       | `code-implementer` | Tests pass; contract satisfied            |
| Review implementation | `code-critic`      | No ADR violations                         |
| Verify tests pass     | `test-executor`    | Green CI                                  |

#### AC-002: User can logout

(Same pattern)

#### AC-003: Tokens expire and refresh

(Same pattern)

### Step 4: Cleanup and Gate

| Task               | Agent               | Output                  |
| ------------------ | ------------------- | ----------------------- |
| Seal build receipt | `build-cleanup`     | `build_receipt.json`    |
| Decide merge       | `merge-decider`     | `merge_decision.md`     |
| Scan for secrets   | `secrets-sanitizer` | `safe_to_publish: true` |

### The Full Task Graph

```
Signal:
  requirements-author → requirements-critic
  bdd-author → bdd-critic

Plan:
  design-optioneer → option-critic → adr-author
  interface-designer → contract-critic
  work-planner

Build (per AC):
  test-author → test-critic
  code-implementer → code-critic
  test-executor

Cleanup:
  build-cleanup → secrets-sanitizer → repo-operator

Gate:
  merge-decider → gate-cleanup
```

---

## Common Mistakes

### 1. Tasks Too Broad

**Mistake:**

> "Implement the authentication system"

**Fix:** Break into AC-sized chunks. Each AC is one microloop.

**Why it matters:** Broad tasks have no clear completion criteria. The agent guesses when to stop.

### 2. Tasks Overlap

**Mistake:**

> Agent A: "Write the login code"
> Agent B: "Write the auth module"

Both touch the same files. Who owns `auth/login.ts`?

**Fix:** Slice by acceptance criterion, not by module. One AC = one owner.

**Why it matters:** Overlapping ownership causes merge conflicts and duplicated work.

### 3. No Evidence Lane

**Mistake:**

> "Refactor the database layer for better performance"

How do we know when this is done? "Better" isn't measurable.

**Fix:** Add evidence: "Response time for `/users` under 100ms (load test proof)"

**Why it matters:** Without evidence, critics can't verify and gates can't decide.

### 4. No Graceful Failure

**Mistake:**

> "Complete all 5 ACs before reporting back"

What if AC-3 requires a schema that doesn't exist?

**Fix:** Allow partial: "Complete what you can. Log blockers. Report progress."

**Why it matters:** All-or-nothing forces guessing. Partial progress is honest progress.

### 5. Mixing Responsibilities

**Mistake:**

> "Write the code and review it"

Critics never fix. Authors never critique. Self-review doesn't work.

**Fix:** Separate: `code-implementer` writes, `code-critic` reviews.

**Why it matters:** Separation maintains accountability and catches real issues.

### 6. Mechanical Work in Agents

**Mistake:**

> Spawning an agent to run `npm test` and report results.

**Fix:** Use the `test-runner` skill instead.

**Why it matters:** Agents are for judgment calls. Skills are for deterministic work.

---

## The Decomposition Checklist

Before assigning work to agents:

- [ ] **Each task has one owner** — no overlap
- [ ] **Each task has crisp success criteria** — testable "done"
- [ ] **Each task has an evidence lane** — how to verify
- [ ] **Each task has graceful failure** — what if blocked
- [ ] **Writers and critics are separate** — no self-review
- [ ] **Mechanical work uses skills** — not agents
- [ ] **Context compression is explicit** — cleanup agents seal flows

---

## Quick Reference: Job Types

| Job Type           | Example Agents                   | Output           | Evidence              |
| ------------------ | -------------------------------- | ---------------- | --------------------- |
| **Shaping**        | requirements-author, bdd-author  | Specs, scenarios | Testability review    |
| **Design**         | design-optioneer, adr-author     | ADRs, contracts  | Feasibility review    |
| **Implementation** | code-implementer, test-author    | Code, tests      | Test execution        |
| **Critique**       | code-critic, test-critic         | Worklists        | Specificity of issues |
| **Compression**    | \*-cleanup                       | Receipts         | Downstream usability  |
| **Decision**       | merge-decider, secrets-sanitizer | Go/no-go docs    | Traceability          |

---

## See Also

- [agent-philosophy.md](../explanation/agent-philosophy.md) — How agents think and fail gracefully
- [design-agents.md](design-agents.md) — How to write agent prompts
- [working-with-microloops.md](working-with-microloops.md) — Writer/critic iteration
- [single-responsibility.md](../explanation/principles/single-responsibility.md) — Why one job per agent
- [architecture.md](../explanation/architecture.md) — System design patterns
- [work-plan.md](../examples/work-plan.md) — Example work plan with task markers
