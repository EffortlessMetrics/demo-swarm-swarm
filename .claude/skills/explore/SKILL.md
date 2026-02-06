# Explore Skill

Systematically explore a codebase to answer questions or understand structure.

## Exploration Modes

### Structure Mode
Map the codebase architecture:
1. Find entry points (main, index, app)
2. Identify major modules/packages
3. Trace dependency flow
4. Document key abstractions

### Search Mode
Find specific functionality:
1. Start with grep for keywords
2. Follow imports/references
3. Map call chains
4. Identify all touch points

### Trace Mode
Follow a specific flow:
1. Start at entry point
2. Step through execution path
3. Document each layer touched
4. Note data transformations

## Output Format
```markdown
## Question/Goal
<what we're trying to understand>

## Findings
<structured answer>

## Key Files
| File | Purpose | Relevance |
|------|---------|-----------|
| path | what it does | why it matters |

## Architecture Notes
<diagrams, flow descriptions>

## Open Questions
<what's still unclear>
```

## Rules
- Use sub-agents (Task tool) for parallel exploration
- Read files before making claims about them
- Note assumptions explicitly
- Don't modify code during exploration
