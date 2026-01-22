# Test Agent - With Skills Section

This is a test fixture representing a compliant agent.

## Skills

- runs-derive: count markers, extract Machine Summary fields
- runs-index: update index.json status

## Behavior

This agent uses demoswarm.sh for mechanical operations:

```bash
bash .claude/scripts/demoswarm.sh count pattern --file receipt.json --regex "REQ-"
bash .claude/scripts/demoswarm.sh index upsert-status --run-id test-run --status VERIFIED
```

## Notes

This agent is compliant because it:

1. Uses demoswarm.sh for skill operations
2. Has a ## Skills section documenting skill usage
