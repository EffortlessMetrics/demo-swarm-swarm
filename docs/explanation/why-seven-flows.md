# Why Seven Flows

> Seven flows because that's where the natural boundaries are. Fewer would blur concerns. More would add overhead.

---

## The Question

Why seven flows? Why these seven? Why not five, or ten, or a different breakdown?

This is not arbitrary. The seven flows emerge from the natural structure of software development work. They encode where thinking changes, where handoffs make sense, and where humans want to check in.

---

## The Design Principles

### 1. Natural Boundaries

Each flow ends at a natural checkpoint:
- Work is complete for that phase
- Artifacts are coherent and useful
- Human feedback makes sense here
- Resume is clean if interrupted

**Test:** If you cannot describe a sensible "pause here" moment, the flow boundary is wrong.

### 2. Single Concern

Each flow has one primary job:

| Flow | Concern |
|------|---------|
| Signal | Understand the problem |
| Plan | Design the solution |
| Build | Implement the solution |
| Review | Address feedback |
| Gate | Decide to merge |
| Deploy | Ship to production |
| Wisdom | Learn from the run |

**Test:** If describing the flow requires "and," consider splitting it.

### 3. Human Touchpoints

Flow boundaries are where humans naturally want to:
- Check progress ("What did we build?")
- Provide feedback ("This requirement is wrong")
- Make decisions ("Should we ship this?")
- Redirect if needed ("Stop, we need to rethink")

**Test:** If humans would never pause here, the boundary adds no value.

---

## The Seven Flows

### Flow 1: Signal

**Why it exists:** You cannot build what you do not understand.

**What it produces:**
- Problem statement (what are we solving?)
- Requirements (what must be true?)
- BDD scenarios (how do we test it?)
- Early risks (what might go wrong?)

**Why it is separate:** Understanding and designing are different cognitive modes. Rushing to design before understanding causes expensive rework. Signal forces the question: "Do we actually know what we are building?"

**Natural boundary:** "We understand the problem. Ready to design?"

---

### Flow 2: Plan

**Why it exists:** Architecture decisions before code prevent expensive pivots.

**What it produces:**
- ADR (how will we build it?)
- Contracts (what are the interfaces?)
- Work breakdown (what are the pieces?)

**Why it is separate:** Designing and implementing require different mental postures. Design requires breadth---surveying options, considering trade-offs. Implementation requires depth---making one approach work. Mixing them produces shallow design and distracted implementation.

**Natural boundary:** "We have a plan. Ready to build?"

---

### Flow 3: Build

**Why it exists:** Implementation is the core work.

**What it produces:**
- Code that satisfies acceptance criteria
- Tests that verify behavior
- Documentation that explains
- Self-review that catches obvious issues

**Why it is separate:** Building is inherently iterative. The writer/critic microloop runs many times: implement, test, fix, test again. This iteration needs room to fail, fix, and retry without the overhead of external review. Build contains the messy work so other flows stay clean.

**Natural boundary:** "Code is ready for external review."

---

### Flow 4: Review

**Why it exists:** External feedback catches what self-review misses.

**What it produces:**
- Feedback harvested from CI, bots, and humans
- Issues addressed systematically
- PR polished for merge
- Confidence that concerns are resolved

**Why it is separate:** Harvesting and addressing feedback is fundamentally different from building. Building is generative---creating something new. Review is reactive---responding to what others found. The mindset shift matters.

**Natural boundary:** "All feedback addressed. Ready for merge decision?"

---

### Flow 5: Gate

**Why it exists:** Someone must decide: ship or not.

**What it produces:**
- Merge decision (go/no-go with reasoning)
- Evidence synthesis (what supports the decision)
- Fix-forward for mechanical issues (when warranted)

**Why it is separate:** Deciding is not building or reviewing. It requires synthesis across all the evidence, judgment about risk and readiness, and authority to commit. A dedicated Gate flow makes this decision explicit rather than implicit.

**Natural boundary:** "Decision made. Merged or bounced."

---

### Flow 6: Deploy

**Why it exists:** Merged code is not shipped code.

**What it produces:**
- Deployment verification (did it reach production?)
- Release notes (what shipped?)
- Monitoring confirmation (is it healthy?)

**Why it is separate:** Deployment has different failure modes than development. Production environment, real users, real consequences. The verification criteria are different: not "does the code work?" but "is it working in production?"

**Natural boundary:** "Deployed and verified."

---

### Flow 7: Wisdom

**Why it exists:** Continuous improvement requires reflection.

**What it produces:**
- Learnings extracted from the run
- Templates improved based on friction
- Patterns documented for future work

**Why it is separate:** Reflection happens after the work, not during. It requires distance from the immediate task and perspective across multiple runs. Wisdom is how the system gets smarter, not just how this feature ships.

