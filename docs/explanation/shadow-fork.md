# Shadow Fork

> The sandbox where stochastic generation is safe.

This document explains Physics #3: how the `.runs/` directory creates a contained workspace where agents operate with full autonomy while the main branch remains protected.

---

## The Problem: Stochastic Generation Has Blast Radius

LLMs are non-deterministic. Given the same prompt twice, you may get different code. Sometimes that code is excellent. Sometimes it breaks things. Sometimes it rewrites files that should not have been touched.

If agents operated directly on the main branch, every generation would be a gamble:

- A bad generation could break production
- A hallucinated refactor could touch 50 files
- A completion-pressure artifact could claim success while leaving chaos

The traditional response is to add permission checks everywhere. But permission theater kills velocity (see [why-ops-first.md](why-ops-first.md)).

We need a different architecture: contain the blast radius, not the agent.

---

## What Is the Shadow Fork?

The Shadow Fork is the `.runs/<run-id>/` directory structure. It's a hermetic workspace where all swarm artifacts live until they graduate through the gate.

```
.runs/
├── feat-auth/              # One run's sandbox
│   ├── run_meta.json       # Run identity
│   ├── signal/             # Flow 1 artifacts
│   ├── plan/               # Flow 2 artifacts
│   ├── build/              # Flow 3 artifacts
│   ├── review/             # Flow 4 artifacts
│   ├── gate/               # Flow 5 artifacts
│   ├── deploy/             # Flow 6 artifacts
│   └── wisdom/             # Flow 7 artifacts
└── feat-oauth/             # Another run's sandbox
    └── ...
```

Each run gets its own isolated directory. Artifacts accumulate there through all seven flows. Nothing escapes to the main branch until the gate approves it.

---

## Why It Exists

The Shadow Fork exists to **contain blast radius** while enabling **default-allow engineering**.

| Without Shadow Fork              | With Shadow Fork                     |
| -------------------------------- | ------------------------------------ |
| Every file write is risky        | Writes are contained to `.runs/`     |
| Permission checks everywhere     | Autonomy inside, gates at boundary   |
| Bad generations affect main      | Bad generations are local to the run |
| Recovery requires git archeology | Recovery is `rm -rf .runs/<run-id>`  |

The key insight: the danger of stochastic generation is not in the generation itself, but in the generation escaping to where it can cause harm. Contain the escape path, and generation becomes safe.

---

## What Agents Can Do Inside

Inside the Shadow Fork, agents have god-mode:

| Capability                   | Why It's Safe                                  |
| ---------------------------- | ---------------------------------------------- |
| Read any file in the repo    | Information doesn't leak; reading is not risk  |
| Write any code               | It's in the sandbox; gate checks before escape |
| Run any test                 | Results stay local; evidence gets recorded     |
| Iterate any number of times  | Cheap compute; only final artifact matters     |
| Delete, refactor, experiment | Shadow Fork is disposable; main is untouched   |
| Make mistakes                | Mistakes are cheap when contained              |

This is not "trust the agent." This is "trust the containment." The agent can do anything because the sandbox contains everything.

---

## What Agents Cannot Do

The Shadow Fork has hard boundaries:

| Blocked Action              | Why                                         |
| --------------------------- | ------------------------------------------- |
| Commit secrets              | `secrets-sanitizer` gates the boundary      |
| Push without approval       | `repo-operator` gates GitHub operations     |
| Affect main branch directly | All changes go through staging/gating       |
| Skip the gate               | Gate is the only path from `.runs/` to main |

The boundaries are not prompts or permissions. They are architectural: there is no path from the Shadow Fork to production that doesn't go through the gate.

---

## How Isolation Protects Main

The protection is physical, not procedural:

### 1. Separate Directory Tree

`.runs/` is not a branch of the codebase; it's a parallel artifact tree. Agents work in `.runs/`, not in `src/` directly.

### 2. Explicit Staging

Before anything leaves the Shadow Fork, it must be explicitly staged. The staging step defines what would be published.

### 3. Gate Before Persist

The canonical order is: **Stage, Sanitize, Persist**. The gate inspects exactly what would cross the boundary. Only clean artifacts proceed.

### 4. Main Stays Clean

Until the gate approves and repo-operator executes, main is untouched. A hundred failed generations leave main exactly as it was.

---

## Recovery When Things Go Wrong

Sometimes the Shadow Fork gets corrupted:

- A run goes sideways
- Artifacts become inconsistent
- Experiments leave debris

Recovery is simple:

### For One Run

```bash
rm -rf .runs/<run-id>/
```

Done. The run is gone. Main is unaffected. Start fresh.

### For All Runs

```bash
rm -rf .runs/
```

Nuclear option. All swarm state is gone. Main remains untouched because nothing graduated.

### The Key Point

**Recovery is cheap because containment is real.** You cannot corrupt what you cannot reach. The Shadow Fork is designed to be disposable.

Compare to operating directly on main:

- Recovery requires `git reflog` archaeology
- Some changes might be pushed
- Collaborators might have pulled
- CI might have run on broken code

The Shadow Fork makes these scenarios impossible. Bad generations die in the sandbox.

---

## Relationship to Default-Allow

The Shadow Fork is what makes default-allow engineering viable.

| Principle                       | How Shadow Fork Enables It         |
| ------------------------------- | ---------------------------------- |
| **No permission checks inside** | Containment is the permission.     |
| **Agents work freely**          | Sandbox absorbs all experiments.   |
| **Gates only at boundaries**    | Boundary is `.runs/` to main.      |
| **Velocity over ceremony**      | Iteration is cheap when contained. |

Without the Shadow Fork, default-allow would be reckless. With it, default-allow is smart engineering.

See [why-ops-first.md](why-ops-first.md) for the full philosophy.

---

## The Mental Model

Think of it as a **clean room** in manufacturing:

- Inside the clean room: work happens freely, experiments run, failures are normal
- At the airlock: everything is inspected before leaving
- Outside the clean room: only inspected artifacts exist

The `.runs/` directory is the clean room. The gate is the airlock. The main branch is the outside world that only sees what passed inspection.

Or think of it as **shadow DOM** in web development:

- Shadow DOM encapsulates component internals
- The outside world sees only the composed result
- Internal structure can change without affecting consumers

The Shadow Fork encapsulates generation internals. Main sees only graduated artifacts.

---

## What Happens at Graduation

When a run passes the gate:

1. **Artifacts are staged** from `.runs/<run-id>/` and project code
2. **secrets-sanitizer** scans the staged surface
3. **repo-operator** checks for anomalies
4. **Both gates must pass** for GitHub operations
5. **Artifacts graduate** to committed state
6. **Run metadata updates** with graduation status

The Shadow Fork is not deleted after graduation. It becomes historical evidence of what was verified and when. The `.runs/` directory is committed, creating an audit trail.

---

## See Also

- [the-physics.md](the-physics.md) - All six physics including Shadow Fork
- [boundary-physics.md](boundary-physics.md) - Stage, Sanitize, Persist in detail
- [why-ops-first.md](why-ops-first.md) - Default-allow philosophy
- [run-state.md](../reference/run-state.md) - Directory structure and schemas
- [why-two-gates.md](why-two-gates.md) - The two-gate model at boundaries
