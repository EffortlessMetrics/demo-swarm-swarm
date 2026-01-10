# Throughput Inversion

> Generation is cheap. Review is expensive. The entire system design follows from this inversion.

This documents the core economic reality that drives everything about how this system is designed.

---

## The Inversion

**Traditional software development:**
- **Bottleneck:** Writing code (human typing speed)
- **Cheap:** Reviewing code (just reading)

**AI-assisted development:**
- **Cheap:** Generating code (1000+ tokens/second)
- **Bottleneck:** Reviewing/trusting code (human attention)

The bottleneck has inverted. This changes everything.

---

## The Numbers

### Generation Speed

| Method | Speed | Cost |
|--------|-------|------|
| Human typing | ~50 words/min | $50-150/hr |
| LLM generation | ~1000 tokens/sec | ~$0.01/1k tokens |

LLMs generate 100-1000x faster than humans type.

### Review Speed

| Method | Speed | Cost |
|--------|-------|------|
| Human reading code | ~100-200 LOC/hr | $50-150/hr |
| LLM reading code | Instant | ~$0.01/1k tokens |

Humans review at roughly the same speed as before. This is the bottleneck.

---

## The Implications

### 1. Generation Is (Nearly) Free

If you can generate 10 variations and pick the best, do it.
If you can regenerate instead of debugging, do it.
If you can over-generate and filter, do it.

Token cost is noise compared to human time cost.

### 2. Review Is The Constraint

Every line a human reads costs real money.
Every decision a human makes costs real attention.
Every context switch costs real cognitive overhead.

Optimize for minimal human review time.

### 3. Trust Becomes The Product

Since generation is cheap and review is expensive, the valuable thing is:
- Not "code" (cheap to generate)
- But "trusted code" (expensive to verify)

The system produces trust, not code.

---

## Design Consequences

### Spend Machine Time, Save Human Time

| Old Thinking | New Thinking |
|--------------|--------------|
| Generate once, debug if wrong | Generate many, pick best |
| Review every line | Review evidence summaries |
| Human runs tests | Machine runs tests, human reads results |
| Human finds issues | Critics find issues, human reviews findings |

### Evidence Over Reading

Since humans can't read everything:
- Produce evidence that verification happened
- Summarize into reviewable chunks
- Highlight what matters (hotspots)
- Make "not measured" explicit

### Verification Loops Are Cheap

Running the code through:
- Tests (machine time)
- Critics (machine time)
- More tests (machine time)
- More critics (machine time)

All cheap. Human review at the end: expensive. Front-load the cheap work.

---

## The Trust Latency Metric

**Trust latency:** How long from "change ready" to "change trusted"

Traditional: Limited by review scheduling, human reading speed
AgOps: Limited by evidence generation, human evidence review

Inversion goal: Make trust latency proportional to evidence review, not code review.

---

## Practical Application

### Do This

- Generate multiple approaches, evaluate programmatically
- Run extensive verification (tests, critics, scans)
- Produce rich evidence (receipts, summaries, hotspots)
- Present compressed review surfaces (PR cockpit)

### Don't Do This

- Generate once and hope
- Skip verification to save tokens
- Dump raw output for human review
- Require humans to read every line

---

## The Meta-Point

The inversion isn't just about speed. It's about what's valuable.

**Before:** Human creativity in writing was valuable, reading was cheap
**After:** Machine generation is cheap, human attention is valuable

Design for the new reality. Spend cheap resources (tokens, machine time) to conserve expensive resources (human attention, trust latency).

---

## See Also

- [Economics](economics.md) — The full ROI math and DevLT optimization
- [AI Physics](ai-physics.md) — LLM failure modes this design addresses
- [PR as Review Surface](pr-as-review-surface.md) — Why evidence compression matters
- [Claims and Evidence](claims-and-evidence.md) — How we produce trust, not just code
