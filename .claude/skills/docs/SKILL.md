# Docs Skill

Documentation work using the Diataxis framework.

## Diataxis Framework

Organize documentation into four categories. This repo's own `docs/` follows Diataxis.

| Type | Purpose | User Need |
|------|---------|-----------|
| **Tutorials** | Learning-oriented | "Help me get started" |
| **How-To Guides** | Task-oriented | "Help me solve a problem" |
| **Reference** | Information-oriented | "Help me find facts" |
| **Explanation** | Understanding-oriented | "Help me understand" |

### Tutorials
- Step-by-step lessons for beginners
- Focus on learning, not accomplishing
- Always work when followed exactly
- Location: `docs/tutorials/`

### How-To Guides
- Recipes for specific tasks
- Assume basic competence
- Focus on results, not learning
- Location: `docs/how-to/`

### Reference
- Technical descriptions
- Accurate and complete
- No explanation of concepts
- Location: `docs/reference/`

### Explanation
- Conceptual discussions
- Why things work the way they do
- Background and context
- Location: `docs/explanation/`

## Link Stability Rules

**Priority: Preserve existing URLs over clean file organization.**

1. **Never break existing links** - external references matter
2. **Prefer redirects over moves** - add redirect, don't delete
3. **Update internal links** - when moving, fix all references
4. **Validate after changes** - run link checker

## Documentation Audit

When auditing existing docs:

```markdown
| File | Current Category | Suggested Category | Links In | Links Out |
|------|------------------|-------------------|----------|-----------|
```

## Workflow

1. **Audit** - Map existing docs to Diataxis categories
2. **Plan** - Identify moves needed, redirect strategy
3. **Execute** - One file at a time, validate links after each
4. **Verify** - Full link check, manual spot check

## Output Format

```markdown
## Documentation Change

### Files Modified
| File | Action | Category |
|------|--------|----------|

### Redirects Added
| Old Path | New Path |

### Link Validation
- Internal links: ✓/✗
- External links: ✓/✗
- Broken links found: <count>
```

## Rules
- Does not write code; writes documentation about code
- Link stability over file organization
- One category per document
- Cross-reference between categories
- Never orphan a URL
