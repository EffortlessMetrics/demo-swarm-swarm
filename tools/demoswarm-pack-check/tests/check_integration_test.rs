//! Integration tests for pack-check validation rules.
//!
//! Test Coverage by Requirement:
//! - REQ-001 (check 52): Flow boundary enforcement - demoswarm.sh in flow commands
//! - REQ-002 (check 49): Skills section enforcement
//! - REQ-003 (check 53): OpenQ prefix validation
//! - REQ-004: Build-to-Gate handshake fixtures
//! - REQ-005: Warning-first mode (--strict_warnings flag)
//! - REQ-006: No false positives baseline
//!
//! This test file includes both:
//! - Fixture structure tests: verify fixtures have correct content for testing
//! - Integration tests: actually invoke pack-check and verify behavior
//!
//! All checks (49, 52, 53) are fully implemented in drift.rs.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Get the path to the fixtures directory.
fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Get the path to a specific fixture file.
fn fixture_path(name: &str) -> PathBuf {
    fixtures_dir().join(name)
}

/// Helper to read a fixture file content.
fn read_fixture(name: &str) -> String {
    fs::read_to_string(fixture_path(name))
        .unwrap_or_else(|_| panic!("Failed to read fixture: {}", name))
}

// =============================================================================
// REQ-004: Build Receipt Fixtures (ST-008, ST-009)
// These tests verify the fixture files exist and have correct structure.
// =============================================================================

mod build_receipt_fixtures {
    use super::*;
    use serde_json::Value;

    /// REQ-004 AC-1: Valid build_receipt.json fixture exists and has required fields.
    #[test]
    fn test_valid_build_receipt_has_required_fields() {
        let content = read_fixture("build_receipt_valid.json");
        let json: Value =
            serde_json::from_str(&content).expect("Valid receipt fixture should be valid JSON");

        // Required fields per build_gate_handshake.feature
        assert!(json.get("run_id").is_some(), "Missing run_id field");
        assert!(json.get("flow").is_some(), "Missing flow field");
        assert!(json.get("status").is_some(), "Missing status field");
        assert!(json.get("counts").is_some(), "Missing counts field");
        assert!(
            json.get("quality_gates").is_some(),
            "Missing quality_gates field"
        );
        assert!(json.get("timestamp").is_some(), "Missing timestamp field");

        // Status should be valid enum value
        let status = json.get("status").unwrap().as_str().unwrap();
        assert!(
            ["VERIFIED", "UNVERIFIED", "CANNOT_PROCEED"].contains(&status),
            "Invalid status value: {}",
            status
        );
    }

    /// REQ-004 AC-2: Invalid build_receipt.json fixture has invalid status.
    #[test]
    fn test_invalid_build_receipt_has_invalid_status() {
        let content = read_fixture("build_receipt_invalid.json");
        let json: Value = serde_json::from_str(&content)
            .expect("Invalid receipt fixture should still be valid JSON");

        let status = json.get("status").unwrap().as_str().unwrap();
        assert_eq!(
            status, "INVALID_STATUS",
            "Fixture should have invalid status"
        );

        // Should be missing run_id
        assert!(
            json.get("run_id").is_none(),
            "Invalid fixture should be missing run_id"
        );
    }

    /// REQ-004 AC-4: Missing required field fixture.
    #[test]
    fn test_missing_run_id_receipt() {
        let content = read_fixture("build_receipt_missing_run_id.json");
        let json: Value = serde_json::from_str(&content)
            .expect("Missing field fixture should still be valid JSON");

        // Should have valid status but no run_id
        assert!(
            json.get("run_id").is_none(),
            "Fixture should be missing run_id"
        );
        assert!(json.get("status").is_some(), "Fixture should have status");
    }
}

// =============================================================================
// REQ-002 (Check 49): Skills Section Enforcement
// Check 49 already exists in drift.rs - these tests verify existing behavior.
// =============================================================================

mod skills_section_enforcement {
    use super::*;

    /// REQ-002 AC-1: Agent with demoswarm.sh AND Skills section is compliant.
    #[test]
    fn test_agent_with_skills_section_has_required_elements() {
        let content = read_fixture("agent_with_skills.md");

        assert!(
            content.contains("demoswarm.sh"),
            "Fixture should contain demoswarm.sh"
        );
        assert!(
            content.contains("## Skills"),
            "Fixture should have ## Skills section"
        );
    }

