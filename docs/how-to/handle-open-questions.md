# Handle Open Questions

> How to handle uncertainty without stopping flows.

When you encounter something you're unsure about, flows don't stop. They record, default, and proceed. This guide covers the decision process for resolving uncertainty.

---

## The Decision Tree

Work through this in order:

### 1. Can I Research This?

Check the codebase, docs, conventions, and external standards.

| Source       | Example                                        |
| ------------ | ---------------------------------------------- |
| Codebase     | Check existing timeout values in config files  |
| ADRs         | Review architectural decisions for this domain |
| NFRs         | Look for non-functional requirements           |
| Standards    | Check RFC specs, REST conventions, etc.        |
| Library docs | Read the documentation for dependencies        |

**If yes:** Research it, make the call, proceed.

### 2. Can I Derive from Context?

Use patterns, similar code, and surrounding context.

| Pattern Source     | What to Look For                       |
| ------------------ | -------------------------------------- |
| Similar features   | How did we solve this elsewhere?       |
| Naming conventions | What pattern does the codebase follow? |
| Error handling     | How do existing endpoints handle this? |
| Test fixtures      | What assumptions do tests make?        |

**If yes:** Derive it, document your reasoning, proceed.

### 3. Is There a Safe Default?

Look for options that are reversible, low-risk, and conventional.

| Safe Default Criteria | Example                                |
| --------------------- | -------------------------------------- |
| Reversible            | Config value that can be changed later |
| Low-risk              | Conservative timeout (30s vs 5s)       |
| Conventional          | Matches existing patterns in codebase  |
| Documented            | Can explain the choice to a reviewer   |

**If yes:** Take the default, mark DEFAULTED, proceed.

### 4. Is This About Authority?

If you're still stuck, ask: Does this require someone's **authority** to decide?

| Authority Required     | Example                             |
| ---------------------- | ----------------------------------- |
| Business relationships | Customer migration timelines        |
| Risk tolerance         | Ship with known issue or delay      |
| Organizational         | Team ownership, resource allocation |
| Compliance             | Security exceptions                 |
| External commitments   | Deadlines, SLAs                     |

**If yes:** Mark NEEDS_HUMAN, document options, proceed with other work.

**If no:** You missed something in steps 1-3. Go back.

---

## DEFAULTED Protocol

When you choose a default, document it fully:

1. **State the question** - What were you uncertain about?
2. **State what you chose** - The specific default
3. **State why** - Your reasoning
4. **State the risk if wrong** - What happens, how to change it
5. **Continue working**

### Example

```markdown
### OQ-BUILD-001: Session token format

- **QID:** OQ-BUILD-001
- **Question:** Should session tokens use JWT or opaque?
- **Context:** ADR-005 doesn't specify token format. No existing session implementation.
- **Default chosen:** JWT
- **Reasoning:** Codebase uses JWT for auth tokens at src/auth/tokens.ts. Consistent with existing patterns.
- **Risk if wrong:** Low - would require migration if we switch formats, but token interface is abstracted.
- **Decided by:** code-implementer during Flow 3
```

### What Makes a Good Default

| Good Default              | Bad Default                  |
| ------------------------- | ---------------------------- |
| Matches existing patterns | Invents new patterns         |
| Conservative/safe choice  | Optimistic/aggressive choice |
| Easy to change later      | Hard to reverse              |
| Documented reasoning      | Undocumented                 |

---

## NEEDS_HUMAN Protocol

When you genuinely need human authority:

1. **State the question** - What decision is needed?
2. **Explain why you cannot decide** - Cite the authority gap
3. **List 2-4 options with trade-offs** - Give humans choices
4. **Note who has authority** - Who should decide?
5. **Continue with other work** - Don't stop the flow

### Example

```markdown
### OQ-PLAN-002: Breaking API change

- **QID:** OQ-PLAN-002
- **Question:** New API breaks compatibility for Acme Corp. Proceed?
- **Context:** Acme uses undocumented session internals. Migration requires their engineering time. Sales says they're in renewal negotiations.
- **Why agent cannot decide:** Customer relationship decision. Agent has no authority over customer relationships or revenue trade-offs.
- **Options:**
  1. Proceed + notify (faster, risks relationship)
  2. Add compatibility shim (6h work, accumulates tech debt)
  3. Delay until customer migrates (unknown timeline)
- **Escalated by:** design-optioneer during Flow 2
- **Needs:** Product owner or account manager decision
```

