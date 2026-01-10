# Adding an Agent

This guide explains how to add a new agent to the DemoSwarm pack.

---

## When to Create an Agent

Create a new agent when you need:
- A **distinct responsibility** not covered by existing agents
- A **fresh context** for token-heavy work
- A **different model tier** (haiku for mechanical work, sonnet for synthesis)

Do NOT create a new agent for:
- Work that fits naturally in an existing agent's context (add to that agent instead)
- One-off customization (use [customize-pack.md](customize-pack.md))

---

## Context Sizing Decision

Before creating an agent, ask: **Does this need its own context?**

| Consideration | New Agent | Existing Agent |
|---------------|-----------|----------------|
| Token budget > 50K for inputs | ✓ | |
| Different model tier needed | ✓ | |
| Distinct responsibility | ✓ | |
| Work shares same loaded files | | ✓ |
| Work is 1-2 additional steps | | ✓ |

**Context Affinity Principle:** If an agent has files open and budget to process them, it should do related work rather than spinning up a new agent.

---

## Agent File Location

Agents live at `.claude/agents/<agent-name>.md`.

Naming conventions:
- Use lowercase with hyphens: `migration-planner.md`
- Pair critics with authors: `<thing>-author.md` + `<thing>-critic.md`
- Cleanup agents: `<flow>-cleanup.md`

---

## Agent Skeleton

### Required Frontmatter

```yaml
---
name: <agent-name>
description: <One line: what it does → what it outputs (location)>.
model: inherit | haiku | sonnet
color: <purple|red|green|blue|orange|cyan|pink>
---
```

**Model guidance:**
- `haiku` — Mechanical work (cleanup, counting, formatting)
- `sonnet` — Almost-Haiku tasks needing slightly more reasoning
- `inherit` — Core creative work (user chooses Sonnet or Opus)

**Color conventions:**
- `purple` — Spec/authoring (requirements, BDD, contracts)
- `red` — Critics (never fix, only assess)
- `green` — Implementation (code, tests, docs)
- `blue` — Verification (auditors, checkers)
- `orange` — Analytics (learning, regression)
- `cyan` — Infrastructure (repo-operator, run-prep)
- `pink` — Reporters (GitHub posting)

### Required Sections

```markdown
You are the **<Agent Name>** (<Flow context>).

<One sentence: what you do, what you don't do.>

## Inputs (best-effort)

Primary:
- `.runs/<run-id>/<flow>/<input-file>.md`

Feedback loop (if present):
- `.runs/<run-id>/<flow>/<critique>.md`

## Output (only)

Write exactly:
- `.runs/<run-id>/<flow>/<output-file>.md`

## Lane + hygiene (non-negotiable)

1. No git ops (no commit/push/checkout).
2. Write only your output file. No temp files.
3. <Domain-specific constraints>

## Handoff Targets

When you complete your work, recommend one of these agents:
- <agent-1>: <what it does> — <when to route there>
- <agent-2>: <what it does> — <when to route there>

## Behavior

### Step 0: Preflight
<Mechanical checks>

### Step 1-N: Work
<Domain-specific steps>

### Final Step: Handoff

Write a prose handoff with:
1. **What I did** — summary of work completed
2. **What's left** — remaining work or blockers
3. **Recommendation** — specific next agent with reasoning

## Philosophy

<One paragraph: the agent's ethos>
```

---

## Lane Hygiene

Every agent has a "lane" — what it can and cannot do.

### Standard Lane Constraints

```markdown
## Lane + hygiene (non-negotiable)

1. No git ops (no commit/push/checkout).
2. Write only your output file. No temp files. No edits to other artifacts.
3. No secrets (no tokens/keys/credentials in outputs).
4. Status axis is boring:
   - `VERIFIED | UNVERIFIED | CANNOT_PROCEED`
   - `CANNOT_PROCEED` is mechanical failure only.
```

### Additional Constraints by Type

**Authors:**
- No critique. Write content; critic evaluates.
- No design decisions (ADR owns "how").

**Critics:**
- Never fix. Write assessment only.
- Indicate in prose whether further iteration would help.

**Cleanup:**
- Counts are mechanical. If you can't derive safely, output `null`.
- Use `demoswarm` shim for all derivations.

---

## Voice and Style

Agent prompts follow the same voice as documentation: factual, scoped, reproducible.

### Do

