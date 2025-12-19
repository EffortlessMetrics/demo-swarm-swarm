# Requirements

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

blockers: []

missing_required: []

concerns:
  - OQ-SIG-002 (PLN vs PLAN prefix discrepancy) remains open; requirements assume PLN/BLD is canonical
  - OQ-SIG-001 (warnings vs failures) open; requirements specify warning-first approach with --strict flag
  - 4 agents using demoswarm.sh may lack Skills sections; enumeration assumed from context_brief.md

## Functional Requirements

### REQ-001: Flow Boundary Enforcement
The system shall validate that flow command files (`.claude/commands/flow-*.md`) do not contain demoswarm.sh invocations or skill-layer CLI syntax.
- AC-1: pack-check reports a warning (or error with --strict) when any flow command file contains the string "demoswarm.sh"
- AC-2: pack-check reports a warning (or error with --strict) when any flow command file contains skill CLI subcommands (count, ms, yaml, index, receipt, receipts, openapi, line, inv, time, openq, secrets)
- AC-3: Existing flow command files in the pack pass validation without warnings when the rule is introduced
- AC-4: The check scans all files matching `.claude/commands/flow-*.md`

### REQ-002: Skills Section Enforcement for demoswarm.sh Users
The system shall validate that all agent files invoking demoswarm.sh include a `## Skills` section.
- AC-1: pack-check reports a warning (or error with --strict) when an agent file in `.claude/agents/` contains "demoswarm.sh" but lacks a `## Skills` heading
- AC-2: The check identifies the specific agent file(s) missing the Skills section in the diagnostic output
- AC-3: Agents that invoke demoswarm.sh indirectly via skill invocation only (without literal "demoswarm.sh" in their prompt text) are not flagged
- AC-4: All agents currently having Skills sections continue to pass validation (baseline established at rule introduction)

### REQ-003: OpenQ Prefix Pattern Validation
The system shall validate that OpenQ question IDs follow the canonical prefix pattern.
- AC-1: pack-check validates QID patterns in `.runs/**/open_questions.md` files match `OQ-<FLOW>-<NNN>` where `<FLOW>` is one of: SIG, PLN, BLD, GAT, DEP, WIS (per stable-markers.md)
- AC-2: QIDs using non-canonical flow codes (e.g., PLAN instead of PLN, BUILD instead of BLD) produce a warning (or error with --strict)
- AC-3: The numeric suffix `<NNN>` must be zero-padded to three digits (001-999)
- AC-4: Existing open_questions.md files in the pack pass validation (or the divergent patterns are enumerated as known exceptions)

### REQ-004: Build-to-Gate Handshake Test Scenario
The system shall include at least one documented test scenario validating the Build-to-Gate receipt handshake.
- AC-1: A test fixture exists at `tools/demoswarm-pack-check/tests/fixtures/` containing a valid build_receipt.json that passes receipt-checker validation
- AC-2: A test fixture exists containing an invalid build_receipt.json that fails receipt-checker validation (missing required field or invalid value)
- AC-3: pack-check test suite includes a test case that exercises the Build receipt validation logic
- AC-4: The test scenario documentation describes the handshake contract being validated (required fields, type constraints, cross-reference expectations)

### REQ-005: Warning-First Validation Mode
The system shall support a warning-first validation mode where new compliance rules produce warnings by default and can be elevated to errors via a flag.
- AC-1: pack-check supports a `--strict` flag that elevates warnings to errors for all new compliance rules (REQ-001, REQ-002, REQ-003)
- AC-2: Without `--strict`, validation completes with exit code 0 even when warnings are present
- AC-3: With `--strict`, validation fails with non-zero exit code when any warning is present
- AC-4: Warning output includes the rule identifier and file location for each violation

### REQ-006: No False Positives on Existing Artifacts
The system shall not produce false positive warnings on existing valid artifacts in the pack.
- AC-1: All existing agent files pass REQ-002 validation or are documented as exceptions requiring remediation
- AC-2: All existing flow command files pass REQ-001 validation
- AC-3: All existing open_questions.md files pass REQ-003 validation or divergent QIDs are enumerated
- AC-4: A validation baseline is established by running pack-check on the current pack state before introducing new rules

## Non-Functional Requirements

### NFR-PERF-001: CI Validation Runtime
The system shall complete pack validation within acceptable CI runtime bounds.
- MET-1: pack-check full validation completes in under 30 seconds on standard CI runner (measured in CI pipeline)
- MET-2: Incremental addition of REQ-001 through REQ-005 rules adds no more than 5 seconds to baseline validation time

