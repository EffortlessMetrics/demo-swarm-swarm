# Authority, Not Difficulty

> The critical distinction between DEFAULTED and NEEDS_HUMAN.

---

## The Core Insight

**NEEDS_HUMAN is about authority, not difficulty.**

Many things feel hard or uncertain but can still be resolved by agents. The question is not "Is this hard?" but "Does this require someone's authority to decide?"

---

## The Test

Ask: Does the decision require someone's **authority** or just someone's **knowledge**?

| If it requires... | Then...                                                       | Status      |
| ----------------- | ------------------------------------------------------------- | ----------- |
| **Knowledge**     | Research, derive, default safely, document reasoning, proceed | DEFAULTED   |
| **Authority**     | Surface options, explain trade-offs, wait for boundary        | NEEDS_HUMAN |

This single distinction keeps flows moving. Most "blocked" situations are knowledge problems disguised as authority problems.

---

## DEFAULTED: Agent Can Handle

Things that seem hard but are actually researchable or derivable:

| Question                               | Why DEFAULTED                                 | The Move                                      |
| -------------------------------------- | --------------------------------------------- | --------------------------------------------- |
| "What's the GDPR retention limit?"     | Legal requirements are documented             | Research it, cite the source                  |
| "Should we encrypt session data?"      | Data sensitivity + codebase patterns tell you | Derive from context                           |
| "What error handling pattern here?"    | Codebase convention exists                    | Follow the pattern                            |
| "Which HTTP status code for this?"     | Standards exist (RFC 7231, etc.)              | Apply the standard                            |
| "How should we structure this module?" | Existing patterns in the repo                 | Mirror what's there                           |
| "What timeout value?"                  | Check existing timeouts                       | Use the existing pattern                      |
| "Should we add logging here?"          | Check existing logging patterns               | Match conventions                             |
| "What's the right abstraction level?"  | Judgment call within your domain              | Make a choice, critic will push back if wrong |

### The DEFAULTED Protocol

1. **Make the safest reasonable choice**
2. **Document the reasoning** (why this default is safe)
3. **Note the risk if wrong** (what happens, how to change it)
4. **Proceed**
5. **Surface at flow boundary** with context

### Example from a Real Flow

From [open-questions.md example](../examples/open-questions.md):

```markdown
### OQ-BUILD-001: Session storage backend

- **QID:** OQ-BUILD-001
- **Question:** Use Redis or in-memory for session storage?
- **Context:** No existing Redis infrastructure in repo. ADR doesn't specify backend.
- **Default chosen:** In-memory with documented upgrade path
- **Reasoning:** Lower complexity for MVP; Redis can be added later. Interface is abstracted via `SessionStore` protocol.
- **Risk if wrong:** Low - interface is abstracted, swap is mechanical (estimated 2h refactor)
- **Decided by:** code-implementer during Flow 3
```

This looks hard. It affects architecture. But it's DEFAULTED because:

- The agent can research options
- A safe default exists (in-memory with abstraction)
- The choice is reversible
- No one's authority is needed to make this call

---

## NEEDS_HUMAN: Requires Authority

Things that genuinely require human authority:

| Question                                           | Why NEEDS_HUMAN                | The Authority Gap                                      |
| -------------------------------------------------- | ------------------------------ | ------------------------------------------------------ |
| "Break compatibility with 3 paying customers?"     | Business relationship decision | Agent has no authority over customer relationships     |
| "Ship with known race condition or delay release?" | Risk tolerance decision        | Agent cannot decide acceptable risk levels for the org |
| "Which team owns this service long-term?"          | Organizational authority       | Agent cannot assign ownership                          |
| "Should we take on this tech debt?"                | Business trade-off             | Involves timeline and resource allocation              |
| "Do we support this legacy format?"                | Product decision               | Involves customer commitments                          |
| "Can we drop support for IE11?"                    | Customer impact                | Affects paying users                                   |
| "Should we proceed despite failing security scan?" | Compliance authority           | Cannot override security policy                        |

### The NEEDS_HUMAN Protocol

1. **Clearly state what decision is needed**
2. **Explain why the agent cannot decide** (cite the authority gap)
3. **Present options with trade-offs**
4. **Note who has authority to decide**
5. **Continue with other work if possible**

### Example from a Real Flow

From [open-questions.md example](../examples/open-questions.md):

