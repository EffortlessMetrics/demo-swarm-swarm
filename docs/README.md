# Documentation

Pick docs by intent (Diataxis): **learn**, **do**, **look up**, **understand**, or **maintain**.

**Guardrails:** Flow behavior and agent contracts live in `.claude/commands/*`, `.claude/agents/*`, and `CLAUDE.md`. Human docs explain and guide; they do not redefine behavior. If anything conflicts, treat `CLAUDE.md` as canonical.

---

## Start Here

- [Quickstart](tutorials/quickstart.md) - run Flow 1 in ~5 minutes
- [Toy Run](tutorials/toy-run.md) - see Flow 1 artifacts without GitHub
- [Validation Run](tutorials/validation-run.md) - exercise Flows 1-4 in a sandbox
- [Customize Pack](how-to/customize-pack.md) - point tests/lint/policy at your stack

---

## Intent Router (Diataxis)

| If you want to... | Go here |
|-------------------|--------|
| Learn by following steps | [Tutorials index](tutorials/README.md) |
| Accomplish a specific task | [How-to index](how-to/README.md) |
| Look up schemas, commands, or enums | [Reference index](reference/README.md) |
| Understand why the pack works this way | [Explanation index](explanation/README.md) |
| Maintain or release the pack | [Maintainers index](maintainers/README.md) |

---

## Quick Navigation

- Pack overview: [../README.md](../README.md)
- Canonical reference: [../CLAUDE.md](../CLAUDE.md) (contracts, flows, invariants)
- One-screen cheatsheet: [../CHEATSHEET.md](../CHEATSHEET.md)
- Repo map: [reference/repo-map.md](reference/repo-map.md)
- Canonical validation: `bash .claude/scripts/pack-check.sh`

---

## Document Ownership

| What | Where | Who changes it |
|------|-------|----------------|
| Flow sequences, gating | `.claude/commands/flow-*.md` | Pack maintainers |
| Agent behavior, outputs | `.claude/agents/*.md` | Pack maintainers |
| Shared invariants | `CLAUDE.md` | Pack maintainers |
| Drift guard | `.claude/scripts/pack-check.sh` | Pack maintainers |
| Human docs | `docs/*` | Anyone (must not conflict with canonical) |
