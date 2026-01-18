# State and Resumption

> State lives on disk. Every call is an implicit resume.

## The Principle

All state is persisted to disk in `.runs/`. There's no in-memory state that would be lost on restart. Any agent can be interrupted and resumed because everything they need is on disk.

## Why This Matters

### Interruption Is Normal

Long-running operations get interrupted:

- Context limits hit
- User stops and restarts
- Errors occur mid-flow
- Machines restart

If state is in memory, interruption means starting over. If state is on disk, you resume where you left off.

### No "Mode" Flags

Agents don't check "am I in build mode?" They check disk:

- "Does build_receipt.json exist?"
- "What ACs are marked complete in ac_status.json?"
- "What artifacts have been produced?"

State IS the disk contents.

### Collaboration Across Sessions

Multiple sessions can work on the same run:

- Session 1 completes Signal
- Session 2 picks up and runs Plan
- Session 3 finishes Build

They collaborate through shared disk state.

## How It Works

### Run Directory Structure

```
.runs/
├── index.json              # Run registry
└── <run-id>/
    ├── run_meta.json       # Run identity
    ├── signal/
    │   ├── requirements.md
    │   ├── features/*.feature
    │   └── signal_receipt.json
    ├── plan/
    │   ├── adr.md
    │   ├── work_plan.md
    │   └── plan_receipt.json
    ├── build/
    │   ├── impl_changes_summary.md
    │   ├── test_execution.md
    │   └── build_receipt.json
    └── ...
```

### Run Identity

`run_meta.json` tracks identity:

```json
{
  "run_id": "feat-auth-abc123",
  "canonical_key": "gh-456",
  "issue_number": 456,
  "task_title": "Add user authentication",
  "created_at": "2024-01-15T10:00:00Z",
  "flows_started": ["signal", "plan", "build"]
}
```

### Index Registry

`index.json` tracks all runs:

```json
{
  "runs": [
    {
      "run_id": "feat-auth-abc123",
      "status": "VERIFIED",
      "last_flow": "build",
      "updated_at": "2024-01-15T12:00:00Z"
    }
  ]
}
```

## The Implicit Resume

### Every Call Checks Disk

When an agent starts:

1. Check what artifacts exist
2. Determine what's been done
3. Pick up where things left off

No explicit "resume" command needed.

### Example: Build Flow Resume

```
User: /flow-3-build feat-auth

Orchestrator checks:
- .runs/feat-auth/build/ exists? Yes
- build_receipt.json exists? No (incomplete)
- ac_status.json shows AC-001 complete, AC-002 in progress

Action: Resume from AC-002
```

### Example: Cleanup Agent Resume

```
Cleanup agent starts

Checks:
- Which artifacts exist?
- What needs to be counted?
- What's already in the receipt?

Acts: Completes whatever is unfinished
```

## Receipts as Checkpoints

### What Receipts Capture

Receipts summarize flow completion:

- What ran
- What was found
- Counts and metrics
- Status assessment

### Receipt Presence = Flow Complete

```
signal_receipt.json exists → Signal completed
plan_receipt.json exists → Plan completed
build_receipt.json exists → Build completed
```

No receipt = flow incomplete or never started.

### Partial Receipts

If a flow is interrupted mid-receipt-write:

- Next run detects incomplete receipt
- Cleanup agent re-runs to complete it
- Artifacts are re-examined

## Run Identity

### The Problem

A run might be known by many names:

- Issue #456
- Branch `feat/auth`
- Run ID `feat-auth-abc123`
- PR #789

### The Solution

`canonical_key` is the authoritative identity. `aliases` track all names:

```json
{
  "run_id": "feat-auth-abc123",
  "canonical_key": "gh-456",
  "aliases": ["feat-auth-abc123", "gh-456", "feat/auth"]
}
```

### Run Directories Never Rename

If identity changes (issue linked to PR), we update:

- `canonical_key` in run_meta.json
- `aliases[]` in run_meta.json

We do NOT rename the run directory.

## Recovering from Failures

### Agent Failure Mid-Flow

Agent crashes after writing some files:

1. Next invocation checks disk
2. Sees partial state
3. Completes remaining work
4. Writes receipt when done

### Disk Corruption

Some file is corrupted:

1. Agent detects unreadable file
2. Reports CANNOT_PROCEED
3. Lists missing_required
4. Human fixes file
5. Re-run succeeds

### Lost Artifacts

Required upstream artifact missing:

1. Agent checks for artifact
2. Notes absence in handoff
3. Proceeds with assumption OR
4. Bounces to produce missing artifact

## State Ownership

### Who Writes What

| State            | Owner                     |
| ---------------- | ------------------------- |
| run_meta.json    | run-prep, signal-run-prep |
| index.json       | cleanup agents            |
| \*\_receipt.json | cleanup agents            |
| ac_status.json   | build-cleanup             |
| Artifacts        | Worker agents             |

### Exclusive Ownership

Each file has one writer:

- Cleanup agents own receipts
- Worker agents own artifacts
- Prep agents own run metadata

No conflicts. Clear ownership.

## Best Practices

### For Agent Prompts

Tell agents to check disk first:

```markdown
## On Start

Check if your artifacts already exist.
If partially complete, resume from where you left off.
```

### For Flow Commands

Tell orchestrators state is on disk:

```markdown
## On Rerun

Read flow_plan.md for navigation state.
Check receipts to see what's complete.
Route based on current disk state.
```

### For Debugging

State is inspectable:

```bash
cat .runs/feat-auth/build/build_receipt.json
ls .runs/feat-auth/build/
```

Everything is files. Everything is readable.

## See Also

- [Stateless Execution](stateless-execution.md) — Why each flow is a fresh context window
- [The Gate Pattern](principles/gate-pattern.md) — Boundary verification
- [Architecture](architecture.md) — The seven architecture laws
