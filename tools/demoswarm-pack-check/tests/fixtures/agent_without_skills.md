# Test Agent - Missing Skills Section

This is a test fixture representing a non-compliant agent.

## Behavior

This agent uses the skill shim but lacks the Skills section:

```bash
bash .claude/scripts/demoswarm.sh ms get --file summary.md --section "Machine Summary" --key status
```

## Notes

This agent violates the pack contract because:

1. It uses the shim script (see code block above)
2. It does NOT have a skill documentation heading
