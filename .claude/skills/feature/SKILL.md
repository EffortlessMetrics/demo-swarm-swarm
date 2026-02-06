# Feature Skill

Feature implementation workflow.

## Input
Provide:
- Feature description/spec
- Acceptance criteria (if available)
- Constraints or requirements

## Workflow

### 1. Understand
```
- What problem does this solve?
- What are the inputs/outputs?
- What are the edge cases?
- What are the constraints?
```

### 2. Plan
```
- Break into implementable steps
- Identify files to create/modify
- Identify dependencies
- Estimate scope
```

### 3. Implement
For each step:
```
1. Write the code
2. Write tests for that code
3. Verify tests pass
4. Move to next step
```

### 4. Integrate
```
- Wire up to rest of system
- Update any configuration
- Update documentation if public API
```

### 5. Verify
```
- Run full test suite
- Manual verification of acceptance criteria
- Check for regressions
```

## Output Format
```markdown
## Feature
<description>

## Implementation
| Step | Files | Status |
|------|-------|--------|

## Tests Added
| Test | What it verifies |
|------|------------------|

## Evidence
- New tests: <count>
- Test suite: <pass/fail>
- Acceptance criteria: <met/unmet>

## Usage
<how to use the new feature>

## Limitations
<what's not included, known constraints>
```

## Rules
- For significant features, prefer `/flow-1-signal` through the full SDLC
- Write tests alongside each implementation step, not at the end
- Don't gold-plate - implement what's specified
- Note but don't implement adjacent features
- Document limitations explicitly
