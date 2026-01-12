# Open Questions (Append-only)

This is an append-only register. New items are added in "Update" blocks. Resolutions are appended as `- A:` lines.

## Stable Marker Contract
- Questions: `^- QID:` then `- Q:`
- Assumptions: `^- Assumption:`
- Resolutions: `^- A:`

## Update: run compliance-drift-proofing

### Questions That Would Change the Spec

#### Category: Architecture

- QID: OQ-PLAN-001
  - Q: Should the new validation rules (REQ-001 through REQ-005) be implemented as a single Rust module or distributed across existing modules (drift.rs, structure.rs, control_plane.rs)? [OPEN]
  - Suggested default: Extend existing modules by category (flow boundary enforcement in drift.rs, skills section in structure.rs, OpenQ patterns in a new validation.rs)
  - Impact if different: A single new module would centralize the new rules but create artificial separation from related existing checks
  - Needs answer by: Flow 3
  - Evidence: problem_statement.md -> Relevant Modules/Files (drift.rs, structure.rs, control_plane.rs)

- QID: OQ-PLAN-002
  - Q: How should the `--strict` flag (REQ-005) interact with existing pack-check exit codes and error reporting? [OPEN]
  - Suggested default: Add a new exit code (e.g., 2) for "warnings present" when --strict is used, distinct from failure (1) and success (0)
  - Impact if different: Reusing exit code 1 for both errors and strict-mode warnings reduces CI diagnostic granularity
  - Needs answer by: Flow 3
  - Evidence: requirements.md -> REQ-005, NFR-COMP-001 (backward compatibility)

- QID: OQ-PLAN-003
  - Q: Should the warning/error distinction be per-rule (each rule has its own severity) or global (--strict elevates all warnings uniformly)? [OPEN]
  - Suggested default: Global --strict flag elevates all new rules uniformly; per-rule severity is a future enhancement
  - Impact if different: Per-rule severity adds configuration complexity but provides finer-grained control
  - Needs answer by: Flow 3
  - Evidence: requirements.md -> REQ-005 AC-1 (--strict elevates warnings)

#### Category: Integration

- QID: OQ-PLAN-004
  - Q: Should REQ-003 (OpenQ prefix validation) normalize to PLN/BLD (per openq-tools/SKILL.md) or PLAN/BUILD (per stable-markers.md and contracts.md)? [OPEN]
  - Suggested default: Normalize to PLN/BLD (openq-tools is the implementation and uses these abbreviations; stable-markers.md line 60 appears to be a documentation error)
  - Impact if different: Normalizing to PLAN/BUILD requires updating openq-tools Rust code and existing QIDs
  - Needs answer by: Flow 2
  - Evidence: openq-tools/SKILL.md lines 31, 140-149 vs contracts.md line 184, stable-markers.md line 60
  - Linked signal question: OQ-SIG-002

- QID: OQ-PLAN-005
  - Q: Should REQ-001 skill subcommand detection be based on a static list in contracts.rs or derived from demoswarm CLI help output? [OPEN]
  - Suggested default: Static list in contracts.rs (already contains skill-related patterns; dynamic detection adds runtime dependency)
  - Impact if different: Dynamic detection auto-updates when skills are added but introduces fragility
  - Needs answer by: Flow 3
  - Evidence: requirements.md -> REQ-001 AC-2, NFR-MAINT-001 MET-1
  - Linked signal question: OQ-SIG-006

- QID: OQ-PLAN-006
  - Q: How should REQ-002 (Skills section enforcement) identify which agents "invoke demoswarm.sh"? [OPEN]
  - Suggested default: Literal string match for "demoswarm.sh" in agent file content (simple, deterministic)
  - Impact if different: Regex-based detection could catch indirect invocations but adds complexity and potential false positives
  - Needs answer by: Flow 3
  - Evidence: requirements.md -> REQ-002 AC-1, AC-3

#### Category: Testing