```markdown
You are the **Requirements Author** (Flow 1).

You write requirements. You do not critique or implement.
```

### Don't

```markdown
You are a brilliant requirements expert who carefully crafts perfect specifications.

Your job is to take the user's vague ideas and transform them into crystal-clear requirements.
```

### Principles

| Principle | Example |
|-----------|---------|
| **Factual, not theatrical** | "You critique. You do not fix." |
| **Constraint-first** | Open with what the agent does and doesn't do |
| **Paths, not abstractions** | `.runs/<run-id>/signal/requirements.md` |
| **No scripts** | Describe behavior, don't prescribe dialogue |

See [Documentation Conventions](../reference/documentation-conventions.md) for voice guidance.

---

## Output Contract

Agent outputs consist of **artifacts** (files written to disk) and **handoffs** (prose responses to the orchestrator).

### Handoff Structure

Every agent ends with a prose handoff:

```markdown
## Handoff

**What I did:** <summary of work completed>

**What's left:** <remaining work, blockers, or "nothing">

**Recommendation:** <specific next agent with reasoning>
```

### Status Concepts

Use these concepts naturally in your prose:

- **Complete / verified** — Work is done, evidence exists, no blockers
- **Incomplete / unverified** — Gaps exist; document what's missing
- **Blocked** — Cannot proceed without external input
- **Mechanical failure** — IO/permissions/tooling broken

### Routing Intent

Express routing naturally in your recommendation:

- "Run fixer next to address the three MINOR issues"
- "This needs to go back to Plan — the API contract doesn't match the ADR"
- "Ready for code-critic review"
- "Blocked until user clarifies the auth approach"

### Machine Summary (Cleanup Agents Only)

Cleanup agents write receipts with structured Machine Summary for audit purposes. This format is **not** for communication between agents — it's derived from prose handoffs when writing receipts.

---

## Stable Markers

Use stable markers for countable items:

| Domain | Marker Pattern | Example |
|--------|----------------|---------|
| Requirements | `### REQ-NNN` | `### REQ-001: User Login` |
| NFRs | `### NFR-DOMAIN-NNN` | `### NFR-SEC-001: Auth Tokens` |
| Acceptance Criteria | `- AC-N:` | `- AC-1: Returns 200` |
| Risks | `- RSK-NNN [SEVERITY]` | `- RSK-001 [HIGH] [SECURITY]` |
| Open Questions | `- QID: OQ-XXX-NNN` | `- QID: OQ-SIG-001` |
| Review Items | `- [ ] RW-NNN` | `- [ ] RW-001` |

These markers enable mechanical counting by cleanup agents.

---

## Critic Pattern

Critics follow a specific contract:

```markdown
---
name: <thing>-critic
description: Harsh review of <thing> vs <standard> → <flow>/<thing>_critique.md.
model: inherit
color: red
---

You are the **<Thing> Critic**.

You critique. You do not fix. You do not perform git ops.

## Inputs

- `.runs/<run-id>/<flow>/<thing>.md`
- Upstream context as needed

## Output (only)

- `.runs/<run-id>/<flow>/<thing>_critique.md`

## Handoff Targets

- **<thing>-author**: Addresses the issues you found
- **fixer**: Applies mechanical fixes for MINOR issues

## Behavior

### Step 1: Review against standards
<What to check>

### Step 2: Categorize findings

```markdown
## Critical Issues
<Must fix to proceed>

## Major Issues
<Should fix for quality>

## Minor Issues
<Nice to fix>
```

### Step 3: Handoff

Write a prose handoff:
- Summarize what you found
- Say whether another iteration would help
- Recommend specific next steps

Example:
"Found 2 CRITICAL and 1 MAJOR issue. The CRITICAL issues are fixable by the author.
Recommend running <thing>-author again with this critique. Another pass should resolve
the blocking issues."

## Philosophy

Critics are harsh and specific. Vague criticism is useless.
```

---

## Cleanup Agent Pattern

Cleanup agents seal the flow and translate prose handoffs into structured receipts:

```markdown
---
name: <flow>-cleanup
description: Finalizes Flow N by deriving counts, writing receipt, updating index.
model: haiku
color: blue
---

You are the **<Flow> Cleanup Agent**.

You seal the envelope. You do not interpret. You verify existence, count mechanically, and write the receipt.

## Outputs

- `.runs/<run-id>/<flow>/<flow>_receipt.json`
- `.runs/<run-id>/<flow>/cleanup_report.md`
- Update `.runs/index.json` (status, last_flow, updated_at only)

## Behavior

### Step 1: Artifact existence
Check required vs optional artifacts.

### Step 2: Mechanical counts
Use demoswarm shim:
```bash
bash .claude/scripts/demoswarm.sh count pattern \
  --file "<file>" --regex "<pattern>" --null-if-missing
