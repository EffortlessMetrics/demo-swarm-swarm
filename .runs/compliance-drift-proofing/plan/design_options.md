# Design Options

## Requirement IDs Enumerated

From `.runs/compliance-drift-proofing/signal/requirements.md`:

### Functional Requirements
- REQ-001: Flow Boundary Enforcement (demoswarm.sh and skill CLI syntax in flow commands)
- REQ-002: Skills Section Enforcement for demoswarm.sh Users
- REQ-003: OpenQ Prefix Pattern Validation
- REQ-004: Build-to-Gate Handshake Test Scenario
- REQ-005: Warning-First Validation Mode (--strict flag)
- REQ-006: No False Positives on Existing Artifacts

### Non-Functional Requirements
- NFR-PERF-001: CI Validation Runtime (under 30 seconds)
- NFR-REL-001: Deterministic Validation Output
- NFR-OPS-001: Diagnostic Clarity (rule ID, file path, line number)
- NFR-COMP-001: Backward Compatibility
- NFR-SEC-001: No Secrets in Validation Output
- NFR-MAINT-001: Pattern Maintainability (constants in contracts.rs)

---

## OPT-001: Inline Extension of Existing Modules

### Description

This option extends the existing pack-check architecture by adding new checks directly to the current module structure without introducing new abstractions. Each new validation rule is implemented as an additional `CheckSpec` in the most appropriate existing module:

- **REQ-001** (flow boundary): Add check 50 to `drift.rs` (aligns with existing checks 38, 39, 40 that already scan flow commands for patterns)
- **REQ-002** (skills section): Already implemented as check 49 in `drift.rs`; verify coverage and possibly enhance
- **REQ-003** (OpenQ prefix): Add check 51 to `drift.rs` with regex for QID pattern validation
- **REQ-004** (test fixtures): Create test fixtures in `tools/demoswarm-pack-check/tests/fixtures/` and unit tests using standard Rust test conventions
- **REQ-005** (--strict flag): Already partially implemented via `--strict_warnings` flag in `cli.rs`; verify behavior matches requirements
- **REQ-006** (baseline): Establish baseline by running pack-check before introducing new rules

New constants (skill CLI subcommands, OpenQ flow codes) are added to `contracts.rs` per NFR-MAINT-001. New regex patterns are added to `Regexes` in `contracts.rs`.

This approach follows the pattern of checks 45-49 that were recently added: create a `CheckSpec`, implement the check function, add any required patterns to `contracts.rs`.

### Requirements Fit

| Requirement | Fit | Notes |
|-------------|-----|-------|
| REQ-001 | SATISFIED | Add check 50 to drift.rs scanning flow-*.md for demoswarm.sh and skill subcommands |
| REQ-002 | SATISFIED | Check 49 already exists; verify AC coverage, enhance if needed |
| REQ-003 | SATISFIED | Add check 51 to drift.rs with QID regex validation |
| REQ-004 | SATISFIED | Standard Rust test fixtures + unit tests in tests/ directory |
| REQ-005 | SATISFIED | --strict_warnings already exists; verify exit code behavior |
| REQ-006 | SATISFIED | Run pack-check before changes; no structural changes needed |
| NFR-PERF-001 | SATISFIED | Pattern matching is O(n) file scan; negligible overhead |
| NFR-REL-001 | SATISFIED | File-based scan; inherently deterministic |
| NFR-OPS-001 | SATISFIED | Reporter already supports check_id, file path; line numbers via existing format_line_matches |
| NFR-COMP-001 | SATISFIED | New checks added; existing behavior unchanged |
| NFR-SEC-001 | SATISFIED | Only file paths and rule violations printed; inherits existing behavior |
| NFR-MAINT-001 | SATISFIED | Constants in contracts.rs; existing pattern |

### Trade-offs

| Dimension | Impact | Rationale |
|----------|--------|-----------|
| Structure (coupling, components) | Low | No new modules; follows existing patterns; minimal coupling increase |
| Velocity (time-to-first-change) | Low | Fastest path; copy existing check patterns |
| Governance (auditability, determinism) | Low | Check IDs continue sequential; clear audit trail |
| Operability (on-call, monitoring, failure modes) | Low | Same operational model; no new failure modes |
| Cost (compute, complexity tax) | Low | Minimal additional regex compilation; no new dependencies |

