# Design Validation for compliance-drift-proofing

## Machine Summary

```yaml
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns:
  - OQ-PLAN-004 (PLN vs PLAN prefix) should be resolved definitively before implementation; assumed PLN/BLD canonical per openq-tools
  - 4 agents missing Skills sections not enumerated (deferred to ST-003/ST-004 per work_plan.md)
  - TBD runbook paths in observability_spec.md (acceptable at Plan phase)
can_further_iteration_help: no
```

## Iteration Control

```yaml
can_further_iteration_help: no
rationale: "All plan artifacts are coherent and bind cleanly. The remaining open questions (PLN prefix, agent enumeration) have documented suggested defaults and are implementation-phase concerns. No CRITICAL or MAJOR issues identified that would benefit from Plan-phase iteration."
```

## Metrics

```yaml
severity_summary:
  critical: 0
  major: 0
  minor: 6
coverage_summary:
  requirements_total: 12
  requirements_addressed: 12
  contracts_defined: 8
  subtasks_planned: 12
  risks_identified: 6
  risks_mitigated: 6
```

## Summary

- All 6 REQ and 6 NFR identifiers from requirements.md are traced through ADR, contracts, test plan, and work plan with explicit bindings.
- ADR clearly chooses OPT-001 (Inline Extension of Existing Modules) with 5 drivers, 2 rejected alternatives, and comprehensive traceability to requirements.
- Contracts (api_contracts.yaml + schema.md) define complete CLI interface, check definitions, exit codes, and Build receipt schema with testable validation points.
- Work plan provides 12 subtasks with clear dependency graph, parallelization waves, rollout strategy, and rollback plan.
- Test plan covers 40 scenarios across 6 feature files with P0/P1/P2 priority assignments based on risk assessment.

## Critical Issues

(None)

## Major Issues

(None)

## Minor Issues

- [MINOR] DC-MIN-001: Observability runbook paths are TBD - ALERT-STRICT-001, ALERT-PERF-001, ALERT-BASELINE-001 reference "docs/troubleshooting/\*.md (TBD)". Acceptable at Plan phase; should be created during Build/Gate. Evidence: observability_spec.md lines 169, 176, 183.

- [MINOR] DC-MIN-002: OQ-PLAN-004 prefix resolution deferred - stable-markers.md and contracts.md use PLAN/BUILD while openq-tools uses PLN/BLD. ADR assumes PLN/BLD is canonical (ASM-002). ST-006 in work_plan.md addresses this, but resolution should be explicit before implementation. Evidence: ADR line 119, impact_map.json IMP-009/IMP-010.

- [MINOR] DC-MIN-003: Test plan scenario count discrepancy - test_plan.md Machine Summary shows "scenarios_total: 40" but the scenario inventory tables sum to 40 exactly (6+6+8+5+8+7). This is correct, but the "counts" section shows "requirements_with_scenarios: 6" which only counts REQs, not NFRs. This is accurate (NFRs are non-behavioral) but could be clarified. Evidence: test_plan.md lines 14-22.

- [MINOR] DC-MIN-004: Risk traceability to SLOs - observability_spec.md maps RSK-008 to "CHECK-050 false negative rate" but no metric measures false negatives. This is correctly documented as manual audit mitigation but the traceability table is imprecise. Evidence: observability_spec.md line 337, observability_critique.md OC-MIN-003.

- [MINOR] DC-MIN-005: Exit code 2 semantics - api_contracts.yaml defines exit code 2 as RUNTIME_ERROR, but OQ-PLAN-002 suggests "exit 2 for warnings with --strict" as an alternative. The contract correctly defines current behavior; the open question is resolved by the contract (warnings + strict = exit 1). Informational only. Evidence: api_contracts.yaml lines 79-82, ADR line 134.

