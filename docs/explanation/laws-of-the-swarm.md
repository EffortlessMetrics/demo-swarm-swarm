# Laws of the Swarm

> The immutable rules that govern everything.

---

## Purpose

These are the eleven laws. Everything else in the pack derives from or supports these. Violating them breaks the system.

**The laws are not rules to follow. They are physics to respect.**

---

## The Laws

### Law 1: Disk Is Memory

State lives on disk, not in chat. Every call is an implicit resume. Artifacts are the work product.

**Corollary:** If you cannot recover from a session reset, the system is broken.

Agents read `.runs/` to understand what happened. They write `.runs/` to record what they did. The chat log is debugging context, not truth. When the session ends, disk state is all that survives.

**Violation:** An agent that needs to be told "we were working on AC-002" because it cannot find that state on disk.
**Correct:** An agent that reads `ac_status.json`, sees AC-001 is done and AC-002 is pending, and continues.

---

### Law 2: Prose Routes Work

The orchestrator reads handoffs and decides. No parsing. No structured routing blocks. Claude understands language.

**Corollary:** If routing requires parsing YAML, you have built a harness, not a Claude-native system.

Agents report what they did and recommend next steps. Orchestrators read those recommendations and route. The communication channel is natural language throughout. Structured data goes to disk for persistence; prose goes to the orchestrator for decisions.

**Violation:** `Result: { "status": "PROCEED", "next_agent": "fixer", "blockers": [] }`
**Correct:** "I completed the implementation. Three tests pass, but the session cleanup test fails because the migration does not exist yet. Recommend routing to fixer with the migration issue, or deferring to the infrastructure subtask."

---

### Law 3: One Agent, One Job

Each agent has a single responsibility, done deeply. If it needs modes, split it.

**Corollary:** Multi-responsibility agents produce shallow work across all responsibilities.

`code-implementer` writes code. `code-critic` reviews code. `fixer` fixes issues. These are not the same agent with a `mode` flag. Separation ensures focus, enables clear handoffs, and prevents conflicts of interest (like a critic reviewing its own fixes).

**Violation:** `code-agent --mode implement` and `code-agent --mode critique`
**Correct:** `code-implementer` and `code-critic` as separate agents

---

### Law 4: Evidence Over Trust

Claims require pointers. Receipts summarize tool outputs. "Not measured" is acceptable. False certainty is not.

**Corollary:** A claim without evidence is not a claim---it is a hope.

When an agent says "tests pass," there must be a receipt showing exit code 0. When an agent says "all requirements satisfied," there must be an AC matrix with pointers. Uncertainty is fine when labeled; certainty without evidence is the failure mode.

**Violation:** "I ran the tests and they all pass." (no evidence)
**Correct:** "Tests pass (see `test_execution.md`: 15 passed, 0 failed, exit code 0)."

---

### Law 5: Fix Forward by Default

"Blocked" is almost always just routing to another agent. True halting is very rare.

**Corollary:** If you say "blocked," you should usually say "routing to X" instead.

### The Reality of "Blocked"

| What People Say | What Actually Happens |
|-----------------|----------------------|
| "Blocked on lint" | Route to auto-linter |
| "Blocked on test failure" | Route to fixer |
| "Blocked on missing import" | Route back to code-implementer |
| "Blocked on design conflict" | Route to design-optioneer |
| "Blocked on unclear spec" | Route to clarifier |

**These are not blocks. They are routing decisions.**

### True Halting (Very Rare)

True halt requires human intervention or external action:

1. **Mechanical failure** — Tooling broken, permissions missing, infra down
2. **Non-derivable decision** — Business choice that can't be inferred from codebase
3. **Unsafe publish boundary** — Secrets detected, must be remediated before continuing

Even then, work often continues in parallel while waiting for resolution.

**Violation:** "BLOCKED: Code style does not match conventions."
**Correct:** "Routing to auto-linter to fix style issues."

**When true halt is appropriate:**
- Mechanical failure (file system permissions, network unavailable)
- Non-derivable decision (two valid designs, human must choose)
- Unsafe boundary (would expose secrets, requires human approval)

