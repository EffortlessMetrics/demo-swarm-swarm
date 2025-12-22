# Architecture Overview

> How the pack is built and why.

---

## Core Philosophy: Ops-First

The DemoSwarm pack is a **build pipeline with guardrails**, not a guardrail pipeline that sometimes builds.

**The shift:** From "Compliance Engine" (policing the robot) to "Code Conveyor Belt" (empowering the robot to ship).

### Objective

Optimize **Dev Lead Time**: minutes of human attention per trusted change.

### Core Constraint

> Tokens are cheap. Context is finite. Attention is expensive.

This constraint shapes every design decision. We maximize engineering output per context window while minimizing human review burden.

---

## The Two Execution Planes

The pack separates **where work happens** from **where gates engage**:

| Plane | Posture | Purpose | Example |
|-------|---------|---------|---------|
| **Work Plane** | Default-allow | Explore, implement, iterate | Reading files, writing code, running tests |
| **Publish Plane** | Gated | Control what leaves the workspace | Commit, push, GitHub post |

### Work Plane (Default-Allow)

Everything up to staging runs without friction:
- Read any files, search code, run checks
- Write tests early, iterate on code freely
- Run tests locally, fix issues as discovered
- Push early to get bot feedback (CI, CodeRabbit)
- Security findings here are **advisory**, not throttles

### Publish Plane (Gated)

