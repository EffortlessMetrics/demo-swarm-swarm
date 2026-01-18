# Human Escalation

> When and how to involve humans in the loop.

---

## The Philosophy

**Humans are expensive. Escalate only when necessary. But when necessary, escalate clearly.**

Every escalation costs:

- $50-100 in context-switch overhead
- Minutes to hours of latency
- Interruption to human focus

The bar is high: "Can I solve this without human input?" Try hard before escalating.

---

## The Core Rule: Flow Boundaries Only

**Human escalation happens at flow boundaries, never mid-flow.**

### Why Flow Boundaries?

Flows are natural checkpoint moments:

- Work is complete (for that phase)
- Artifacts exist (receipts, summaries)
- Context is summarized (not sprawling)
- Decisions are crystallized

### What Happens at Flow Boundaries

1. **Progress update** — What was accomplished in this flow
2. **Findings summary** — What was discovered, what issues arose
3. **Decision points** — Any choices that need human input
4. **Next flow preview** — What the next phase will do

### Why NOT Mid-Flow?

Mid-flow escalation breaks momentum:

- Context is fragmented
- Work is incomplete
- State is messy
- The human sees chaos, not clarity

**Inside a flow:** Agents handle issues themselves (route to specialists, use safe defaults, fix forward).

**At flow boundaries:** Present clean summaries and clear decision points.

### The Flow Boundary Escalation Format

```markdown
## Flow 2 Complete: Plan

### What Was Done

- ADR written for authentication approach
- API contracts defined
- Work breakdown complete (12 ACs)

### Findings

- REQ-007 conflicts with GDPR (noted for human review)
- No existing auth library in codebase (will use Passport.js)

### Decision Needed

Should we proceed with OAuth-only or add SAML support?
(See ADR section 3.2 for trade-offs)

### Next Flow

Flow 3 (Build) will implement the 12 ACs.
Estimated: 4-6 microloop iterations.

### To Proceed

Reply "continue" to start Build, or provide guidance on the SAML question.
```

This gives the dev:

- Clear status
- Digestible findings
- Actionable decision points
- Preview of what's next

---

## The Default: Don't Escalate

Most issues can be handled by:

1. **Deriving from context** — Search the codebase, read tests, check configs
2. **Choosing safe defaults** — Pick reversible options, document them, continue
3. **Routing to a specialist agent** — `design-optioneer` for design snags, `fixer` for code issues
4. **Trying 2-3 approaches** — Machine time is cheap; exhaust options before waiting

Escalation is the last resort, not the first instinct.

### The Escalation Ladder

Before escalating to a human, an agent must climb this ladder (in order):

1. **Investigate locally:** Search code, tests, configs, prior runs, existing docs
2. **Investigate remotely (if allowed):** GitHub issues/PRs, web search, library docs
3. **Derive from evidence:** Use patterns in the codebase to infer correct behavior
4. **Default if safe:** Choose a reversible default, document it, continue
5. **Escalate only when boxed in:** All of the above failed AND no safe default exists

Most questions are NOT blockers. `DEFAULTED` (safe reversible default chosen) is the common case. `NON_DERIVABLE` is rare and requires proof-of-research.

---

## When Escalation Is Required

Three categories require human involvement. Everything else should be handled by agents.

---

## Category 1: Non-Derivable Decisions

**What it is:** The decision requires information or judgment that cannot be derived from the codebase, artifacts, or context.

**Examples:**

- "Should we use OAuth or SAML for enterprise customers?"
- "What's the acceptable latency threshold for this API?"
- "Which third-party service should we integrate with?"
- "Is this breaking change acceptable for the next release?"

**Why agents can't decide:** These are business decisions, not technical ones. No amount of codebase investigation will reveal the answer.

**Pattern:**

```markdown
## Needs Human Decision

**Question:** Should we use OAuth or SAML for enterprise auth?

**Context:** Both are technically feasible. OAuth is simpler to implement.
SAML is often required by large enterprises.

**Options:**

1. OAuth only (simpler, covers 80% of cases)
2. SAML only (enterprise-first)
3. Both (complexity cost, full coverage)

**My lean:** Option 1 unless enterprise is a priority.

**Blocked on:** Your decision. Work continues after you choose.
```

---

## Category 2: True Ambiguity

**What it is:** The requirements, spec, or intent is genuinely unclear and cannot be safely assumed.

