# Developer Experience

> UX, accessibility, and the economics of investing in quality.

---

## The Goal

We are building **developer enablement**: cheap iteration buys verification so humans spend time on decisions, not toil.

The goal is not "replace devs." The goal is:
- Remove toil
- Ship a healthier codebase
- Make review time valuable, not exhausting
- Leave the codebase better than we found it

---

## Write for Humans

When writing docs, prompts, or outputs:
- Explain "why," not just "what"
- Avoid shaming or gatekeeping
- Prefer clear headings + short sections
- Include examples and "what to do next"
- Assume good faith

### The Standard

| Good | Bad |
|------|-----|
| "This pattern helps because..." | "Obviously you should..." |
| "If you're seeing X, try Y" | "You shouldn't have done X" |
| "Here's how to get started" | Wall of text with no entry point |

---

## UX Is First-Class

The PR cockpit is a UI. Treat it like one.

### Design for Scanning

- Short summary tables, not paragraphs
- Clear hotspots, not comprehensive lists
- Explicit "not measured," not silent gaps
- Evidence pointers, not inline dumps

### Design for Decisions

Reviewers need to decide: merge or not merge. Give them:
- What changed (summary)
- Why it changed (intent link)
- What was verified (evidence)
- Where to spot-check (hotspots)
- What's unknown (explicit gaps)

---

## Accessibility Matters

- Do not rely on color alone to convey meaning
- Ensure keyboard navigation works in any tooling
- Provide text alternatives for visual content
- Test with screen readers when building UIs
- Use high contrast for critical information

The cockpit should be usable by everyone, not just people with perfect vision and a mouse.

---

## Open Source Sensibility

This pack exists to be:
- **Teachable** — Learn by using, not by studying first
- **Portable** — Adapt to different stacks and contexts
- **Kind** — Welcoming to the next contributor

### For Contributors

When someone new arrives:
- The README tells them where to start
- The first flow works out of the box
- Errors are explanatory, not cryptic
- The docs assume nothing beyond basics

### For Maintainers

When evolving the pack:
- Explain changes in commit messages
- Update docs with the code
- Run pack-check before pushing
- Keep backward compatibility when reasonable

---

## Anti-Austerity

When value is real, spend compute.

### What We Spend On

- **Clarity** — Better evidence summaries, cleaner receipts
- **Maintainability** — Well-structured code, useful documentation
- **Verification depth** — More critic passes, mutation testing
- **Accessibility** — Better PR cockpit, clearer human interfaces

### The Principle

Optimize for **Quality : Dev Attention**, not for cheap.

Machine time is cheap and getting cheaper. Human attention is expensive and getting more so. Trade accordingly.

---

## See Also

- [economics.md](../../docs/explanation/economics.md) — The math that justifies spending compute
- [teaching-repo.md](../../docs/explanation/teaching-repo.md) — The teaching philosophy
- [pr-as-review-surface.md](../../docs/explanation/pr-as-review-surface.md) — Why UX matters for PRs
