# Code as Binary

> We don't read the assembly. We read the specs, the results, the summaries. And we ship with confidence.

---

## The Paradigm Shift

Traditional development follows a hands-on sequence:

**Write code -> Test code -> Review code -> Ship code**

Every line is authored by a human. Every line is read by a reviewer. Quality depends on human attention at every step.

AgOps development inverts this:

**Write specs -> Generate code -> Verify code -> Review evidence -> Ship**

Humans define intent. Machines generate and verify. Humans review the evidence and make the call.

This isn't automation. It's a change in what we consider source.

---

## Code Is the Binary

We used to program in assembly. Then we moved to C. Then Python. Now we move again.

### The Abstraction Ladder

| Era | Source | Compiler | Binary |
|-----|--------|----------|--------|
| 1960s | Assembly | Assembler | Machine code |
| 1980s | C | cc | Assembly |
| 2000s | Python | Interpreter | Bytecode |
| Now | **Specs** | **Swarm** | **Implementation** |

### What This Means

**Source code:** BDD scenarios, requirements, ADRs, API contracts

These artifacts capture intent. They're human-authored, human-reviewed, and human-owned. They define what we're building and why.

**Compiler:** The swarm (Signal -> Plan -> Build flows)

The swarm transforms intent into implementation. It explores the codebase, makes implementation decisions, writes code, writes tests, and verifies results.

**Binary:** Implementation code (Rust/Python/TypeScript)

The output. Just like we don't read x86 instructions to understand what a program does, we don't read every line of generated code. We read the specs and verify the evidence.

### The Key Insight

You don't read the binary. You read the specs and the results.

When a C program misbehaves, you don't debug the assembly output. You debug the C source. When generated code misbehaves, you don't debug the TypeScript output. You debug the specs or the evidence.

---

## The Assembly Line

Work moves through a conveyor belt of specialized stations. Each station transforms input into output. Each station has specialists who do one thing well.

```
Feature Idea
    |
    v
[Signal] --> Requirements, BDD scenarios
    |
    v
[Plan] --> ADR, contracts, work breakdown
    |
    v
[Build] --> Code + tests + verification
    |
    v
[Review] --> Critics find issues, fixes applied
    |
    v
[Gate] --> Ship/no-ship decision with evidence
    |
    v
Trusted Artifact
```

### Station Specialization

| Station | Specialists | Output |
|---------|-------------|--------|
| **Signal** | requirements-author, requirements-critic | Testable requirements, BDD scenarios |
| **Plan** | design-optioneer, plan-critic | ADR, API contracts, work breakdown |
| **Build** | code-implementer, test-author, code-critic | Working code, tests, critiques |
| **Review** | pr-feedback-harvester, fixer | Addressed feedback, clean code |
| **Gate** | merge-decider, secrets-sanitizer | Ship/no-ship decision with evidence |

### The Developer's View

The developer sees the finished product at the end. They don't stand at each station watching parts assemble. They review the final artifact with its evidence trail.

This is exactly how manufacturing works. The engineer designs the product, reviews quality reports, and approves shipment. They don't watch every weld.

---

## Shift Left

Catch issues early by front-loading quality. Every problem found earlier is cheaper to fix.

### The Cost Curve

| Phase | Cost to Fix |
|-------|-------------|
| Requirements | Cheap (change a sentence) |
| Design | Moderate (update the ADR) |
| Implementation | Expensive (rewrite code) |
| Review | Very expensive (code already exists) |
| Production | Catastrophic (users affected) |

### BDD First

Before writing code, write the behavior.

**Scenarios define expected behavior:**
```gherkin
Scenario: User logs in with valid credentials
  Given a registered user with email "user@example.com"
  When the user submits valid credentials
  Then the user receives an authentication token
  And the user is redirected to the dashboard
```

**Scenarios are testable:**
The Given/When/Then structure maps directly to test setup, action, and assertion.

**Scenarios are human-readable:**
Stakeholders can validate behavior without reading code. Product managers can verify intent without understanding implementation.

### Tests Prove Intent

Tests don't just "check code works." They prove code matches intent.

The traditional view: "Tests verify the code is correct."
The shift-left view: "Tests prove the code implements the spec."

When tests pass, we have evidence that implementation matches intent. When tests fail, we have evidence of drift between spec and code.

### Mutation Testing

The extreme shift-left: prove tests actually test the code.

**The process:**
1. Inject mutations (break the code deliberately)
2. Run tests
3. Evaluate results

**The interpretation:**
- If tests fail on mutant: Tests catch the breakage (good)
- If tests pass on mutant: Tests don't actually verify that behavior (hollow)

