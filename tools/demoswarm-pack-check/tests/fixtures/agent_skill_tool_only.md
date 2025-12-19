# Test Agent - Skill Tool Invocation Only

This is a test fixture for an agent that uses skills via the Skill() tool.

## Behavior

This agent invokes skills through the formal Skill tool:

1. Invoke the runs-derive skill to count markers:
   Use: Skill(skill: "runs-derive")

2. Process the results

## Notes

This agent is compliant because:
1. It does NOT contain the literal shim script reference
2. It invokes skills through the Skill() tool mechanism
3. Therefore it is NOT required to have a ## Skills section
   (the Skills section requirement only applies to shim script users)
