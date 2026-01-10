# Documentation Governance

How to prevent documentation drift and duplication as the pack grows.

---

## The Problem

As documentation grows:
- Same concept gets explained in multiple places
- Explanations drift and contradict
- Nobody knows which doc is authoritative
- Maintenance becomes impossible

This doc establishes rules to prevent that.

---

## One Canonical Place Per Concept

Every concept has ONE authoritative location. Everything else links to it.

| Concept | Canonical Location | Everything Else Does |
|---------|-------------------|---------------------|
| Run state schema | `docs/reference/run-state.md` | Links to it |
| Trust model | `docs/reference/trust-model.md` | Links to it |
| Agent philosophy | `docs/explanation/agent-philosophy.md` | Links to it |
| Flow commands | `CLAUDE.md` + `.claude/commands/` | Links to it |
| Quality surfaces | `docs/reference/pr-quality-scorecard.md` | Links to it |
| Stable markers | `docs/reference/stable-markers.md` | Links to it |
| Contracts and handoffs | `docs/reference/contracts.md` | Links to it |
| CLI commands | `docs/reference/demoswarm-cli.md` | Links to it |

**When you want to explain something already documented:**

```markdown
See [Trust Model](../reference/trust-model.md) for how evidence creates trust.
```

Don't re-explain. Link.

---

## Doc Types and Their Purpose

| Type | Location | Purpose | Updates When |
|------|----------|---------|--------------|
| Reference | `docs/reference/` | Precise definitions, schemas, contracts | Implementation changes |
| Explanation | `docs/explanation/` | Why things work the way they do | Understanding evolves |
| How-to | `docs/how-to/` | Step-by-step procedures | Process changes |

**Don't mix types.** A reference doc shouldn't have tutorials. A how-to shouldn't define schemas.

For voice and style within each type, see [Documentation Conventions](../reference/documentation-conventions.md).

---

## CLAUDE.md Is Special

CLAUDE.md is:
- Attached to every agent thread
- The repo-level policy document
- The entry point for agents

Rules for CLAUDE.md:
- Keep it concise (it's read constantly)
- Define policy, link to details
- If it gets long, extract to a linked doc
- Never duplicate what's better explained elsewhere

---

## Adding New Docs

Before creating a new doc:

1. **Search first** - Does this concept already have a home?
2. **If yes** - Update the existing doc or link to it
3. **If no** - Create the new doc in the right location (reference/explanation/how-to)
4. **Add to index** - Update the relevant README.md
5. **Cross-reference** - Add "See Also" links from related docs

---

## Updating Existing Docs

When you update a doc:

1. **Check for duplicates** - Search for the concept in other docs
2. **Update all or link** - Either update all mentions or convert duplicates to links
3. **Preserve canonical location** - Don't move the authoritative definition

---

## Marking Planned vs Implemented

If documenting something not yet built:

```markdown
> **Status: Planned** - This feature is designed but not yet implemented.
```

Remove the status marker when implemented. Don't leave stale "planned" markers.

---

## Deprecation

When something is no longer accurate:

1. **Don't delete immediately** - Someone may be referencing it
2. **Add deprecation notice:**
   ```markdown
   > **Deprecated:** This approach was replaced by [X](link).
   > This doc remains for historical reference.
   ```
3. **Remove after one release cycle** (or when no longer referenced)

---

## The Index Files

Each doc folder has a README.md that indexes its contents:
- `docs/reference/README.md`
- `docs/explanation/README.md`
- `docs/how-to/README.md`

**Keep indexes current.** When you add a doc, add it to the index.

---

## Cross-Reference Conventions

**See Also sections** go at the end of docs:

```markdown
## See Also

- [Related Concept](../path/to/doc.md) - Brief description
- [Another Doc](../path/to/other.md) - Why it's relevant
```

**Inline links** for specific references:

```markdown
The [trust model](../reference/trust-model.md) explains how...
```

---

## Common Violations

| Violation | Fix |
|-----------|-----|
| Explaining the trust model in three different docs | Pick one canonical location, link from others |
| CLAUDE.md has a 500-word explanation of agent philosophy | Keep summary in CLAUDE.md, full explanation in `docs/explanation/agent-philosophy.md` |
| How-to doc defines the receipt schema | Reference doc defines schema, how-to links to it |
| New doc created without updating index | Always update the relevant README.md |

---

## Maintenance Checklist

Periodic doc hygiene (monthly or before major releases):

- [ ] All docs in indexes are current
- [ ] No dead links
- [ ] No duplicate explanations of same concept
- [ ] "Planned" markers reflect reality
- [ ] Deprecated docs are marked or removed
- [ ] CLAUDE.md is still concise

---

## See Also

- [Documentation Conventions](../reference/documentation-conventions.md) - Voice, style, terminology
- [CLAUDE.md](../../CLAUDE.md) - Example of concise policy doc
- [Maintain the Pack](maintain-the-pack.md) - Pre-commit checklist for maintainers
