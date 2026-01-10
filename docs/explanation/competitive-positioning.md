# Competitive Positioning

> We're not competing on capability. We're competing on trust. And trust requires evidence.

---

## The Landscape

The AI coding tool market has three established categories. This pack creates a fourth.

---

## Category 1: Copilots

**Examples:** GitHub Copilot, Cursor, Windsurf, Codeium

**Model:** "Autocomplete on steroids." AI as an IDE plugin that suggests code as you type.

**Strengths:**
- Low latency (instant suggestions)
- Low friction (just keep typing)
- Good for small, obvious completions
- Tight IDE integration

**Limitations:**
- Synchronous (requires human driver)
- Session-bound (no persistence)
- No verification (suggestions may be wrong)
- No architecture (file-by-file view)

**When they win:** Quick completions, boilerplate, obvious patterns.

**When they lose:** Large features, complex refactors, overnight work.

### AgOps Comparison

Copilots are **synchronous assistants**. AgOps is an **asynchronous team**.

You can't tell Copilot "build this feature while I sleep." You can tell AgOps.

The fundamental difference: Copilots augment the human in the loop. AgOps operates with the human at the review boundary. One requires continuous attention; the other requires only periodic review.

---

## Category 2: Black Box Employees

**Examples:** Devin, Factory, OpenDevin, AutoDev

**Model:** "The AI Engineer." Give it a ticket, get a PR. The process is opaque.

**Strengths:**
- End-to-end automation
- Minimal human involvement
- Impressive demos
- "Just works" (when it works)

