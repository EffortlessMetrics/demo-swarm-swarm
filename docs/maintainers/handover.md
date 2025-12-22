# Maintainer Handover

> If you are taking over this repo, start here.

**What this repo is:** The DemoSwarm `.claude/` SDLC pack.

**Core philosophy:** Ops-First. Engineering is default-allow; publishing is gated. See [why-ops-first.md](../explanation/why-ops-first.md).

**What's canonical:**
- `.claude/commands/flow-*.md` — Flow sequences and gating
- `.claude/agents/*.md` — Agent behavior and outputs
- `CLAUDE.md` — Shared invariants and control-plane blocks
- `.claude/scripts/pack-check.sh` — Drift guard

Human docs **explain and guide**—they do not redefine behavior.

---

## Quick sanity check

```bash
bash .claude/scripts/pack-check.sh
```

If this is green, the pack contracts are intact.

---

## Where to find things

| What you need | Where it lives |
|---------------|----------------|
| Flow behavior | `.claude/commands/flow-*.md` |
| Agent prompts | `.claude/agents/*.md` |
| Canonical contracts | `CLAUDE.md` |
| Validation details | [pack-check.md](../reference/pack-check.md) |
| Release process | [release-checklist.md](release-checklist.md) |
| Validation history | [validation-log.md](validation-log.md) |
| Stack adaptation | [how-to/customize-pack.md](../how-to/customize-pack.md) |
| Validation run | [tutorials/validation-run.md](../tutorials/validation-run.md) |
| Contracts index | [reference/contracts.md](../reference/contracts.md) |
| Stable markers | [reference/stable-markers.md](../reference/stable-markers.md) |

---

## How releases work

See [release-checklist.md](release-checklist.md) for the full process.

Quick summary:
1. Ensure `pack-check.sh` is green
2. Run a validation run (see [validation-run.md](../tutorials/validation-run.md))
3. Update CHANGELOG.md
4. Tag and release

---

## How to validate

1. **Structural validation:** `bash .claude/scripts/pack-check.sh`
2. **Behavioral validation:** Run the [validation run](../tutorials/validation-run.md)
3. **Record results:** Log in [validation-log.md](validation-log.md)

See [pack-check.md](../reference/pack-check.md) for what pack-check enforces.

---

## Changing contracts safely

Contracts live in `CLAUDE.md` and are enforced by `pack-check.sh`.

When changing a contract:

1. Update the canonical source (`CLAUDE.md`)
2. Update `pack-check.sh` to enforce the new contract
3. Update all agents/commands that use the contract
4. Run `pack-check.sh` to verify alignment
5. Run a validation run to verify behavior
6. Document the change in CHANGELOG.md

Common drift patterns:

| Symptom | Likely cause |
|---------|--------------|
| pack-check fails on enum | Agent uses non-canonical status value |
| pack-check fails on heading | Casing mismatch (e.g., "Block" vs "block") |
| Missing Machine Summary | Critic/verifier doesn't emit required block |
| Routing doesn't work | Field name drift (`route_to` vs `route_to_agent`) |

---

## Non-negotiable contracts (do not break)

These are the pack invariants. Breaking them breaks the pack.

1. **Repo root + paths**: All commands from repo root, all paths repo-root-relative
2. **Status axis**: `VERIFIED | UNVERIFIED | CANNOT_PROCEED` (closed enum)
3. **Routing enum**: `PROCEED | RERUN | BOUNCE | FIX_ENV` (closed enum)
4. **Two gates for GH ops**: `safe_to_publish` AND `proceed_to_github_ops`
5. **Publish surface only**: secrets-sanitizer scans flow surface, not everything
6. **Reseal pattern**: If `modified_files: true`, rerun until false or safe-bail

See `CLAUDE.md` for full documentation of each.

---

## The "Feel" Test

When modifying this pack, verify the "Feel" remains Ops-First:

1.  **The "Typo Fix" Test:** If a human edits a file while the swarm runs, `repo-operator` MUST include it ("Extras") without crashing.
2.  **The "Lazy Agent" Test:** If an agent deletes a test to make the build pass, `repo-operator` MUST refuse to push.
3.  **The "Broken Config" Test:** If Flow 3 pushes a bad config that fails CI, it MUST stop to fix it before building more features.
4.  **The "Nits" Test:** Flow 3 MUST ignore "nits" (comments) to maintain velocity. Flow 4 MUST catch and fix them.

These tests verify the Ops-First philosophy is intact: **velocity in the Work Plane, guardrails only at the Publish Plane boundary.**

---

## See also

- [CLAUDE.md](../../CLAUDE.md) — Pack reference + canonical contracts
- [architecture.md](../explanation/architecture.md) — Ops-First philosophy, compressors, context affinity
- [why-ops-first.md](../explanation/why-ops-first.md) — Work Plane vs Publish Plane
- [ai-physics.md](../explanation/ai-physics.md) — LLM-specific design constraints
- [pack-check.md](../reference/pack-check.md) — What pack-check enforces
- [reference/contracts.md](../reference/contracts.md) — Control-plane blocks index
- [tutorials/validation-run.md](../tutorials/validation-run.md) — Validation run
- [release-checklist.md](release-checklist.md) — Release process