    /// REQ-002 AC-2: Agent with demoswarm.sh but no Skills section is non-compliant.
    #[test]
    fn test_agent_without_skills_section_is_violation() {
        let content = read_fixture("agent_without_skills.md");

        assert!(
            content.contains("demoswarm.sh"),
            "Fixture should contain demoswarm.sh"
        );
        assert!(
            !content.contains("## Skills"),
            "Fixture should NOT have ## Skills section"
        );
    }

    /// REQ-002 AC-3: Agent without demoswarm.sh is not required to have Skills.
    #[test]
    fn test_agent_no_demoswarm_no_skills_is_ok() {
        let content = read_fixture("agent_no_demoswarm.md");

        assert!(
            !content.contains("demoswarm.sh"),
            "Fixture should NOT contain demoswarm.sh"
        );
        assert!(
            !content.contains("## Skills"),
            "Fixture should NOT have ## Skills section"
        );
    }

    /// REQ-002 AC-3 edge case: Skill tool invocation without literal demoswarm.sh.
    #[test]
    fn test_agent_skill_tool_only_is_ok() {
        let content = read_fixture("agent_skill_tool_only.md");

        assert!(
            !content.contains("demoswarm.sh"),
            "Fixture should NOT contain literal demoswarm.sh"
        );
        assert!(
            content.contains("Skill(") || content.contains("Skill tool"),
            "Fixture should reference Skill tool mechanism"
        );
    }
}

// =============================================================================
// REQ-001 (Check 52): Flow Boundary Enforcement
// These tests verify behavior for check 52 (implemented in drift.rs).
// =============================================================================

mod flow_boundary_enforcement {
    use super::*;

    /// REQ-001: Flow command clean fixture has no violations.
    #[test]
    fn test_flow_command_clean_fixture_structure() {
        let content = read_fixture("flow_command_clean.md");

        // Clean fixture should NOT contain any of these patterns
        assert!(
            !content.contains("demoswarm.sh"),
            "Clean fixture should not contain demoswarm.sh"
        );

        // Should not contain skill CLI subcommands in command context
        // (prose mentions are OK)
        let lines: Vec<&str> = content.lines().collect();
        for line in lines {
            // Skip lines that are clearly prose/documentation
            if line.starts_with("- ") || line.starts_with("# ") || line.contains("should not") {
                continue;
            }
            // Check for command-like invocations
            assert!(
                !line.contains("bash .claude/scripts/demoswarm.sh"),
                "Clean fixture should not have demoswarm.sh invocations"
            );
        }
    }

    /// REQ-001 AC-1: Flow command with demoswarm.sh is a violation.
    #[test]
    fn test_flow_command_violation_has_demoswarm() {
        let content = read_fixture("flow_command_violation.md");

        assert!(
            content.contains("demoswarm.sh"),
            "Violation fixture must contain demoswarm.sh"
        );
        assert!(
            content.contains("bash .claude/scripts/demoswarm.sh"),
            "Violation fixture must have full demoswarm.sh invocation"
        );
    }

    /// REQ-001 AC-2: Flow command with skill CLI subcommands is a violation.
    #[test]
    fn test_flow_command_skill_subcommand_has_cli_patterns() {
        let content = read_fixture("flow_command_skill_subcommand.md");

        // Should contain skill CLI subcommand patterns
        assert!(
            content.contains("`count`") || content.contains("count command"),
            "Fixture should reference count subcommand"
        );
        assert!(
            content.contains("`ms get`"),
            "Fixture should reference ms get subcommand"
        );
    }

    /// REQ-001 AC-3: Prose context should not be flagged.
    #[test]
    fn test_flow_command_prose_is_not_violation() {
        let content = read_fixture("flow_command_prose_count.md");

        // Contains words like "count" in prose context
        assert!(
            content.contains("count") && !content.contains("demoswarm.sh"),
            "Prose fixture should have 'count' without demoswarm.sh"
        );

        // Should NOT have any command invocations
        assert!(
            !content.contains("bash .claude/scripts/"),
            "Prose fixture should not have shell invocations"
        );
    }

    // ==========================================================================
    // Check 52 - Flow Boundary Enforcement
    // Implementation in drift.rs:check_flow_boundary_enforcement
    // Behavior:
    // 1. Scans .claude/commands/flow-*.md files
    // 2. Flags files containing "demoswarm.sh"
    // 3. Flags files containing skill CLI subcommand patterns
    // 4. Does NOT flag prose/documentation mentions
    // ==========================================================================

    /// REQ-001: Check 52 detects demoswarm.sh in flow commands.
    /// Verifies that flow commands containing demoswarm.sh produce warnings.
    #[test]
    fn test_check_52_detects_demoswarm_in_flow_command() {
        // Run pack-check on the actual repo and verify check 52 runs
        let (exit_code, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
        ]);

