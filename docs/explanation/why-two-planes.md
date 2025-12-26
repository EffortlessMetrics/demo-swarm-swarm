# Why Two Planes (Control vs Audit)

> Control plane vs audit plane: what they are and why they're separate.

**Note:** This doc is about **Control Plane vs Audit Plane** (how routing works). For **Work Plane vs Publish Plane** (where gates engage), see [Why Ops-First](why-ops-first.md).

---

## The problem

When agents write artifacts and orchestrators need to route:

**Option A: Reread the artifact**
- Parse the prose
- Extract status/routing info
- Risk: Interpretation varies, parsing is fragile

**Option B: Return structured blocks**
- Agent returns a small, machine-readable block
- Orchestrator routes on the block
- Artifact exists for humans and audit

The pack uses Option B.

---

## Two planes defined

### Control plane

Machine-readable blocks returned by agents:

```yaml
## Gate Result
status: CLEAN
safe_to_publish: true
proceed_to_github_ops: true
...
```

Properties:
- Small, structured, deterministic
- Returned in agent response
- Used for routing decisions
- Closed vocabulary (enums)

### Audit plane

Durable files written to `.runs/<run-id>/`:

- `requirements.md`
- `code_critique.md`
- `secrets_scan.md`
- `*_receipt.json`

Properties:
- Rich, contextual, inspectable
- Written to disk
- Used for human review, reruns, debugging
- Can contain prose, code, details

---

## How they interact

```
Agent runs
  ├─→ Writes audit artifacts (files)
  └─→ Returns control block (response)

Orchestrator
  ├─→ Routes on control block
  └─→ Does NOT reread files for routing
```

Example: secrets-sanitizer

1. Scans publish surface
2. Writes `secrets_scan.md` and `secrets_status.json` (audit)
3. Returns `## Gate Result` block (control)
4. Orchestrator routes on Gate Result
5. Later: humans inspect `secrets_scan.md` if needed

---

## Why not just use files?

### Parsing fragility

Prose varies. "Status: Verified" vs "The status is verified" vs "All checks passed (verified)".

Structured blocks have fixed format:
```yaml
status: VERIFIED
```

### Rereading cost

Files might be large. Rereading and parsing adds latency and failure modes.

Control blocks are returned immediately with the response.

### Drift risk

If routing logic parses files, file format becomes a contract. Changes to artifact structure can break routing.

Separating planes lets artifacts evolve without breaking automation.

---

## Examples

### Critic (Machine Summary)

```yaml
## Machine Summary
status: UNVERIFIED
recommended_action: RERUN
route_to_agent: code-implementer
blockers:
  - Missing error handling in auth module
can_further_iteration_help: yes
```

Orchestrator routes on this. The detailed critique in the artifact body is for humans.

### Repo Operator

```yaml
## Repo Operator Result
operation: checkpoint
status: COMPLETED
proceed_to_github_ops: true
commit_sha: abc123
anomaly_paths: []
```

Orchestrator checks `proceed_to_github_ops`. The commit SHA and paths are for audit.

### Receipt

```json
{
  "status": "VERIFIED",
  "recommended_action": "PROCEED",
  "counts": { ... }
}
```

Reporters read receipts (audit). Flow routing already happened via control blocks.

---

## Rules

1. **Orchestrators route on control blocks, not files**
2. **Control blocks use closed vocabularies** (enums, not prose)
3. **Audit artifacts can be rich and detailed**
4. **Don't duplicate routing logic in file parsing**

---

## See also

- [architecture.md](architecture.md) — Overall pack design
- [contracts.md](../reference/contracts.md) — Control-plane block schemas
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
