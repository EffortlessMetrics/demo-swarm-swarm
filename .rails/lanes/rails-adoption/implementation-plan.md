# Implementation plan: rails-adoption

## Objective

Adopt `.rails/` as the durable repository knowledge base and establish indexed first artifacts.

## PR sequence

1. Add framework footprint and human-facing docs.
2. Add templates for proposals/specs/ADRs/lanes/closeouts and support/policy references.
3. Add initial proposal, ADR, spec, and lane tracker artifacts.

## Dependencies

- Maintainer agreement on external namespace boundaries.
- Follow-up validator work for index and lane semantics.

## Proof strategy

- `git diff --check`
- Index and artifact link review via `.rails/index.toml`