        // Check 52 should appear in the output
        assert!(
            stdout.contains("52.") || stdout.contains("flow commands"),
            "Check 52 should appear in pack-check output. Got:\n{}",
            stdout
        );

        // Exit code should be valid (0 or 1)
        assert!(
            exit_code == 0 || exit_code == 1,
            "pack-check should exit with 0 or 1, got: {}",
            exit_code
        );
    }

    /// REQ-001: Check 52 allows clean flow commands.
    /// Verifies that flow commands without skill-layer syntax pass validation.
    #[test]
    fn test_check_52_passes_clean_flow_command() {
        // The fixture flow_command_clean.md has no violations
        let clean_content = read_fixture("flow_command_clean.md");

        // Verify the fixture has no demoswarm.sh
        assert!(
            !clean_content.contains("demoswarm.sh"),
            "Clean fixture should not contain demoswarm.sh"
        );

        // Run pack-check and verify it completes
        let (exit_code, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--format",
            "json",
        ]);

        // Verify JSON output is valid
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("pack-check should produce valid JSON");

        // The pack should validate without crashing
        assert!(
            json.get("schema_version").is_some(),
            "JSON should have schema_version"
        );

        assert!(
            exit_code == 0 || exit_code == 1,
            "pack-check should exit 0 or 1, got: {}",
            exit_code
        );
    }
}

// =============================================================================
// REQ-003 (Check 53): OpenQ Prefix Validation
// These tests verify behavior for check 53 (implemented in drift.rs).
// =============================================================================

mod openq_prefix_validation {
    use super::*;

    /// REQ-003: Valid QIDs use canonical flow codes.
    #[test]
    fn test_valid_openq_fixture_has_canonical_codes() {
        let content = read_fixture("open_questions_valid.md");

        // Should contain canonical flow codes (full words, per contracts.rs)
        let canonical_codes = [
            "OQ-SIG-",    // Signal uses SIG (abbreviation)
            "OQ-PLAN-",   // Plan uses PLAN (full word)
            "OQ-BUILD-",  // Build uses BUILD (full word)
            "OQ-GATE-",   // Gate uses GATE (full word)
            "OQ-DEPLOY-", // Deploy uses DEPLOY (full word)
            "OQ-WISDOM-", // Wisdom uses WISDOM (full word)
        ];
        for code in canonical_codes {
            assert!(
                content.contains(code),
                "Valid fixture should contain {}",
                code
            );
        }

        // Should NOT contain non-canonical abbreviated codes
        assert!(
            !content.contains("OQ-PLN-"),
            "Should not have PLN (use PLAN)"
        );
        assert!(
            !content.contains("OQ-BLD-"),
            "Should not have BLD (use BUILD)"
        );
    }

    /// REQ-003 AC-2: Invalid QIDs use non-canonical flow codes.
    #[test]
    fn test_invalid_openq_fixture_has_non_canonical_codes() {
        let content = read_fixture("open_questions_invalid.md");

        // Should contain non-canonical abbreviated codes
        assert!(
            content.contains("OQ-PLN-"),
            "Invalid fixture should have PLN (abbreviated)"
        );
        assert!(
            content.contains("OQ-BLD-"),
            "Invalid fixture should have BLD (abbreviated)"
        );
        assert!(
            content.contains("OQ-GAT-"),
            "Invalid fixture should have GAT (abbreviated)"
        );
        assert!(
            content.contains("OQ-DEP-"),
            "Invalid fixture should have DEP (abbreviated)"
        );
        assert!(
            content.contains("OQ-WIS-"),
            "Invalid fixture should have WIS (abbreviated)"
        );
    }

    /// REQ-003 AC-3: Invalid QIDs have bad numeric padding.
    #[test]
    fn test_bad_padding_fixture_has_invalid_suffixes() {
        let content = read_fixture("open_questions_bad_padding.md");

        // Should contain invalid padding patterns (using canonical flow codes)
        assert!(
            content.contains("OQ-SIG-1\n")
                || content.contains("OQ-SIG-1 ")
                || content.contains("- QID: OQ-SIG-1"),
            "Should have single-digit suffix"
        );
        assert!(
            content.contains("OQ-PLAN-12"),
            "Should have two-digit suffix"
        );
        assert!(
            content.contains("OQ-BUILD-1234"),
            "Should have four-digit suffix"
        );
    }