- QID: OQ-PLAN-007
  - Q: What is the minimum viable structure for the Build-to-Gate test fixtures (REQ-004)? [OPEN]
  - Suggested default: Two fixtures in `tools/demoswarm-pack-check/tests/fixtures/`: (1) valid_build_receipt.json matching build_receipt schema, (2) invalid_build_receipt.json with one missing required field
  - Impact if different: More comprehensive fixtures (wrong types, invalid enums) increase coverage but also test maintenance
  - Needs answer by: Flow 3
  - Evidence: requirements.md -> REQ-004 AC-1 through AC-4
  - Linked signal question: OQ-SIG-010

- QID: OQ-PLAN-008
  - Q: Should pack-check test coverage include unit tests for each new rule, integration tests for --strict behavior, or both? [OPEN]
  - Suggested default: Both: unit tests for individual rule logic in Rust, plus integration tests that run pack-check on fixtures
  - Impact if different: Unit tests only would miss --strict interaction bugs; integration tests only would miss edge cases
  - Needs answer by: Flow 3
  - Evidence: requirements.md -> REQ-004 AC-3, NFR-REL-001 (deterministic output)

#### Category: Migration

- QID: OQ-PLAN-009
  - Q: How should we handle the 4 agents currently missing Skills sections (per OQ-SIG-004)? [OPEN]
  - Suggested default: Identify them during implementation, add missing Skills sections as part of this work, document in ADR
  - Impact if different: If some are intentionally exempt, need exemption mechanism in validation rule
  - Needs answer by: Flow 3
  - Evidence: requirements.md -> REQ-002, problem_statement.md -> concerns (4 of 14 agents may be missing sections)
  - Linked signal question: OQ-SIG-004

- QID: OQ-PLAN-010
  - Q: What is the validation baseline process for establishing no false positives (REQ-006)? [OPEN]
  - Suggested default: Run pack-check on current pack state before introducing new rules; capture output as baseline; verify new rules produce no warnings on existing valid artifacts
  - Impact if different: Without explicit baseline, false positives may be introduced unknowingly
  - Needs answer by: Flow 3
  - Evidence: requirements.md -> REQ-006 AC-4

### Assumptions Made to Proceed

- Assumption: The existing pack-check Rust infrastructure (drift.rs, structure.rs, control_plane.rs, contracts.rs) is stable and can be extended for new validation rules.
  - Rationale: problem_statement.md lists these as relevant modules; 29 existing checks indicates mature infrastructure
  - Impact if wrong: May need refactoring before adding new checks
  - Linked question: OQ-PLAN-001

- Assumption: PLN/BLD are the canonical flow codes for OpenQ prefixes (openq-tools is authoritative over stable-markers.md).
  - Rationale: openq-tools/SKILL.md is the implementation source of truth; stable-markers.md line 60 and contracts.md line 184 appear to have documentation drift
  - Impact if wrong: Would need to update openq-tools Rust code and all existing PLN/BLD QIDs
  - Linked question: OQ-PLAN-004, OQ-SIG-002

- Assumption: The --strict flag pattern is sufficient for warning-first adoption; per-rule severity configuration is not needed for initial implementation.
  - Rationale: REQ-005 specifies a single --strict flag; per-rule configuration adds complexity without demonstrated need
  - Impact if wrong: Would need configuration infrastructure before implementation
  - Linked question: OQ-PLAN-003

- Assumption: Test fixtures should be committed files, not dynamically generated.
  - Rationale: Signal open questions (OQ-SIG-010) already suggested committed fixtures; reviewability and determinism favor static files
  - Impact if wrong: Would need CI infrastructure to generate fixtures
  - Linked question: OQ-PLAN-007, OQ-SIG-010

- Assumption: The 4 agents missing Skills sections are gaps to remediate, not intentional exceptions.
  - Rationale: No exemption policy documented; consistency is the pack norm; ASM-004 in requirements.md takes same position
  - Impact if wrong: Would need exemption list in validation rule
  - Linked question: OQ-PLAN-009, OQ-SIG-004

### Resolutions (if any)

(none yet)

### Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
output_path: .runs/compliance-drift-proofing/plan/open_questions.md
questions_added: 10
assumptions_added: 5
missing_required: []
blockers: []
concerns:
  - stable-markers.md and contracts.md say PLAN/BUILD but openq-tools uses PLN/BLD (OQ-PLAN-004 captures this)
  - 4 agents missing Skills sections not yet enumerated (deferred to implementation per OQ-PLAN-009)
