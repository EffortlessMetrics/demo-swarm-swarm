# Creating a New Flow

This guide explains how to add a new flow to the DemoSwarm pack.

---

## When to Create a Flow

Create a new flow when you have:
- A **distinct phase** in the SDLC (e.g., "Migration Planning" between Plan and Build)
- **Unique station ordering** that doesn't fit existing flows
- **Different gating semantics** (what constitutes "done")

Do NOT create a new flow for:
- Adding an agent to an existing flow (see [add-an-agent.md](add-an-agent.md))
- Customizing behavior (see [customize-pack.md](customize-pack.md))

---

## Flow Rhythm

Every flow follows this rhythm:

```
dispatch → bounded work → compress → checkpoint → exit/resume
```

### The Pattern

1. **Dispatch:** User runs `/flow-n-name` with a run-id
2. **Bounded Work:** Domain agents do their jobs (microloops, harvesting)
3. **Compress:** Cleanup agent derives counts and writes receipt
4. **Checkpoint:** Sanitize → Commit → Push (if allowed)
5. **Exit/Resume:** If context is exhausted, exit with `PARTIAL`; user reruns to continue

### Compressor Stations

Identify the "heavy read → light output" points in your flow. These are natural yield points:

| Station Type | Input | Output | Example |
|--------------|-------|--------|---------|
| **Harvester** | API firehose, logs | `blockers[]`, counts | `pr-feedback-harvester` |
| **Executor** | Test suite, lint rules | pass/fail summary | `test-executor` |
| **Cleanup** | All flow artifacts | receipt JSON | `build-cleanup` |

At each compressor station, it's natural to:
- Change instructions
- Cap work
- Checkpoint/exit with `PARTIAL`
- Resume cleanly later

### Exit Semantics

| Status | Meaning | Rerun Behavior |
|--------|---------|----------------|
| `VERIFIED` | Flow completed with executed evidence | No rerun needed |
| `UNVERIFIED` | Gaps exist; verification incomplete | May rerun to resolve |
| `PARTIAL` | Real progress made; context exhausted | Rerun continues from disk state |
| `CANNOT_PROCEED` | Mechanical failure (IO/tooling) | Fix environment, then rerun |

**Key invariant:** `PARTIAL` is valid for unbounded loops (Flow 4 Review). It means "made progress, didn't finish, safe to resume."

---

## Flow Skeleton

Every flow command lives at `.claude/commands/flow-<n>-<name>.md`.

### Required Frontmatter

```yaml
---
description: Run Flow <n> (<phase name>): <one-line purpose>.
argument-hint: "[optional-run-id] <required inputs>"
---
```

### Required Sections

A flow command must have these sections in order:

```markdown
# Flow <n>: <Name>

You are orchestrating Flow <n> of the SDLC swarm.

## Working Directory + Paths (Invariant)

- All commands run from **repo root**.
- All paths in this doc are **repo-root-relative**.
- Run artifacts live under: `.runs/<run-id>/`
- Flow artifacts live under: `.runs/<run-id>/<flow-name>/`

## Your Goals

- <Goal 1>
- <Goal 2>

## Before You Begin (Required)

### Two State Machines

<Explain TodoWrite + flow_plan.md>

### Suggested TodoWrite Items

<Station list as todo items>

## Agents to Use

<Agent catalog by category>

## Orchestration Outline

<Step-by-step with routing logic>

## Artifact Outputs

<Table of outputs with source agents>

## Status States

<VERIFIED / UNVERIFIED / CANNOT_PROCEED semantics>

## Completion

<When the flow is done>

## Orchestrator Kickoff

### Station order + templates
### TodoWrite (copy exactly)
```

---

## The Two State Machines

Flows use two complementary state machines:

### 1. TodoWrite (Session Navigation)

Ephemeral tracking for the current session:

