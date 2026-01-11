# pack-check Scope

> What pack-check enforces, and what it refuses to enforce.

---

## Purpose

Pack-check validates that the pack is correctly configured and will not fail at runtime. It is a **mechanical sanity check**, not a quality gate.

---

## What pack-check DOES Enforce

### Structural Integrity

- Required files exist (CLAUDE.md, flow commands, agent prompts)
- File references in commands/prompts point to files that exist
- Skills are registered and have valid SKILL.md files
- Agent frontmatter `name:` matches filename

### Syntax Validity

- Markdown files parse correctly
- YAML/JSON files are valid
- Frontmatter is properly delimited

### Contract Compliance

- Cleanup agents reference their flow receipts
- GH agents enforce two gates (`safe_to_publish`, `proceed_to_github_ops`)
- repo-operator has Repo Operator Result section
- No banned patterns (removed/deprecated concepts)
- No old taxonomy (FR-* instead of REQ-*)

### Ownership Boundaries

- Cleanup agents use `demoswarm.sh` shim, not bespoke pipelines
- Skill ownership enforced (secrets commands only in secrets-sanitizer, etc.)
- Flow commands delegate to agents, not directly to skill CLI
- No line-continuation bypass of shim validation

### Reference Consistency

- No CLAUDE.md behavioral substitution (`See CLAUDE.md > ...`)
- No raw git commands in flow commands (must delegate to repo-operator)
- QID patterns in open questions use canonical flow codes

---

## What pack-check REFUSES to Enforce

### Content Quality

- "Is this prompt good enough?"
- "Does this doc explain things well?"
- "Are there enough examples?"

Pack-check is not a writing critic.

### Routing Correctness

- "Should code-critic route to fixer or back to implementer?"
- "Is this the right agent for this task?"

Routing is a design decision, not a checkable property.

### Template Compliance

- "Does this receipt have all the sections?"
- "Is the handoff in the right format?"

Agents communicate in natural language. Rigid templates defeat the purpose.

### Process Adherence

- "Did all flows run in order?"
- "Were all critics consulted?"

Flow flexibility is a feature. Pack-check does not enforce ceremony.

### Semantic Correctness

- "Does the ADR actually address the requirements?"
- "Are the test assertions meaningful?"

These require judgment. Pack-check is mechanical.

---

## The Philosophy

Pack-check answers: **"Will this pack execute without crashing?"**

Pack-check does NOT answer: **"Is this pack good?"**

Quality comes from:
- Critics (code-critic, test-critic, design-critic, etc.)
- Gates (verification, evidence)
- Human review

Pack-check just makes sure the machinery works.

---

## Warning vs Blocking

**Blocking errors** (pack-check fails):
- Missing required files (CLAUDE.md, flow commands, agents, skills)
- Broken file references
- Missing frontmatter `name:` field
- Frontmatter name/filename mismatch
- Banned patterns in pack files

**Warnings** (pack-check passes with notes):
- Missing optional sections in CLAUDE.md
- Missing customize-pack command (optional but recommended)
- Flow boundary violations (demoswarm.sh in flow commands)
- Non-canonical OpenQ flow codes

Warnings inform; they do not block.

---

## What This Prevents

This scope statement prevents pack-check from becoming:
- A style police
- A process enforcer
- A template validator
- A bureaucratic gate

Future maintainers may be tempted to "tighten quality" by adding checks. This doc is the counterweight: pack-check stays mechanical.

---

## Extending pack-check

If you need to add a check, ask:

1. Is this checking execution capability or content quality?
2. Can this be verified mechanically without judgment?
3. Would a false positive block legitimate work?

If the answer to #1 is "content quality" or #2 is "no" or #3 is "yes", do not add it to pack-check. Use a critic agent instead.

---

## Warning-First Philosophy

**Prefer WARN over FAIL.** A blocking error stops the entire pack from running. That's a high bar.

### When to FAIL (blocking)

An issue should block execution only when it:

1. **Breaks execution** — The pack will crash, error, or hang if this isn't fixed
2. **Creates misleading interfaces** — Users or agents will be deceived (missing gates appear to pass, broken references appear to work)
3. **Compromises boundary safety** — Publish gates bypassed, ownership boundaries violated, secrets exposed

### When to WARN (non-blocking)

Everything else. Specifically:

- Style preferences (formatting, naming conventions beyond contracts)
- Missing optional features (customize-pack, optional sections)
- Suboptimal patterns that still work correctly
- Things that might cause confusion but won't cause failure

### The cost of false FAILs

A blocking error that fires incorrectly:
- Stops legitimate work
- Trains maintainers to work around pack-check
- Erodes trust in the validation system

When in doubt, warn. A warning that's ignored costs little. A block that's wrong costs a lot.

---

## How to Add a New Check

Before adding a check, prove it's necessary.

### 1. Name the failure mode it prevents

Not "enforce X" but "prevent Y from happening." Examples:

- **Good:** "Prevents agents from bypassing secrets scan before GitHub posts"
- **Bad:** "Ensures proper documentation format"