### Reversibility
- Rating: Easy
- Switch effort: If we later want to modularize, checks can be extracted without changing their logic
- Blast radius if wrong: Limited to pack-check tool; no runtime impact on flows

### Risks

| Risk | Likelihood | Impact | Mitigation (if chosen) |
|------|------------|--------|------------------------|
| drift.rs becomes unwieldy | Low | Low | Module already has 14 checks; 2 more is manageable |
| Check ID collision | Low | Med | Review mod.rs comments before assigning IDs 50, 51 |
| --strict_warnings behavior mismatch | Med | Low | Add tests to verify exit code behavior matches REQ-005 |

### Assumptions
- Check 49 fully addresses REQ-002 (impact if wrong: need to enhance check 49 logic)
- --strict_warnings flag already provides the needed warning-to-error elevation (impact if wrong: additional CLI changes needed)
- Two new checks (50, 51) will not exceed 30-second CI budget (impact if wrong: need optimization)

### When to Choose This
Choose this option when speed of delivery is paramount and the existing architecture has proven sufficient. Best when the pack-check codebase is already well-understood by implementers.

---

## OPT-002: Modular Compliance Rules Framework

### Description

This option introduces a new validation module with a pluggable rules architecture. Instead of adding checks directly to existing modules, it creates a new `compliance.rs` module (or `checks/compliance.rs`) that provides:

1. **A ComplianceRule trait** defining the interface for validation rules
2. **Rule registry** allowing rules to be registered and executed uniformly
3. **Rule metadata** including severity (warning/error), category (boundary/pattern/schema), and documentation
4. **Per-rule severity** enabling fine-grained control beyond global --strict

Structure:
```
tools/demoswarm-pack-check/src/
  checks/
    compliance.rs       # New: ComplianceRule trait + registry
    compliance/         # New: directory for rule implementations
      flow_boundary.rs  # REQ-001
      openq_prefix.rs   # REQ-003
      mod.rs
  contracts.rs          # Add constants as in OPT-001
```

Each rule is a struct implementing `ComplianceRule`:
```rust
pub trait ComplianceRule {
    fn id(&self) -> &str;           // "COMP-001"
    fn title(&self) -> &str;
    fn category(&self) -> Category; // Boundary, Pattern, Schema
    fn default_severity(&self) -> Severity; // Warning, Error
    fn check(&self, cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()>;
}
```

The registry discovers and executes rules, and the CLI can filter by category or elevate specific rules to errors.

### Requirements Fit

| Requirement | Fit | Notes |
|-------------|-----|-------|
| REQ-001 | SATISFIED | FlowBoundaryRule implementing ComplianceRule |
| REQ-002 | PARTIAL | Existing check 49 would need migration or coexistence |
| REQ-003 | SATISFIED | OpenQPrefixRule implementing ComplianceRule |
| REQ-004 | SATISFIED | Test fixtures + unit tests for new rules |
| REQ-005 | SATISFIED | Per-rule severity enables finer control; --strict elevates all |
| REQ-006 | TRADE_OFF | Requires baseline validation before and after migration |
| NFR-PERF-001 | SATISFIED | Same scan logic; trait indirection is negligible |
| NFR-REL-001 | SATISFIED | Rules execute in deterministic order via registry |
| NFR-OPS-001 | SATISFIED | Rule metadata includes ID and category for richer diagnostics |
| NFR-COMP-001 | PARTIAL | Coexistence with existing checks requires careful sequencing |
| NFR-SEC-001 | SATISFIED | Same output constraints; no additional exposure |
| NFR-MAINT-001 | SATISFIED | Rules are self-contained modules; constants still in contracts.rs |

### Trade-offs

| Dimension | Impact | Rationale |
|----------|--------|-----------|
| Structure (coupling, components) | Med | Introduces new abstraction layer; more moving parts |
| Velocity (time-to-first-change) | Med | Higher initial investment; trait design + registry |
| Governance (auditability, determinism) | Low | Rule IDs explicit; category metadata improves auditability |
| Operability (on-call, monitoring, failure modes) | Med | New failure modes (rule registration, trait dispatch) |
| Cost (compute, complexity tax) | Med | Trait indirection; new module to maintain |

