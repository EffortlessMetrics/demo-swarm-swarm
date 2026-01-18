# Documentation

Pick docs by intent: **learn**, **do**, **look up**, **understand**, or **maintain**.

---

## Start Here

| Goal                     | Doc                                            |
| ------------------------ | ---------------------------------------------- |
| Run Flow 1 in 5 minutes  | [Quickstart](tutorials/quickstart.md)          |
| Customize for your stack | [Customize Pack](how-to/customize-pack.md)     |
| Understand the design    | [Architecture](explanation/architecture.md)    |
| Recover from failures    | [Failure Recovery](how-to/failure-recovery.md) |

---

## By Category (Diataxis)

| If you want to...                   | Go here                              |
| ----------------------------------- | ------------------------------------ |
| Learn by following steps            | [Tutorials](tutorials/README.md)     |
| Accomplish a specific task          | [How-to](how-to/README.md)           |
| Look up schemas or enums            | [Reference](reference/README.md)     |
| Understand why things work this way | [Explanation](explanation/README.md) |
| Maintain or release the pack        | [Maintainers](maintainers/README.md) |

---

## Quick Links

- [README.md](../README.md) — Pack overview
- [CLAUDE.md](../CLAUDE.md) — Canonical reference (contracts, flows)
- [Documentation conventions](reference/documentation-conventions.md)
- `bash .claude/scripts/pack-check.sh` — Validate pack

---

## Ownership

| What              | Where                        | Who                                       |
| ----------------- | ---------------------------- | ----------------------------------------- |
| Flow behavior     | `.claude/commands/flow-*.md` | Pack maintainers                          |
| Agent behavior    | `.claude/agents/*.md`        | Pack maintainers                          |
| Shared invariants | `CLAUDE.md`                  | Pack maintainers                          |
| Human docs        | `docs/*`                     | Anyone (must not conflict with canonical) |