- [MINOR] DC-MIN-006: BuildReceipt.tests sub-object flexibility - schema.md marks `tests` as Required=Yes but within tests, passed/failed/skipped are Required=No. This is intentional flexibility (metrics may not always be available) but the asymmetry could confuse implementers. Documentation is adequate. Evidence: schema.md lines 98-108, contract_critique.md CC-MIN-002.

## Traceability Gaps

(None - all requirement identifiers have design coverage)

Verified coverage for all REQ/NFR identifiers:

| Identifier    | ADR                  | Contracts                       | Test Plan                 | Work Plan                | Observability              |
| ------------- | -------------------- | ------------------------------- | ------------------------- | ------------------------ | -------------------------- |
| REQ-001       | DR-001, lines 61, 43 | x-checks.new.50                 | 6 scenarios               | ST-002                   | CHECK-050, SLO-PERF-001    |
| REQ-002       | DR-005, lines 62, 90 | x-checks.existing.49            | 6 scenarios               | ST-003, ST-004           | CHECK-049                  |
| REQ-003       | lines 63, 45         | x-checks.new.51, OpenQId schema | 8 scenarios               | ST-005                   | CHECK-051                  |
| REQ-004       | DR-003, lines 64, 46 | BuildReceipt schema             | 5 scenarios               | ST-008, ST-009, ST-010   | Handshake contract         |
| REQ-005       | DR-002, lines 65, 47 | x-cli-interface.strict_warnings | 8 scenarios               | ST-007                   | Exit code behavior         |
| REQ-006       | lines 66, 49         | Baseline validation             | 7 scenarios               | ST-011                   | ALERT-BASELINE-001         |
| NFR-PERF-001  | DR-004, line 67      | Exit code timing                | P2 non-behavioral         | ST constraints           | SLO-PERF-001, SLO-PERF-002 |
| NFR-REL-001   | DR-003, line 68      | Deterministic output            | NFR-REL-001 verification  | All STs                  | SLO-REL-001, SLO-REL-002   |
| NFR-OPS-001   | DR-004, line 69      | CheckDiagnostic schema          | NFR-OPS-001 verification  | ST-012                   | Diagnostic format          |
| NFR-COMP-001  | DR-002, line 70-71   | Exit code preservation          | NFR-COMP-001 verification | Rollout strategy         | Backward compat            |
| NFR-SEC-001   | line 71              | No secrets in output            | NFR-SEC-001 code review   | Test fixture constraints | PII/secrets guidance       |
| NFR-MAINT-001 | line 72              | x-constants (contracts.rs)      | NFR-MAINT-001 code review | ST-001                   | Constants location         |

## Questions for Humans

- Q1: OQ-PLAN-004 (PLN vs PLAN) - Should the canonical flow codes be PLN/BLD (per openq-tools implementation) or PLAN/BUILD (per stable-markers.md documentation)? Suggested default: PLN/BLD (openq-tools is the implementation source of truth). Impact: Determines REQ-003 regex pattern and which docs need updating.

- Q2: OQ-PLAN-009 (Missing Skills sections) - Are the 4 agents potentially missing Skills sections (per initial concern) gaps to fix or intentional exceptions? Suggested default: Gaps to fix (add sections). Impact: If exceptions, need exemption mechanism in check 49.

- Q3: OQ-IFACE-001 (Prose vs CLI detection) - Should Check 50 flag skill subcommands appearing in prose descriptions, or only in CLI invocation contexts? Suggested default: Only CLI contexts (code blocks, lines with "bash" or "demoswarm"). Impact: Affects false positive rate.

## Strengths

- **Comprehensive ADR with decision drivers**: ADR explicitly binds each driver to requirements and NFRs with concrete rationale. 5 drivers, 6 risks, 6 assumptions, all with stable markers.

- **Complete requirements traceability**: All 6 REQ and 6 NFR identifiers appear in api_contracts.yaml x-traceability section, schema.md Requirements to Interfaces table, test_plan.md coverage matrix, and work_plan.md subtask linkage.