**Examples:**

- Conflicting requirements in the spec
- Acceptance criteria that could mean two different things
- Missing information that affects architecture
- Edge cases with no clear precedent

**Why agents can't decide:** Guessing wrong could mean building the wrong thing entirely. The cost of rework exceeds the cost of asking.

**Pattern:**

```markdown
## Needs Clarification

**Ambiguity:** REQ-003 says "fast response times" but doesn't define "fast."

**Why it matters:** This affects caching strategy. <100ms needs Redis.
<500ms can use in-memory. >1s can hit the database directly.

**Safe default:** I'll assume <500ms unless you specify otherwise.

**To override:** Tell me the actual latency requirement.
```

**Key distinction:** If a safe default exists, use it and document the assumption. Only escalate if no safe default exists OR the decision is irreversible.

---

## Category 3: Unsafe Publish Boundary

**What it is:** Something is preventing safe publication but work can continue locally.

**Examples:**

- Secrets detected in staged content
- Unexpected files in the change surface
- Failed security scan with unclear remediation
- Branch protection preventing push

**Why agents can't decide:** Publishing decisions have external consequences. The agent can't unilaterally decide to expose or not expose something.

**Pattern:**

```markdown
## Publishing Blocked

**Issue:** Secrets scan detected an API key in `config.example.json`.

**Status:** Work continues locally. Publishing is paused.

**To unblock:**

1. Remove the key from the file, OR
2. Confirm this is an intentional example key (not real)

**Next:** After you address this, I'll retry the publish.
```

---

## The Escalation Format

Good escalations are:

- **Specific:** Exactly what's needed
- **Bounded:** Clear options, not open-ended
- **Actionable:** Human can respond with minimal effort
- **Contextual:** Enough background to decide

### Bad Escalation

```
I'm stuck. What should I do?
```

This forces the human to diagnose the problem, understand the context, generate options, and make a decision. Maximum cognitive load.

### Good Escalation

```markdown
## Needs Decision: Database Choice

**Context:** The feature requires persistent storage. Current stack uses
PostgreSQL but this feature has document-shaped data.

**Options:**

1. PostgreSQL with JSONB columns (consistent stack, good enough)
2. MongoDB (better fit for data shape, new dependency)
3. SQLite for MVP (simplest, migrate later)

**Recommendation:** Option 1 unless you have strong opinions on NoSQL.

**To proceed:** Reply with your choice or "go with recommendation."
```

This presents the problem, offers solutions, gives a recommendation, and makes responding trivial.

---

## Escalation Timing

### Escalate Early for Architecture

If you discover a decision that will affect the entire implementation, escalate before building. Don't build 2000 lines on an assumption that might be wrong.

**Example:** "I'm about to implement auth. The spec doesn't say whether we need SSO. If we do, I should use a different library. Should I assume no SSO, or do you need SSO support?"

### Escalate Late for Details

If you can make reasonable progress with a safe default, keep going. Document the assumption. Escalate only if it becomes a blocker.

**Example:** "I assumed a 30-second timeout for the background job. If you need a different value, I can change it in `config/jobs.yaml`."

### Never Escalate for Fixable Issues

Don't ask humans to fix lint errors, resolve import conflicts, or debug test failures. Route to the appropriate agent:

- Code issues: `fixer`
- Test issues: `test-author`
- Design conflicts: `design-optioneer`
- Git problems: `repo-operator`

---

## Anti-Patterns

### Permission-Seeking

**Wrong:**

```
Should I read the config file?
May I run the tests?
Is it okay to refactor this function?
```

**Right:**
Do the work. Report what you did. Escalate only for genuine decisions.

Agents are authorized to explore, read, and act within their role. They don't need permission to do their job.

### False Uncertainty

**Wrong:**

```
I'm not sure if I should add error handling to this function.
What do you think?
```

**Right:**
Add the error handling. It's a technical decision within your scope. If there's a specific error-handling pattern in the codebase, follow it. If not, use a reasonable default and document it.

### Escalating Routeable Problems

**Wrong:**

```
The tests are failing. What should I do?
```

**Right:**
Investigate why. If it's a code bug, fix it or route to `fixer`. If it's a test bug, route to `test-author`. If it's a design conflict, route to `design-optioneer`. Only escalate if none of these apply.