**Example:**
```
Original: if (balance > 0) { allow_withdrawal(); }
Mutant:   if (balance >= 0) { allow_withdrawal(); }
```

If tests pass on the mutant, they exercise the code path but don't verify the `> 0` condition. The tests are hollow.

Mutation testing closes the loop: specs define intent, tests verify intent, mutation testing verifies tests.

---

## Breaking Down Large Ideas

Large dev ideas become individual workable pieces. The assembly line can't process a boulder; it needs gravel.

### The Decomposition Flow

1. **Capture intent**
   - Problem statement: What are we solving?
   - Success criteria: How do we know we're done?
   - Stakeholders: Who cares about this?

2. **Shape requirements**
   - REQ/NFR markers: Testable claims about behavior
   - Acceptance criteria: Specific, verifiable conditions
   - Edge cases: What happens when things go wrong?

3. **Write BDD scenarios**
   - Happy paths: Expected behavior
   - Error paths: Failure handling
   - Edge cases: Boundary conditions

4. **Plan work**
   - ADR: Design decisions and rationale
   - API contracts: Interface definitions
   - AC breakdown: Individual implementable units

5. **Build in slices**
   - One AC at a time
   - Verified as you go
   - Critics review each slice

6. **Review and gate**
   - Evidence-based decision
   - Ship or bounce with reasoning

### Agent-Sized Chunks

Each chunk should be:

**Small enough:** One agent can handle it without context exhaustion. If an AC requires exploring 50 files and making 20 changes, it's too big.

**Bounded enough:** Verification is fast. If testing an AC takes 30 minutes, it's too big to iterate on effectively.

**Clear enough:** Success is unambiguous. If you can't tell whether an AC is "done" without interpretation, it needs refinement.

The conveyor belt moves work through, piece by piece. Each piece is verified before the next begins.

---

## What the Dev Reads

The developer's attention is precious. Spend it on what matters.

### What the Dev Does NOT Read

- Every line of generated code
- Every test assertion
- Implementation details of routine changes
- Mechanical transformations (format fixes, import sorting)

Reading all generated code is like reading compiled assembly. Theoretically possible, practically wasteful, strategically wrong.

### What the Dev DOES Read

**Specs:** What we intended to build
- Requirements with REQ/NFR markers
- BDD scenarios defining behavior
- ADR explaining design decisions
- API contracts specifying interfaces

**Evidence:** Test results, critic findings, receipts
- Test execution summaries (47 passed, 0 failed)
- Critic findings (2 MAJOR issues found, addressed)
- Gate decisions (PASS: all evidence thresholds met)