---

### Law 6: Gate at Boundaries

Default-allow engineering inside the workspace. Gates engage at publish boundaries only.

**Corollary:** Gates everywhere creates permission theater, not safety.

Agents can read any file, write any code, run any test. No permission checks inside the workspace. Gates engage when crossing boundaries: commit, push, GitHub post. This separation prevents "security theater" where agents spend more time proving they are allowed to act than actually acting.

**The boundaries:**
- Commit: secrets-sanitizer scans staged changes
- Push: repo-operator checks for anomalies
- GitHub post: content restrictions apply

**Violation:** "Before reading this file, let me check if I have permission..."
**Correct:** Read the file. Gates engage only when publishing.

---

### Law 7: Local Resolution First

Before bouncing flows, try 2-3 targeted specialist calls. Bounce only when truly necessary.

**Corollary:** Bouncing is expensive. Microloops are cheap.

When an agent hits a logic gap, design contradiction, or implementation snag: call a reasoning agent within the current flow. Route to `design-optioneer`, `adr-author`, or `impact-analyzer` for a surgical fix. Re-plan locally. Resume. Bounce only when the specialists agree the entire architecture is invalid.

**Violation:** "This does not match the ADR. BOUNCE to Plan flow."
**Correct:** "This does not match the ADR. Calling design-optioneer to propose a local amendment."

---

### Law 8: Truth Flows Downward

Tool outputs > derived facts > intent > implementation > narrative. When sources conflict, trust flows down.

**Corollary:** An agent's claim does not override tool output.

The hierarchy:
1. **Tool outputs:** Exit codes, stdout, test results (what actually happened)
2. **Derived facts:** Counts and metrics extracted from outputs
3. **Intent:** BDD scenarios, ADRs, contracts (what we meant to build)
4. **Implementation:** Code (what we actually built)
5. **Narrative:** Agent chat, status updates (useful for reasoning, not for truth)

**Violation:** "The agent said tests passed, so we proceed."
**Correct:** "The test runner shows exit code 0 and 0 failures. We proceed."

---

### Law 9: Artifacts Reduce Future Work

Artifacts exist because they are worth reading later. If an artifact has no future reader, do not create it.

**Corollary:** Artifacts for parsers are smell. Artifacts for humans are product.

Every artifact should have a purpose: the next agent needs it for context, the reviewer needs it for audit, or the future developer needs it for understanding. Receipts summarize evidence. ADRs explain decisions. Test reports show what happened. If an artifact exists only because "the process says so," that is a smell.

**Violation:** Generating `process_compliance_log.json` that no one reads.
**Correct:** Writing `impl_changes_summary.md` that the reviewer will scan before approving.

---

### Law 10: The System Improves

Wisdom feeds back into templates. The operating manual evolves. Failures make the factory smarter.

**Corollary:** A system that makes the same mistake twice is not learning.

Flow 7 (Wisdom) extracts learnings from completed runs. Patterns that fail repeatedly get addressed. Templates that produce friction get refined. Agent prompts that cause confusion get clarified. The pack is not static---it evolves based on evidence.

**Violation:** The same type failure occurs across three runs. No change is proposed.
**Correct:** After the second failure, Flow 7 proposes a template update or agent prompt clarification.

---

### Law 11: Keep Going

Flows run to completion. Counts are not exit criteria. Almost everything is routing.

**Corollary:** "3 tries then move on" is a system failure. "3 tries" → run it again.

### What "Keep Going" Means

- **Counts are never completion criteria.** "We've run 3 times" justifies nothing.
- **Counts are never "move on" criteria.** You don't proceed because you're tired of trying.

### Stagnation Is Evidence-Based

**Definition:** no new signal (same failure signature, same evidence, no meaningful diff change).

Response: reroute, don't stop:
- **Same failure, no new signal** → route to a different agent, change approach
- **Oscillation** (toggling between states) → break the cycle by routing differently

Counts alone don't detect stagnation. Evidence does.

