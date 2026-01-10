# Working with Microloops

> How to use the writer ↔ critic pattern effectively.

Microloops are adversarial iterations between writer and critic agents. They're the core quality mechanism in Flows 1-3.

---

## The Pattern

A microloop pairs a **writer** (produces artifacts) with a **critic** (reviews but never fixes):

```
┌─────────┐     artifact     ┌─────────┐
│  Writer │ ───────────────► │  Critic │
└─────────┘                  └────┬────┘
     ▲                            │
     │    critique + worklist     │
     └────────────────────────────┘
```

The orchestrator routes on the critic's **prose handoff**, not by parsing structured fields or re-reading files.

---

## Microloop Pairs by Flow

### Flow 1: Signal
| Writer | Critic | Artifact |
|--------|--------|----------|
| `requirements-author` | `requirements-critic` | `requirements.md` → `requirements_critique.md` |
| `bdd-author` | `bdd-critic` | `features/*.feature` → `bdd_critique.md` |

### Flow 2: Plan
| Writer | Critic | Artifact |
|--------|--------|----------|
| `design-optioneer` | `option-critic` | `design_options.md` → `option_critique.md` |
| `interface-designer` | `contract-critic` | `api_contracts.yaml` → `contract_critique.md` |
| `observability-designer` | `observability-critic` | `observability_spec.md` → `observability_critique.md` |

### Flow 3: Build
| Writer | Critic | Artifact |
|--------|--------|----------|
| `test-author` | `test-critic` | Tests → `test_critique.md` |
| `code-implementer` | `code-critic` | Code → `code_critique.md` |
| `doc-writer` | `doc-critic` | Docs → `doc_critique.md` |

---

## Default Cadence (Bounded)

The default microloop runs 2 passes:

```
1) Writer pass: call writer agent
2) Critique pass: call critic agent, read its prose handoff
3) Writer pass: apply critique worklist (if any; may be no-op)
4) Critique pass: confirm changes addressed
```

**Continue beyond 2 passes only when:**
- The critic's handoff recommends another iteration
- The critic indicates further iteration would help

---

## Routing on the Handoff

After each critic pass, read the **prose handoff** and route based on understanding:

```markdown
## Handoff

**What I found:** <summary of review findings>

**What's left:** <issues that remain, or "nothing">

**Recommendation:** <specific next step with reasoning>
```

### Routing Rules

| What the critic says | Action |
|---------------------|--------|
| "Work is complete, no issues" | **Exit loop** — proceed to next station |
| "Mechanical failure" (IO/permissions/tooling) | **Stop** — fix environment first |
| "This needs to go back to [flow/agent]" | **Bounce** — route as recommended |
| "Recommend another iteration" + "this should help" | **Rerun** — call writer with critique worklist |
| "Another iteration won't help" | **Exit loop** — proceed with blockers recorded |
| "Ready to proceed despite issues" | **Exit loop** — proceed even if unverified |

The orchestrator reads and understands the prose. There is no parsing of `recommended_action` or `can_further_iteration_help` fields.

---

## Example: Test Microloop

```
Orchestrator calls test-author
  → test-author writes tests + test_changes_summary.md
  → Handoff: "Wrote 8 unit tests. Ready for test-critic review."

Orchestrator calls test-critic
  → test-critic reviews tests
  → Handoff: "Found 1 MAJOR issue: missing edge case coverage for null input.
              Recommend running test-author again with this finding.
              Another iteration should resolve this."

Orchestrator calls test-author with critique worklist
  → test-author addresses the finding, updates tests
  → Handoff: "Added null input edge case test. Ready for test-critic review."

Orchestrator calls test-critic again
  → test-critic reviews updates
  → Handoff: "Tests look complete. No blocking issues found.
              Ready for code-implementer."

Orchestrator exits loop, proceeds to code-implementer
```

---

## Loop Bounds

Microloops are **bounded** to prevent infinite iteration:

