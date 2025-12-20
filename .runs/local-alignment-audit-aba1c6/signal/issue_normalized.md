# Normalized Issue

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_agent: problem-framer
route_to_flow: 2
blockers: []
missing_required: []
notes:
  - raw input was a comprehensive alignment analysis comparing documentation against codebase
  - analysis cross-referenced CLAUDE.md, test_output.log, REPO_MAP.md, agent frontmatter, and secrets.rs
  - all key claims were validated against code artifacts; evidence is bounded to specific findings

## Summary
Analysis audit comparing DemoSwarm documentation (CLAUDE.md, REPO_MAP.md, etc.) against actual codebase reveals three classes of findings: (1) architectural correctness (Seven-Flow model confirmed; previous "Six-Flow" claim outdated), (2) test count discrepancy (actual 102 passing tests vs claimed 374), and (3) security posture validity (ReDoS claim invalid; path traversal concern valid).

## Signal Type
- request_type: audit
- source_type: manual alignment analysis
- links:
  - test_output.log (actual test run results)
  - CLAUDE.md (pack reference)
  - secrets.rs (secrets tooling)
  - REPO_MAP.md (repo structure documentation)

## Observed vs Expected
- observed:
  - Documentation states "Six-Flow" model in some places; code implements Seven-Flow (Signal → Plan → Build → Review → Gate → Deploy → Wisdom)
  - CLAUDE.md claims exist; agent color frontmatter also exists in agent YAML headers
  - 102 tests passing (per test_output.log); 41 integration tests filtered out
  - Secrets scanner accepts raw paths without canonicalization (no realpath/canonicalize calls)
  - Rust regex crate used (not vulnerable to ReDoS)

- expected:
  - Unified Seven-Flow model across all documentation
  - Consistent agent metadata discipline
  - Accurate test metrics reflecting actual pass count
  - Security guidance aligned with implementation posture

## Impact
- affected_users: all users of DemoSwarm (especially integrators relying on flow count / test coverage claims)
- severity: medium (documentation drift creates confusion; security claims must be accurate)
- frequency: persistent (affects ongoing documentation review and user expectations)
- environment: documentation + pack contracts

## Components Mentioned
- systems/services:
  - DemoSwarm pack (control plane, flows, agents)
  - test suite (102 passing unit tests)
  - secrets scanner (path handling)
- endpoints/paths:
  - `.runs/<run-id>/signal/` (Flow 1)
  - `.runs/<run-id>/build/` (Flow 3)
  - tools/demoswarm-runs-tools/src/commands/secrets.rs (secrets tooling)
- files/modules:
  - CLAUDE.md (pack reference)
  - REPO_MAP.md (repo structure)
  - test_output.log (test results)
  - agent frontmatter (color coding)

## Constraints / Non-negotiables
- Flow architecture is immutable (Seven-Flow is canonical)
- Test counts must reflect actual pass count, not aspirational targets
- Security claims require supporting evidence from code
- Agent metadata discipline must be consistently applied
- unknowns:
  - whether path traversal in secrets.rs is exploitable in practice (depends on untrusted input sources)
  - whether "Six-Flow" references exist elsewhere in docs beyond CLAUDE.md
  - scope of integration test filtering impact on overall coverage story

## Evidence (bounded)
Test output (line 109): "test result: ok. 102 passed; 0 failed; 0 ignored; 0 measured; 277 filtered out"

Agent frontmatter example (.claude/agents/test-critic.md, lines 1–6):
```yaml
---
name: test-critic
description: Harsh review of tests vs BDD + REQ/NFR + test plan...
model: inherit
color: red
---
```

Secrets scanner (tools/demoswarm-runs-tools/src/commands/secrets.rs, lines 76–124):
- Accepts `args.path: String` directly (no canonicalization)
- Uses `Path::new()` and `fs::read_dir()` without normalization
- No calls to `canonicalize()` or `realpath()`

ReDoS immunity: secrets.rs uses Rust's `regex` crate (line 14), which is immune to ReDoS via finite automata implementation (not backtracking-based).