**Analysis:** Summaries, hotspots, risk areas
- Implementation summary (what changed and why)
- Hotspots (where to look if spot-checking)
- Known limitations (what wasn't verified)

**Decisions:** Ship/no-ship memos with reasoning
- Merge decision with evidence citations
- Explicit gaps and their assessed risk
- Recommendation with rationale

### The Review Surface

Just like we don't read assembly anymore:

| Surface | Purpose | Audience |
|---------|---------|----------|
| **Specs** | Define intent | Human review (primary) |
| **Evidence** | Prove verification | Human audit |
| **Summaries** | Compress findings | Human review |
| **Code diff** | Spot-check/audit | Human when needed |

The PR description is the product. Evidence tables show verification. Hotspots guide spot-checks. The diff is for audit, not primary review.

---

## Why This Works

### Economic Logic

**Cost comparison:**

| Activity | Cost |
|----------|------|
| Code generation | Cheap (tokens) |
| Code verification | Cheap (machine time) |
| Human review of code | Expensive (attention) |
| Human review of evidence | Cheap (compressed) |

The equation is simple: spend machine iteration to buy human confidence.

A 2000-line change reviewed line-by-line costs hours of senior attention. The same change reviewed via evidence summary costs minutes. The quality is the same (often better, since machine verification is exhaustive). The attention cost is radically different.

**The trade:**
- Machine cycles are cheap and parallelizable
- Human attention is expensive and serialized
- Convert cheap cycles into expensive confidence

### Quality Logic

**Shift-left catches issues early:**
A requirement typo caught in Signal costs nothing. The same typo caught in production costs support tickets, hotfixes, and reputation.

**Adversarial loops find issues:**
Author vs critic opposition defeats sycophancy. The critic's job is to find problems. Problems get found.

**Evidence proves claims:**
"Tests passed" with a test report is verifiable. "Tests passed" without evidence is narrative. Evidence enables trust.

**Boundaries prevent escape:**
Gates at publish boundaries catch what slipped through. Secrets scanning prevents credential leaks. Review gates prevent premature merge.

### Scale Logic

**One dev can review many changes:**
When review is evidence-based, a developer can review 10 changes in the time it would take to review 1 line-by-line. Volume stops being the bottleneck.

**One change can be large:**
A 5000-line change is terrifying when reviewed line-by-line. It's manageable when reviewed via evidence summary. Size stops being the constraint.

**Quality doesn't degrade at volume:**
The quality comes from the system (verification, criticism, evidence), not from human heroics (late nights, caffeine, hope). The system doesn't get tired. The system doesn't miss things because it's Friday afternoon.

---

## The Economics in Detail

### Token Cost vs Attention Cost

Consider a typical feature implementation:

| Activity | Token Cost | Attention Cost |
|----------|------------|----------------|
| Generate code | ~50K tokens | 0 minutes |
| Run tests | ~5K tokens | 0 minutes |
| Critic review | ~20K tokens | 0 minutes |
| Fix issues | ~30K tokens | 0 minutes |
| Generate evidence | ~10K tokens | 0 minutes |
| **Human review of evidence** | 0 tokens | **5 minutes** |
| **Ship decision** | 0 tokens | **2 minutes** |

Total: ~115K tokens, 7 minutes of attention.

Compare to traditional:

| Activity | Token Cost | Attention Cost |
|----------|------------|----------------|
| Write code | 0 tokens | 120 minutes |
| Run tests | 0 tokens | 5 minutes |
| Self-review | 0 tokens | 30 minutes |
| Respond to review | 0 tokens | 45 minutes |
| **Total** | 0 tokens | **200 minutes** |

Tokens are cheap. Attention is expensive. The math is obvious.

### DevLT: The Real Metric

**DevLT (Developer Lead Time):** Minutes of human attention per trusted change.

This is what we optimize. Not lines of code. Not token efficiency. Not agent count.

A change that costs 100K tokens but requires 5 minutes of human review beats a change that costs 10K tokens but requires 30 minutes of human review.

---

## The Mindset Shift

### From Craftsman to Architect

The traditional developer is a craftsman. They shape each line of code with care. They take pride in the elegance of their implementations. They review every character.

The AgOps developer is an architect. They design the building. They review the quality reports. They approve the final structure. They don't lay every brick.

This isn't a demotion. It's an elevation. The architect's judgment matters more than the brick-layer's endurance.

### From Line-by-Line to Evidence-Based

Old question: "Does this code look right?"
New question: "Does the evidence show it works?"

Old review: Read every line, hope to spot issues.
New review: Check evidence table, spot-check hotspots if needed.

Old trust: "I read it carefully, so it's probably fine."
New trust: "Tests pass, critics found nothing major, evidence is complete."

### From Writing to Specifying

Old work: Write the code that implements the feature.
New work: Specify the behavior the feature should have.

The specification is the creative work. The implementation is mechanical. Let machines do mechanical work.

---

## Practical Implications

### What Changes for the Developer

**Before starting:**
- Define the problem clearly
- Write acceptance criteria
- Consider edge cases upfront

**During development:**
- Review evidence as it accumulates
- Respond to genuine ambiguity when agents escalate
- Make design decisions that agents can't derive

**At completion:**
- Review the evidence summary
- Spot-check hotspots
- Make the ship decision

**What doesn't change:**
- Ultimate responsibility for quality
- Design judgment
- Ship/no-ship authority

### What Changes for the Codebase

**Higher specification quality:**
Requirements must be testable. BDD scenarios must be complete. Contracts must be explicit. Vague intent produces vague implementations.

**More evidence artifacts:**
Receipts, test reports, critic findings, decision memos. The audit trail is rich. Future readers understand what happened.

**Larger individual changes:**
When verification is cheap, changes can be larger. When evidence is complete, large changes are reviewable.

---

## The Core Insight

> We don't read the assembly. We read the specs, the results, the summaries. And we ship with confidence.

The code is the binary. The specs are the source. The evidence is the proof.

When we stopped reading assembly, we didn't lose control. We gained abstraction. We could think about programs at a higher level. We could build larger systems.

When we stop reading every line of implementation, we don't lose control. We gain abstraction. We think about systems at the level of intent and evidence. We build with more confidence, not less.

The paradigm shift is real. The economics are compelling. The quality is provable.

Welcome to development where code is the binary.

---

## See Also

- [operating-model.md](operating-model.md) - How the system operates
- [pr-as-review-surface.md](pr-as-review-surface.md) - Evidence-based review
- [claims-and-evidence.md](claims-and-evidence.md) - Evidence discipline
- [adversarial-loops.md](adversarial-loops.md) - Author/critic opposition
- [what-makes-this-different.md](what-makes-this-different.md) - Breaking old assumptions