```markdown
- [ ] run-prep (establish run infrastructure)
- [ ] <agent-1>
- [ ] <agent-2> ↔ <critic-2> (microloop; 2 passes default)
- [ ] <flow>-cleanup (finalize receipt)
- [ ] secrets-sanitizer (publish gate)
- [ ] repo-operator (checkpoint commit)
- [ ] gh-issue-manager (if allowed)
- [ ] gh-reporter (if allowed)
```

### 2. `flow_plan.md` (Durable State)

Written to `.runs/<run-id>/<flow>/flow_plan.md`:

```markdown
# Flow <n>: <Name> Plan for `<run-id>`

## Planned Steps

- [ ] run-prep
- [ ] <agent-1>
- [ ] <agent-2> ↔ <critic-2> (microloop)
- [ ] <flow>-cleanup
- [ ] secrets-sanitizer
- [ ] repo-operator
- [ ] gh-issue-manager
- [ ] gh-reporter

## Progress Notes

<Update as each step completes>
```

**Timing rule:** Create TodoWrite immediately. Write `flow_plan.md` only AFTER run-prep creates `.runs/<run-id>/<flow>/`.

---

## Station Ordering Pattern

Flows follow a consistent structure:

```
┌─────────────────────────────────────────────────────┐
│ 1. SETUP                                             │
│    - run-prep (or signal-run-prep for Flow 1)        │
│    - repo-operator (ensure branch)                   │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│ 2. DOMAIN WORK                                       │
│    - Your flow-specific agents                       │
│    - Microloops (writer ↔ critic)                    │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│ 3. SEALING                                           │
│    - <flow>-cleanup (writes receipt)                 │
│    - secrets-sanitizer (publish gate)                │
│    - repo-operator (checkpoint commit)               │
└─────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────┐
│ 4. REPORTING (optional)                              │
│    - gh-issue-manager                                │
│    - gh-reporter                                     │
└─────────────────────────────────────────────────────┘
```

---

## Microloop Template

For writer ↔ critic stations:

```markdown
#### Microloop Template (writer ↔ critic)

1) Writer pass: call `<writer>`
2) Critique pass: call `<critic>` and read its control-plane Result
3) Apply pass: call `<writer>` once using the critic's worklist
4) Re-critique: call `<critic>` again

Continue looping beyond default two passes only when:
- critic returns `recommended_action: RERUN`, and
- `can_further_iteration_help: yes`, and
- the critic's open items are specific

Otherwise proceed with `UNVERIFIED` + blockers recorded.
```

---

## Control-Plane Blocks

Flows route on control-plane blocks, NOT by re-reading files.

### Gate Result (from secrets-sanitizer)

```yaml
## Gate Result
status: CLEAN | FIXED | BLOCKED
safe_to_commit: true | false
safe_to_publish: true | false
modified_files: true | false
findings_count: <int>
blocker_kind: NONE | MECHANICAL | SECRET_IN_CODE | SECRET_IN_ARTIFACT
blocker_reason: <string | null>
```

### Repo Operator Result (from repo-operator)

```yaml
## Repo Operator Result
operation: checkpoint | build | stage | merge | other
status: COMPLETED | COMPLETED_WITH_WARNING | COMPLETED_WITH_ANOMALY | FAILED | CANNOT_PROCEED
proceed_to_github_ops: true | false
commit_sha: <sha>
publish_surface: PUSHED | NOT_PUSHED
anomaly_classification:
  unexpected_staged_paths: []
  unexpected_unstaged_paths: []
  unexpected_untracked_paths: []
anomaly_paths: []
```

### Agent Result (from cleanup agents)

```yaml
## <Flow> Cleanup Result
status: VERIFIED | UNVERIFIED | CANNOT_PROCEED
recommended_action: PROCEED | RERUN | BOUNCE | FIX_ENV
route_to_flow: <1-7 | null>
route_to_agent: <agent-name | null>
missing_required: []
blockers: []
```

---

## Routing Logic Template

Include routing guidance for each major decision point:

```markdown
**Route on the <Agent> Result block** (not by re-reading the file):
- If `status: CANNOT_PROCEED` → **FIX_ENV**; stop and require human intervention
- If `recommended_action: BOUNCE` → follow `route_to_flow`/`route_to_agent`
- If `recommended_action: RERUN` → rerun the specified agent
- If `recommended_action: PROCEED` → continue to next station
```

---

## Cleanup Agent Requirements

Every flow needs a `<flow>-cleanup` agent that:

1. **Verifies artifacts exist** (required vs optional)
2. **Derives counts mechanically** using `demoswarm` shim
3. **Writes the receipt** (`.runs/<run-id>/<flow>/<flow>_receipt.json`)
4. **Updates `.runs/index.json`** (status, last_flow, updated_at only)
5. **Writes cleanup_report.md** (evidence of derivation)
6. **Creates SKIPPED stubs** for missing station artifacts

See [add-an-agent.md](add-an-agent.md) for cleanup agent patterns.

---

## Publish Surface

Define what the flow produces for secrets-sanitizer to scan:

```markdown
## Publish Surface for Flow <n>

- `.runs/<run-id>/<flow>/`
- `.runs/<run-id>/run_meta.json`
- `.runs/index.json`
- <any code/test changes for Build/Review>
```

---

## Receipt Schema

Every flow produces a receipt at `.runs/<run-id>/<flow>/<flow>_receipt.json`:

```json
{
  "run_id": "<run-id>",
  "flow": "<flow-name>",

  "status": "VERIFIED | UNVERIFIED | CANNOT_PROCEED",
  "recommended_action": "PROCEED | RERUN | BOUNCE | FIX_ENV",
  "route_to_flow": null,
  "route_to_agent": null,

  "missing_required": [],
  "missing_optional": [],
  "blockers": [],
  "concerns": [],

  "counts": {
    "<metric>": null
  },

  "quality_gates": {
    "<critic>": "VERIFIED | UNVERIFIED | null"
  },

  "stations": {
    "<station>": { "executed": true, "result": "PASS | FAIL | SKIPPED | UNKNOWN" }
  },

  "evidence_sha": "<current HEAD>",
  "generated_at": "<ISO8601>",

  "key_artifacts": [],
  "completed_at": "<ISO8601>"
}
```

---

## Checklist: Before Merging a New Flow

- [ ] Flow command at `.claude/commands/flow-<n>-<name>.md`
- [ ] Cleanup agent at `.claude/agents/<flow>-cleanup.md`
- [ ] All domain agents created (see [add-an-agent.md](add-an-agent.md))
- [ ] Flow registered in `settings.json` (if using skills)
- [ ] `CLAUDE.md` updated with:
  - [ ] Flow in "The Seven Flows" table
  - [ ] Receipt path in "Receipts" table
  - [ ] Publish surface in "Publish Surface" table
- [ ] Pack-check passes: `bash .claude/scripts/pack-check.sh`
- [ ] Test run completed successfully

---

## Example: Flow 2.5 (Database Migration)

A hypothetical example:

```yaml
---
description: Run Flow 2.5 (Database Migration): design migration strategy and rollback plan.
argument-hint: "[run-id]"
---
```

Station order:
1. `run-prep` (establish `.runs/<run-id>/migration/`)
2. `migration-analyzer` (analyze current schema + changes)
3. `migration-planner` ↔ `migration-critic` (microloop)
4. `rollback-designer` (design rollback strategy)
5. `migration-cleanup` (seal receipt)
6. `secrets-sanitizer` (publish gate)
7. `repo-operator` (checkpoint)
8. `gh-issue-manager` / `gh-reporter` (if allowed)

Outputs:
- `migration_plan.md`
- `rollback_strategy.md`
- `migration_receipt.json`

---

## See Also

- [add-an-agent.md](add-an-agent.md) — How to add agents to flows
- [architecture.md](../explanation/architecture.md) — Design patterns
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