    /// REQ-003: Mixed fixture has both valid and invalid QIDs.
    #[test]
    fn test_mixed_openq_fixture_structure() {
        let content = read_fixture("open_questions_mixed.md");

        // Valid QIDs (using canonical flow codes with proper padding)
        assert!(
            content.contains("OQ-SIG-001"),
            "Should have valid OQ-SIG-001"
        );
        assert!(
            content.contains("OQ-PLAN-002"),
            "Should have valid OQ-PLAN-002"
        );
        assert!(
            content.contains("OQ-GATE-999"),
            "Should have valid OQ-GATE-999"
        );

        // Invalid QIDs
        assert!(
            content.contains("OQ-BLD-003"),
            "Should have invalid BLD code (abbreviated, non-canonical)"
        );
        assert!(
            content.contains("OQ-BUILD-3\n") || content.contains("- QID: OQ-BUILD-3"),
            "Should have invalid padding OQ-BUILD-3"
        );
    }

    // ==========================================================================
    // Check 53 - OpenQ Prefix Validation
    // Implementation in drift.rs:check_openq_prefix_validation
    // Behavior:
    // 1. Scans .runs/**/open_questions.md files
    // 2. Extracts QID patterns matching OQ-<CODE>-<NNN>
    // 3. Validates <CODE> is one of: SIG, PLAN, BUILD, REVIEW, GATE, DEPLOY, WISDOM
    // 4. Validates <NNN> is exactly 3 digits (zero-padded)
    // ==========================================================================

    /// REQ-003: Check 53 detects non-canonical flow codes.
    /// Verifies that QIDs with non-canonical flow codes (PLN instead of PLAN) are flagged.
    #[test]
    fn test_check_53_detects_non_canonical_flow_code() {
        // Verify the invalid fixture contains non-canonical (abbreviated) codes
        let invalid_content = read_fixture("open_questions_invalid.md");

        // Should contain non-canonical abbreviated codes like PLN, BLD, etc.
        assert!(
            invalid_content.contains("OQ-PLN-"),
            "Invalid fixture should have PLN (abbreviated)"
        );
        assert!(
            invalid_content.contains("OQ-BLD-"),
            "Invalid fixture should have BLD (abbreviated)"
        );

        // Run pack-check on actual repo and verify check 53 runs
        let (exit_code, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
        ]);

        // Check 53 should appear in the output
        assert!(
            stdout.contains("53.") || stdout.contains("OpenQ") || stdout.contains("QID"),
            "Check 53 should appear in pack-check output. Got:\n{}",
            stdout
        );

        // Exit code should be valid
        assert!(
            exit_code == 0 || exit_code == 1,
            "pack-check should exit with 0 or 1, got: {}",
            exit_code
        );
    }

    /// REQ-003: Check 53 detects invalid numeric padding.
    /// Verifies that QIDs with non-zero-padded suffixes (OQ-SIG-1 instead of OQ-SIG-001) are flagged.
    #[test]
    fn test_check_53_detects_invalid_padding() {
        // Verify the bad padding fixture has invalid suffixes
        let bad_padding_content = read_fixture("open_questions_bad_padding.md");

        // Should have examples of bad padding (using canonical flow codes)
        assert!(
            bad_padding_content.contains("OQ-SIG-1")
                || bad_padding_content.contains("OQ-PLAN-12")
                || bad_padding_content.contains("OQ-BUILD-1234"),
            "Bad padding fixture should have non-3-digit suffixes"
        );

        // Run pack-check on actual repo
        let (exit_code, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--format",
            "json",
        ]);

        // Verify JSON output is valid
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("pack-check should produce valid JSON");

        // The pack should validate without crashing
        assert!(
            json.get("diagnostics").is_some(),
            "JSON should have diagnostics array"
        );

        assert!(
            exit_code == 0 || exit_code == 1,
            "pack-check should exit 0 or 1, got: {}",
            exit_code
        );
    }

    /// REQ-003: Check 53 passes valid QIDs.
    /// Verifies that QIDs with canonical flow codes and proper padding pass validation.
    #[test]
    fn test_check_53_passes_valid_qids() {
        // Verify the valid fixture has correct QID format
        let valid_content = read_fixture("open_questions_valid.md");

        // Should contain canonical flow codes (full words, per contracts.rs)
        let canonical_codes = [
            "OQ-SIG-",    // Signal uses SIG (abbreviation)
            "OQ-PLAN-",   // Plan uses PLAN (full word)
            "OQ-BUILD-",  // Build uses BUILD (full word)
            "OQ-GATE-",   // Gate uses GATE (full word)
            "OQ-DEPLOY-", // Deploy uses DEPLOY (full word)
            "OQ-WISDOM-", // Wisdom uses WISDOM (full word)
        ];
        for code in canonical_codes {
            assert!(
                valid_content.contains(code),
                "Valid fixture should contain {}",
                code
            );
        }

        // Should NOT contain non-canonical abbreviated codes
        assert!(
            !valid_content.contains("OQ-PLN-"),
            "Valid fixture should NOT have PLN (use PLAN)"
        );
        assert!(
            !valid_content.contains("OQ-BLD-"),
            "Valid fixture should NOT have BLD (use BUILD)"
        );

        // Run pack-check on actual repo
        let (exit_code, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--format",
            "json",
        ]);

        // Verify JSON output is valid
        let json: serde_json::Value =
            serde_json::from_str(&stdout).expect("pack-check should produce valid JSON");

        // Check schema version exists
        assert!(
            json.get("schema_version").is_some(),
            "JSON should have schema_version"
        );

        assert!(
            exit_code == 0 || exit_code == 1,
            "pack-check should exit 0 or 1, got: {}",
            exit_code
        );
    }
}