**Limitations:**
- Opaque process (can't see what happened)
- Trust without evidence
- Hard to debug failures
- Compliance nightmare

**When they win:** Simple, well-specified tasks in low-risk contexts.

**When they lose:** Regulated industries, complex systems, anything requiring audit.

### AgOps Comparison

Black boxes rely on **capability** ("I'm smart, trust me").
AgOps relies on **verification** ("Here's the evidence, you decide").

A bank can't hire a black box. They need the forensic ledger.

The core difference is epistemological. Black boxes ask you to trust their process. AgOps shows you what happened at every step. When something goes wrong, black boxes offer apologies. AgOps offers artifacts, receipts, and decision memos that explain exactly where and why.

---

## Category 3: Frameworks

**Examples:** LangChain, LangGraph, AutoGen, CrewAI

**Model:** "LEGO bricks." SDKs for building your own agent systems.

**Strengths:**
- Maximum flexibility
- Build exactly what you need
- Good for novel applications
- Active communities

**Limitations:**
- Blank page problem
- No opinions = endless decisions
- Quality depends on builder
- Easy to build badly

**When they win:** Custom applications, research, novel use cases.

**When they lose:** Teams that need to ship, not experiment.

### AgOps Comparison

Frameworks are **toolkits**. AgOps is a **factory**.

Frameworks say "here are tools, good luck." AgOps says "here's a working pipeline with verified output."

The trade-off is clear: frameworks give you infinite flexibility and zero guardrails. You can build anything, including all the failure modes that AgOps explicitly prevents (confabulation, scope drift, hidden assumptions). AgOps is opinionated precisely because unguided agent systems fail in predictable ways.

---

## Category 4: AgOps (This Pack)

**Model:** "Industrial operating system for verified software." An opinionated pipeline that produces evidence, not just code.

**Core thesis:**
- LLMs are untrusted components
- Verification is the product
- Evidence enables review
- Boundaries enable safety

**Strengths:**
- Transparent process (full audit trail)
- Evidence-based output (receipts, decision memos)
- Verification built-in (tests, critics, gates)
- Enterprise-viable (compliance, audit)
- Asynchronous (fire and forget)

**Limitations:**
- Learning curve (new mental model)
- Setup investment (pack customization)
- Overkill for trivial tasks
- Requires embracing the paradigm

**When it wins:**
- Large, complex features
- Regulated environments
- High-quality requirements
- Asynchronous workflows
- Teams that value verification

**When it loses:**
- Quick one-liners
- Exploratory prototypes
- Teams unwilling to change workflow

---

## The Positioning Matrix

| Factor | Copilots | Black Boxes | Frameworks | AgOps |
|--------|----------|-------------|------------|-------|
| Speed to first output | Instant | Hours | Days | Hours |
| Transparency | Low | None | Variable | High |
| Verification | None | Claimed | DIY | Built-in |
| Compliance-ready | No | No | DIY | Yes |
| Async capable | No | Yes | DIY | Yes |
| Scale of change | Small | Medium | Any | Large |
| Setup cost | Low | Low | High | Medium |

---

## The Differentiator

AgOps wins by admitting what others hide: **LLMs are untrustworthy.**

While competitors race to claim "our model doesn't hallucinate," AgOps assumes:

> "The model WILL hallucinate, and here is the architectural cage that catches it."

This is why:
- Evidence surfaces exist (claims must have pointers)
- Critics attack every output (author-critic microloops)
- Gates require proof (verification, not vibes)
- Receipts preserve truth (audit artifacts, not chat logs)

### The Cage, Not the Claim

Other tools market capability. We market containment.

The question isn't "how smart is your AI?" The question is "what happens when your AI is wrong?"

- Copilots: The human catches it (hopefully)
- Black boxes: Nobody knows (until production)
- Frameworks: Depends on how you built it
- AgOps: The critic catches it, the gate blocks it, the receipt explains it

---

## Who Should Use What

**Use Copilots when:**
- You're actively coding
- Tasks are small and obvious
- You want acceleration, not automation

**Use Black Boxes when:**
- Tasks are simple and low-risk
- You don't need audit trails
- "Just works" is acceptable

**Use Frameworks when:**
- You're building something novel
- You have time to experiment
- You want maximum control

**Use AgOps when:**
- Features are large and complex
- Verification matters
- You need to ship with confidence
- Compliance/audit is required
- You want to review evidence, not lines

---

## The Trust Spectrum

All four categories can be placed on a trust spectrum:

```
← Requires human attention                        Autonomous →

    Copilots          Frameworks         AgOps         Black Boxes
       |                  |                |                |
   "I'll help        "Build your       "I'll work,     "I'll handle
    while you         own trust         you'll           everything,
    drive"            model"            review"          trust me"
```

AgOps occupies a specific position: **autonomous but accountable**. It works without you, but it doesn't ask you to trust without evidence.

---

## The Enterprise Lens

For enterprise buyers, the choice often comes down to:

| Concern | Copilots | Black Boxes | Frameworks | AgOps |
|---------|----------|-------------|------------|-------|
| SOC 2 / compliance | Vendor handles | Unknown | Build it yourself | Receipts exist |
| Audit trail | Chat logs | Opaque | DIY | Forensic ledger |
| Incident investigation | Ask the dev | Ask the vendor | Dig through code | Read the artifacts |
| Explainability | "AI suggested it" | "AI did it" | "We built it this way" | "Here's the evidence" |

AgOps isn't just a different tool. It's a different answer to "what happened?"

---

## Why "AgOps"?

The name signals the category:

- **DevOps** = Development + Operations (culture/process for shipping)
- **MLOps** = ML + Operations (culture/process for models)
- **AgOps** = Agents + Operations (culture/process for agent systems)

It's not about the AI being smart. It's about the system being operational.

---

## The Key Insight

> We're not competing on capability. We're competing on trust. And trust requires evidence.

Every competitor asks: "How capable is the AI?"
We ask: "How do you know what happened?"

Capability without verification is a liability. The smarter the AI, the more convincing its confabulations. The only defense is architectural: gates that require proof, critics that attack output, receipts that preserve truth.

AgOps is the bet that enterprise software development needs **verification infrastructure**, not just smarter autocomplete.

---

## See Also

- [claims-and-evidence.md](claims-and-evidence.md) — Why evidence pointers matter
- [ai-physics.md](ai-physics.md) — LLM failure modes this design addresses
- [what-makes-this-different.md](what-makes-this-different.md) — Assumptions that don't apply here
- [CLAUDE.md](../../CLAUDE.md) — Pack reference