### Reversibility
- Rating: Moderate
- Switch effort: If framework proves unnecessary, rules can be flattened back to check functions
- Blast radius if wrong: pack-check tool only; could have temporary coexistence with old checks

### Risks

| Risk | Likelihood | Impact | Mitigation (if chosen) |
|------|------------|--------|------------------------|
| Over-engineering for 2-3 rules | Med | Med | Start with minimal trait; avoid premature abstraction |
| Migration disruption | Med | Med | Run old and new checks in parallel during transition |
| Increased test surface | Low | Low | Each rule is independently testable |
| Team learning curve | Med | Low | Document trait interface; provide example rule |

### Assumptions
- Pack-check will continue to grow with more compliance rules (impact if wrong: framework is unnecessary overhead)
- Per-rule severity will be valuable for incremental adoption (impact if wrong: global --strict is sufficient)
- Existing checks should eventually migrate to this framework (impact if wrong: two systems to maintain)

### When to Choose This
Choose this option when you anticipate significant growth in compliance rules and want a foundation for future extensibility. Best when the team values explicit interfaces and self-documenting rule metadata.

---

## OPT-003: Minimal + Deferred Architecture

### Description

This option implements the absolute minimum to satisfy requirements while explicitly deferring architectural investment. It prioritizes:

1. **REQ-002**: Verify existing check 49 is sufficient; fix any gaps with minimal changes
2. **REQ-005**: Verify --strict_warnings behavior; add tests but no new CLI flags
3. **REQ-001 + REQ-003**: Implement as bash guards in `check-doc-drift.sh` rather than Rust
4. **REQ-004**: Create minimal test fixtures (one valid, one invalid) without test infrastructure changes
5. **REQ-006**: Manual baseline verification (run pack-check, document output)

Rationale: The problem statement notes that check-doc-drift.sh already has 6 drift guards. REQ-001 (pattern in flow commands) and REQ-003 (QID pattern) are simple regex checks that can be implemented in bash in ~20 lines each. This avoids Rust compilation and keeps the change surface minimal.

Structure:
```
scripts/
  check-doc-drift.sh    # Add guards #7 (flow boundary) and #8 (OpenQ prefix)
tools/demoswarm-pack-check/
  tests/fixtures/
    build_receipt_valid.json
    build_receipt_invalid.json
```

The bash guards would use standard grep/awk patterns consistent with existing guards 1-6.

### Requirements Fit

| Requirement | Fit | Notes |
|-------------|-----|-------|
| REQ-001 | SATISFIED | Bash guard in check-doc-drift.sh; grep for demoswarm.sh in flow-*.md |
| REQ-002 | SATISFIED | Existing check 49 in Rust; verify coverage |
| REQ-003 | SATISFIED | Bash guard for QID pattern; grep -E "OQ-(SIG\|PLN\|BLD\|GAT\|DEP\|WIS)-[0-9]{3}" |
| REQ-004 | SATISFIED | Minimal test fixtures; no Rust test changes |
| REQ-005 | PARTIAL | --strict_warnings exists; bash guards have separate exit codes |
| REQ-006 | SATISFIED | Manual baseline; documented in ADR |
| NFR-PERF-001 | SATISFIED | Bash grep is fast; negligible impact |
| NFR-REL-001 | SATISFIED | grep output is deterministic |
| NFR-OPS-001 | PARTIAL | Bash guards lack structured rule IDs; less diagnostic clarity |
| NFR-COMP-001 | SATISFIED | No changes to existing pack-check behavior |
| NFR-SEC-001 | SATISFIED | grep prints lines, not file contents; matches existing guards |
| NFR-MAINT-001 | TRADE_OFF | Two tools (bash + Rust) to maintain; no shared constants |

### Trade-offs