Gates engage only when crossing the boundary:
- **Commit**: secrets-sanitizer scans staged changes
- **Push**: repo-operator checks for anomalies
- **GitHub post**: content mode restricts what gets posted (not what's analyzed)

If a gate blocks, **keep working locally**. Gates constrain publishing, not thinking.

**Key insight:** This separation prevents "security theater" where agents spend more time proving they're allowed to act than actually acting.

---

## Key Design Patterns

### 1. Agents as Compressors

**The Problem:** Raw reality (logs, diffs, API responses) is heavy. Carrying it through flows exhausts context.

**The Solution:** Specialized agents ingest massive context, perform work, and output a **small truth artifact**.

```
┌─────────────────────────────────────────┐
│  pr-feedback-harvester                  │
│                                         │
│  IN:  100KB of GitHub API JSON          │
│       (reviews, comments, checks)       │
│                                         │
│  OUT: pr_feedback.md (~2KB)             │
│       + Result block (~200 bytes)       │
└─────────────────────────────────────────┘
```

Examples:
- `test-executor`: 10K lines of console logs → `test_execution.md` (status + top failures)
- `pr-feedback-harvester`: GitHub API firehose → `pr_feedback.md` + normalized `blockers[]`
- `build-cleanup`: All flow artifacts → `build_receipt.json` (mechanical counts)

**Benefit:** The orchestrator reads the compressed output, not the raw inputs. This keeps flow context clean and prevents hallucination from context stuffing.

**Rule:** Workers may be heavy; their outputs must be light and stable.

### 2. Context Affinity

**Principle:** If an agent has a file open and the token budget to process it, it should do the related work.

Don't spin up a new agent (and pay the startup cost) just for bureaucratic purity.

| Context Loaded | Owner | Combined Duties |
|----------------|-------|-----------------|
| `src/*.ts`, `ac_matrix.md` | `code-implementer` | Logic, docstrings, local refactor, debug removal |
| `features/*.feature`, `tests/*.test.ts` | `test-author` | Test writing, fixture updates, spec feedback |
| `git status`, `git diff` | `repo-operator` | Staging, extras detection, security guard, commit/push |
| GitHub API JSON | `pr-feedback-harvester` | Harvesting, triage, summarizing |

**Efficiency wins:**
- We don't have a separate "Anomaly Detector" agent—repo-operator sees anomalies while staging
- We don't fetch data in one agent and analyze in another—harvester ingests and emits signal in one pass

### 3. Receipts-First

Every flow produces a receipt (`<flow>_receipt.json`).

Receipts are:
- **Mechanical:** Counts from grep/wc/parse, never estimated
- **Sealed:** Once written, reporters read them—they don't recompute
- **Canonical:** The source of truth for flow outcome
- **Logs, not gatekeepers:** Receipts describe what happened; the repo's current state determines outcomes

**State-First Verification:** The repo's current state (HEAD + working tree + staged diff) is the thing you're building and shipping. Receipts help investigate what happened—but they're not the primary verification mechanism once the repo has moved.

### 4. Critics Never Fix

Critics write harsh assessments; implementers apply fixes.

```
author → artifact → critic → critique → author → improved artifact → ...
```

**Why:** Separation prevents "critic fixes its own issues" loops and maintains clear accountability.

### 5. Microloops

Writer ↔ Critic iteration until:
- `status: VERIFIED`, OR
- `can_further_iteration_help: no`

Default cadence: 2 passes (write → critique → write → critique → proceed).

**Why:** Bounded iteration prevents infinite loops while ensuring quality.

### 6. Intent + Extras (Embrace Ad-Hoc Fixes)

The orchestrator tells agents **what to produce** (intent). Agents figure out **what paths to touch** (execution).

When staging, expect "extras" (files changed outside the expected set):
1. **Stage them** by default (assume the developer did them for a reason)
2. **Record them** in `extra_changes.md`
3. **Do not block** unless they trigger a hard guardrail (test deletion)

**Why:** Developers jump in to fix typos or tweak config while the swarm runs. This is collaboration, not attack.

### 7. Anti-Reward-Hacking Guard

Agents can "reward hack" by deleting failing tests to pass the test executor.

**Guard:** Before committing, `repo-operator` scans the staged diff for deleted test files:
```bash
git diff --cached --name-status | grep "^D" | grep -E "(test|spec|_test\.|\.test\.)"
```

If deleted tests are found without explicit justification, the commit is blocked.

**Why:** Quality metrics cannot be gamed by removing the measuring stick.

---

## The Data Model: Two Planes (Control vs Audit)

Separate from Work/Publish planes, the pack has two **data planes**:

| Plane | Artifacts | Purpose | Lifecycle |
|-------|-----------|---------|-----------|
| **Control** | `Gate Result`, `Machine Summary`, Result blocks | Routing decisions | Ephemeral (read once, route, discard) |
| **Audit** | `*_receipt.json`, `*.md` artifacts, `index.json` | Record of what happened | Durable (committed to git) |

**Crucial rule:** Orchestrators route on Control Plane blocks, not by re-parsing files.

```
Agent runs
  ├─→ Writes audit artifacts (files)
  └─→ Returns control block (response)

Orchestrator
  ├─→ Routes on control block
  └─→ Does NOT reread files for routing
```

---

## The Seven Flows

| Flow | Input | Output | Purpose |
|------|-------|--------|---------|
| 1. Signal | Raw request | Requirements, BDD, risks | Shape the work |
| 2. Plan | Signal outputs | ADR, contracts, plans | Design the solution |
| 3. Build | Plan outputs | Code, tests, reviews | Implement with tests |
| 4. Review | Build outputs + Draft PR | PR feedback, worklist | Harvest PR feedback |
| 5. Gate | Review outputs | Merge decision | Pre-merge verification |
| 6. Deploy | Gate outputs | Verification, deployment | Release to mainline |
| 7. Wisdom | All outputs | Learnings, regressions | Close feedback loops |

### Flow 3: Build (The Construction Site)

**Vibe:** High Velocity. "Push early, fail fast."

Key stations:
1. **AC Microloops:** Test ↔ Critic ↔ Code ↔ Critic (per acceptance criterion)
2. **Early PR Bootstrap:** After first vertical slice, push + create Draft PR to get bots spinning
3. **Feedback Check:** Harvest PR feedback, route on blockers (CRITICAL only during Build)
4. **Global Hardening:** standards-enforcer (polish) → test-executor (verify)
5. **Ship:** Seal receipt → sanitize → push

### Flow 4: Review (The Inspection Chamber)

**Vibe:** High Rigor. "Drain the swamp."

Key stations:
1. **Harvest:** Full PR feedback (all severities, including nits)
2. **Worklist Loop:** Unbounded iteration until complete or context exhausted
3. **Context Checkpoint:** If context > 80%, checkpoint and exit with `PARTIAL` status
4. **Re-Harvest Cadence:** Push → re-harvest after every N items (capture new bot comments)

### Flow 6: Deploy (Mainline Promotion)

**Two-Axis Verdict:**
- `deploy_action`: COMPLETED | SKIPPED | FAILED
- `governance_enforcement`: VERIFIED | VERIFIED_RULESET | UNVERIFIED_PERMS | NOT_CONFIGURED | UNKNOWN

This separates "what happened" (deploy action) from "can we verify protections" (governance enforcement).

---

## Agent Taxonomy

| Family | Color | Behavior |
|--------|-------|----------|
| Shaping | Yellow | Early signal processing |
| Spec | Purple | Write requirements/design |
| Implementation | Green | Write code/tests/docs |
| Critic | Red | Harsh review (never fixes) |
| Verification | Blue | Audit and check |
| Analytics | Orange | Analysis and learning |
| Infra | Cyan | Git and run infrastructure |
| Reporter | Pink | GitHub posting |
| Cleanup | Various | Seal receipts, update index |

### Key Agents and Their Contexts

| Agent | Role | Context Strategy |
|-------|------|------------------|
| `repo-operator` | State Manager | Intent-based staging; embraces extras; guards test deletion |
| `pr-feedback-harvester` | The Eyes | Compressor; ingests API JSON, outputs normalized blockers |
| `secrets-sanitizer` | The Janitor | Fix-first pre-commit hook; redacts in-place; doesn't route |
| `test-executor` | Verifier | Compressor; runs suite, outputs pass/fail summary |
| `standards-enforcer` | Polisher | Runs formatters, strips debug artifacts |
| `code-implementer` | Writer | Writes code + docstrings; focuses on correctness |
| `*-cleanup` | Auditors | Verify logical outcomes; write SKIPPED stubs for missing steps |

---

## Safety Boundaries

### Two-Gate Rule

GitHub operations require BOTH:
1. `safe_to_publish: true` (secrets-sanitizer)
2. `proceed_to_github_ops: true` (repo-operator)

**Why:** No accidental exposure or push of unexpected content.

### Single-Pass Sanitization

The sanitizer runs **once** before push:
1. Scan staged files and allowlist artifacts
2. Auto-fix: redact secrets in-place
3. Do NOT trigger a reseal loop

**Why:** The old behavior created "Compliance Recursion" where redacting triggered receipt regeneration, burning tokens on paperwork instead of engineering.

### Safe-Bail

When publishing can't proceed safely:
- `checkpoint_mode: local_only`
- Never push
- Flow completes UNVERIFIED with evidence

**Why:** Prefer local completion over stuck or exposed state.

---

## Deterministic Tooling

### Why Rust over Bash

We replaced ad-hoc bash pipelines with the `demoswarm` CLI because:
- **The "Bash Tax":** `grep` behaves differently on GNU vs BSD. `sed` is a minefield.
- **The Shim:** `.claude/scripts/demoswarm.sh` ensures consistent behavior across platforms.

### The Shim Pattern

Agents **always** invoke via shims:
```bash
# Never this:
grep -c "pattern" file.md

# Always this:
bash .claude/scripts/demoswarm.sh count pattern --file "file.md" --regex "pattern"
```

---

## What Lives Where

| Content | Location |
|---------|----------|
| Flow behavior | `.claude/commands/flow-*.md` |
| Agent behavior | `.claude/agents/*.md` |
| Shared invariants | `CLAUDE.md` |
| Mechanical helpers | `.claude/skills/*/SKILL.md` |
| Validation | `.claude/scripts/pack-check.sh` |
| Run artifacts | `.runs/<run-id>/` (in target repo) |

---

## The "Feel Test"

The system is working when:

1. **You can fix a typo** in `README.md` while the swarm builds, and `repo-operator` just includes it ("Extras")
2. **You delete a test**, and `repo-operator` refuses to push ("Anti-Reward Hacking")
3. **CI fails** on AC-1, and Flow 3 stops immediately to fix it ("Pulse")
4. **CodeRabbit suggests a nit**, and Flow 3 ignores it, but Flow 4 fixes it ("Triage")

---

## See Also

- [why-ops-first.md](why-ops-first.md) — The philosophy behind default-allow engineering
- [ai-physics.md](ai-physics.md) — LLM-specific design constraints
- [why-two-planes.md](why-two-planes.md) — Control vs audit plane separation
- [why-two-gates.md](why-two-gates.md) — GitHub ops gating
- [CLAUDE.md](../../CLAUDE.md) — Full pack reference