| Pass Type | Default Limit | When to Extend |
|-----------|---------------|----------------|
| Writer → Critic | 2 full passes | Only when critic says another iteration would help |
| Maximum iterations | 3-4 | Rarely; indicates upstream issue |

**If a loop won't converge:**
1. The critic should say "another iteration won't help" in its handoff
2. Blockers remain documented in the critique
3. Flow proceeds with issues recorded
4. Issues surface in Gate (Flow 5) or Review (Flow 4)

---

## Worklist Management

Critics produce worklists in their Machine Summary:

```yaml
## Machine Summary
blockers:
  - "Missing auth check in POST /users endpoint"
  - "No validation for email format"
concerns:
  - "Consider adding rate limiting (not blocking)"
```

The writer addresses **blockers** (blocking issues). **Concerns** are advisory.

### Worklist Resolution States

| State | Meaning |
|-------|---------|
| Addressed | Writer fixed the issue |
| Deferred | Explicitly logged in Decision Log with rationale |
| Out of scope | Issue requires upstream changes (BOUNCE) |

---

## Decision Log (for deferred worklists)

When you cannot or choose not to resolve a critic worklist, log it:

```markdown
## Decision Log

### 2025-12-22: Deferred contract-critic worklist

**Worklist items deferred:**
- Schema migration for user_roles table

**Rationale:**
Migration requires coordination with existing deployments. Will address in follow-up run.

**Tracking:**
- Issue draft added to feedback_actions.md
```

---

## Anti-Patterns

### 1. Re-reading Files for Routing

**Wrong:**
```
Call critic
Read critique file
Parse for issues
Decide to rerun
```

**Right:**
```
Call critic
Route based on the prose handoff it returns
```

### 2. Infinite Loops

**Wrong:**
```
while critic says issues exist:
    call writer
    call critic
```

**Right:**
```
for pass in [1, 2]:
    call writer
    call critic
    if critic says "complete" or "another iteration won't help":
        break
```

### 3. Parsing Structured Fields for Routing

**Wrong:**
```
if critic.recommended_action == "RERUN":
    call writer again
```

**Right:**
```
Read critic's handoff prose
Understand the recommendation
Route accordingly
```

### 4. Fixing in Critics

Critics **never fix**. If a critic is modifying code/tests/docs, that's a bug. Critics review and produce worklists.

### 5. Skipping Critique Worklists

If a critic recommends another iteration, the writer must address the findings. Don't skip it hoping the next station will fix it.

---

## TodoWrite Tracking

Microloops are tracked as **one todo**:

```
- [ ] test-author ↔ test-critic (microloop)
```

Not:
```
- [ ] test-author (pass 1)
- [ ] test-critic (pass 1)
- [ ] test-author (pass 2)
- [ ] test-critic (pass 2)
```

The microloop completes when the loop exits (VERIFIED or can't help further).

---

## Microloop Template

Use this template for any writer ↔ critic pair:

```markdown
### Microloop: <writer> ↔ <critic>

**Entry:** <preconditions>

**Loop:**
1. Call `<writer>` with context
2. Call `<critic>`, read its prose handoff
3. Route on handoff:
   - Critic says "work is complete" → exit
   - Critic reports mechanical failure → stop (fix environment)
   - Critic recommends upstream routing → bounce as recommended
   - Critic recommends iteration + "this should help" → goto 1 with worklist
   - Critic says "another iteration won't help" → exit (issues recorded)
   - Critic says "ready to proceed" → exit (issues recorded)

**Exit:** When loop terminates, update TodoWrite and flow_plan.md
```

---

## See Also

- [agent-data-flows.md](../reference/agent-data-flows.md) — Producer/consumer relationships
- [agents-index.md](../reference/agents-index.md) — Agent listings
- [contracts.md](../reference/contracts.md) — Machine Summary schema
- [glossary.md](../reference/glossary.md) — Microloop definition
