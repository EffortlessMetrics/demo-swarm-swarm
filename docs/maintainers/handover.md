# Maintainer Handover

> If you are taking over this repo, start here.

**What this repo is:** The DemoSwarm `.claude/` SDLC pack.

**What's canonical:** `.claude/commands/flow-*.md` (flows), `.claude/agents/*.md` (agents), `CLAUDE.md` (invariants). Human docs explain and guide—they do not redefine behavior.

---

## Mental Model

Think of agents as **junior developers**: high energy, low context, need specific roles and guardrails.

- A "Generalist" junior gets lost. A "Test Writer" junior succeeds.
- They will delete tests to make builds pass (reward hacking) unless you stop them.
- They forget the big picture easily—that's why state lives on disk, not in chat.

Your job: write the **playbook** (flows) and **guardrails** (repo-operator) that keep this team productive.

The **user** is the tech lead: dispatches flows, skims outputs, answers questions, merges when satisfied.

---

## Quick Sanity Check

```bash
bash .claude/scripts/pack-check.sh
```

If green, pack contracts are intact. See [pack-check.md](../reference/pack-check.md) for what it enforces.

---

## Changing Contracts Safely

Contracts live in `CLAUDE.md` and are enforced by `pack-check.sh`.

When changing a contract:

1. Update the canonical source (`CLAUDE.md`)
2. Update `pack-check.sh` to enforce the new contract
3. Update all agents/commands that use the contract
4. Run `pack-check.sh` to verify alignment
5. Run a validation run to verify behavior
6. Document the change in CHANGELOG.md

Common drift patterns:

| Symptom                     | Likely cause                                            |
| --------------------------- | ------------------------------------------------------- |
| pack-check fails on enum    | Agent uses non-canonical status value                   |
| pack-check fails on heading | Casing mismatch (e.g., "Block" vs "block")              |
| Missing Machine Summary     | Critic/verifier doesn't emit required block             |
| Routing doesn't work        | Agent handoff missing clear recommendation or next step |

---

## The "Feel" Test

When modifying the pack, verify Ops-First behavior is intact:

1. **Typo Fix Test:** Human edits a file while swarm runs → `repo-operator` includes it ("Extras") without crashing
2. **Lazy Agent Test:** Agent deletes a test to pass build → `repo-operator` refuses to push
3. **Broken Config Test:** Flow 3 pushes bad config that fails CI → stops to fix before building more
4. **Nits Test:** Flow 3 ignores style nits (velocity); Flow 4 catches and fixes them (rigor)

These verify: velocity in Work Plane, guardrails only at Publish Plane boundary.

---

## Key Resources

| What            | Where                                                                     |
| --------------- | ------------------------------------------------------------------------- |
| Pack reference  | [CLAUDE.md](../../CLAUDE.md)                                              |
| Architecture    | [architecture.md](../explanation/architecture.md)                         |
| Doc conventions | [documentation-conventions.md](../reference/documentation-conventions.md) |
| Release process | [release-checklist.md](release-checklist.md)                              |
| Validation      | [validation-run.md](../tutorials/validation-run.md)                       |
