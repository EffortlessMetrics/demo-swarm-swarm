# Flow Test - Skill Subcommand

This is a test fixture with skill CLI subcommand violations.

## Purpose

This flow incorrectly references skill CLI subcommands.

## Instructions

1. Use the `count` command to tally markers in the receipt
2. Use `ms get` to extract Machine Summary fields
3. Use `index upsert-status` to update the run index

## Notes

These subcommand patterns should be flagged:

- count (as a CLI subcommand reference)
- ms get
- index upsert-status
