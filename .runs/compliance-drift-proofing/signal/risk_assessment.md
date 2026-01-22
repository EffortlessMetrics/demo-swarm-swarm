# Risk Assessment

## Machine Summary

status: VERIFIED

recommended_action: PROCEED
route_to_flow: null
route_to_agent: null

blockers: []

missing_required: []

concerns:

- RSK-001 (HIGH) documents prior Gate bounce on #49; warning-first mode (REQ-005) provides mitigation
- 10 open questions remain unresolved; suggested defaults are adopted but may need revision in Flow 2
- PLN vs PLAN prefix discrepancy (OQ-SIG-002) should be resolved before REQ-003 implementation
- RSK-008 (LOW) added: hardcoded skill list may drift as pack evolves

severity_summary:
critical: 0
high: 1
medium: 4
low: 3

## Context

- flow: signal
- run_id: compliance-drift-proofing
- iteration: 2
- inputs_used:
  - `.runs/compliance-drift-proofing/run_meta.json`
  - `.runs/compliance-drift-proofing/signal/early_risks.md`
  - `.runs/compliance-drift-proofing/signal/problem_statement.md`
  - `.runs/compliance-drift-proofing/signal/requirements.md`
  - `.runs/compliance-drift-proofing/signal/open_questions.md`
  - `.runs/compliance-drift-proofing/signal/verification_notes.md`
  - `.runs/compliance-drift-proofing/signal/scope_estimate.md`
- prior_risk_assessments_seen:
  - `.runs/compliance-drift-proofing/signal/risk_assessment.md` (iteration 1)

## Risk Register

| ID      | Category    | Severity | Status    | Summary                                                                   | Owner            |
| ------- | ----------- | -------- | --------- | ------------------------------------------------------------------------- | ---------------- |
| RSK-001 | OPS         | HIGH     | OPEN      | Prior #49 bounce - incomplete foundation may indicate hidden complexity   | pack-maintainers |
| RSK-002 | DATA        | MEDIUM   | OPEN      | PLN/PLAN prefix inconsistency may cause validation confusion              | pack-maintainers |
| RSK-003 | OPS         | MEDIUM   | OPEN      | 4 agents missing Skills sections require remediation alongside validation | agent-authors    |
| RSK-004 | COMPLIANCE  | MEDIUM   | MITIGATED | Warning-first delays enforcement; timeline documented                     | pack-maintainers |
| RSK-005 | PERFORMANCE | LOW      | MITIGATED | CI runtime impact bounded by NFR-PERF-001 metrics                         | pack-maintainers |
| RSK-006 | SECURITY    | LOW      | MITIGATED | Test fixture secrets addressed by NFR-SEC-001 MET-2                       | pack-maintainers |
| RSK-007 | OPS         | MEDIUM   | OPEN      | Scope overlap with #49 may cause merge conflicts                          | pack-maintainers |
| RSK-008 | DATA        | LOW      | OPEN      | Hardcoded skill CLI subcommand list may drift as new skills are added     | pack-maintainers |

## Risk Details

### RSK-001: Prior #49 Gate Bounce Indicates Implementation Complexity

- Category: OPS
- Severity: HIGH
- Status: OPEN
- Evidence:
  - `.runs/compliance-drift-proofing/signal/problem_statement.md` (Prior Art section: "issue #49 bounced at Gate")
  - `.runs/compliance-drift-proofing/signal/early_risks.md` (RSK-001 rated HIGH with same rationale)
  - `.runs/compliance-drift-proofing/signal/scope_estimate.md` ("Prior work bounced" listed as complexity driver)
- Root Cause Analysis:
  - Issue #49 attempted comprehensive documentation ownership alignment
  - Scope was broad (restructuring documentation boundaries)
  - Implementation may have encountered unforeseen integration issues
  - Gate bounce indicates the approach was technically sound but execution was incomplete