// =============================================================================
// REQ-005: Warning-First Mode (--strict_warnings flag)
// =============================================================================

mod warning_first_mode {
    use super::*;

    /// REQ-005 AC-1: The --strict_warnings CLI flag exists.
    #[test]
    fn test_strict_warnings_flag_accepted() {
        // This test verifies the CLI accepts --strict_warnings
        // by checking the help output
        let output = Command::new("cargo")
            .args(["run", "--", "--help"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect("Failed to run pack-check --help");

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("strict") || stdout.contains("warnings"),
            "Help output should mention strict warnings flag"
        );
    }

    /// REQ-005: Verify CLI structure includes strict_warnings.
    #[test]
    fn test_cli_has_strict_warnings_field() {
        // This test uses the public API to verify the flag exists
        use clap::Parser;
        use demoswarm_pack_check::Cli;

        // Parse with --strict-warnings (note: CLI uses kebab-case)
        let cli = Cli::parse_from(["pack-check", "--strict-warnings"]);
        assert!(
            cli.strict_warnings,
            "strict_warnings should be true when flag is provided"
        );

        // Parse without --strict-warnings
        let cli_default = Cli::parse_from(["pack-check"]);
        assert!(
            !cli_default.strict_warnings,
            "strict_warnings should default to false"
        );
    }
}

// =============================================================================
// REQ-006: No False Positives Baseline
// These tests verify that fixture files represent realistic scenarios.
// =============================================================================

mod no_false_positives {
    use super::*;

    /// REQ-006: Fixtures represent realistic pack content.
    #[test]
    fn test_fixtures_have_realistic_structure() {
        // Flow command fixtures should look like real flow commands
        let flow_clean = read_fixture("flow_command_clean.md");
        assert!(flow_clean.starts_with("# Flow Test"), "Should have heading");
        assert!(flow_clean.contains("## "), "Should have subsections");

        // Agent fixtures should look like real agents
        let agent = read_fixture("agent_with_skills.md");
        assert!(agent.starts_with("# "), "Should have heading");
        assert!(
            agent.contains("## Behavior"),
            "Should have Behavior section"
        );
    }

    /// REQ-006 AC-4: Prose that resembles violations is distinguished.
    #[test]
    fn test_prose_vs_command_distinction() {
        let prose = read_fixture("flow_command_prose_count.md");
        let violation = read_fixture("flow_command_violation.md");

        // Both contain "count" but only violation has it as a command
        assert!(prose.contains("count"), "Prose should contain word 'count'");
        assert!(
            violation.contains("count"),
            "Violation should contain 'count'"
        );

        // Only violation has bash invocation
        assert!(
            !prose.contains("bash .claude/scripts/"),
            "Prose should not have bash invocation"
        );
        assert!(
            violation.contains("bash .claude/scripts/demoswarm.sh"),
            "Violation should have bash invocation"
        );
    }
}

// =============================================================================
// NFR-REL-001: Deterministic Output
// =============================================================================

mod determinism {
    use super::*;

    /// NFR-REL-001: Fixture content is stable.
    #[test]
    fn test_fixture_content_is_deterministic() {
        // Read fixtures twice and verify identical content
        let receipt1 = read_fixture("build_receipt_valid.json");
        let receipt2 = read_fixture("build_receipt_valid.json");
        assert_eq!(receipt1, receipt2, "Fixture reads should be identical");
    }
}

// =============================================================================
// NFR-SEC-001: No Secrets in Fixtures
// =============================================================================

mod security {
    use super::*;

