# Principle: Local Resolution

> Exhaust local options before escalating.

## The Principle

When agents encounter problems, they should try to resolve them locally before bouncing to a previous flow or escalating to a human. Machine time is cheap; human attention and flow restarts are expensive.

## Why This Matters

### Flow Restarts Are Expensive

Bouncing from Build back to Plan means:

- Losing build progress
- Re-running design work
- Context reset
- Human re-engagement

A local fix takes minutes. A bounce takes hours.

### Most Problems Are Local

The implementation doesn't match the spec? Usually:

- Misread the spec (re-read it)
- Missed a detail (add it)
- Made a mistake (fix it)

Rarely: "The entire architecture is wrong."

### The Stubborn PM

A good PM doesn't give up at the first obstacle:

- "The design doesn't say how to handle this edge case"
- -> Make a reasonable choice, document it, continue
- Not: -> Bounce to design, halt everything

### Human Time Is Precious

Every bounce that reaches a human should be worth their attention:

- Genuine architectural issues
- Business/product decisions
- Trade-offs requiring judgment

Not:

- "I wasn't sure about a detail"
- "The spec was slightly ambiguous"
- "I need permission to make a choice"

## How It Works

### The Resolution Ladder

When encountering a problem, try in order:

1. **Can I figure it out?**
   - Re-read the spec
   - Check related code
   - Look at tests for intent
   - -> If yes: proceed with understanding

2. **Can I make a reasonable choice?**
   - Document the assumption
   - Implement the sensible default
   - Flag it for review
   - -> If yes: proceed with documented assumption

3. **Can a specialist agent help?**
   - Route to design-optioneer for micro-decision
   - Route to impact-analyzer for scope assessment
   - -> If yes: get answer, continue locally

4. **Is this truly an upstream problem?**
   - Architecture fundamentally broken
   - Requirements contradictory
   - Human decision required
   - -> If yes: bounce with clear explanation

### Investigate -> Derive -> Default -> Escalate

**Investigate:** Look for the answer in existing artifacts
**Derive:** Figure it out from context and patterns
**Default:** Make a reasonable choice, document it
**Escalate:** Only when above options are exhausted

### Write-Through on Local Fixes

When a specialist (design-optioneer, impact-analyzer) resolves a snag locally:

- Update the relevant plan artifact immediately
- Edit `adr.md`, `work_plan.md`, or `ac_matrix.md`
- The fix survives context resets

Don't just "remember" the fix — write it down.

## Examples

### Good: Local Resolution

**Problem:** Spec says "session timeout" but doesn't specify duration.

**Local fix:**

1. Check codebase for existing timeouts -> Found 15m used elsewhere
2. Document assumption: "Using 15m consistent with existing patterns"
3. Implement with 15m
4. Flag in handoff: "Assumed 15m timeout; verify this is correct"

**No bounce needed.**

### Good: Specialist Assist

**Problem:** Implementation reveals the API design can't meet latency requirements.

**Local fix:**

1. Route to design-optioneer: "Can we adjust the API to batch requests?"
2. Optioneer proposes micro-redesign (batch endpoint)
3. Optioneer updates api_contracts.yaml
4. Implementation continues with batched approach

**No flow bounce. Design adapted locally.**

### Bad: Premature Bounce

**Problem:** "The spec doesn't explicitly say what error code to return."

**Premature bounce:** "Bouncing to Signal to clarify requirements."

**Should have:** Checked HTTP conventions, used 400 for client errors, documented the choice.

### Justified Bounce

**Problem:** "Requirements say 'real-time sync' but also 'offline-first'. These contradict."

**Investigation:** Checked all artifacts. No resolution.
**Derivation:** Can't reasonably choose — fundamentally different architectures.
**Escalation:** "Bouncing to Signal. REQ-001 and REQ-003 contradict. Need product decision on sync model."

**This bounce is earned.**

## Anti-Patterns

### X First-Obstacle Bounce

Hit any ambiguity -> immediately bounce upstream.

**Fix:** Try the resolution ladder first.

### X Guess and Pray

Make arbitrary choices without documenting them.

**Fix:** Document assumptions explicitly. Flag for review.

### X Silent Assumptions

Make choices but don't tell anyone.

**Fix:** Every assumption in the handoff. "I assumed X because Y."

### X Infinite Local Loops

Keep trying locally when genuinely stuck.

**Fix:** Recognize when escalation is needed. "Stubborn" has limits.

## The Handoff on Local Resolution

When you resolved locally:

```markdown
## Handoff

**What I did:** Implemented the auth endpoint.

**Assumptions made:**

- Session timeout: 15m (consistent with existing code)
- Error format: RFC 7807 problem details (matches other endpoints)
- Rate limit: 100 req/min (common default, flagged for review)

**Recommendation:** Review assumptions during PR. Proceed to tests.
```

When you need to bounce:

```markdown
## Handoff

**What I tried:**

- Checked spec: no guidance on sync model
- Checked codebase: no existing pattern
- Asked design-optioneer: confirmed this is architectural

**Why I'm stuck:** REQ-001 (real-time) and REQ-003 (offline-first)
require contradictory architectures. Can't proceed without product decision.

**Recommendation:** Bounce to Signal. Clarify sync requirements before design.
```

## See Also

- [Graceful Outcomes](graceful-outcomes.md) — Reporting stuck states
- [The Microloop](../../how-to/working-with-microloops.md) — Iteration before escalation