If you can't name a concrete failure, you don't have a check — you have a preference.

### 2. Show how it causes real drift

Describe a scenario where the missing check caused actual problems:

- "Agent X posted credentials because gate Y wasn't enforced"
- "Flow command called git directly, causing repo-operator state to desync"

Hypothetical drift doesn't justify checks. Wait for the actual failure, then add the check.

### 3. Make the check minimal and durable

- Check the smallest thing that catches the failure mode
- Use patterns that won't break when prose changes
- Avoid regex that matches too broadly

**Bad:** Check that a section contains specific phrases
**Good:** Check that a section heading exists

### 4. Avoid parsing prose

Natural language changes. Checks that parse prose become:
- Brittle (break on rewording)
- Frustrating (block legitimate edits)
- Circumvented (maintainers learn the magic words)

Check structure, not content. Check presence, not phrasing.

### Example: Adding a check correctly

**Failure mode:** Cleanup agents that don't use the demoswarm.sh shim can't be audited.

**Real drift:** Cleanup agent used bespoke bash, counts diverged from runs-derive, orchestrator made wrong routing decision.

**Minimal check:** Grep for `demoswarm.sh` in cleanup agent files. Warn if absent.

**Why this works:** Checks structure (shim invocation), not prose. Won't break if agent instructions change. Catches the actual failure mode.

---

## What pack-check Is NOT

Pack-check does **not** exist to:

- **Enforce bureaucracy** — "Did you fill out all the forms correctly?"
- **Police writing style** — "Is this phrased the way we prefer?"
- **Create compliance theater** — "Check passes = quality assured"

These are failure modes of validation systems, not goals.

Pack-check exists to answer one question: **"Will this pack execute without crashing?"**

If a check doesn't serve that question, it doesn't belong in pack-check. Put it in a critic agent, a review checklist, or a human review process — somewhere that can exercise judgment.

---

## Running pack-check

```bash
# Full check (human-readable output)
bash .claude/scripts/pack-check.sh --no-color

# Machine-readable output
bash .claude/scripts/pack-check.sh --format json

# Warnings as errors (CI mode)
bash .claude/scripts/pack-check.sh --strict
```

---

## Current Check Inventory

These checks are implemented in `tools/demoswarm-pack-check/`:

| ID | Check | Type | Description |
|----|-------|------|-------------|
| 1 | Required agents | blocking | Core agent files present |
| 2 | Flow commands | blocking | All 6 flow commands present |
| 4 | Cleanup receipts | blocking | Cleanup agents reference receipts + index.json |
| 6 | Agent frontmatter | blocking | Names match filenames, no duplicates |
| 7 | Old taxonomy | blocking | No FR-* patterns (use REQ-*) |
| 8 | Banned patterns | blocking | No deprecated/removed concepts |
| 9 | Required skills | blocking | All 7 skill directories present |
| 10 | CLAUDE.md | blocking | Core config file with required sections |
| 14 | RUN_BASE alias | blocking | No RUN_BASE (use explicit paths) |
| 15 | Customizer | warning | customize-pack command + pack-customizer agent |
| 17 | gh-reporter output | blocking | Safe Output Contract section present |
| 18 | repo-operator Result | blocking | Repo Operator Result block with required fields |
| 19 | GH agents gates | blocking | Both publish gates enforced |
| 23 | Typed NFR | blocking | No bare NFR-### (use NFR-DOMAIN-###) |
| 30 | Flow-specific actions | blocking | No domain verdict keywords in actions |
| 32 | CANNOT_PROCEED | blocking | Invariant documented correctly |
| 38 | ensure_branch op | blocking | No operation: ensure_branch in flows |
| 39 | Raw git | blocking | No raw git commands in flow commands |
| 40 | CLAUDE.md substitution | blocking | No "See CLAUDE.md > ..." patterns |
| 42 | Issue drafts | blocking | Standardize on feedback_actions.md |
| 45 | Cleanup shim | blocking | Cleanup agents use demoswarm.sh |
| 46 | Skill ownership | blocking | Restricted commands only in allowed agents |
| 47 | Shim line continuation | blocking | No line-wrap bypass of shim |
| 48 | Direct demoswarm | blocking | Must use bash shim, not direct invocation |
| 49 | Skills section | blocking | Agents using demoswarm.sh have ## Skills |
| 50 | GH body hygiene | blocking | Heredoc pattern, no forbidden patterns |
| 52 | Flow boundary | warning | No demoswarm.sh or skill CLI in flow commands |
| 53 | OpenQ prefix | warning | Canonical flow codes in QIDs |
| 54 | Critics handoff | blocking | Critics have ## Handoff section |
| 55 | Clear job section | blocking | Agents document their role |

---

## See also

- [pack-check.md](pack-check.md) - Invocation and output format
- [troubleshoot.md](../how-to/troubleshoot.md) - When pack-check fails
- [contracts.md](contracts.md) - What pack-check validates