- **Well-structured work breakdown**: 12 subtasks with explicit dependencies, 4 parallelization waves, clear rollout phases, and rollback lever documented.

- **Test plan with priority assignment**: P0/P1/P2 priority based on risk assessment (RSK-001 prior bounce, NFR-COMP-001 backward compat). 40 scenarios mapped to test types (unit/integration/contract/fuzz/perf).

- **Coherent contracts**: api_contracts.yaml and schema.md align with consistent schemas (CheckDiagnostic, RunReport, BuildReceipt, OpenQId). Error model uniform across text and JSON output.

- **Observability adapted for CLI context**: Correctly translates traditional observability concepts (metrics, SLOs, alerts) to CLI diagnostic output patterns with measurable signals.

- **Option critique validates decision-readiness**: option_critique.md confirms OPT-001 is recommended with high confidence, explicit conditions for alternative choices, and all options comparable on consistent axes.

- **Assumption tracking**: Assumptions documented in multiple artifacts (ADR ASM-001 through ASM-006, schema.md ASM-IFACE-001 through ASM-IFACE-004, open_questions.md) with impact-if-wrong analysis.

## Inventory (machine countable)

- DC_MINOR: DC-MIN-001
- DC_MINOR: DC-MIN-002
- DC_MINOR: DC-MIN-003
- DC_MINOR: DC-MIN-004
- DC_MINOR: DC-MIN-005
- DC_MINOR: DC-MIN-006

---

## Detailed Validation

### 1) Handshake Validation (Sentinel Checks)

**design_options.md**

- Contains `## Machine Summary` block: PASS (lines 308-321)
- Contains at least one `## OPT-###:` option heading: PASS (OPT-001 line 25, OPT-002 line 92, OPT-003 line 179)
- No template placeholders in machine fields: PASS

**adr.md**

- Contains `## Machine Summary Block` section: PASS (lines 184-200)
- Contains `ADR_CHOSEN_OPTION:` marker: PASS (line 159 "ADR_CHOSEN_OPTION: OPT-001")
- Contains at least one `DRIVER:` line: PASS (lines 24, 27, 30, 33, 36)
- No template placeholders in machine fields: PASS

### 2) Requirements to Plan Coverage

All 6 REQ identifiers appear with explicit markers:

- REQ-001 through REQ-006 referenced in ADR Decision section (lines 61-66), api_contracts.yaml x-traceability, test_plan.md scenario inventory, work_plan.md subtask linkage

All 6 NFR identifiers appear with explicit markers:

- NFR-PERF-001, NFR-REL-001, NFR-OPS-001, NFR-COMP-001, NFR-SEC-001, NFR-MAINT-001 referenced in ADR (lines 67-72), schema.md traceability table, test_plan.md non-behavioral verification

Requirements are not vague or missing identifiers - they bind to specific plan artifacts.

### 3) Options to ADR Binding

ADR clearly chooses OPT-001 by stable ID:

- Line 40: "We choose **OPT-001: Inline Extension of Existing Modules**."
- Line 159: "ADR_CHOSEN_OPTION: OPT-001"

Rejected alternatives documented with rationale:

- OPT-002: Lines 78, 165-166 "ADR_ALT: OPT-002"
- OPT-003: Lines 80-81, 167 "ADR_ALT: OPT-003"

Trade-offs captured from design_options.md comparison matrix carried into ADR Consequences section (lines 82-98).

### 4) ADR to Contracts Binding

Externally-visible behavior implied by REQs has contract surface:

- REQ-001: Check 50 defined in api_contracts.yaml x-checks.new.50 with patterns and scan_targets
- REQ-002: Check 49 defined in api_contracts.yaml x-checks.existing.49
- REQ-003: Check 51 defined in api_contracts.yaml x-checks.new.51 with OpenQId schema
- REQ-004: BuildReceipt schema in api_contracts.yaml components/schemas/BuildReceipt
- REQ-005: --strict_warnings flag in api_contracts.yaml x-cli-interface.arguments