**Natural boundary:** "Run complete. Improvements captured."

---

## Why Not Fewer?

### Combining Signal + Plan

**What you would get:** Requirements and design blur together.

**What breaks:**
- Plans based on incomplete understanding
- Requirements that bake in design assumptions
- Harder to challenge either in isolation
- "But we already designed it" blocks requirement changes

**The failure mode:** Teams discover mid-build that they misunderstood the problem, but the design is already committed.

### Combining Build + Review

**What you would get:** Self-review bias.

**What breaks:**
- Builders miss their own mistakes (they know what they meant)
- Feedback becomes defensive (challenging my work)
- No clean handoff point for external eyes

**The failure mode:** "I reviewed my own code and it looks fine" ships bugs.

### Combining Review + Gate

**What you would get:** The person addressing feedback also decides to ship.

**What breaks:**
- Pressure to declare "done" after fixing things
- No independent judgment on readiness
- "I fixed everything" becomes "therefore we ship"

**The failure mode:** Merge decisions become rubber stamps.

### Combining Gate + Deploy

**What you would get:** Merge decision and production deployment blur.

**What breaks:**
- No pause point between "approved" and "shipped"
- Cannot easily roll back the decision
- Production issues conflated with gate issues

**The failure mode:** "The gate passed" used to explain production incidents.

---

## Why Not More?

### Splitting Build into Implement / Test / Document / Self-Review

**Temptation:** These are distinct activities. Maybe they should be distinct flows.

**Why not:** They iterate together in tight microloops. Write code, write test, discover issue, fix code, fix test, add documentation, review, find issue, fix again. Splitting into flows would add coordination overhead at every cycle. The iteration is the point.

**The cost:** Build is the longest flow. But the alternative---four flows that constantly hand off to each other---is worse.

### Separate Gates for Each Concern (Security Gate, Quality Gate, Compliance Gate)

**Temptation:** Different experts for different concerns. Maybe security should have its own flow.

**Why not:** One synthesized decision is better than multiple fragmented ones. A dedicated Gate flow integrates all concerns: security findings, test results, code quality, compliance checks. The gatekeeper weighs everything together. Multiple gates create confusion about who actually decides.

**The cost:** Gate must understand all concerns. But the alternative---three gates that might disagree---is worse.

### Separate Flow for Hotfix vs Feature

**Temptation:** Hotfixes need speed. Features need thoroughness. Maybe they should be different.

**Why not:** They are the same flows with different scope. A hotfix is Signal (small) -> Build (small) -> Gate (fast) -> Deploy (urgent). The flows adapt to the work. Separate "hotfix flows" would duplicate everything and drift over time.

**The cost:** Flows must handle both modes. But the alternative---parallel flow definitions---is worse.

---

## The Flow Boundaries

| After Flow | Human Checkpoint | Key Question |
|------------|------------------|--------------|
| Signal | Requirements review | "Is this the right problem?" |
| Plan | Design review | "Is this a good approach?" |
| Build | Self-review complete | "Is it ready for external eyes?" |
| Review | Feedback addressed | "Is it ready for merge decision?" |
| Gate | Merge decision | "Does this ship?" |
| Deploy | Deployment verified | "Is it working in production?" |
| Wisdom | Run retrospective | "What did we learn?" |

Each boundary answers a question. If the answer is unclear, the work is not ready to cross.

---

## The Rhythm

The seven flows create a rhythm:

```
1. Understand (Signal)      - What are we doing?
2. Design (Plan)            - How will we do it?
3. Build (Build + Review)   - Make it work, make it right
4. Decide (Gate)            - Should we ship?
5. Ship (Deploy)            - Put it in production
6. Learn (Wisdom)           - What did we learn?
```

This is the natural rhythm of software development, encoded into explicit flows.

**Understand -> Design -> Build -> Decide -> Ship -> Learn.**

The rhythm is not invented. It is observed. Every software project follows this pattern, whether explicitly or implicitly. The seven flows make the pattern explicit.

---

## The Key Insight

**Seven flows because that's where the natural boundaries are.**

Fewer would blur concerns:
- Understanding mixed with designing
- Building mixed with reviewing
- Deciding mixed with shipping

More would add overhead:
- Handoffs where iteration should flow
- Gates where speed matters
- Coordination where focus is needed

The seven flows are not arbitrary. They emerge from the structure of software work itself. Change the number and you either blur important distinctions or add unnecessary friction.

---

## See Also

- [flow-composition.md](flow-composition.md) --- How flows chain and depend on each other
- [architecture.md](architecture.md) --- Core philosophy and patterns
- [laws-of-the-swarm.md](laws-of-the-swarm.md) --- The immutable rules
- [why-ops-first.md](why-ops-first.md) --- Why gates engage only at boundaries