- Impact Assessment:
  - Implementation may encounter same blockers that caused #49 to bounce
  - Complexity may be underestimated; M t-shirt size may drift to L
  - Blocking issues may not surface until Build or Gate phases
  - Developer time wasted if this work also bounces at Gate
- Likelihood: MEDIUM (mitigations in place differentiate this work from #49)
- Mitigation:
  - REQ-005 (warning-first mode) enables incremental rollout without hard failures
  - Scope is narrower than #49 (focused on syntactic validation rules, not documentation restructuring)
  - Problem statement explicitly declares non-goal of superseding #49
  - Decomposition into 6 separable subtasks (ST1-ST6) reduces single-point-of-failure risk
  - Non-goal explicitly excludes semantic/behavioral validation (syntactic checks only)
- Verification:
  - Each subtask can be implemented and tested independently
  - Build flow runs pack-check on existing artifacts to detect regressions early
  - Gate flow validates that no existing valid artifacts fail new rules (REQ-006)
  - 40 BDD scenarios provide comprehensive acceptance criteria
- Recommendation:
  - Proceed with warning-first design as primary mitigation
  - Plan flow should review #49 Gate bounce reasons and document known blockers
  - Consider incremental PRs per subtask rather than monolithic implementation

### RSK-002: PLN vs PLAN Prefix Inconsistency

- Category: DATA
- Severity: MEDIUM
- Status: OPEN
- Evidence:
  - `.runs/compliance-drift-proofing/signal/open_questions.md` (OQ-SIG-002: "stable-markers.md line 60 uses PLAN/BUILD but openq-tools uses PLN/BLD")
  - `.runs/compliance-drift-proofing/signal/requirements.md` (ASM-002: "PLN/BLD abbreviations are canonical over PLAN/BUILD")
- Root Cause Analysis:
  - Documentation (stable-markers.md) and implementation (openq-tools) diverged
  - No automated enforcement existed to detect this drift
  - Manual documentation updates did not synchronize with code changes
  - The very drift this feature is designed to prevent already exists in the pack
- Impact Assessment:
  - REQ-003 validation may produce false positives if canonical prefix is wrong
  - Existing QIDs using PLAN/BUILD would need migration if PLN/BLD is enforced
  - Documentation inconsistency erodes confidence in pack contracts
  - If assumption ASM-002 is wrong, openq-tools Rust code needs modification
- Likelihood: LOW (openq-tools is implementation source of truth; docs likely outdated)
- Mitigation:
  - Assumption ASM-002 adopts PLN/BLD as canonical (matches implementation in openq-tools)
  - OQ-SIG-002 flagged for resolution by Flow 3 (before implementation)
  - stable-markers.md line 60 to be updated if assumption holds
  - NFR-MAINT-001 MET-2 requires OpenQ flow codes as constants in contracts.rs
- Verification:
  - Flow 2 (Plan) should resolve OQ-SIG-002 definitively
  - Update stable-markers.md to match openq-tools before Build phase
  - REQ-006 validation confirms no false positives on existing artifacts
- Recommendation:
  - Resolve OQ-SIG-002 in Flow 2 before proceeding to implementation
  - Document decision in ADR if prefix normalization requires migration

### RSK-003: Agents Missing Skills Sections Require Concurrent Remediation

- Category: OPS
- Severity: MEDIUM
- Status: OPEN
- Evidence:
  - `.runs/compliance-drift-proofing/signal/problem_statement.md` (Who Is Affected: "10 of 14 users have them" implies 4 missing)
  - `.runs/compliance-drift-proofing/signal/requirements.md` (ASM-004: "The 4 agents using demoswarm.sh without Skills sections are gaps to fix")
  - `.runs/compliance-drift-proofing/signal/open_questions.md` (OQ-SIG-004: specific agents not enumerated)
- Root Cause Analysis:
  - Skills section requirement was introduced after some agents were authored
  - No automated enforcement existed to catch missing sections
  - Agent authors may not have been aware of the requirement
  - Gap detection requires explicit audit (grep-based enumeration)
- Impact Assessment:
  - REQ-002 validation will immediately produce warnings on 4 agents
  - Agent remediation work runs in parallel with validation rule development
  - If agents are intentionally exempt, validation logic needs exemption handling
  - False positives risk if agents use demoswarm.sh only in comments/examples
- Likelihood: HIGH (audit evidence suggests 4 agents are affected)
- Mitigation:
  - OQ-SIG-004 asks for enumeration; suggested default is "no exemptions"
  - Work plan should include agent remediation as explicit subtask
  - Warning-first mode (REQ-005) prevents CI breakage while remediation occurs
  - REQ-002 AC-3 excludes agents using demoswarm.sh only via skill invocation
- Verification:
  - Audit via grep to enumerate specific agents before Build phase
  - Add Skills sections to identified agents during Build phase
  - REQ-002 AC-4 confirms existing agents with Skills sections continue to pass
- Recommendation:
  - Enumerate the 4 missing agents in Flow 2 (Plan) work plan
  - Include agent remediation in work decomposition alongside ST3
  - Document any intentional exemptions rather than assuming none exist

### RSK-004: Warning-First Mode May Delay Compliance Enforcement

- Category: COMPLIANCE
- Severity: MEDIUM
- Status: MITIGATED
- Evidence:
  - `.runs/compliance-drift-proofing/signal/requirements.md` (REQ-005: "new compliance rules produce warnings by default")
  - `.runs/compliance-drift-proofing/signal/open_questions.md` (OQ-SIG-001: "warnings vs failures open question")
  - `.runs/compliance-drift-proofing/signal/early_risks.md` (RSK-004: "teams never opt into --strict")
- Root Cause Analysis:
  - Incremental adoption strategy prioritizes backward compatibility over immediate enforcement
  - Without mandatory enforcement timeline, warnings can be ignored indefinitely
  - Warning fatigue is a known anti-pattern in developer tooling
  - No policy exists for when --strict becomes mandatory
- Impact Assessment:
  - Drift can continue if warnings are ignored and --strict is never enabled
  - Long-term compliance debt accumulates without enforcement deadlines
  - Warning fatigue may reduce attention to legitimate violations
  - Investment in validation rules has reduced ROI if never enforced
- Likelihood: MEDIUM (depends on organizational discipline)
- Mitigation:
  - REQ-005 explicitly documents --strict flag as enforcement mechanism
  - NFR-OPS-001 MET-1 requires warnings include rule ID and file path (actionable)
  - Future CI gate can require --strict (documented in early_risks.md)
  - Migration path for each rule documented per NFR-COMP-001 MET-3
  - REQ-005 provides infrastructure; enforcement policy is separate concern
- Verification:
  - pack-check documentation (--help) includes timeline guidance
  - CI configuration can add --strict assertion when ready
  - Monitoring warning counts over time detects drift vs improvement
- Recommendation:
  - Status: MITIGATED because enforcement mechanisms exist
  - Document recommended timeline for warning-to-error promotion in CLAUDE.md
  - Consider pack version milestone where --strict becomes default

### RSK-005: CI Runtime Impact from New Validation Rules

- Category: PERFORMANCE
- Severity: LOW
- Status: MITIGATED
- Evidence:
  - `.runs/compliance-drift-proofing/signal/requirements.md` (NFR-PERF-001: "< 30 seconds total", "< 5 seconds incremental")
  - `.runs/compliance-drift-proofing/signal/verification_notes.md` (MET-1, MET-2: timing assertions in CI)
  - `.runs/compliance-drift-proofing/signal/early_risks.md` (RSK-005: "profile pack-check after each rule")
- Root Cause Analysis:
  - Each validation rule adds scanning and pattern matching overhead
  - 3-5 new rules (REQ-001 through REQ-003, plus infrastructure) add cumulative cost
  - Complex regex patterns can be performance hotspots
  - File system traversal overhead compounds with more rules
- Impact Assessment:
  - Adding 3-5 new validation rules increases pack-check runtime
  - Exceeding 30-second bound degrades developer feedback loop
  - Complex regex patterns or file scanning could be performance hotspots
  - CI pipeline duration impacts merge velocity
- Likelihood: LOW (Rust implementation provides good baseline performance)
- Mitigation:
  - NFR-PERF-001 establishes explicit bounds (30s total, 5s incremental)
  - CI pipeline includes timing assertions to catch regressions
  - Incremental addition measured independently per rule
  - Rust implementation (not bash) provides baseline performance advantage
  - Current pack-check is well under 30 seconds (per early_risks.md)
- Verification:
  - CI job measures baseline before rule introduction
  - Each new rule validated against 5-second incremental budget
  - pack-check profiling identifies hotspots if bounds exceeded
- Recommendation:
  - Status: MITIGATED because NFRs bound the risk
  - Profile pack-check in Build phase after each rule implementation
  - Optimize regex patterns if approaching bounds

### RSK-006: Test Fixtures May Contain Sensitive Data Patterns

- Category: SECURITY
- Severity: LOW
- Status: MITIGATED
- Evidence:
  - `.runs/compliance-drift-proofing/signal/requirements.md` (NFR-SEC-001 MET-2: "Test fixtures do not contain real secrets")
  - `.runs/compliance-drift-proofing/signal/verification_notes.md` (MET-2: "Review test fixtures for real credentials")
  - `.runs/compliance-drift-proofing/signal/early_risks.md` (RSK-006: "Use obviously synthetic values")
- Root Cause Analysis:
  - Test fixtures for REQ-004 (Build-to-Gate handshake) contain JSON receipts
  - Realistic-looking data aids testing but risks false positive from secrets-sanitizer
  - Copy-paste from production artifacts could accidentally include real credentials
  - Developer convenience may prioritize realism over safety
- Impact Assessment:
  - Realistic-looking secrets in fixtures could confuse secrets-sanitizer
  - Test data could accidentally leak if committed without review
  - False positive blocking from secrets-tools during Gate
  - secrets-sanitizer may flag fixtures, blocking publish surface
- Likelihood: LOW (explicit NFR addresses this)
- Mitigation:
  - NFR-SEC-001 MET-2 explicitly requires no real secrets in fixtures
  - REQ-004 AC-1/AC-2 specify fixture requirements without realistic secrets
  - Code review in Gate phase verifies fixture safety
  - pack-check only prints paths, not file contents (NFR-SEC-001 MET-1)
  - OQ-SIG-010 asks about fixture location; committed fixtures are reviewable
- Verification:
  - Gate flow runs secrets-sanitizer on test fixtures
  - Code review checklist includes fixture secret audit
  - Test fixtures use obviously synthetic placeholders (e.g., "TEST_VALUE_ONLY")
- Recommendation:
  - Status: MITIGATED because NFR-SEC-001 addresses this explicitly
  - Include fixture review in Build phase code critique
  - Document synthetic value convention in test fixture headers

### RSK-007: Scope Overlap with Bounced #49 May Cause Conflicts

- Category: OPS
- Severity: MEDIUM
- Status: OPEN
- Evidence:
  - `.runs/compliance-drift-proofing/signal/problem_statement.md` (Non-Goals: "not superseding #49")
  - `.runs/compliance-drift-proofing/signal/open_questions.md` (OQ-SIG-008: "relationship to bounced #49")
  - `.runs/compliance-drift-proofing/signal/early_risks.md` (RSK-007: "Proceed independently per OQ-SIG-008")
- Root Cause Analysis:
  - Issue #49 and this work both modify pack validation infrastructure
  - Common files: drift.rs, contracts.rs, check-doc-drift.sh
  - #49 is OPEN (bounced at Gate) and may resume at any time
  - No coordination mechanism exists between concurrent pack development efforts
- Impact Assessment:
  - If #49 work resumes, changes may conflict with this implementation
  - File-level merge conflicts in drift.rs, contracts.rs, check-doc-drift.sh
  - Duplicated effort if both tracks implement similar rules
  - Conflict resolution overhead delays both efforts
- Likelihood: MEDIUM (#49 is OPEN and may resume)
- Mitigation:
  - OQ-SIG-008 suggested default: "Proceed independently"
  - This work is additive (new rules) not modifying #49 artifacts
  - Clear scope boundary: this work adds enforcement, #49 addressed boundaries
  - Communication path exists if #49 resumes (same maintainer team)
  - Problem statement non-goal: "not superseding #49"
- Verification:
  - Monitor #49 issue status during implementation
  - If #49 resumes, coordinate on file ownership before merge
  - Use atomic commits per subtask to simplify conflict resolution
- Recommendation:
  - Proceed independently per suggested default
  - Document in ADR that this work complements but does not depend on #49
  - If #49 resumes, escalate for coordination rather than proceeding blindly

### RSK-008: Hardcoded Skill CLI Subcommand List May Drift

- Category: DATA
- Severity: LOW
- Status: OPEN
- Evidence:
  - `.runs/compliance-drift-proofing/signal/early_risks.md` (RSK-008: "skill CLI subcommand list drift")
  - `.runs/compliance-drift-proofing/signal/open_questions.md` (OQ-SIG-006: "dynamic vs hardcoded skill list")
  - `.runs/compliance-drift-proofing/signal/requirements.md` (NFR-MAINT-001 MET-1: "constant definition in contracts.rs")
- Root Cause Analysis:
  - REQ-001 requires validating that flow commands do not contain skill CLI subcommands
  - Subcommand list (count, ms, yaml, index, receipt, receipts, openapi, line, inv, time, openq, secrets) is hardcoded
  - When new skills or subcommands are added, the list becomes stale
  - No automated mechanism detects when the list needs updating
- Impact Assessment:
  - New skill subcommands will not be caught by REQ-001 validation
  - Drift between actual skills and validated list reduces enforcement effectiveness
  - Maintenance burden: remember to update contracts.rs when adding skills
  - False sense of compliance if validation is incomplete
- Likelihood: MEDIUM (skills are relatively stable but do evolve)
- Mitigation:
  - NFR-MAINT-001 MET-1 requires skill list as constant in contracts.rs (single update location)
  - NFR-MAINT-001 MET-3 bounds change locality to 2 files (contracts.rs + CLAUDE.md)
  - OQ-SIG-006 remains open on dynamic vs hardcoded; suggested default is hardcoded
  - Documentation update process can include contracts.rs in skill addition checklist
- Verification:
  - Code review for skill additions should include contracts.rs update check
  - pack-check --help lists recognized subcommands (discrepancy is visible)
  - CI could compare contracts.rs list against .claude/skills/ directory (enhancement)
- Recommendation:
  - Accept LOW risk with hardcoded list per OQ-SIG-006 suggested default
  - Document update process when adding new skills
  - Consider future enhancement: automated skill discovery from directory structure

## Cross-Risk Interactions

### RSK-001 + RSK-007 (Compounding Complexity)

The prior bounce of #49 (RSK-001) combined with potential scope overlap (RSK-007) creates a compounding risk. If #49 bounced due to issues that this work inherits (e.g., same validation approach), AND #49 resumes with conflicting changes, the combined complexity could push the estimate from M to L or cause this work to also bounce at Gate.

Mitigation: The narrower scope of this work (validation rules only, not documentation restructuring) reduces the overlap surface. Warning-first mode (REQ-005) provides a graceful degradation path if blockers emerge. Problem statement explicitly limits scope to syntactic checks (non-goal: semantic validation).

### RSK-002 + RSK-003 (Documentation vs Implementation Gap)

The PLN/PLAN discrepancy (RSK-002) and missing Skills sections (RSK-003) both represent gaps between documentation and implementation. Addressing them in parallel creates coordination overhead - the validation rules (REQ-002, REQ-003) must be implemented after the documentation is corrected, not before.

Mitigation: Resolve OQ-SIG-002 (prefix canonicalization) in Flow 2 before implementation. Enumerate missing agents in Flow 2 work plan. Use warning-first mode to allow validation development to proceed while remediation occurs in parallel.

### RSK-004 + RSK-005 (Enforcement vs Performance Tradeoff)

Warning-first mode (RSK-004) delays enforcement, but adding strict mode later may reveal performance issues (RSK-005) that were hidden when rules only produced warnings. The performance bounds in NFR-PERF-001 should be validated with --strict enabled, not just in warning mode.

Mitigation: NFR-PERF-001 metrics apply regardless of warning/error mode. CI validation should measure timing with --strict flag to ensure bounds hold when enforcement is enabled.

### RSK-008 + RSK-002 (Pattern Drift Compounds)

Both RSK-008 (skill list drift) and RSK-002 (prefix inconsistency) represent data pattern maintenance challenges. The same organizational discipline required to keep prefix patterns synchronized must also maintain the skill subcommand list. If one drifts, the other likely will too.

Mitigation: NFR-MAINT-001 centralizes pattern definitions in contracts.rs. A single "pattern contract audit" during skill/flow additions can verify both are current. Future enhancement could automate this check.

## Risk Category Analysis

### Security (NFR-SEC-001)

- RSK-006 (test fixtures) is the only security risk identified
- Status: MITIGATED through explicit NFR requirements
- Validation tools print paths not contents, limiting exposure surface
- Test fixture review is part of Gate phase code review
- No elevated security risks identified for this feature

### Compliance

- RSK-004 addresses compliance enforcement delay
- Status: MITIGATED through --strict flag infrastructure
- Enforcement policy is separate from enforcement infrastructure
- No pack contract violations introduced by this feature
- Feature designed to prevent future contract violations

### Performance (NFR-PERF-001)

- RSK-005 addresses CI runtime impact
- Status: MITIGATED through explicit bounds (30s total, 5s incremental)
- Rust implementation provides good baseline performance
- Profiling recommended during Build phase
- Current baseline is well under bounds

### Data

- RSK-002 (prefix inconsistency) and RSK-008 (skill list drift) are data pattern risks
- Both require ongoing maintenance discipline
- NFR-MAINT-001 centralizes pattern definitions
- Resolution path documented for both

### Operational

- RSK-001 (prior bounce), RSK-003 (missing Skills), RSK-007 (scope overlap) are operational
- RSK-001 is the highest severity risk (HIGH)
- All have documented mitigations and verification strategies
- Deployment considerations: warning-first mode enables gradual rollout

## Deltas Since Prior (if any)

- NEW: [RSK-008]
- CHANGED: [RSK-001, RSK-002, RSK-003, RSK-004, RSK-005, RSK-006, RSK-007]
- CLOSED: []

Changes in iteration 2:

- RSK-008 added from early_risks.md (skill CLI subcommand list drift)
- All existing risks enhanced with:
  - Root cause analysis
  - Likelihood assessment
  - Cross-reference to open questions
  - Updated evidence from iteration 2 artifacts
- New cross-risk interaction: RSK-008 + RSK-002 (pattern drift compounds)
- Risk category analysis section added per task requirements
- severity_summary updated: low: 3 (was 2)

## Recommended Next

1. **Proceed to Flow 2 (Plan)** with status VERIFIED - no unmitigated CRITICAL risks; HIGH risk RSK-001 has documented mitigations
2. **Resolve OQ-SIG-002** (PLN vs PLAN) definitively in Flow 2 before Build implementation
3. **Enumerate the 4 missing agents** (RSK-003) in Flow 2 work plan with explicit remediation tasks
4. **Review #49 Gate bounce reasons** during Plan phase to avoid repeating same blockers
5. **Monitor #49 status** - if work resumes, escalate for coordination before proceeding to Build
