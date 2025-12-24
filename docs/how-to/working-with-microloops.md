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

The orchestrator routes on the critic's **control-plane Result block**, not by re-reading files.

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
2) Critique pass: call critic agent, capture Result block
3) Writer pass: apply critique worklist (if any; may be no-op)
4) Critique pass: confirm changes moved
```

**Continue beyond 2 passes only when:**
- `recommended_action: RERUN` AND
- `can_further_iteration_help: yes`

---

## Routing on the Result Block

After each critic pass, route on the **control-plane Result block**:

```yaml
## <Critic> Result
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
route_to_flow: <1-7 | null>
route_to_agent: <agent | null>
can_further_iteration_help: yes | no
blockers: [...]
```

### Routing Rules

| Condition | Action |
|-----------|--------|
| `status: VERIFIED` | **Exit loop** — proceed to next station |
| `recommended_action: FIX_ENV` | **Stop** — mechanical failure (IO/permissions/tooling) |
| `recommended_action: BOUNCE` | **Bounce** — route to `route_to_flow`/`route_to_agent` |
| `recommended_action: RERUN` + `can_further_iteration_help: yes` | **Rerun** — call writer with critique worklist |
| `recommended_action: RERUN` + `can_further_iteration_help: no` | **Exit loop** — proceed (issues recorded) |
| `recommended_action: PROCEED` | **Exit loop** — proceed even if UNVERIFIED |
| Missing `recommended_action` | Use `can_further_iteration_help` as tie-breaker |

---

## Example: Test Microloop

```
Orchestrator calls test-author
  → test-author writes tests + test_changes_summary.md

Orchestrator calls test-critic
  → test-critic reviews tests, returns Result:
      status: UNVERIFIED
      recommended_action: RERUN
      can_further_iteration_help: yes
      blockers: ["Missing edge case coverage for null input"]

Orchestrator calls test-author with critique worklist
  → test-author addresses blockers, updates tests

Orchestrator calls test-critic again
  → test-critic reviews updates, returns Result:
      status: VERIFIED
      recommended_action: PROCEED

Orchestrator exits loop, proceeds to code-implementer
```

---

## Loop Bounds

Microloops are **bounded** to prevent infinite iteration:

| Pass Type | Default Limit | When to Extend |
|-----------|---------------|----------------|
| Writer → Critic | 2 full passes | Only when `can_further_iteration_help: yes` |
| Maximum iterations | 3-4 | Rarely; indicates upstream issue |

**If a loop won't converge:**
1. The critic should set `can_further_iteration_help: no`
2. Blockers remain in the critique
3. Flow proceeds with UNVERIFIED status
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
Route on returned Result block
```

### 2. Infinite Loops

**Wrong:**
```
while critic.status != VERIFIED:
    call writer
    call critic
```

**Right:**
```
for pass in [1, 2]:
    call writer
    call critic
    if critic.status == VERIFIED or critic.can_further_iteration_help == "no":
        break
```

### 3. Fixing in Critics

Critics **never fix**. If a critic is modifying code/tests/docs, that's a bug. Critics review and produce worklists.

### 4. Skipping Critique Worklists

If a critic returns `RERUN`, the writer must address the worklist. Don't skip it hoping the next station will fix it.

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
2. Call `<critic>`, capture Result block
3. Route on Result:
   - VERIFIED → exit
   - FIX_ENV → stop (mechanical failure)
   - BOUNCE → route to indicated flow/agent
   - RERUN + can_help: yes → goto 1 with worklist
   - RERUN + can_help: no → exit (issues recorded)
   - PROCEED → exit (issues recorded)

**Exit:** When loop terminates, update TodoWrite and flow_plan.md
```

---

## See Also

- [agent-data-flows.md](../reference/agent-data-flows.md) — Producer/consumer relationships
- [agents-index.md](../reference/agents-index.md) — Agent listings
- [contracts.md](../reference/contracts.md) — Machine Summary schema
- [glossary.md](../reference/glossary.md) — Microloop definition
