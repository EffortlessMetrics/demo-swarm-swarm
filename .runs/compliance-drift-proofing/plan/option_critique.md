# Option Critique for compliance-drift-proofing

## Machine Summary
status: VERIFIED
recommended_action: PROCEED
route_to_flow: null
route_to_agent: null
blockers: []
missing_required: []
concerns: []

## Metrics
severity_summary:
  critical: 0
  major: 0
  minor: 3
options_summary:
  options_found: 3
  options_with_comparable_axes: 3
  options_missing_risks: 0
  options_missing_rollout: 0
  decision_criteria_present: yes

## Summary

The design options document is **decision-ready** and demonstrates high quality in several dimensions:

- **Strong distinctness**: Three genuinely different approaches (incremental extension, framework abstraction, minimal bash-based) with clear architectural differences
- **Excellent comparability**: Consistent structure across all options with comparable trade-off analysis and a unified comparison matrix
- **Thorough traceability**: Requirements fit tables comprehensively map all 6 FReqs and 6 NFRs to each option with explicit fit ratings
- **Honest risk assessment**: Each option identifies concrete risks with likelihood/impact ratings and mitigation strategies
- **Clear recommendation**: OPT-001 suggested with explicit rationale tied to requirement IDs and conditions that would change the choice
- **Open questions captured**: 4 questions affecting choice with suggested defaults

Minor improvements identified relate to testability coverage, rollout specificity, and risk mitigation concreteness—none are blockers to ADR authoring.

## Decision Readiness

Ready for ADR: **yes**

The document provides everything needed for ADR authoring:
- Clear default recommendation (OPT-001) with high confidence
- Well-articulated "when to choose" guidance for each option
- Explicit conditions that would change the recommendation
- Comprehensive requirements coverage analysis
- Concrete risks with mitigations

## Findings

### Distinctness

**All three options are genuinely distinct approaches:**

- **OPT-001**: Incremental, copy-paste existing patterns in drift.rs (structural: low complexity, low coupling increase)
- **OPT-002**: Architectural, introduce trait-based pluggable framework (structural: medium complexity, new abstraction layer)
- **OPT-003**: Minimal, bash-based guards in check-doc-drift.sh (structural: splits validation across two tools)

Each differs in trust boundary (Rust vs bash), implementation approach (trait system vs inline checks vs regex guards), and operational complexity (single tool vs two tools, compilation vs script execution).

- [MINOR] OPT-MIN-001: OPT-002 "Requirements Fit" shows REQ-002 as PARTIAL (existing check 49 coexistence), but doesn't specify whether check 49 would migrate to the new framework or remain separate. This ambiguity is minor because it's an implementation detail that won't affect the architectural decision.

### Comparability / Criteria

**Options are highly comparable:**

- Unified comparison matrix (line 256-265) enables direct comparison on 6 dimensions
- Consistent trade-off structure across all options (Structure, Velocity, Governance, Operability, Cost)
- Requirements Fit tables use the same structure (requirement ID, fit rating, notes)
- Each option has "When to Choose This" guidance

Decision criteria are explicit in the "Suggested Default" section (lines 269-286): speed of delivery, proven patterns, backward compatibility risk, and future growth expectations.

- [MINOR] OPT-MIN-002: The comparison matrix shows "REQ coverage (count)" as 6/6, 5/6, 6/6, but doesn't clarify that OPT-002's 5/6 is due to REQ-002 being "PARTIAL" rather than completely unsatisfied. The requirements fit table shows this correctly, but the summary metric could be misleading without reading the detail. This is informational only—ADR author should note that OPT-002 addresses REQ-002 but requires migration/coexistence planning.

### Traceability to Requirements / Constraints

**Strong traceability demonstrated:**

- All options include comprehensive Requirements Fit tables mapping all 6 FReqs and 6 NFRs
- Fit ratings use consistent vocabulary (SATISFIED, PARTIAL, TRADE_OFF) with explanatory notes
- Constraints from problem_statement.md are acknowledged (e.g., three-tier ownership, backward compatibility, CI runtime bounds)
- OPT-001 correctly notes check 49 already exists for REQ-002 (lines 32, 46)
- OPT-003 acknowledges NFR-OPS-001 as PARTIAL due to bash guards lacking structured rule IDs (line 217)

All options acknowledge the core constraints: backward compatibility (NFR-COMP-001), CI runtime (NFR-PERF-001), and pack-check as preferred venue (ASM-002 from requirements.md).

