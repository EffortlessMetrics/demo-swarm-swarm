# How to Design Agents

> Practical guide for creating Claude-native agent prompts.

---

## Before You Start

### The Two Reasons for Agents

Create a new agent when you need:

1. **A distinct responsibility** - The work doesn't fit naturally in an existing agent
2. **A fresh context** - Token-heavy work that would exhaust an existing context

Don't create an agent for:
- Work that fits in an existing agent (extend that agent instead)
- One-off customization (use pack config)
- Bureaucratic separation (if context is shared, keep the work together)

### Agent Categories

| Category | Color | Behavior | Example |
|----------|-------|----------|---------|
| **Spec** | Purple | Write artifacts (requirements, designs) | `requirements-author` |
| **Critic** | Red | Review harshly (never fix) | `code-critic` |
| **Implementation** | Green | Write code, tests, docs | `code-implementer` |
| **Verification** | Blue | Check and audit | `artifact-auditor` |
| **Analytics** | Orange | Analyze and learn | `regression-analyst` |
| **Infrastructure** | Cyan | Git, run setup | `repo-operator` |
| **Reporter** | Pink | GitHub posting | `pr-commenter` |
| **Cleanup** | Various | Seal receipts, count artifacts | `build-cleanup` |

Know which category your agent falls into before writing it.

---

## Agent Prompt Template

```markdown
---
name: <agent-name>
description: <What it does> -> <what it produces> (<location>).
model: inherit
color: <category color>
---

You are the **<Agent Name>**.

<One sentence: what you do. One sentence: what you don't do.>

## What You'll Need

Primary:
- `.runs/<run-id>/<flow>/<input-file>.md`

Feedback (if present):
- `.runs/<run-id>/<flow>/<critique>.md`

## What You Produce

- `.runs/<run-id>/<flow>/<output-file>.md`

## What Success Looks Like

<Description of good output - focus on value, not schema>

## Tips

<Positive guidance on how to do the job well>

## If You're Stuck

<Graceful handling of blockers - hierarchy of responses>

## Handoff

**What I did:** <summary of work completed>

**What's left:** <remaining work or "nothing">

**Recommendation:** <specific next step with reasoning>
```

---

## Writing Each Section

### Your Job (Opening Paragraph)

**Goal:** One clear sentence about what the agent does, one about what it doesn't.

**Good:**
```markdown
You are the **Requirements Author**.

You author requirements. You do not critique. You do not perform git ops.
```

**Bad:**
```markdown
You are a brilliant requirements expert who carefully crafts perfect specifications.

Your job is to take the user's vague ideas and transform them into crystal-clear requirements that any developer could implement.
```

**Why:** The first version is factual and constraint-first. The second is theatrical and vague about actual behavior.

**Tips:**
- Open with what the agent does (positive)
- Follow with what it doesn't do (boundary)
- Keep to 1-2 sentences total
- No adjectives ("brilliant", "careful", "perfect")

### What You'll Need

**Goal:** Tell the agent exactly what to read.

**Good:**
```markdown
## What You'll Need

Primary:
- `.runs/<run-id>/signal/problem_statement.md`
- `.runs/<run-id>/signal/requirements.md`

Feedback (if present):
- `.runs/<run-id>/signal/requirements_critique.md`
```

**Bad:**
```markdown
## What You'll Need

Read the relevant inputs from the run folder. Check for any critique files that might exist.
```

**Why:** Agents should know exactly what paths to read, not guess based on vague descriptions.

**Tips:**
- List specific file paths
- Mark optional inputs clearly
- Distinguish primary inputs from feedback loops
- Use repo-root-relative paths (`.runs/<run-id>/...`)

### What Success Looks Like

**Goal:** Describe outcomes, not schemas.

**Good:**
```markdown
## What Success Looks Like

Requirements that a stranger could turn into deterministic tests without asking follow-ups. Each requirement has one behavior, measurable acceptance criteria, and no vague terms like "secure" or "appropriate" without bounding predicates.
```

**Bad:**
```markdown
## What Success Looks Like

The output file must contain:
- A Summary section with Status field
- A Functional Requirements section with REQ-### headers
- An NFR section with NFR-DOMAIN-### headers
- Each requirement must have AC-N list items
```

**Why:** Schema-focused descriptions become checklists that agents satisfy mechanically. Outcome-focused descriptions help agents understand why the work matters.

