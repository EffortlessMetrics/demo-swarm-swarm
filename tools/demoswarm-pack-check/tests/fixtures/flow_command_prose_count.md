# Flow Test - Prose Context

This is a test fixture where skill-like words appear in prose context only.

## Purpose

This flow discusses validation concepts without invoking skill tools.

## Instructions

1. Review the count of requirements documented in requirements.md
2. Check the index to find related runs (refers to conceptual index)
3. Use the receipt to verify completion (documentation reference)

## Notes

The words "count", "index", and "receipt" appear here in natural prose.
They should NOT be flagged as violations because:

- They are not CLI command invocations
- They are not preceded by any shim script
- They describe concepts, not tool calls

This validates that we do not produce false positives on prose.