The orchestrator's job is to keep things moving. When progress stalls, route to unstick. That's orchestration.

### The Only External Constraints (All Rare)

| Constraint | What It Means | How Rare |
|------------|---------------|----------|
| **Budget** | Tokens, time, or CI minutes exhausted | Occasional |
| **Access** | Tooling broken, permissions missing, infra down | Rare |
| **Authority** | Non-derivable decision with no safe default | Very rare (prefer DEFAULTED + log) |

When an external constraint hits, checkpoint UNVERIFIED and continue when the constraint clears. The flow still runs to completion—it just ends with honest state instead of green evidence.

| Wrong | Right |
|-------|-------|
| "3 tries, moving on" | "3 tries, running again" |
| "Stagnation detected, stopping" | "Stagnation detected, routing to different agent" |
| "Max iterations, proceeding as done" | "Still not converged, change approach" |
| "Timeout, assuming success" | "External constraint hit, checkpointing UNVERIFIED" |

**Violation:** "We tried 3 times, proceeding to Gate anyway."
**Correct:** "3 tries with same failure. Routing to a different agent to unstick."

---

## How to Use These Laws

### As Design Check

When proposing a change, ask:
- Does this violate any law?
- Does this support any law?
- Is the violation justified and documented?

**Example:** "This new agent would both implement and review code." Violates Law 3. Either split into two agents, or document why this exception is necessary.

### As Debugging Tool

When something fails, ask:
- Which law was violated?
- How can the system prevent this violation?

**Example:** An agent claims completion but tests fail later. Law 4 was violated (claim without evidence). The fix: ensure the agent runs tests and includes results in the handoff.

### As Teaching Tool

When explaining the system, reference:
- Which laws govern this behavior?
- Why does this law exist?

**Example:** "Why does the orchestrator not parse the agent's JSON output?" Law 2: Prose routes work. Claude understands language. Parsing YAML is building a harness.

---

## The Laws Are Physics

These are not preferences or style choices. They are the physics that makes the system work.

| Law Violated | What Breaks |
|--------------|-------------|
| Law 1 (Disk Is Memory) | Work disappears on session reset |
| Law 2 (Prose Routes Work) | Routing becomes fragile, requires harness maintenance |
| Law 3 (One Agent, One Job) | Shallow work everywhere, unclear accountability |
| Law 4 (Evidence Over Trust) | Process confabulation, false completions |
| Law 5 (Fix Forward) | Everything blocks, nothing ships |
| Law 6 (Gate at Boundaries) | Permission theater, agents paralyzed by access checks |
| Law 7 (Local Resolution) | Expensive bounces for trivial issues |
| Law 8 (Truth Flows Downward) | Agents override exit codes, hallucinated success |
| Law 9 (Artifacts Reduce Work) | Artifact bloat, noise drowns signal |
| Law 10 (System Improves) | Same failures repeat indefinitely |
| Law 11 (Keep Going) | Early exit, count-based completion, treating routing as stopping |

The laws emerged from failure. They encode what breaks when ignored.

---

## Amendment Process

Laws can evolve, but the bar is high:

1. **Proposed change must be tested.** Run it on real work. Observe the outcomes.
2. **Must improve the system measurably.** Fewer failures, faster cycle time, clearer handoffs.
3. **Must not break existing guarantees.** Laws protect invariants that other laws depend on.

**Process:**
- Propose in Flow 7 (Wisdom) based on evidence from completed runs
- Document the failure mode the amendment addresses
- Test in isolation before pack-wide adoption
- Update all dependent documentation (CLAUDE.md, agent prompts, flow commands)

---

## See Also

- [architecture.md](architecture.md) --- Core philosophy and the seven architecture laws
- [agent-philosophy.md](agent-philosophy.md) --- How agents think and fail gracefully
- [truth-hierarchy.md](truth-hierarchy.md) --- The 5-layer epistemology in detail
- [why-ops-first.md](why-ops-first.md) --- Default-allow engineering philosophy
- [ai-physics.md](ai-physics.md) --- LLM failure modes that motivated this design
