# DSWARM-PROP-0001: Rails knowledge base

Status: accepted
Owner: docs-and-governance
Created: 2026-05-21
Target milestone: vNext
Linked specs: DSWARM-SPEC-0001
Linked ADRs: DSWARM-ADR-0001
Linked lanes: rails-adoption

## Problem

The repository lacks a durable, tool-agnostic source-of-truth structure for long-lived product and implementation artifacts.

## Users and surfaces

Maintainers, contributors, and downstream tooling that needs stable access to proposal/spec/ADR/lane/closeout artifacts.

## Success criteria

A portable `.rails/` footprint exists with indexed artifacts and lane-focused sequencing.

## Proposed shape

Adopt `.rails/` as the durable framework directory and keep agent/tool namespaces awareness-only.

## Alternatives considered

Keep artifacts in agent-owned directories (`.codex/`, `.spec/`) or use repo-specific hidden directories. Rejected for portability and ownership ambiguity.

## Specs to create or update

- DSWARM-SPEC-0001

## Architecture decisions needed

- DSWARM-ADR-0001

## Implementation campaign shape

1. Add framework footprint and docs.
2. Add templates and initial indexed artifacts.
3. Add validators and closeout flows.

## Evidence plan

`git diff --check` for formatting and repository review via tracked artifacts in `.rails/index.toml`.

## Risks

Drift between indexed artifacts and filesystem paths without validation tooling.

## Non-goals

Migrating or modifying external namespaces such as `.codex/` and `.spec/`.

## Exit criteria

`.rails/` is established, documented, and linked with at least one proposal/spec/ADR and one active lane.
