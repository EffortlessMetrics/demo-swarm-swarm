# Forensic Skill

Evidence-first execution. Trust receipts, not prose.

## Principles
- Exit codes don't lie
- Git diffs don't hallucinate
- If it isn't logged, it didn't happen
- Measure the bolt, don't ask the intern
- These principles underpin the flow model's evidence panel — claims require pointers

## Required Evidence
For any implementation task, produce:

### Commands
```
Command: <exact command>
Exit code: <0/1/etc>
Output: <relevant portion>
```

### Changes
```
Files modified: <count>
Lines added: <+N>
Lines removed: <-N>
```

### Verification
```
Tests: <pass/fail count>
Lint: <clean/warnings>
Build: <success/failure>
```

### Unknowns
```
Not tested: <what>
Assumptions: <what>
Risks: <what>
```

## Execution Pattern
1. Run the command
2. Capture the output
3. Report the exit code
4. Only then interpret

## Anti-Patterns
- "Tests should pass" → Run them, show output and exit code
- "This looks correct" → Prove it with a command
- "I've implemented X" → Show the diff and verification
- "It works" → Show the evidence

## Output Format
Every response ends with:
```markdown
## Evidence
| Check | Result | Details |
|-------|--------|---------|
| Build | ✓/✗ | exit code |
| Tests | ✓/✗ | pass/fail |
| Lint | ✓/✗ | warnings |

### Commands Run
<list with exit codes — every command must show its exit code>

### Unknowns
<what wasn't verified>
```
