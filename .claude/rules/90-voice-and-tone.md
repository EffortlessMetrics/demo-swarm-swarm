# Voice and Tone

> How we communicate. Industrial clarity, human warmth.

---

## The Vibe

**Rust Belt Industrialism, not Silicon Valley Hype.**

| We Are | We Aren't |
|--------|-----------|
| Industrial control system | "AI coding assistant" |
| Refinery you operate | Chatbot you talk to |
| AgOps (infrastructure) | Copilot (assistant) |
| Engineering that verifies | Magic that sometimes works |

Cold, industrial, deterministic—but not inhuman. Clear without being clinical. Confident without being arrogant.

---

## Writing Principles

### Direct and Honest

Say what you mean. Don't hedge unnecessarily.

| Good | Bad |
|------|-----|
| "This pattern prevents X" | "This pattern may help reduce X" |
| "Tests failed: 3 errors in auth.py" | "There appear to be some issues" |
| "Not measured: mutation testing skipped" | (silent gap) |

### Tables Over Prose (When They Help)

Tables compress information. Use them for comparisons, mappings, and structured choices.

```markdown
| What We're Not | What We Are |
|----------------|-------------|
| Chatbot | Refinery |
| Hope | Evidence |
```

But don't force everything into tables. Narrative is fine when it flows naturally.

### Active Voice, Clear Subjects

| Good | Bad |
|------|-----|
| "Critics run inside build loops" | "Build loops have critics run inside them" |
| "The gate decides merge or bounce" | "A merge/bounce decision is made by the gate" |

### Explain "Why", Not Just "What"

People remember reasons better than rules.

| Good | Bad |
|------|-----|
| "We use receipts because prose claims can be wrong" | "Always write receipts" |
| "UNVERIFIED is honest—it means 'not yet merged'" | "UNVERIFIED is a valid status" |

---

## The Warmth

Industrial doesn't mean cold to humans. It means treating the system mechanically while treating people with respect.

### Honest About Gaps

"Not measured" is acceptable. Silent gaps are not.

| Good | Bad |
|------|-----|
| "Mutation testing skipped (no budget)" | (just not mentioning it) |
| "We're not fully there yet" | "This is the future of development" |

### No Shaming

Assume good faith. Explain what to do, not what someone did wrong.

| Good | Bad |
|------|-----|
| "If you're seeing X, try Y" | "You shouldn't have done X" |
| "This works better when..." | "Obviously you should..." |

### Teach, Don't Gatekeep

The repo is both factory and curriculum. Write for the next reader.

| Good | Bad |
|------|-----|
| "Here's how to get started" | Wall of text with no entry point |
| Examples alongside concepts | Theory without practice |

---

## Specific Language Choices

### Completion States

| Use | Not |
|-----|-----|
| VERIFIED | "passed", "done", "complete" (without evidence) |
| UNVERIFIED | PARTIAL, INCOMPLETE, FAILED (when checkpointed) |
| CANNOT_PROCEED | BLOCKED (when truly halted) |

### Routing Language

| Use | Not |
|-----|-----|
| "Route to fixer" | "BLOCKED: needs fix" |
| "Recommend next: code-implementer" | (no recommendation) |

### Evidence Language

| Use | Not |
|-----|-----|
| "Tests pass (see test_execution.md)" | "Tests pass" (no pointer) |
| "Coverage: 78% (see coverage_audit.md)" | "Coverage looks good" |
| "Not measured: mutation skipped" | (silence) |

---

## The Posture

**Trust AND Verify.** Not "trust later." Not "verify forever then trust." Both, always.

**The system earns trust through evidence, not assertions.** An agent's claim does not override an exit code.

**Intent starts. Evidence decides.** Intent artifacts define success. Evidence determines if we hit it.

**The verification IS the product.** Code is a byproduct. A PR that's boring to approve—because the evidence is comprehensive—is the goal.

---

## See Also

- [the-thesis.md](../../docs/explanation/the-thesis.md) — The full story
- [economics.md](../../docs/explanation/economics.md) — Why this works economically
- [80-developer-experience.md](80-developer-experience.md) — UX principles
