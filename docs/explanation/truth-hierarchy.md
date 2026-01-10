# Truth Hierarchy

> The physics that governs what the system believes when sources conflict.

---

## The Core Rule

> **When sources conflict, trust flows downward: kernel > derived > intent > binary > narrative.**

This is not a preference. It is the epistemology of the pack. Every agent, flow, and gate operates under this hierarchy. Violating it creates "process confabulation"—agents claiming success without evidence.

---

## The 5 Layers

### Layer 1: Kernel Truth (Absolute)

**What it is:** The raw output of the operating system, filesystem, and tools.

| Source | Examples |
|--------|----------|
| Filesystem | File exists/doesn't exist, file contents |
| Git | `git status`, `git diff`, `git rev-parse HEAD` |
| Tool exit codes | `pytest` returns 0 or 1, `npm test` exits with code |
| CI status | GitHub Actions check run state |
| Shell execution | Command output, stderr, return code |

**This layer cannot be argued with.** The OS is always right. When an agent claims tests passed but `echo $?` shows exit code 1, the exit code wins.

**Example:**
```bash
# Agent claims: "All tests pass"
# Kernel truth:
pytest tests/ --tb=short
# Exit code: 1
# FAILED tests/test_auth.py::test_login_invalid_password
```

The kernel says there's a failure. The agent's claim is invalidated.

### Layer 2: Derived Truth (Mechanical)

**What it is:** Counts and metrics produced by deterministic tools operating on kernel outputs.

| Source | Examples |
|--------|----------|
| grep/wc pipelines | `grep -c "FAIL" test_report.md` |
| jq queries | `jq '.test_count' results.json` |
| Receipts | `build_receipt.json` summarizing tool outputs |
| Markers | `REQ-001`, `NFR-PERF-001` counts extracted by regex |

**The shim pattern:** We force agents to ask the kernel rather than narrate. The `demoswarm` CLI and `.claude/scripts/` shims ensure deterministic extraction.

**Why derived, not kernel?** These outputs depend on:
1. The tool running correctly (kernel truth)
2. The regex/query being correct (potential human error)
3. The file existing and being readable (kernel truth)

**Example:**
```bash
# Derived truth:
bash .claude/scripts/demoswarm.sh count --file ".runs/foo/build/test_execution.md" --regex "FAIL"
# Output: 5

# This means there are 5 failures. Not "approximately 5" or "probably 5".
# But if the regex is wrong, the count is wrong.
```

### Layer 3: Intent Truth

**What it is:** What we meant to build. The "source code" in the "code as binary" metaphor.

| Source | Examples |
|--------|----------|
| BDD scenarios | `features/*.feature` |
| ADRs | `plan/adr.md` |
| API contracts | `plan/api_contracts.yaml` |
| Requirements | `signal/requirements.md` |
| AC matrix | `plan/ac_matrix.md` |

**This layer is authoritative for design intent.** When implementation diverges from intent, we ask: was the intent wrong, or is the implementation wrong? The intent layer doesn't self-verify—it requires comparison with kernel/derived layers.

**Example:**
```gherkin
# Intent (BDD scenario):
Scenario: User logs in with valid credentials
  Given a registered user "alice"
  When alice logs in with correct password
  Then the response status is 200

# If the implementation returns 201, which is wrong?
# Check the intent: ADR says "200 for successful auth"
# The implementation is wrong, not the intent.
```

### Layer 4: Binary Output

**What it is:** The implementation code itself—what we actually built.

| Source | Examples |
|--------|----------|
| Source code | `src/**/*.ts`, `lib/**/*.py` |
| Test code | `tests/**/*.test.ts` |
| Config files | `package.json`, `pyproject.toml` |
| Generated code | Build outputs, compiled assets |

**Treated like compiled output.** We inspect it when needed, but we don't pretend line-by-line review scales. The binary layer is checked *against* higher layers (intent, derived, kernel), not trusted in isolation.

**Example:**
```typescript
// Binary (implementation):
async function login(email: string, password: string): Promise<Response> {
  const user = await db.findUser(email);
  if (user && await bcrypt.compare(password, user.passwordHash)) {
    return { status: 200, body: { token: generateJWT(user) } };
  }
  return { status: 401, body: { error: "Invalid credentials" } };
}

// This looks correct. But does it match intent? Does it pass kernel verification?
// The code itself is not proof. Tests running and passing is proof.
```

### Layer 5: Ephemeral Narrative

**What it is:** Agent chat, status updates, working notes, prose summaries.

| Source | Examples |
|--------|----------|
| Agent responses | "I implemented the login flow" |
| Chat history | Conversation between orchestrator and agent |
| Status updates | "Tests are passing" |
| Working notes | `impl_changes_summary.md` prose sections |

**Useful for debugging, not for truth.** Narrative helps humans understand what happened, but it is never the source of truth for pass/fail decisions.

**Example:**
```markdown
# Agent says:
"I've implemented all 5 acceptance criteria. Tests are passing and the code is ready for review."

# This is narrative. It might be true. It might be hallucination.
# To verify: check derived truth (test counts), check kernel truth (exit codes).
```

---

## The Review Posture

How humans should engage with each layer:

| Layer | Human Posture |
|-------|---------------|
| **Kernel** | Enforced mechanically. No human review needed—tools verify. |
| **Derived** | Review the receipts. Spot-check the counts. Trust but verify. |
| **Intent** | Primary review target. Do the requirements make sense? Are ACs complete? |
| **Binary** | Spot-check guided by evidence. Don't line-review everything—use critics. |
| **Narrative** | Disposable after the run. Useful for context, not for decisions. |

