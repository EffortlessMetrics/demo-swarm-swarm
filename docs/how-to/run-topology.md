# Run Topology

> Set up the recommended swarm repo + branch model.

**Goal:** Configure a swarm workspace that runs flows autonomously while keeping your human repo calm.

**Prereqs:** Git basics, a target repo to work with.

---

## The two-repo model

Recommended default:

```
my-project/           # Human workspace (main development)
my-project-swarm/     # Swarm workspace (runs flows, commits freely)
```

Why:

- Swarm activity doesn't interrupt human development
- `.runs/` artifacts are committed and reviewable
- You can open PRs from swarm to origin when ready

---

## Setting up the swarm repo

### Option A: Fork or clone

```bash
# Clone your origin repo
git clone <origin-url> my-project-swarm
cd my-project-swarm

# Copy the pack
cp -r /path/to/demo-swarm/.claude .
```

### Option B: Fresh repo with pack

```bash
mkdir my-project-swarm && cd my-project-swarm
git init

# Copy pack + any base files you need
cp -r /path/to/demo-swarm/.claude .
```

---

## Branch model

The pack uses this branch convention:

| Branch | Purpose |
|--------|---------|
| `main` | Swarm mainline (what Flow 5 merges into) |
| `run/<run-id>` | Per-run working branch |

### How it works

1. `run-prep` / `signal-run-prep` ensures `run/<run-id>` exists
2. Flows checkpoint artifacts to this branch
3. Flow 5 merges `run/<run-id>` → `main` (if Gate verdict is MERGE)

---

## Commit cadence

Every flow commits to preserve the audit trail:

| Flow | What gets committed |
|------|---------------------|
| 1 (Signal) | `.runs/<run-id>/signal/` + index + meta |
| 2 (Plan) | `.runs/<run-id>/plan/` + index + meta |
| 3 (Build) | `.runs/<run-id>/build/` + staged code/tests + index + meta |
| 4 (Gate) | `.runs/<run-id>/gate/` + index + meta |
| 5 (Deploy) | `.runs/<run-id>/deploy/` + merge to main + tags |
| 6 (Wisdom) | `.runs/<run-id>/wisdom/` + index + meta |

Flow 3 has two commit types:
- Checkpoint commit (audit artifacts only)
- Build commit (code + tests)

---

## `.runs/` is Git content

The `.runs/` directory is committed by default. Do not gitignore it.

Size discipline:
- Summaries over raw dumps
- No pasting full issue bodies into artifacts
- Keep artifacts "reviewable diff" sized

---

## Swarm repo autonomy

Key invariant: **the swarm repo is autonomous**.

- Flows run end-to-end in the swarm repo
- Flow 5 merges into `*-swarm/main` (the swarm's mainline)
- The pack does **not** merge into the upstream human repo by default

Upstream export is a separate concern (see [upstream-export.md](upstream-export.md)).

---

## See also

- [upstream-export.md](upstream-export.md) — Moving work to human repo
- [work-without-github.md](work-without-github.md) — Running without `gh`
- [CLAUDE.md](../../CLAUDE.md) — Full pack reference