**Tips:**
- Focus on the value delivered ("tests can be written from this")
- Use concrete examples of good output
- Explain what makes the output useful to the next agent
- Save structure for a separate "Format" section if needed

### Tips

**Goal:** Positive guidance on doing the job well.

**Good:**
```markdown
## Tips

- **Be specific about location:** File, line number, symbol name. Make it easy to find.
- **Explain why it matters:** Contract violations break clients. ADR violations break scaling.
- **Name who should fix:** code-implementer for logic bugs, design-optioneer for ADR questions.
- **Scope tightly:** Only critique what's in scope. Out-of-scope issues are someone else's job.
```

**Bad:**
```markdown
## Rules

- DO NOT include out-of-scope issues
- NEVER make vague complaints without file paths
- You MUST categorize every finding by severity
- You are FORBIDDEN from fixing issues yourself
```

**Why:** Positive framing ("be specific") is easier to follow than negative framing ("don't be vague"). Tips guide behavior; rules constrain it.

**Converting constraints to guidance:**

| Constraint (avoid) | Guidance (prefer) |
|-------------------|-------------------|
| "DO NOT include out-of-scope issues" | "Scope tightly. Only critique what's in scope." |
| "NEVER make vague complaints" | "Be specific about location. Make it easy to find." |
| "You MUST categorize findings" | "Categorize findings so the fixer knows what to prioritize." |
| "FORBIDDEN from fixing" | "You critique. Leave fixing to the implementer." |

### If You're Stuck

**Goal:** Give the agent a hierarchy of responses when blocked.

**Good:**
```markdown
## If You're Stuck

1. **Re-read context:** The answer is often in the inputs you already have.
2. **Search the codebase:** Look for patterns, existing implementations, test expectations.
3. **Make an assumption:** Document it and proceed. Safe defaults are fine.
4. **Log a question:** Append to `open_questions.md` and continue with the rest.
5. **Report partial progress:** If truly blocked, say what you accomplished before hitting the issue.
```

**Bad:**
```markdown
## If You're Stuck

If you encounter any issues, stop and report the blocker to the orchestrator.
```

**Why:** The first version gives the agent multiple successful exits. The second forces failure as the only option, which encourages guessing to avoid reporting failure.

**Key insight:** Agents under pressure to complete will guess rather than fail. Giving them legitimate partial-completion paths reduces guessing.

### Handoff

**Goal:** Template for reporting back to the orchestrator.

Every handoff answers three questions:

1. **What I did** - Summary of work completed
2. **What's left** - Remaining work, blockers, or "nothing"
3. **Recommendation** - Specific next step with reasoning

**Good:**
```markdown
## Handoff

**What I did:** Wrote 5 functional requirements covering login, logout, session management. All have atomic AC lists. Applied 3/4 critique items.

**What's left:** REQ-003 (password reset) has ambiguous timeout - defaulted to 24h but flagged in open_questions.md.

**Recommendation:** Proceed to requirements-critic. The one unresolved critique item is MINOR (style preference).
```

**Bad:**
```markdown
## Handoff

Requirements complete. Ready for next step.
```

**Why:** The detailed handoff tells the orchestrator exactly what happened and why the recommendation makes sense. The brief handoff hides information needed for routing.

**Always make a recommendation.** Name specific agents when you know them. Explain your reasoning.

---

## Common Mistakes

### Constraint Lists Instead of Guidance

**Problem:**
```markdown
## Rules
- DO NOT modify files outside your scope
- NEVER guess at behavior
- You are FORBIDDEN from...
- You MUST NOT under any circumstances...
```

**Fix:** Convert to positive guidance in a "Tips" section. Describe what TO DO, not what NOT to do.

### Multiple Responsibilities

**Problem:**
```markdown
You are the **Code Author and Reviewer**.

You write code and then review your own work for quality issues.
```

**Fix:** Split into two agents. Critics never fix; authors never critique. This separation maintains accountability.

### Structured Output Requirements

**Problem:**
```markdown
Your output MUST be valid YAML with the following schema:
```yaml
status: VERIFIED | UNVERIFIED
blockers: []
findings:
  - severity: CRITICAL | MAJOR | MINOR
    location: string
    message: string
```

**Fix:** Describe outcomes, not schemas. If structure matters, put it in a separate "Format" or "Machine Summary" section, not as the primary success criteria.

### Missing Graceful Failure

**Problem:**
```markdown
## Behavior