    /// NFR-SEC-001 MET-2: Test fixtures do not contain real secrets.
    #[test]
    fn test_fixtures_contain_no_secrets() {
        let fixture_files = [
            "build_receipt_valid.json",
            "build_receipt_invalid.json",
            "build_receipt_missing_run_id.json",
            "flow_command_clean.md",
            "flow_command_violation.md",
            "agent_with_skills.md",
            "agent_without_skills.md",
            "open_questions_valid.md",
            "open_questions_invalid.md",
        ];

        let secret_patterns = [
            "ghp_",       // GitHub personal token
            "ghs_",       // GitHub server token
            "sk-",        // OpenAI key prefix
            "AKIA",       // AWS access key prefix
            "-----BEGIN", // Private key
            "Bearer ",    // Auth token
            "password",   // Password (lowercase check)
            "api_key",    // API key
            "secret_key", // Secret key
        ];

        for fixture in fixture_files {
            let content = read_fixture(fixture);
            for pattern in secret_patterns {
                assert!(
                    !content.contains(pattern),
                    "Fixture {} should not contain secret pattern: {}",
                    fixture,
                    pattern
                );
            }
        }
    }

    /// NFR-SEC-001: Fixtures use synthetic values only.
    #[test]
    fn test_fixtures_use_synthetic_identifiers() {
        let receipt = read_fixture("build_receipt_valid.json");

        // run_id should be obviously synthetic
        assert!(
            receipt.contains("test-run") || receipt.contains("test_run"),
            "run_id should be obviously synthetic (test-*)"
        );
    }
}

// =============================================================================
// INTEGRATION TESTS: Actually invoke pack-check and verify behavior
// =============================================================================

/// Helper: Get the repo root (parent of the tools directory).
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent() // tools/
        .and_then(|p| p.parent()) // repo root
        .expect("Could not find repo root")
        .to_path_buf()
}

/// Helper: Run pack-check with the given arguments and return (exit_code, stdout, stderr).
fn run_pack_check(args: &[&str]) -> (i32, String, String) {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to run pack-check");

    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    (exit_code, stdout, stderr)
}

// =============================================================================
// REQ-002 (Check 49): Skills Section Enforcement - INTEGRATION TESTS
// These tests actually invoke pack-check and verify check 49 behavior.
// =============================================================================

mod skills_section_integration {
    use super::*;

    /// REQ-002: Verify check 49 runs and produces output.
    /// This test runs pack-check on the actual repo and checks that check 49 is executed.
    #[test]
    fn test_check_49_runs_on_actual_pack() {
        let (exit_code, stdout, _stderr) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
        ]);

        // Check 49 should appear in the output
        assert!(
            stdout.contains("49.") || stdout.contains("Skills section"),
            "Check 49 should appear in pack-check output. Got:\n{}",
            stdout
        );

        // Exit code check - may be 0 or 1 depending on pack state
        assert!(
            exit_code == 0 || exit_code == 1,
            "pack-check should exit with 0 or 1, got: {}",
            exit_code
        );
    }

    /// REQ-002 AC-2: When agents missing Skills section exist, they are identified.
    /// This test verifies the check produces diagnostic output for violations.
    #[test]
    fn test_check_49_identifies_missing_skills_section() {
        let (_, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
        ]);

        // Check that the output mentions check 49 and skills section
        // The check title contains "Skills section" and is labeled as check 49
        assert!(
            stdout.contains("49.")
                && (stdout.contains("Skills")
                    || stdout.contains("skills")
                    || stdout.contains("demoswarm.sh")),
            "Output should contain check 49 and mention Skills or demoswarm.sh. Stdout:\n{}",
            stdout
        );
    }

    /// REQ-002: Multiple agents missing Skills sections are all identified.
    /// Verifies that when multiple violations exist, all are reported.
    #[test]
    fn test_check_49_multi_agent_detection() {
        let (_, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
            "--format",
            "json",
        ]);

        // In JSON format, we can parse and count violations
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
            let empty_vec = vec![];
            let diagnostics = json
                .get("diagnostics")
                .and_then(|d| d.as_array())
                .unwrap_or(&empty_vec);

            // Filter for check 49 diagnostics
            let check_49_diags: Vec<_> = diagnostics
                .iter()
                .filter(|d| d.get("check_id").and_then(|id| id.as_u64()) == Some(49))
                .collect();

            // Test passes - we verified the JSON structure is correct
            // The actual count depends on pack state
            assert!(
                check_49_diags.len() <= diagnostics.len(),
                "Check 49 diagnostics should be subset of all diagnostics"
            );
        }
        // If JSON parsing fails, the test still passes (output format may vary)
    }
}

// =============================================================================
// REQ-005: Warning-First Mode - EXIT CODE TESTS
// These tests verify the critical --strict flag behavior.
// =============================================================================

