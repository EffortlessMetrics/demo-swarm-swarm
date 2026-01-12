# Codebase as Mold

> The existing codebase shapes all future generation.

---

## The Core Insight

When you use a stochastic compiler (LLM) to generate code, it doesn't start from nothing. It starts from:
- The codebase you show it
- The patterns it observes
- The conventions it infers

This means **the codebase is a mold**. Whatever shape it has, that's the shape of what gets generated next.

---

## Schema Gravity

The flow structure and existing patterns exert gravitational pull on new output.

**Good codebase:**
- Clear conventions → consistent new code
- Well-defined interfaces → appropriate integrations
- Tested patterns → testable additions
- Clean structure → organized additions

**Messy codebase:**
- Inconsistent patterns → random style choices
- Unclear interfaces → awkward integrations
- Untested code → more untested code
- Spaghetti structure → more spaghetti

The generator will match what it sees. Schema gravity is real.

---

## Implications

### 1. Technical Debt Compounds

In traditional development, technical debt slows you down. In agentic development, technical debt **gets copied**.

| Traditional | Agentic |
|-------------|---------|
| Debt makes coding slower | Debt gets replicated |
| You can work around bad patterns | Generator learns bad patterns |
| One messy file affects one file | One messy file affects all similar files |

This makes debt remediation even more important.

### 2. Refactoring Is Investment

Improving the mold improves all future output:
- Better interfaces → better generated integrations
- Cleaner conventions → cleaner generated code
- Better test patterns → better generated tests

Refactoring pays compound interest.

### 3. Maintainability Is First-Class

Maintainability work is not a luxury. It's infrastructure.

| Work Type | Traditional Value | Agentic Value |
|-----------|------------------|---------------|
| Style consistency | Nice to have | Shapes all generation |
| Interface clarity | Helps humans | Helps generation too |
| Test patterns | Safety net | Template for new tests |
| Documentation | Onboarding | Context for generation |

The codebase teaches the generator what "good" looks like.

---

## What This Means for Practice

### Invest in the Mold

Before starting a major feature:
- Is the relevant area well-structured?
- Are the interfaces clear?
- Are there good patterns to follow?

If not, consider improving the mold first.

### Watch for Pattern Drift

Generated code will match existing patterns. If you see bad patterns spreading:
- The generator is doing its job (matching context)
- The fix is to improve the source patterns
- Then regenerate or refactor

### Document Conventions

Explicit conventions help:
- Humans understand the codebase
- Generators follow the patterns
- Critics check for violations

Conventions are not just for humans anymore.

---

## The Flywheel

```
Better mold → Better generation → Easier review → More confidence
→ More willingness to refactor → Better mold
```

Investing in codebase quality creates a virtuous cycle.

---

## Anti-Patterns

### Hoping the Generator Will Clean Things Up

It won't. It will match what's there. If you want clean output, provide clean input.

### Ignoring Legacy Mess

Legacy code shapes new code. If you never improve it, you replicate it forever.

### Treating Refactoring as Optional

In agentic development, refactoring is infrastructure investment, not housekeeping.

---

## The Mental Model

Think of the codebase as a **template factory**:
- Each pattern is a template
- Each convention is a constraint
- Each interface is a slot

What you put in the factory determines what comes out.

No one "wrote" the assembly under your C compiler. Similarly, no one "writes" the implementation code in agentic development. But someone designed the language, the conventions, the standard library. That's your job now: design the mold.

---

## See Also

- [the-thesis.md](the-thesis.md) — The broader shift in developer roles
- [economics.md](economics.md) — Why investing in the mold pays off
- [../rules/40-evidence-and-quality.md](../../.claude/rules/40-evidence-and-quality.md) — Maintainability in the quality panel