No gaps identified in requirements traceability.

### Risks / Failure Modes / Operability

**Honest and concrete risk assessment:**

Each option includes a risk table with likelihood, impact, and mitigation columns:

- OPT-001: 3 risks (drift.rs growth, check ID collision, --strict behavior mismatch)
- OPT-002: 4 risks (over-engineering, migration disruption, test surface, learning curve)
- OPT-003: 4 risks (inconsistent --strict, two tools to maintain, cross-platform issues, diagnostic regression)

Risks are specific enough to be actionable (e.g., "Check ID collision" mitigated by "Review mod.rs comments before assigning IDs 50, 51").

**Operational concerns are explicitly covered in trade-off tables:**
- OPT-001: "Operability (on-call, monitoring, failure modes)" = Low impact
- OPT-002: "Operability" = Med impact due to "New failure modes (rule registration, trait dispatch)"
- OPT-003: "Operability" = Med impact due to "Two scripts to run in CI; two failure modes to handle"

- [MINOR] OPT-MIN-003: OPT-001 risk table lists "Check ID collision" as Low likelihood / Med impact, but the mitigation is passive ("Review mod.rs comments"). A more concrete mitigation would be "Reserve check IDs 50-51 in mod.rs documentation before implementation" or "Add check ID registry validation to pack-check tests." This is minor because manual review is acceptable for the small number of checks being added (2).

### Rollout / Migration / Backout

**Rollout considerations are addressed:**

- All options include "Reversibility" ratings (Easy, Moderate) with switch effort and blast radius analysis
- REQ-005 (warning-first mode) is acknowledged as the rollout strategy across all options
- REQ-006 (no false positives) is explicitly covered in requirements fit tables
- OPT-002 acknowledges migration complexity: "Run old and new checks in parallel during transition" (line 165)
- OPT-003 acknowledges "Bash guards can be migrated to Rust checks later without breaking changes" (line 233)

All options correctly identify that the blast radius is limited to pack-check tool (no runtime impact on flows).

No significant gaps in rollout planning.

### Testability / Verification Strategy

**Testability is addressed but could be more explicit:**

- REQ-004 (test fixtures) is covered in all options with SATISFIED fit ratings
- All options note that test fixtures should be "committed files, not dynamically generated" (line 303 in Shared Assumptions)
- OPT-001 mentions "Standard Rust test fixtures + unit tests" (line 34)
- OPT-002 mentions "Each rule is independently testable" (line 166)

However, none of the options explicitly discuss how the new checks will be tested beyond test fixtures—e.g., integration tests that verify warning/error output format, --strict flag behavior, or deterministic output (NFR-REL-001).

This gap is acceptable for options evaluation but should be addressed in implementation planning during ADR authoring.

## Notes for ADR Author

**Context to carry forward:**

1. **Existing check 49 coverage**: OPT-001 and OPT-003 assume check 49 already addresses REQ-002 (Skills section enforcement) but note verification is needed. ADR should confirm this assumption or plan enhancement. See impact_map.json IMP-002 for grep evidence.

2. **PLN vs PLAN prefix resolution**: Open question OQ-SIG-002 affects REQ-003 implementation across all options. impact_map.json shows this affects 3 files (stable-markers.md, contracts.md, demoswarm-cli.md). ADR should either resolve this or document the chosen assumption explicitly.

3. **--strict flag verification**: All options assume existing --strict_warnings flag (cli.rs line 38) matches REQ-005 requirements, but this needs verification. If exit code behavior doesn't match, OPT-001/OPT-003 require small CLI changes while OPT-002's per-rule severity provides more flexibility.

4. **Future growth consideration**: The recommendation explicitly states conditions that would favor OPT-002 (10+ rules in 6 months, cross-team contribution, per-rule severity needs). ADR should document whether these conditions are expected.

5. **Test strategy**: While all options cover REQ-004 test fixtures, none elaborate on integration test strategy for the validation tool itself. ADR implementation plan should address NFR-REL-001 (deterministic output) and NFR-OPS-001 (diagnostic format) verification.

6. **Risk RSK-001 from early_risks.md**: Prior work (issue #49) bounced at Gate. All options mitigate via warning-first mode and narrower scope (syntactic vs semantic checks), but ADR should explicitly acknowledge this historical context.

## Inventory (machine countable)
- OPT_CRITICAL: (none)
- OPT_MAJOR: (none)
- OPT_MINOR: OPT-MIN-001, OPT-MIN-002, OPT-MIN-003