```markdown
### OQ-PLAN-004: Breaking change affects 3 paying customers

- **QID:** OQ-PLAN-004
- **Question:** New session format breaks API compatibility for Acme Corp, Globex, and Initech. Proceed anyway?
- **Context:** These 3 customers use undocumented session internals. Migration requires their engineering time. Sales says Acme is in renewal negotiations.
- **Why agent cannot decide:** Business relationship decision. Agent has no authority over customer relationships or revenue trade-offs.
- **Options:**
  1. Proceed + notify (faster, risks relationships)
  2. Add compatibility shim (6h work, keeps tech debt)
  3. Delay until customers migrate (unknown timeline)
- **Escalated by:** design-optioneer during Flow 2
- **Needs:** Product owner or account manager decision
```

This is NEEDS_HUMAN because:

- The decision affects external parties
- Revenue and relationships are at stake
- No amount of research resolves the trade-off
- Someone with business authority must decide

---

## Why This Matters

Most "blocked" situations are not actually blocked. They are DEFAULTED situations where the agent hesitated.

### The Throughput Cost

| Pattern                       | What Happens                                                                                   |
| ----------------------------- | ---------------------------------------------------------------------------------------------- |
| Agent defaults safely         | Flow continues, question documented, human reviews at boundary                                 |
| Agent escalates unnecessarily | Flow stops, human context-switches, 30-60 min delay, human does research agent could have done |

Every unnecessary escalation costs:

- $50-100 in context-switch overhead
- Minutes to hours of latency
- Interruption to human focus
- Lost momentum in the flow

### The False Escalation

Many questions feel like they need authority but actually need knowledge:

| Feels Like Authority                  | Actually Knowledge                            |
| ------------------------------------- | --------------------------------------------- |
| "What should the API return?"         | Check REST conventions and existing endpoints |
| "Is this the right abstraction?"      | Make a judgment call; critic will push back   |
| "What's the performance requirement?" | Check NFRs or default conservatively          |
| "Should we add validation here?"      | Check existing validation patterns            |
| "What error message?"                 | Check existing error messages                 |

The test: **Can I research this? Can I derive it from patterns? Is there a safe default?** If yes to any, it's DEFAULTED.

---

## The Escalation Ladder

Before marking NEEDS_HUMAN, agents must climb this ladder:

```
1. Investigate locally
   - Search code, tests, configs
   - Check prior runs and existing docs
   - Look at test fixtures and examples
           |
           v
2. Investigate remotely (if allowed)
   - GitHub issues/PRs
   - Web search for standards
   - Library documentation
           |
           v
3. Derive from evidence
   - Use patterns in the codebase
   - Infer from surrounding code
   - Match existing conventions
           |
           v
4. Default if safe
   - Choose a reversible option
   - Document the reasoning
   - Note risk if wrong
           |
           v
5. Escalate only when boxed in
   - All above failed
   - No safe default exists
   - Authority gap is genuine
```

Most questions resolve at step 3 or 4. Step 5 is rare.

---

## Anti-Patterns

### Bad NEEDS_HUMAN (Should Be DEFAULTED)

| Question                                | Why It's Actually DEFAULTED                   |
| --------------------------------------- | --------------------------------------------- |
| "Should I use camelCase or snake_case?" | Check codebase, follow convention             |
| "Is this the right abstraction?"        | Make a choice, critic will push back if wrong |
| "What's the performance requirement?"   | Check NFRs or default conservatively          |
| "Which test framework?"                 | Use what's already in the repo                |
| "Where should this file go?"            | Follow existing directory structure           |
| "Should I add comments here?"           | Follow existing comment patterns              |

These all have safe defaults or can be derived from the codebase.

### Bad DEFAULTED (Should Be NEEDS_HUMAN)

| Decision                                | Why It Requires Authority                        |
| --------------------------------------- | ------------------------------------------------ |
| "I decided we should break the API"     | Customer impact requires business authority      |
| "I chose to skip security review"       | Risk tolerance requires organizational authority |
| "I assigned ownership to Team X"        | Organizational authority required                |
| "I committed to a delivery date"        | Business commitment                              |
| "I deprecated this feature"             | Affects users, requires product authority        |
| "I accepted the security vulnerability" | Compliance authority required                    |

These involve external commitments, business relationships, or organizational authority that agents do not have.

---

## The Authority Taxonomy

### Types of Authority Agents Lack