### NFR-REL-001: Deterministic Validation Output
The system shall produce deterministic validation output across runs.
- MET-1: Running pack-check twice on identical input produces byte-identical output (verified by CI test)
- MET-2: Warning/error ordering is stable (sorted by file path, then rule ID)

### NFR-OPS-001: Diagnostic Clarity
The system shall provide clear, actionable diagnostic output for validation failures.
- MET-1: Each warning/error includes: rule ID, file path, line number (if applicable), violation description
- MET-2: Suggested remediation is included in diagnostic output or referenced in documentation
- MET-3: pack-check --help documents all new validation rules and their behavior

### NFR-COMP-001: Backward Compatibility
The system shall maintain backward compatibility with existing valid artifacts.
- MET-1: Existing pack-check exit codes and output format are preserved (CI pipelines do not break)
- MET-2: New rules are warning-only by default; --strict opt-in for enforcement
- MET-3: Migration path documented for any artifacts that require changes to pass new rules

### NFR-SEC-001: No Secrets in Validation Output
The system shall not expose secrets in validation diagnostic output.
- MET-1: pack-check does not print file contents containing potential secrets (only file paths and rule violations)
- MET-2: Test fixtures do not contain real secrets or credentials

### NFR-MAINT-001: Pattern Maintainability
The system shall support maintainable evolution of validation patterns.
- MET-1: Skill CLI subcommand list (REQ-001) is defined as a constant in contracts.rs, not inline regex
- MET-2: OpenQ flow codes (REQ-003) are defined as a constant, not inline regex
- MET-3: Adding a new skill or flow code requires change to at most 2 files (contracts.rs and CLAUDE.md)

Domain Notes:
- MAINT: maintainability/evolution concerns for validation infrastructure

## Assumptions Made
- **ASM-001**: pack-check (Rust) is the appropriate venue for new structural validation rules. (why: problem_statement.md identifies pack-check as Rust-based and the primary validation tool)
  - Impact if wrong: Implementation would shift to check-doc-drift.sh (Bash), requiring different implementation approach
- **ASM-002**: PLN/BLD abbreviations are canonical over PLAN/BUILD for OpenQ flow prefixes. (why: openq-tools/SKILL.md is the implementation source of truth)
  - Impact if wrong: Would need to update openq-tools Rust code and existing QIDs using PLN/BLD
- **ASM-003**: Warning-before-failure is acceptable for new validation rules. (why: enables incremental adoption without breaking existing artifacts)
  - Impact if wrong: All new checks would need immediate enforcement, requiring migration before deployment
- **ASM-004**: The 4 agents using demoswarm.sh without Skills sections are gaps to fix, not intentional exceptions. (why: no documented exemption policy exists)
  - Impact if wrong: Would need to document exemption criteria rather than add missing sections
- **ASM-005**: Build-to-Gate is the most critical handoff and should be validated first before expanding to other boundaries. (why: problem statement success criteria focus on Build-to-Gate specifically)
  - Impact if wrong: Would need broader test fixtures for Signal-to-Plan and Plan-to-Build from the start
- **ASM-006**: Tool consolidation (merging pack-check and check-doc-drift.sh) is out of scope. (why: explicitly stated as non-goal in problem_statement.md)
  - Impact if wrong: Requirements would need to address consolidation architecture

## Questions for Humans
- Q: Should REQ-001 scan skill names dynamically from .claude/skills/ directory or use a hardcoded list? Suggested default: Hardcoded list in contracts.rs. Impact if different: Dynamic discovery adds complexity and requires directory parsing infrastructure.
- Q: Which 4 agents are missing Skills sections? Suggested default: Derive from audit (agents containing "demoswarm.sh" minus agents with "## Skills" heading). Impact if different: If specific agents are intentionally exempt, need exemption list.
- Q: Should REQ-004 test fixtures cover invalid receipts beyond missing fields (e.g., wrong types, invalid enum values)? Suggested default: One valid and one invalid fixture is minimum; additional cases are enhancement. Impact if different: More comprehensive fixtures increase test coverage but add maintenance burden.
- Q: Is the 30-second CI runtime bound (NFR-PERF-001) appropriate? Suggested default: 30 seconds is reasonable for current pack size. Impact if different: Tighter bound may require validation optimization; looser bound may impact developer feedback loop.