1. Read the inputs
2. Analyze the code
3. Write the critique
```

**Fix:** Add an "If You're Stuck" section with a hierarchy of responses. Agents need legitimate exits besides "complete" and "fail."

### Theatrical Voice

**Problem:**
```markdown
You are a brilliant, meticulous code reviewer with decades of experience. Your keen eye catches issues that lesser reviewers miss. Approach each review with the gravitas it deserves.
```

**Fix:** Factual, constraint-first opening. "You critique code. You do not fix." No adjectives, no persona.

---

## Examples

### Good: Code Critic

```markdown
---
name: code-critic
description: Review implementation against requirements, ADR, and contracts -> build/code_critique.md.
model: inherit
color: red
---

You are the **Code Critic**.

You find issues in implementation. You do not fix them.

## What You'll Need

- `.runs/<run-id>/build/impl_changes_summary.md`
- `.runs/<run-id>/plan/adr.md`
- `.runs/<run-id>/plan/api_contracts.yaml`
- `.runs/<run-id>/signal/requirements.md`

## What You Produce

- `.runs/<run-id>/build/code_critique.md`

## What Success Looks Like

A critique that explains what's wrong, why it matters, and who should fix it. Specific enough that the implementer can find and fix each issue without asking follow-ups.

## Tips

- **Be specific about location:** File, line, symbol. Make it findable.
- **Explain impact:** "Breaks horizontal scaling" not just "violates ADR."
- **Name the fixer:** code-implementer for logic, design-optioneer for ADR questions.
- **Scope tightly:** Only critique in-scope requirements.

## If You're Stuck

- **Missing implementation summary?** Report that implementer needs to run first.
- **Code doesn't exist?** That's a finding - document missing implementation.
- **Partial progress?** Report what you reviewed before hitting the blocker.

## Handoff

**What I found:** <summary of issues or "implementation is solid">

**What's left:** <issues to fix or "nothing">

**Recommendation:** <proceed to test-critic | route to code-implementer | etc.>
```

**Why this works:**
- Single clear responsibility (find issues, don't fix)
- Specific input paths
- Outcome-focused success criteria
- Positive tips, not constraint lists
- Graceful failure paths
- Clear handoff template

### Bad: Kitchen Sink Agent

```markdown
---
name: super-agent
description: Does everything for the build phase.
model: inherit
color: green
---

You are the **Super Agent**.

You are responsible for implementing code, writing tests, reviewing your work, and ensuring quality. You have deep expertise in all areas of software development.

## Rules

- DO NOT commit any code
- NEVER skip writing tests
- You MUST achieve 80% coverage
- DO NOT modify files outside your assigned scope
- NEVER guess at requirements
- You are FORBIDDEN from merging anything

## What to Do

Read the plan. Implement the code. Write tests. Review your implementation. Fix any issues. Ensure quality meets standards. Document everything.

## Output

Write your results to the appropriate files. Make sure everything is correct.
```

**Why this fails:**
- Multiple responsibilities (implement, test, review, fix)
- Theatrical voice ("deep expertise")
- Constraint-heavy rules instead of guidance
- Vague instructions ("read the plan", "appropriate files")
- No graceful failure paths
- No handoff template

---

## Checklist

Before committing your agent prompt:

- [ ] **Single clear job?** One sentence describes what it does; one says what it doesn't.
- [ ] **Positive framing?** Tips section uses "do X" not "don't do Y."
- [ ] **Specific paths?** Input/output sections list exact file paths.
- [ ] **Graceful failure?** "If You're Stuck" section with hierarchy of responses.
- [ ] **Clear handoff?** Template with "What I did / What's left / Recommendation."
- [ ] **Outcome-focused?** Success criteria describe value, not schema.
- [ ] **Factual voice?** No adjectives, no persona, no theatrical language.
- [ ] **Category alignment?** Color matches agent type (red for critics, green for implementation, etc.).

---

## See Also

- [agent-philosophy.md](../explanation/agent-philosophy.md) - How agents think, act, and fail gracefully
- [add-an-agent.md](add-an-agent.md) - Mechanics of adding an agent to the pack
- [architecture.md](../explanation/architecture.md) - System design patterns
- [documentation-conventions.md](../reference/documentation-conventions.md) - Voice and anti-patterns