| Authority Type             | Examples                                             | Who Has It                  |
| -------------------------- | ---------------------------------------------------- | --------------------------- |
| **Business relationships** | Customer commitments, partner agreements             | Account managers, sales     |
| **Risk tolerance**         | Ship despite known issues, accept tech debt timeline | Engineering lead, product   |
| **Organizational**         | Team ownership, headcount, resource allocation       | Managers, leads             |
| **Compliance**             | Security exceptions, legal requirements              | Security, legal, compliance |
| **Product direction**      | Feature scope, user experience trade-offs            | Product managers            |
| **External commitments**   | Deadlines, SLAs, contract terms                      | Leadership                  |

### Types of Decisions Agents CAN Make

| Decision Type                | Examples                                            |
| ---------------------------- | --------------------------------------------------- |
| **Technical implementation** | Algorithm choice, data structure, code organization |
| **Convention following**     | Naming, formatting, file structure                  |
| **Standard application**     | HTTP status codes, error formats, REST conventions  |
| **Pattern matching**         | Following existing code patterns                    |
| **Safe defaults**            | Timeouts, limits, thresholds (when reversible)      |
| **Abstraction design**       | Interface structure, module boundaries              |

---

## Flow Mechanics

### Questions Don't Stop Flows

Flows continue with documented assumptions:

```
Encounter uncertainty
        |
        v
Can I resolve it? (research, derive, default)
    |           |
   Yes          No (genuine authority gap)
    |           |
    v           v
Default it    Mark NEEDS_HUMAN
    |           |
    v           v
Document      Document + continue with other work
    |           |
    v           v
Continue      Queue for boundary review
    |           |
    +-----+-----+
          |
          v
    At flow boundary: present summary to human
```

### The Flow Boundary

Humans are asked at flow boundaries, not mid-flow. The ask includes:

- What was done
- What was assumed (with reasoning)
- What decisions are requested (with options)
- What the agent recommends

This gives humans context to make good decisions quickly.

---

## Worked Examples

### Example 1: Timeout Value

**Situation:** No timeout specified in requirements.

**Wrong approach:** "NEEDS_HUMAN: What timeout should I use?"

**Right approach:**

1. Search for existing timeouts in codebase: found `30s` in 5 places
2. Check industry standards: REST APIs typically use 30-60s
3. Default to 30s (matches codebase, reversible, safe)
4. Document: "Using 30s timeout per existing patterns. Change in config.yaml if different value needed."

**Status:** DEFAULTED

### Example 2: Breaking Change

**Situation:** New format would break compatibility for 3 customers.

**Wrong approach:** "DEFAULTED: I'll add a compatibility shim."

**Right approach:**

1. Research: Cannot determine business value of speed vs compatibility
2. Options: (1) break + notify, (2) shim, (3) delay
3. Each has trade-offs only business can weigh
4. Document with options and trade-offs
5. Continue with other work while awaiting decision

**Status:** NEEDS_HUMAN

### Example 3: Architecture Choice

**Situation:** Could use sync or async processing.

**Wrong approach:** "NEEDS_HUMAN: Should I use sync or async?"

**Right approach:**

1. Check ADR: mentions "responsive UI" (suggests async)
2. Check codebase: existing similar features use async
3. Check NFRs: no specific latency requirements
4. Default to async (matches patterns, reversible, safer for scale)
5. Document: "Using async per ADR 'responsive UI' goal and existing patterns."

**Status:** DEFAULTED

---

## The Economic Reality

Human attention is expensive. Machine iteration is cheap.

| Action                        | Cost                                                                |
| ----------------------------- | ------------------------------------------------------------------- |
| Agent researches and defaults | ~0 DevLT (machine time)                                             |
| Agent escalates unnecessarily | ~30 min DevLT (human context-switch, research, respond)             |
| Agent escalates appropriately | ~5 min DevLT (human makes informed decision with options presented) |

Good escalation is:

- Rare (most questions are DEFAULTED)
- Well-researched (human sees what was tried)
- Option-rich (human picks from choices, doesn't generate them)
- Actionable (human can respond in <2 minutes)

---

## Summary

> The question is not "Is this hard?" but "Does this require authority?"

- **Knowledge problems:** Research, derive, default, proceed (DEFAULTED)
- **Authority problems:** Present options, wait for boundary, let humans decide (NEEDS_HUMAN)

Most "blocked" is actually DEFAULTED in disguise. Climb the escalation ladder. Make reversible defaults. Document reasoning. Keep flows moving.

---

## See Also

- [human-escalation.md](human-escalation.md) — Full escalation protocol
- [agent-philosophy.md](agent-philosophy.md) — Research-first autonomy
- [routing-table.md](../reference/routing-table.md) — Where work routes
- [open-questions.md](../examples/open-questions.md) — Example register
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
