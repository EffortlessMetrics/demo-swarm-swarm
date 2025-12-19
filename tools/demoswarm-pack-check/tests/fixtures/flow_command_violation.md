# Flow Test - Violation

This is a test fixture representing a flow command with demoswarm.sh violation.

## Purpose

This flow incorrectly invokes skill-layer commands directly.

## Instructions

1. Run: bash .claude/scripts/demoswarm.sh count pattern --file receipt.json
2. Use the count to verify completion

## Notes

This flow violates the three-tier ownership model by calling demoswarm.sh directly.
Flows should delegate to agents, not invoke skill-layer tools.