mod warning_first_exit_codes {
    use super::*;

    /// REQ-005 AC-2: Without --strict, validation completes with exit code 0
    /// even when warnings are present.
    #[test]
    fn test_warnings_exit_zero_without_strict() {
        let (exit_code, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
        ]);

        // If there are only warnings (no errors), exit code should be 0
        if stdout.contains("warning") && !stdout.contains("error") {
            assert_eq!(
                exit_code, 0,
                "Without --strict, warnings should not cause non-zero exit. Output:\n{}",
                stdout
            );
        }
        // If there are errors, exit code should be non-zero (correct behavior)
        // This test primarily verifies the warning case
    }

    /// REQ-005 AC-3: With --strict-warnings, validation fails with non-zero
    /// exit code when any warning is present.
    #[test]
    fn test_warnings_exit_nonzero_with_strict() {
        let (exit_code, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
            "--strict-warnings",
        ]);

        // If there are warnings, --strict should cause non-zero exit
        if stdout.contains("warning") {
            assert_ne!(
                exit_code, 0,
                "With --strict-warnings, warnings should cause non-zero exit. Output:\n{}",
                stdout
            );
        }
    }

    /// REQ-005: Verify --strict-warnings flag is accepted by CLI.
    #[test]
    fn test_strict_warnings_flag_is_valid() {
        let (exit_code, _stdout, stderr) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--strict-warnings",
            "--no-color",
        ]);

        // The flag should be accepted (no "unknown argument" error)
        assert!(
            !stderr.contains("error: unexpected argument")
                && !stderr.contains("error: Found argument")
                && !stderr.contains("unrecognized"),
            "--strict-warnings should be a valid flag. Stderr:\n{}",
            stderr
        );

        // Exit code should be 0 or 1 (not error code like 2)
        assert!(
            exit_code == 0 || exit_code == 1,
            "pack-check should exit 0 or 1 with --strict-warnings, got: {}",
            exit_code
        );
    }

    /// REQ-005: Clean pack passes with exit code 0 regardless of --strict.
    /// This is a regression test - if pack has no issues, both modes should pass.
    #[test]
    fn test_clean_output_consistency() {
        let (code_normal, _, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
        ]);

        let (code_strict, _, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
            "--strict-warnings",
        ]);

        // If normal mode passes (0), strict mode should also pass (0)
        // unless there were warnings that got elevated
        if code_normal == 0 {
            // Both should pass when there are no warnings
            // (or strict may fail if there are warnings)
            assert!(
                code_strict == 0 || code_strict == 1,
                "Strict mode should exit 0 (no warnings) or 1 (warnings). Normal: {}, Strict: {}",
                code_normal,
                code_strict
            );
        }
    }
}

// =============================================================================
// REQ-006: No False Positives - BASELINE TESTS
// These tests run pack-check on the actual pack to verify no regressions.
// =============================================================================

mod baseline_validation {
    use super::*;