**The formula:**
- Humans review layers 2 + 3 (derived truth + intent)
- Layer 1 is enforced mechanically (CI, test runners, linters)
- Layer 4 is spot-checked guided by evidence from layers 1-3
- Layer 5 is context, not evidence

---

## Why This Matters

### Prevents Process Confabulation

**The problem:** LLMs under success pressure will claim completion without evidence. They will "remember" that tests passed when they didn't run them. They will assert code is correct because they wrote it.

**The solution:** Force truth verification through the hierarchy. An agent claiming success must be backed by derived truth (receipt shows pass) which must be backed by kernel truth (exit code was 0).

### Creates Predictable Escalation

**The problem:** When information conflicts, agents can get stuck or make arbitrary choices.

**The solution:** A deterministic escalation path. When in doubt, ask the kernel. `git status` always wins over an agent's memory of what was staged.

### Keeps Receipts Honest

**The problem:** Receipts could become narrative ("I think we passed") instead of evidence ("exit code: 0, failures: 0").

**The solution:** Receipts summarize tool outputs (derived truth), not opinions. A receipt that says `test_count: 15, failures: 0` is verifiable. A receipt that says "tests look good" is not.

---

## Conflict Resolution Examples

### Example 1: Agent Claims Success, Kernel Disagrees

```
Narrative (Layer 5):
  Agent: "All tests pass. Ready for review."

Kernel (Layer 1):
  $ pytest tests/ --tb=short
  FAILED tests/test_auth.py::test_session_expiry
  Exit code: 1

Resolution:
  Kernel wins. There is a test failure. Agent claim is invalidated.
  Route to fixer with the specific failure.
```

### Example 2: Derived Count vs Narrative Estimate

```
Narrative (Layer 5):
  Agent: "I addressed all 3 critical issues from the review."

Derived (Layer 2):
  $ grep -c "CRITICAL" review_worklist.md
  5

Resolution:
  Derived wins. There are 5 critical issues, not 3.
  Agent either miscounted or only addressed 3 of 5.
  Route back to complete the remaining 2.
```

### Example 3: Intent vs Implementation Mismatch

```
Intent (Layer 3):
  ADR: "Session tokens expire after 24 hours"

Binary (Layer 4):
  const TOKEN_EXPIRY = 60 * 60 * 1000; // 1 hour

Resolution:
  Intent is authoritative for design decisions.
  Implementation is wrong. Route to fixer with:
  "Token expiry should be 24h per ADR, currently 1h"
```

### Example 4: Receipt Staleness

```
Derived (Layer 2):
  build_receipt.json: { "evidence_sha": "abc123", "test_status": "PASS" }

Kernel (Layer 1):
  $ git rev-parse HEAD
  def456

Resolution:
  Receipt is stale (evidence_sha != HEAD).
  Kernel state (current HEAD) is what matters.
  Re-run verification against current HEAD.
```

### Example 5: Missing Evidence

```
Narrative (Layer 5):
  Agent: "I ran the linter and it passed."

Derived (Layer 2):
  (no lint_report.md exists)

Kernel (Layer 1):
  (no lint command in shell history, no output captured)

Resolution:
  Narrative without kernel/derived backup is unverified.
  Treat as UNVERIFIED. Run the linter now to get kernel truth.
```

---

## Implementation Implications

### For Agents

1. **Don't estimate—ask the kernel.** Run the command, check the exit code, report what happened.
2. **Don't narrate counts—derive them.** Use `grep`, `wc`, `jq` to get actual numbers.
3. **Don't claim without evidence.** If you say tests pass, show the exit code.

### For Cleanup Agents

1. **Verify claims against kernel outputs.** Read `test_execution.md` for actual results.
2. **Detect drift.** If `ac_status.json` says "done" but tests show failures, that's a forensic mismatch.
3. **Report staleness.** If `evidence_sha != HEAD`, mark the receipt as stale.

### For Orchestrators

1. **Route on derived truth.** Use Machine Summary blocks, not prose.
2. **Verify before proceeding.** When a gate matters, confirm kernel truth.
3. **Don't trust narrative for pass/fail.** An agent saying "done" is not the same as evidence showing completion.

### For Critics

1. **Ground critiques in layers 1-3.** "This function lacks error handling" (binary) is weaker than "This function throws on invalid input, but no test covers that path" (derived from test coverage).
2. **Reference intent when applicable.** "ADR requires X, but implementation does Y."

---

## The Hierarchy in Practice

### Merge Decision Flow

```
1. Kernel: CI checks passing? (GitHub API status)
2. Derived: Receipt shows all tests green? (build_receipt.json)
3. Intent: All ACs marked complete? (ac_status.json)
4. Binary: Code looks reasonable? (diff review)
5. Narrative: Agent recommends merge? (only if 1-4 are green)

If any higher layer fails, lower layers don't matter.
CI red + "I think it's fine" = CI red.
```

### Debugging a Failure

```
1. Start at kernel: What did the command actually output?
2. Check derived: Does the receipt match the kernel output?
3. Compare to intent: What were we trying to do?
4. Inspect binary: Does the code match the intent?
5. Read narrative: What did the agent think it was doing?

Work top-down. The answer is usually in layers 1-2.
```

---

## See Also

- [trust-model.md](../reference/trust-model.md) — Evidence hierarchy and verification boundaries
- [ai-physics.md](ai-physics.md) — LLM failure modes that motivated this design
- [architecture.md](architecture.md) — Core patterns and laws
- [why-two-planes.md](why-two-planes.md) — Control vs audit plane separation