```

### Step 3: Quality gates
Read critic prose handoffs and derive status.

### Step 4: Write receipt
JSON with counts, gates, stations, evidence_sha.

The receipt includes routing fields (`recommended_action`, `route_to_flow`, `route_to_agent`) for audit trail purposes. Derive these from the agent prose handoffs — they are not used for live routing.

### Step 5: Update index
```bash
bash .claude/scripts/demoswarm.sh index upsert-status \
  --index ".runs/index.json" \
  --run-id "<run-id>" \
  --status "<status>" \
  --last-flow "<flow>"
```

### Step 6: Write cleanup_report.md
Evidence of derivation.

## Philosophy

Cleanup does not interpret. Prefer `null` + evidence over invented precision.
Receipts are audit logs — the orchestrator routes on prose handoffs, not receipt fields.
```

---

## Registering the Agent

After creating the agent file:

1. **Add to flow command** — Update `.claude/commands/flow-<n>-<name>.md`:
   - Add to "Agents to Use" section
   - Add to station order
   - Add to TodoWrite template

2. **Add to settings.json** — If the agent should be available as a Task tool target:
   ```json
   {
     "agents": {
       "<agent-name>": {
         "description": "<description>",
         "tools": ["All tools"]
       }
     }
   }
   ```

3. **Update pack-check** — If the agent has a contract that should be validated, add checks to pack-check.

---

## Checklist: Before Merging a New Agent

- [ ] Agent file at `.claude/agents/<agent-name>.md`
- [ ] Frontmatter complete (name, description, model, color)
- [ ] Voice is factual, not theatrical (see [documentation conventions](../reference/documentation-conventions.md))
- [ ] Lane hygiene documented
- [ ] Handoff targets section included
- [ ] Handoff structure documented in behavior
- [ ] Stable markers used for countable items
- [ ] Added to flow command(s)
- [ ] Pack-check passes: `bash .claude/scripts/pack-check.sh`
- [ ] Test run with the agent completed

---

## Examples

### Minimal Author

```markdown
---
name: migration-planner
description: Design database migration strategy → plan/migration_plan.md.
model: inherit
color: purple
---

You are the **Migration Planner** (Flow 2.5).

You plan migrations. You do not critique or implement.

## Inputs

- `.runs/<run-id>/plan/adr.md`
- Schema files (if present)

## Output (only)

- `.runs/<run-id>/plan/migration_plan.md`

## Handoff Targets

- **migration-critic**: Reviews your migration plan
- **rollback-designer**: Designs rollback strategy after plan is approved

## Lane + hygiene (non-negotiable)

1. No git ops.
2. Write only migration_plan.md.
3. No secrets.

## Behavior

### Step 1: Analyze schema changes
...

### Step 2: Design migration steps
...

### Step 3: Handoff

Write a prose handoff explaining what you planned and recommending migration-critic review.
```

### Minimal Critic

```markdown
---
name: migration-critic
description: Review migration plan for safety and rollback → plan/migration_critique.md.
model: inherit
color: red
---

You are the **Migration Critic**.

You critique. You never fix.

## Inputs

- `.runs/<run-id>/plan/migration_plan.md`

## Output (only)

- `.runs/<run-id>/plan/migration_critique.md`

## Handoff Targets

- **migration-planner**: Addresses the issues you found
- **rollback-designer**: Proceeds if migration plan is acceptable

## Behavior

### Step 1: Check for rollback strategy
...

### Step 2: Check for data safety
...

### Step 3: Handoff

Write a prose handoff:
- Summarize findings with severity markers
- Say whether another iteration would help
- Recommend next step (rerun planner or proceed to rollback-designer)
```

---

## See Also

- [Documentation Conventions](../reference/documentation-conventions.md) — Voice and anti-patterns
- [create-a-flow.md](create-a-flow.md) — How to create a new flow
- [architecture.md](../explanation/architecture.md) — Design patterns
- [stable-markers.md](../reference/stable-markers.md) — Marker conventions
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
