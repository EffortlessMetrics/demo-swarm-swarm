---
globs:
  - docs/**/*.md
---

# Docs and Teaching

> The repo is both factory and curriculum.

---

## The Teaching Intent

This is not just a working system. It is a **teaching system**.

The repo teaches the mental model while you use it:

- CLAUDE.md provides the contract
- Agent prompts are executable job descriptions
- Docs explain the why behind the what
- Flows demonstrate patterns in action

**The repo is the curriculum.**

---

## One Concept, One Canonical Home

Don't duplicate doctrine across multiple docs.

| Good                                     | Bad                                      |
| ---------------------------------------- | ---------------------------------------- |
| Link to the canonical doc                | Copy the content to multiple places      |
| "See [laws-of-the-swarm.md] for details" | Repeat the laws in three different files |

When content needs to appear in multiple contexts, summarize and link.

---

## Teaching Posture

For new concepts, structure as:

1. **Assumption people bring** — What readers likely believe coming in
2. **Failure mode of that assumption** — What goes wrong if you keep believing it
3. **Mechanism we use instead** — How this system works differently
4. **What good looks like** — Concrete examples of correct behavior
5. **Example artifact** — Realistic output that demonstrates the concept

This structure meets people where they are and shows them where to go.

---

## Graduated Complexity

Layer information by depth:

| If you need...     | Read...             | Depth        |
| ------------------ | ------------------- | ------------ |
| The basics         | CLAUDE.md           | Summary      |
| The concepts       | `docs/explanation/` | Intermediate |
| The how-to         | `docs/how-to/`      | Practical    |
| The specifications | `docs/reference/`   | Full detail  |

Start simple. Add depth when needed. Don't force understanding everything before doing anything.

---

## Executable Documentation

The distinction between "docs" and "code" is intentionally blurred:

| Artifact      | Documentation? | Executable?               |
| ------------- | -------------- | ------------------------- |
| Agent prompts | Yes            | Yes                       |
| Flow commands | Yes            | Yes                       |
| CLAUDE.md     | Yes            | Yes (shapes all behavior) |
| Skills        | Yes            | Yes                       |

This means:

- Outdated docs fail visibly (the flow breaks)
- Examples are always runnable (they ARE the implementation)
- You can't have working code with wrong documentation

---

## Visual Language

When diagrams help:

- Prefer Mermaid (readable in text, renders nicely)
- Keep diagrams small and readable
- Follow `docs/reference/visual-style.md`
- Do not rely on color alone to convey meaning (accessibility)

Diagrams should clarify, not decorate.

---

## Keeping Project Memory Lean

Don't import huge docs into CLAUDE.md or rules.

| Layer     | Size   | Purpose                           |
| --------- | ------ | --------------------------------- |
| Rules     | Small  | Constitution that loads reliably  |
| CLAUDE.md | Medium | Contract that shapes all behavior |
| Docs      | Large  | Textbook for deep understanding   |

Rules and CLAUDE.md are what Claude loads every time. Keep them lean. Docs are the reference library—they can be comprehensive.

---

## For Pack Maintainers

Every change should teach correctly:

1. **Update CLAUDE.md** when contracts change
2. **Update rules** when physics change
3. **Update agent prompts** when behavior changes
4. **Update explanation docs** when rationale changes
5. **Run pack-check** to verify consistency

If a change only updates the implementation without updating the teaching layer, the next user will learn the wrong thing.

---

## See Also

- [teaching-repo.md](../../docs/explanation/teaching-repo.md) — Full teaching philosophy
- [how-claude-md-works.md](../../docs/explanation/how-claude-md-works.md) — CLAUDE.md mechanics
- [visual-style.md](../../docs/reference/visual-style.md) — Diagram guidelines
