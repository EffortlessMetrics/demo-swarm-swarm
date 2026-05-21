# DSWARM-ADR-0001: Rails framework footprint

Status: accepted
Date: 2026-05-21
Owner: docs-and-governance
Linked proposal: DSWARM-PROP-0001
Linked specs: DSWARM-SPEC-0001

## Decision

Long-term proposal/spec/ADR/lane/closeout artifacts live in `.rails/`. Agent/tool-specific state remains external.

## Context

The repository needs a portable footprint that survives tool changes and supports validators, portal surfaces, and future command-palette interfaces.

## Consequences

Durable artifacts gain stable ownership and indexability, while external namespaces stay awareness-only.

## Alternatives considered

Repo-scoped hidden directories and agent-owned spaces were rejected because they couple long-lived truth to tool execution state.

## Follow-up specs / plans

Implement indexed artifact graph constraints and lane tracker validation.