### What Requires Authority

| Category       | Examples                                              |
| -------------- | ----------------------------------------------------- |
| Business       | Customer commitments, partner agreements, pricing     |
| Risk           | Ship despite known issues, accept security exceptions |
| Organizational | Team ownership, headcount, resource allocation        |
| Compliance     | Security policy exceptions, legal requirements        |
| External       | Deadlines, SLAs, contract terms                       |

---

## When Questions Surface

Questions are recorded anytime. Humans answer at flow boundaries.

### During the Flow

```
Encounter uncertainty
        |
        v
Work through decision tree (steps 1-4)
        |
    +---+---+
    |       |
 DEFAULTED  NEEDS_HUMAN
    |       |
    v       v
Document   Document + continue other work
    |       |
    +---+---+
        |
        v
Continue the flow
```

### At Flow Boundary

Provide humans with:

- **Summary of DEFAULTED items** - What was assumed and why
- **Summary of NEEDS_HUMAN items** - What decisions are needed
- **Impact of each** - What depends on these decisions

This gives humans grouped context, not mid-flow interrupts.

### Example Boundary Summary

```markdown
## Open Questions Summary

### Defaults Taken (3)

| QID          | Question      | Default   | Risk                   |
| ------------ | ------------- | --------- | ---------------------- |
| OQ-BUILD-001 | Token format  | JWT       | Low - reversible       |
| OQ-BUILD-002 | Cache backend | In-memory | Low - abstracted       |
| OQ-BUILD-003 | Timeout value | 30s       | Low - matches patterns |

### Needs Human Decision (1)

| QID         | Question            | Options                | Who Decides   |
| ----------- | ------------------- | ---------------------- | ------------- |
| OQ-PLAN-002 | Break API for Acme? | Proceed / Shim / Delay | Product owner |
```

---

## The Economics

Why this matters:

| Pattern                       | Cost                                                    |
| ----------------------------- | ------------------------------------------------------- |
| Agent researches and defaults | ~0 DevLT (machine time only)                            |
| Agent escalates unnecessarily | ~30 min DevLT (human context-switch, research, respond) |
| Agent escalates appropriately | ~5 min DevLT (human picks from researched options)      |

Good escalation is:

- **Rare** - Most questions resolve via research or defaults
- **Well-researched** - Human sees what was tried
- **Option-rich** - Human picks from choices, doesn't generate them
- **Actionable** - Human can respond in under 2 minutes

---

## Anti-Patterns

### Bad DEFAULTED (Should Be NEEDS_HUMAN)

| Decision                            | Why It Requires Authority                        |
| ----------------------------------- | ------------------------------------------------ |
| "I decided we should break the API" | Customer impact requires business authority      |
| "I chose to skip security review"   | Risk tolerance requires organizational authority |
| "I assigned ownership to Team X"    | Organizational authority required                |
| "I committed to a delivery date"    | Business commitment                              |

### Bad NEEDS_HUMAN (Should Be DEFAULTED)

| Question                                | Why It's Actually DEFAULTED                   |
| --------------------------------------- | --------------------------------------------- |
| "Should I use camelCase or snake_case?" | Check codebase, follow convention             |
| "Is this the right abstraction?"        | Make a choice, critic will push back if wrong |
| "What's the performance requirement?"   | Check NFRs or default conservatively          |
| "Which test framework?"                 | Use what's already in the repo                |
| "Where should this file go?"            | Follow existing directory structure           |

---

## See Also

- [authority-not-difficulty.md](../explanation/authority-not-difficulty.md) - The distinction between knowledge and authority
- [open-questions.md](../examples/open-questions.md) - Example open questions register
- [working-with-microloops.md](working-with-microloops.md) - How flows handle iteration
- [troubleshoot.md](troubleshoot.md) - When things go wrong