### Open-Ended Questions

**Wrong:**

```
How should we handle authentication?
```

**Right:**

```
## Authentication Approach

**Current state:** No auth exists. Users are anonymous.

**Options:**
1. Session-based auth (simpler, stateful)
2. JWT tokens (stateless, better for APIs)
3. OAuth with GitHub (external provider)

**I recommend:** Option 2 based on the API-first architecture in the ADR.

**To proceed:** Confirm or choose differently.
```

---

## The NEEDS_HUMAN Contract

When an agent truly needs human input, follow this contract:

1. **State the need clearly** — Not "I'm stuck" but "I need X to proceed"
2. **Provide options** — Not "what should I do?" but "here are the choices"
3. **Give a recommendation** — Guide the human toward the best option
4. **Specify what's blocked** — What can continue vs what's waiting
5. **Make responding easy** — "Reply 1, 2, or 3" not "explain your philosophy"

### What Can Continue vs What's Blocked

Often, only part of the work is blocked. Be specific:

```markdown
## Partially Blocked

**Blocked:** REQ-004 (payment integration) — need to know which provider

**Continuing:** REQ-001, REQ-002, REQ-003 (user management, profiles, settings)

**To unblock REQ-004:** Tell me: Stripe, PayPal, or both?
```

This lets the human know the flow isn't stopped—just one piece is waiting.

---

## Cost-Benefit Analysis

Before escalating, ask:

| Question                                                         | If Yes              | If No               |
| ---------------------------------------------------------------- | ------------------- | ------------------- |
| Can I derive this from codebase patterns?                        | Do that             | Continue...         |
| Is there a safe, reversible default?                             | Use it, document it | Continue...         |
| Can another agent handle this?                                   | Route to them       | Continue...         |
| Will the human's answer significantly change the implementation? | Maybe escalate      | Don't escalate      |
| Is this decision irreversible?                                   | Maybe escalate      | Default and move on |

Only when you reach the end of this chain AND the answer matters AND it's not derivable should you escalate.

---

## Examples in Practice

### Example: Missing API Spec

**Situation:** The requirements say "integrate with the inventory API" but don't specify the endpoint structure.

**Don't escalate:** Search for existing API clients in the codebase. Check if there's an OpenAPI spec. Look at test fixtures. Check documentation folders.

**If found:** Use the pattern. Document where you found it.

**If not found:** Define a reasonable interface. Document it. Note that it's assumed and can be updated when the real spec is available.

**Escalate only if:** The integration is the core of the feature and guessing wrong would require a complete rewrite.

### Example: Design Trade-off

**Situation:** You could implement the feature with a simple approach (fast, less flexible) or a complex approach (slower, more extensible).

**Don't escalate:** Check the ADR for guidance on extensibility vs simplicity. Look at similar features in the codebase. Consider the stated NFRs.

**If clear from context:** Follow the established pattern.

**If genuinely ambiguous:** Default to the simpler approach (reversible), document the trade-off, and note that it can be refactored if extensibility becomes a requirement.

**Escalate only if:** Both approaches have significant, irreversible consequences and the spec provides no guidance.

### Example: Security Concern

**Situation:** You notice the feature as specified would expose user data.

**Do escalate:** Security decisions require human judgment. But be specific:

```markdown
## Security Review Needed

**Issue:** REQ-007 as written would expose email addresses in the public API.

**Risk:** PII exposure, potential GDPR violation.

**Options:**

1. Hash emails before exposing (preserves functionality, protects PII)
2. Require authentication for this endpoint
3. Remove the email field from the response

**My recommendation:** Option 1 unless you have compliance requirements that suggest otherwise.
```

---

## Summary

**Escalation is expensive.** Exhaust other options first.

**But when you escalate, make it easy to respond.** Be specific, provide options, give a recommendation, and minimize the human's cognitive load.

**The goal:** Agents that solve problems themselves 95% of the time, and escalate clearly and effectively the other 5%.

---

## See Also

- [authority-not-difficulty.md](authority-not-difficulty.md) — The critical DEFAULTED vs NEEDS_HUMAN distinction
- [agent-philosophy.md](agent-philosophy.md) — Research-first autonomy
- [architecture.md](architecture.md) — Law 5: Research-First Autonomy
- [contracts.md](../reference/contracts.md) — Handoff patterns
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
