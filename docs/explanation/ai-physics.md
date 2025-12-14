# The AI Physics of the Pack

> Why the pack is designed this way: sealed stations, microloops, and mechanical truth.

This document explains the **implicit design constraints** that govern the DemoSwarm architecture. These constraints exist to mitigate specific LLM failure modes (hallucination, context drift, laziness) and leverage their strengths (critique, synthesis).

---

## 1. The "Sealed Station" Pattern

**Rule:** Agents do not talk to each other via chat history. They read from disk and write to disk.

### The Problem: Context Entropy
If Agent A passes context to Agent B via conversation history (the "Telephone Game"), details degrade. Agent C (10 turns later) will hallucinate constraints that were dropped 5 turns ago.

### The Solution
We force every agent to be a **Sealed Station**:
1.  **Read Fresh:** Start with zero context. Read `requirements.md` or `plan_receipt.json` fresh from the disk.
2.  **Do Work:** Perform the task.
3.  **Write State:** Commit the result to the Audit Plane (files).
4.  **Die:** The context is discarded. The next agent starts fresh.

**Benefit:** We reset context entropy to zero at every station.

---

## 2. Microloops (Author ↔ Critic)

**Rule:** Never trust an author to critique their own work.

### The Problem: Self-Correction Blindness
LLMs are poor at self-correction in a single pass. They tend to justify their first draft. However, they are **excellent** at critiquing "someone else's" work.

### The Solution
We artificially induce critical distance by splitting the persona:
1.  **Author:** Writes the draft (optimistic).
2.  **Critic:** Reviews the draft against a checklist (pessimistic).
3.  **Loop:** The Author fixes the issues found by the Critic.

**Stop Condition:** The loop ends when the Critic says `can_further_iteration_help: no` or the status is `VERIFIED`.

---

## 3. Mechanical Truth (Null Over Guess)

**Rule:** Agents must not estimate or guess metrics.

### The Problem: The Helpful Assistant
Agents love to be helpful. If a file is missing, they might try to infer what the count *should* be (e.g., "I see 0 tests, but maybe they are implied...").

### The Solution
1.  **Rust Tooling:** We use `demoswarm` CLI for counting (grep/regex), not LLM vibes.
2.  **Explicit Nulls:** We instruct agents to output `null` (not `0`) if a file is missing or unreadable. A `null` in a receipt is a signal to the human (missing data); a guessed `0` is a lie.

---

## 4. The "Two-Plane" Data Model

**Rule:** Separate the routing logic from the human-readable artifact.

| Plane | Artifacts | Purpose | Lifecycle |
| :--- | :--- | :--- | :--- |
| **Control Plane** | `Gate Result`, `Repo Operator Result`, `Machine Summary` | **Routing.** Determines "What happens next?" (Bounce, Merge, Rerun). | Ephemeral (read once, then ignored). |
| **Audit Plane** | `*_receipt.json`, `index.json`, `git_status.md` | **Record.** Determines "What happened?" (History, Compliance). | Durable (committed to git). |

**Crucial Nuance:**
*   Orchestrators route based on the **Control Plane**.
*   Future flows read the **Audit Plane**.
*   **Sync Requirement:** `*-cleanup` agents ensure these two planes match before the flow ends.

---

## 5. Deterministic Tooling

**Rule:** Bash is too creative; use Rust for determinism.

We replaced ad-hoc bash pipelines with the `demoswarm` CLI because:
*   **The "Bash Tax":** `grep` behaves differently on GNU vs BSD. `sed` is a minefield.
*   **The Shim:** `.claude/scripts/demoswarm.sh` ensures we use the same binary logic on Linux, macOS, and Windows.

---

## Summary for Maintainers

1.  **Trust the Shims:** Don't bypass `.claude/scripts/demoswarm.sh`.
2.  **Trust the Gates:** If Flow 4 bounces, don't force-merge. Fix the upstream flow.
3.  **Respect the Planes:** Never let an agent route based on a file read; force it to route based on the Machine Summary block.
