# DSWARM-SPEC-0001: Rails artifact graph contract

Status: accepted
Owner: docs-and-governance
Created: 2026-05-21
Linked proposal: DSWARM-PROP-0001
Linked ADRs: DSWARM-ADR-0001
Linked lane: rails-adoption
Linked issues:
Linked PRs:
Support-tier impact: none
Policy impact: references only

## Problem

Without a contract, Rails artifacts can drift in location, ownership, and linkage.

## Behavior

- Rails artifacts must be indexed through `.rails/index.toml`.
- Owned artifact paths must live under `.rails/`.
- External namespaces may be listed but not owned.
- Specs define behavior, not PR order.
- Lane trackers define focused implementation sequencing.

## Non-goals

Defining external tool behavior for `.codex/`, `.spec/`, `.claude/`, or `.jules/`.

## Required evidence

Repository paths and index entries align, and no owned artifact path points into external namespaces.

## Acceptance examples

An indexed proposal/spec/ADR/lane set with valid paths and links under `.rails/`.

## Test mapping

Initial proof command: `git diff --check`.

## Implementation mapping

`.rails/index.toml`, `.rails/lanes/*/tracker.toml`, and artifact markdown files.

## CI proof

Current manual proof command with future validator command in xtask.

## Metrics / promotion rule

Promote when automated validator enforces uniqueness, path ownership, and link resolution.

## Failure modes

Missing files, duplicate IDs, or owned artifact paths under external namespaces must fail validation.