Error model coherent across endpoints:

- Level enum (pass/warn/fail) applies uniformly to CheckDiagnostic
- Exit codes defined consistently (0=SUCCESS, 1=FAILURE, 2=RUNTIME_ERROR)
- Text and JSON output formats documented with consistent diagnostic structure

### 5) Contracts to Test Plan Binding

Test plan covers contract surfaces:

- 40 scenarios across 6 feature files covering all REQ identifiers
- Contract Test Plan section (test_plan.md lines 236-249) references:
  - Receipt schema assertions (BuildReceipt)
  - Exit code contract (0/non-zero)
  - Diagnostic output contract (rule ID, file path)

BDD scenarios map to requirements:

- flow_boundary_enforcement.feature (6 scenarios) -> REQ-001
- skills_section_enforcement.feature (6 scenarios) -> REQ-002
- openq_prefix_validation.feature (8 scenarios) -> REQ-003
- build_gate_handshake.feature (5 scenarios) -> REQ-004
- warning_first_mode.feature (8 scenarios) -> REQ-005
- no_false_positives.feature (7 scenarios) -> REQ-006

### 6) Design to Observability Binding

Observability spec defines measurable signals for critical journeys:

- SLO-PERF-001: < 30 seconds (NFR-PERF-001), measured via stats.duration_ms
- SLO-PERF-002: < 5 seconds per rule, measured via baseline delta
- SLO-REL-001: 100% byte-identical output, measured via hash comparison
- SLO-REL-002: Sorted by file path + rule ID

Alerts are actionable with CI integration:

- ALERT-STRICT-001: Exit code != 0 when --strict enabled (BLOCKING)
- ALERT-PERF-001: duration_ms > 30000 (WARNING)
- ALERT-BASELINE-001: Violation count > baseline (BLOCKING after migration)

Not just "log something" - explicit fields, metrics, SLIs defined.

### 7) Design to Work Plan Binding

Work plan includes tasks for:

- Migrations: ST-006 (PLN/BLD documentation normalization)
- Instrumentation: ST-001 (constants in contracts.rs)
- Testing: ST-008, ST-009, ST-010 (fixtures and test cases)
- Rollout: Phase 0-3 strategy with observability signals for phase gates
- Rollback: Lever documented (remove checks from drift.rs)

All subtasks have REQ/NFR linkage:

- ST-001 -> REQ-001, REQ-003, NFR-MAINT-001
- ST-002 -> REQ-001, NFR-PERF-001, NFR-REL-001, NFR-OPS-001
- ST-003, ST-004 -> REQ-002, REQ-006
- ST-005 -> REQ-003, NFR-PERF-001, NFR-REL-001, NFR-OPS-001
- ST-006 -> REQ-003, NFR-MAINT-001
- ST-007 -> REQ-005, NFR-COMP-001
- ST-008, ST-009, ST-010 -> REQ-004, NFR-SEC-001, NFR-REL-001
- ST-011 -> REQ-006
- ST-012 -> NFR-OPS-001

### 8) Cross-Artifact Consistency

All artifacts use consistent identifiers:

- Requirements: REQ-001 through REQ-006, NFR-PERF-001 through NFR-MAINT-001
- Options: OPT-001, OPT-002, OPT-003
- Risks: RSK-001 through RSK-006
- Assumptions: ASM-001 through ASM-006 (ADR), ASM-IFACE-001 through ASM-IFACE-004 (schema)
- Drivers: DR-001 through DR-005
- Open questions: OQ-PLAN-001 through OQ-PLAN-010, OQ-IFACE-001, OQ-IFACE-002
- Subtasks: ST-001 through ST-012

All Machine Summary blocks use consistent status enum: VERIFIED
All recommended_action values are: PROCEED
No contradictions between artifacts detected.