| Dimension | Impact | Rationale |
|----------|--------|-----------|
| Structure (coupling, components) | Med | Splits validation across two tools; harder to understand |
| Velocity (time-to-first-change) | Low | Bash is fast to write and test |
| Governance (auditability, determinism) | Med | Two exit codes to interpret; less unified diagnostics |
| Operability (on-call, monitoring, failure modes) | Med | Two scripts to run in CI; two failure modes to handle |
| Cost (compute, complexity tax) | Low | Minimal code; bash has no compilation |

### Reversibility
- Rating: Easy
- Switch effort: Bash guards can be migrated to Rust checks later without breaking changes
- Blast radius if wrong: Limited; bash guards are isolated scripts

### Risks

| Risk | Likelihood | Impact | Mitigation (if chosen) |
|------|------------|--------|------------------------|
| Inconsistent --strict behavior | High | Med | Document that bash guards always fail on violation |
| Two tools to maintain | High | Med | Accept as technical debt; migrate later if needed |
| Cross-platform issues | Med | Med | Test bash guards on Windows (Git Bash) |
| Diagnostic quality regression | Med | Low | Add verbose mode to bash guards |

### Assumptions
- Bash guards are acceptable for CI (impact if wrong: need Rust implementation)
- Windows CI runs bash via Git Bash or WSL (impact if wrong: need Windows-native checks)
- REQ-001 and REQ-003 are simple enough for regex-only detection (impact if wrong: need AST parsing)

### When to Choose This
Choose this option when time pressure is high and architectural debt is acceptable. Best for proof-of-concept or when future requirements are uncertain.

---

## Comparison Matrix

| Dimension | OPT-001 | OPT-002 | OPT-003 |
|-----------|---------|---------|---------|
| REQ coverage (count) | 6/6 | 5/6 | 6/6 |
| NFR coverage (count) | 6/6 | 5/6 | 4/6 |
| Implementation effort | Low | Med | Low |
| Reversibility | Easy | Moderate | Easy |
| Ops burden | Low | Med | Med |
| Primary risk | Module growth | Over-engineering | Tool fragmentation |

---

## Suggested Default (non-binding)

suggested_default: OPT-001
confidence: High

Rationale (tie to IDs):
- REQ-001, REQ-003: Existing drift.rs pattern (checks 38-49) is proven and well-understood; adding 2 more checks is incremental
- REQ-002: Check 49 already exists; only verification and possible enhancement needed
- REQ-005: --strict_warnings flag already exists; verification over new implementation
- NFR-MAINT-001: Constants in contracts.rs pattern already established; no new structure needed
- NFR-COMP-001: No architectural changes means zero backward compatibility risk

What would change this:
- If the team anticipates 10+ compliance rules in the next 6 months, prefer OPT-002 for better organization
- If Rust compilation is a CI bottleneck and bash is sufficient, prefer OPT-003 for speed
- If REQ-005 requires per-rule severity configuration, prefer OPT-002 for the trait-based approach
- If cross-team contribution is expected (rules from multiple authors), prefer OPT-002 for clearer interfaces

---

## Open Questions Affecting Choice

- Q: Does the existing --strict_warnings flag behavior match REQ-005 exactly (exit code 1 when warnings + strict)? — default if unanswered: Assume yes; verify during implementation
- Q: Is check 49 sufficient for REQ-002 or does it need enhancement? — default if unanswered: Verify via grep audit; enhance if gaps found
- Q: Will there be more than 5 compliance rules in the next year? — default if unanswered: Assume no; OPT-001 is sufficient
- Q: Is PLN/BLD canonical for OpenQ prefixes (OQ-PLAN-004)? — default if unanswered: Yes per openq-tools/SKILL.md; update docs to match

---

## Shared Assumptions

- pack-check (Rust) is the preferred venue for structural validation per ASM-002 in requirements.md
- PLN/BLD abbreviations are canonical over PLAN/BUILD per openq-tools/SKILL.md
- Warning-before-failure is acceptable for new validation rules per REQ-005
- Test fixtures should be committed files, not dynamically generated
- The 4 agents missing Skills sections (if any) are gaps to remediate, not intentional exceptions

---

## Machine Summary
status: VERIFIED

recommended_action: PROCEED
route_to_agent: null
route_to_flow: null

missing_required: []

blockers: []

options_proposed: 3
suggested_default: OPT-001
confidence: High