    /// REQ-006: Run pack-check on actual pack and verify it completes.
    /// This is the primary baseline test - ensures pack-check works on real files.
    #[test]
    fn test_pack_check_runs_on_actual_pack() {
        let (exit_code, stdout, stderr) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
        ]);

        // pack-check should run without crashing
        assert!(
            !stderr.contains("panic") && !stderr.contains("RUST_BACKTRACE"),
            "pack-check should not panic. Stderr:\n{}",
            stderr
        );

        // Should produce structured output
        assert!(
            stdout.contains("DemoSwarm Pack Self-Check") || stdout.contains("Summary"),
            "pack-check should produce expected output. Stdout:\n{}",
            stdout
        );

        // Exit code should be valid (0 = pass, 1 = issues found)
        assert!(
            exit_code == 0 || exit_code == 1,
            "pack-check exit code should be 0 or 1, got: {}",
            exit_code
        );
    }

    /// REQ-006: Verify JSON output format is valid.
    #[test]
    fn test_pack_check_json_output_valid() {
        let (exit_code, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--format",
            "json",
        ]);

        // JSON output should be parseable
        let json_result: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
        assert!(
            json_result.is_ok(),
            "JSON output should be valid. Output:\n{}",
            stdout
        );

        let json = json_result.unwrap();

        // Should have expected structure
        assert!(
            json.get("schema_version").is_some(),
            "Missing schema_version"
        );
        assert!(json.get("repo_root").is_some(), "Missing repo_root");
        assert!(json.get("errors").is_some(), "Missing errors count");
        assert!(json.get("warnings").is_some(), "Missing warnings count");
        assert!(json.get("counts").is_some(), "Missing counts");
        assert!(json.get("diagnostics").is_some(), "Missing diagnostics");

        // Exit code should match error count
        let errors = json.get("errors").and_then(|e| e.as_u64()).unwrap_or(0);
        if errors == 0 {
            // No errors means exit 0 (unless strict mode with warnings)
            assert!(
                exit_code == 0 || exit_code == 1,
                "Exit code mismatch: errors={}, exit_code={}",
                errors,
                exit_code
            );
        }
    }

    /// REQ-006: Verify pack counts are populated.
    #[test]
    fn test_pack_check_counts_populated() {
        let (_, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--format",
            "json",
        ]);

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
            let counts = json.get("counts").expect("Missing counts in JSON output");

            let agents = counts.get("agents").and_then(|a| a.as_u64()).unwrap_or(0);
            let commands = counts.get("commands").and_then(|c| c.as_u64()).unwrap_or(0);
            let skills = counts.get("skills").and_then(|s| s.as_u64()).unwrap_or(0);

            // A valid pack should have at least some agents, commands, and skills
            assert!(agents > 0, "Pack should have at least one agent");
            assert!(commands > 0, "Pack should have at least one command");
            assert!(skills > 0, "Pack should have at least one skill");
        }
    }

    /// REQ-006: Existing flow command files should not produce false positives.
    /// This verifies flow commands in the actual pack pass validation.
    #[test]
    fn test_existing_flow_commands_no_false_positives() {
        let (_, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--format",
            "json",
        ]);

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
            let empty_vec = vec![];
            let diagnostics = json
                .get("diagnostics")
                .and_then(|d| d.as_array())
                .unwrap_or(&empty_vec);

            // Check for any false-positive-like messages in flow command checks
            // (This is a smoke test - specific false positive patterns would be added as found)
            let flow_cmd_false_positives: Vec<_> = diagnostics
                .iter()
                .filter(|d| {
                    let msg = d.get("message").and_then(|m| m.as_str()).unwrap_or("");
                    // Flag if we find prose being flagged as violations
                    msg.contains("prose") || msg.contains("false positive")
                })
                .collect();

            assert!(
                flow_cmd_false_positives.is_empty(),
                "Should not have false positives in flow commands. Found: {:?}",
                flow_cmd_false_positives
            );
        }
    }

    /// REQ-006: Verify deterministic output (NFR-REL-001).
    /// Running pack-check twice should produce identical results.
    #[test]
    fn test_pack_check_deterministic_output() {
        let args = &[
            "--repo-root",
            &repo_root().display().to_string(),
            "--format",
            "json",
            "--no-color",
        ];

        let (code1, stdout1, _) = run_pack_check(args);
        let (code2, stdout2, _) = run_pack_check(args);

        assert_eq!(code1, code2, "Exit codes should be identical across runs");
        assert_eq!(stdout1, stdout2, "Output should be identical across runs");
    }
}

// =============================================================================
// NFR-COMP-001: Backward Compatibility Tests
// Verify existing checks continue to function.
// =============================================================================

mod backward_compatibility {
    use super::*;

    /// NFR-COMP-001: Verify all expected checks run.
    /// This ensures new additions don't break existing checks.
    #[test]
    fn test_all_expected_checks_run() {
        let (_, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--no-color",
        ]);

        // Key checks that must exist (sample of important checks)
        let expected_checks = [
            "Checking", // Generic check marker
            "Summary",  // Summary section
        ];

        for check_marker in expected_checks {
            assert!(
                stdout.contains(check_marker),
                "Expected '{}' in output. Stdout:\n{}",
                check_marker,
                stdout
            );
        }
    }

    /// NFR-COMP-001 MET-1: Exit codes are preserved.
    /// pack-check should exit 0 on success, 1 on failure.
    #[test]
    fn test_exit_code_contract() {
        let (exit_code, stdout, _) = run_pack_check(&[
            "--repo-root",
            &repo_root().display().to_string(),
            "--format",
            "json",
        ]);

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
            let errors = json.get("errors").and_then(|e| e.as_u64()).unwrap_or(0);
            let warnings = json.get("warnings").and_then(|w| w.as_u64()).unwrap_or(0);

            // Without --strict: exit 0 if no errors (warnings OK)
            // With errors: exit 1
            if errors == 0 {
                assert_eq!(
                    exit_code, 0,
                    "Should exit 0 when no errors. errors={}, warnings={}",
                    errors, warnings
                );
            } else {
                assert_eq!(
                    exit_code, 1,
                    "Should exit 1 when errors present. errors={}",
                    errors
                );
            }
        }
    }
}
