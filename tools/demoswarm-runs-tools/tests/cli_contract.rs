use assert_cmd::{Command, cargo::cargo_bin_cmd};
use predicates::str::contains;
use std::fs;
use std::io::Write;
use tempfile::{NamedTempFile, TempDir};

fn demoswarm() -> Command {
    let mut cmd = cargo_bin_cmd!("demoswarm");
    // Clear DEMOSWARM_STRICT to ensure tests don't inherit it from environment
    cmd.env_remove("DEMOSWARM_STRICT");
    cmd
}

fn demoswarm_with_env(key: &str, value: &str) -> Command {
    let mut cmd = cargo_bin_cmd!("demoswarm");
    cmd.env_remove("DEMOSWARM_STRICT");
    cmd.env(key, value);
    cmd
}

#[test]
fn ms_get_missing_file_returns_null_and_zero_exit() {
    let mut cmd = demoswarm();
    cmd.args([
        "ms",
        "get",
        "--file",
        "./__missing_machine_summary.md",
        "--section",
        "## Machine Summary",
        "--key",
        "status",
        "--null-if-missing",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn count_pattern_honors_null_if_zero_flag() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "no markers present").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "pattern",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--regex",
        "^IMPL_FILE_CHANGED:",
        "--null-if-zero",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn invalid_regex_does_not_break_contract() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "content").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "pattern",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--regex",
        "[",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn receipt_get_missing_file_returns_null_and_logs_missing() {
    let mut cmd = demoswarm();
    cmd.args([
        "receipt",
        "get",
        "--file",
        "./__nonexistent_receipt.json",
        "--key",
        "status",
    ]);

    cmd.assert()
        .success()
        .stdout("null\n")
        .stderr(contains("discovery_method: missing"));
}

#[test]
fn receipt_get_reads_direct_file() {
    let mut tmp = NamedTempFile::with_suffix(".json").expect("temp file");
    writeln!(tmp, r#"{{"status": "VERIFIED", "run_id": "test-001"}}"#).expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "receipt",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--key",
        "status",
    ]);

    cmd.assert()
        .success()
        .stdout("VERIFIED\n")
        .stderr(contains("discovery_method: direct_read"));
}

#[test]
fn receipt_get_missing_key_returns_null() {
    let mut tmp = NamedTempFile::with_suffix(".json").expect("temp file");
    writeln!(tmp, r#"{{"status": "VERIFIED"}}"#).expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "receipt",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--key",
        "nonexistent_key",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn receipt_get_invalid_json_returns_null() {
    let mut tmp = NamedTempFile::with_suffix(".json").expect("temp file");
    writeln!(tmp, "not valid json {{{{").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "receipt",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("null\n");
}

// ============================================================================
// Strict mode tests (--strict flag and DEMOSWARM_STRICT env var)
// ============================================================================
//
// The demoswarm CLI has a "scalar stdout contract" where commands return null
// on soft failures rather than returning errors. Strict mode (exit code 2)
// applies to parse errors and hard errors, not soft failures.
//
// The --strict flag is a global clap flag that can be placed before or after
// subcommands. For parse errors, only the env var is checked (since we can't
// access the flag if parsing failed).

#[test]
fn strict_flag_shown_in_help() {
    let mut cmd = demoswarm();
    cmd.args(["--help"]);

    // Help output goes to stderr in clap for this CLI
    cmd.assert().success().stderr(contains("--strict"));
}

#[test]
fn strict_flag_accepted_before_subcommand() {
    // Verify --strict is recognized as a valid flag
    let mut cmd = demoswarm();
    cmd.args(["--strict", "time", "now"]);

    // Should succeed (time now always works) - verifies flag is accepted
    cmd.assert().success();
}

#[test]
fn strict_flag_accepted_after_subcommand() {
    // Verify --strict works as a global flag after subcommand
    let mut cmd = demoswarm();
    cmd.args(["time", "--strict", "now"]);

    // Should succeed - verifies global flag positioning works
    cmd.assert().success();
}

#[test]
fn strict_env_var_causes_exit_code_2_on_parse_error() {
    // Parse errors (like missing required subcommand) respect DEMOSWARM_STRICT
    let mut cmd = demoswarm_with_env("DEMOSWARM_STRICT", "1");
    cmd.args(["invalid-subcommand"]);

    cmd.assert().code(2).stdout("null\n");
}

#[test]
fn strict_env_var_true_causes_exit_code_2_on_parse_error() {
    let mut cmd = demoswarm_with_env("DEMOSWARM_STRICT", "true");
    cmd.args(["invalid-subcommand"]);

    cmd.assert().code(2).stdout("null\n");
}

#[test]
fn strict_env_var_yes_causes_exit_code_2_on_parse_error() {
    let mut cmd = demoswarm_with_env("DEMOSWARM_STRICT", "yes");
    cmd.args(["invalid-subcommand"]);

    cmd.assert().code(2).stdout("null\n");
}

#[test]
fn without_strict_returns_exit_code_0_on_parse_error() {
    let mut cmd = demoswarm();
    cmd.args(["invalid-subcommand"]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn soft_failure_returns_null_with_exit_0_regardless_of_strict() {
    // Soft failures (like missing file) return null with exit 0, even with --strict
    // This is the "scalar stdout contract" - errors expressed via null stdout
    let mut cmd = demoswarm();
    cmd.args([
        "--strict",
        "receipt",
        "get",
        "--file",
        "./__nonexistent_receipt.json",
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("null\n");
}

// ============================================================================
// Secrets configurable patterns tests
// ============================================================================

#[test]
fn secrets_scan_with_patterns_file_json() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create a patterns file
    let patterns_path = tmp_dir.path().join("patterns.json");
    fs::write(
        &patterns_path,
        r#"{
        "patterns": [
            {"pattern": "custom_secret_[0-9]{6}", "type": "custom-secret"}
        ]
    }"#,
    )
    .expect("write patterns");

    // Create a file with a custom secret
    let secret_file = tmp_dir.path().join("test.txt");
    fs::write(&secret_file, "My custom_secret_123456 is here").expect("write secret file");

    // Output file
    let output_path = tmp_dir.path().join("output.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        secret_file.to_str().expect("path"),
        "--output",
        output_path.to_str().expect("output path"),
        "--patterns-file",
        patterns_path.to_str().expect("patterns path"),
    ]);

    cmd.assert().success().stdout("SECRETS_FOUND\n");

    // Verify the output JSON contains the custom secret type
    let output_content = fs::read_to_string(&output_path).expect("read output");
    assert!(output_content.contains("custom-secret"));
}

#[test]
fn secrets_scan_with_patterns_file_yaml() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create a YAML patterns file
    let patterns_path = tmp_dir.path().join("patterns.yaml");
    fs::write(
        &patterns_path,
        r#"patterns:
  - pattern: "yaml_token_[A-Za-z0-9]{10}"
    type: yaml-custom-token
"#,
    )
    .expect("write patterns");

    // Create a file with a custom secret
    let secret_file = tmp_dir.path().join("test.txt");
    fs::write(&secret_file, "Token: yaml_token_abcd1234EF").expect("write secret file");

    // Output file
    let output_path = tmp_dir.path().join("output.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        secret_file.to_str().expect("path"),
        "--output",
        output_path.to_str().expect("output path"),
        "--patterns-file",
        patterns_path.to_str().expect("patterns path"),
    ]);

    cmd.assert().success().stdout("SECRETS_FOUND\n");

    // Verify the output JSON contains the custom secret type
    let output_content = fs::read_to_string(&output_path).expect("read output");
    assert!(output_content.contains("yaml-custom-token"));
}

#[test]
fn secrets_scan_invalid_patterns_file_regex() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create a patterns file with invalid regex
    let patterns_path = tmp_dir.path().join("patterns.json");
    fs::write(
        &patterns_path,
        r#"{
        "patterns": [
            {"pattern": "[invalid(regex", "type": "bad-pattern"}
        ]
    }"#,
    )
    .expect("write patterns");

    // Create a file to scan
    let secret_file = tmp_dir.path().join("test.txt");
    fs::write(&secret_file, "some content").expect("write file");

    // Output file
    let output_path = tmp_dir.path().join("output.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        secret_file.to_str().expect("path"),
        "--output",
        output_path.to_str().expect("output path"),
        "--patterns-file",
        patterns_path.to_str().expect("patterns path"),
    ]);

    cmd.assert().success().stdout("PATTERN_ERROR\n");

    // Verify the output JSON contains the error
    let output_content = fs::read_to_string(&output_path).expect("read output");
    assert!(output_content.contains("PATTERN_ERROR"));
    assert!(output_content.contains("Invalid regex"));
}

#[test]
fn secrets_scan_missing_patterns_file() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create a file to scan
    let secret_file = tmp_dir.path().join("test.txt");
    fs::write(&secret_file, "some content").expect("write file");

    // Output file
    let output_path = tmp_dir.path().join("output.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        secret_file.to_str().expect("path"),
        "--output",
        output_path.to_str().expect("output path"),
        "--patterns-file",
        "/nonexistent/patterns.json",
    ]);

    cmd.assert().success().stdout("PATH_BOUNDARY_ERROR\n");
}

#[test]
fn secrets_scan_output_in_new_subdirectory() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let secret_file = tmp_dir.path().join("test.txt");
    fs::write(&secret_file, "clean content").expect("write file");

    // Output in a new subdirectory
    let output_subdir = tmp_dir.path().join("new_subdir");
    let output_path = output_subdir.join("output.json");
    
    // Do NOT create output_subdir (simulating new run dir)

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        secret_file.to_str().expect("path"),
        "--output",
        output_path.to_str().expect("output path"),
    ]);

    cmd.assert().success().stdout("CLEAN\n");
    
    // Verify output file exists
    assert!(output_path.exists());
    assert!(output_subdir.exists());
}

#[test]
fn secrets_scan_merges_builtin_and_custom_patterns() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create a patterns file with a custom pattern
    let patterns_path = tmp_dir.path().join("patterns.json");
    fs::write(
        &patterns_path,
        r#"{
        "patterns": [
            {"pattern": "custom_api_[0-9]{8}", "type": "custom-api-key"}
        ]
    }"#,
    )
    .expect("write patterns");

    // Create a file with both a built-in secret and a custom secret
    let secret_file = tmp_dir.path().join("test.txt");
    fs::write(
        &secret_file,
        "GitHub: ghp_abcdefghijklmnopqrstuvwxyz1234567890\nCustom: custom_api_12345678",
    )
    .expect("write file");

    // Output file
    let output_path = tmp_dir.path().join("output.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        secret_file.to_str().expect("path"),
        "--output",
        output_path.to_str().expect("output path"),
        "--patterns-file",
        patterns_path.to_str().expect("patterns path"),
    ]);

    cmd.assert().success().stdout("SECRETS_FOUND\n");

    // Verify the output JSON contains both secret types
    let output_content = fs::read_to_string(&output_path).expect("read output");
    assert!(output_content.contains("github-token"));
    assert!(output_content.contains("custom-api-key"));
}

#[test]
fn secrets_redact_custom_type_with_patterns_file() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create a patterns file
    let patterns_path = tmp_dir.path().join("patterns.json");
    fs::write(
        &patterns_path,
        r#"{
        "patterns": [
            {"pattern": "custom_key_[A-Za-z0-9]{12}", "type": "custom-key"}
        ]
    }"#,
    )
    .expect("write patterns");

    // Create a file with a custom secret
    let secret_file = tmp_dir.path().join("test.txt");
    fs::write(&secret_file, "Secret: custom_key_abcd12345678").expect("write file");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "redact",
        "--file",
        secret_file.to_str().expect("path"),
        "--type",
        "custom-key",
        "--patterns-file",
        patterns_path.to_str().expect("patterns path"),
    ]);

    cmd.assert().success().stdout("ok\n");

    // Verify the file was redacted
    let content = fs::read_to_string(&secret_file).expect("read file");
    assert!(content.contains("[REDACTED:custom-key]"));
    assert!(!content.contains("custom_key_abcd12345678"));
}

#[test]
fn secrets_redact_unknown_type_without_patterns_file() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create a file
    let secret_file = tmp_dir.path().join("test.txt");
    fs::write(&secret_file, "some content").expect("write file");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "redact",
        "--file",
        secret_file.to_str().expect("path"),
        "--type",
        "unknown-type",
    ]);

    cmd.assert()
        .success()
        .stdout("null\n")
        .stderr(contains("Unknown secret type"));
}

#[test]
fn secrets_redact_builtin_type_still_works() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create a file with a GitHub token
    let secret_file = tmp_dir.path().join("test.txt");
    fs::write(
        &secret_file,
        "Token: ghp_abcdefghijklmnopqrstuvwxyz12345678900",
    )
    .expect("write file");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "redact",
        "--file",
        secret_file.to_str().expect("path"),
        "--type",
        "github-token",
    ]);

    cmd.assert().success().stdout("ok\n");

    // Verify the file was redacted
    let content = fs::read_to_string(&secret_file).expect("read file");
    assert!(content.contains("[REDACTED:github-token]"));
    assert!(!content.contains("ghp_"));
}

// =============================================================================
// yaml get - Extract fields from fenced YAML blocks
// =============================================================================

#[test]
fn yaml_get_extracts_field_from_fenced_block() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"# Document

```yaml
status: VERIFIED
run_id: test-001
```

More content here.
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("VERIFIED\n");
}

#[test]
fn yaml_get_handles_quoted_values() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"```yaml
message: "Hello, World!"
single: 'single quoted'
```"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--key",
        "message",
    ]);

    cmd.assert().success().stdout("Hello, World!\n");
}

#[test]
fn yaml_get_missing_file_returns_null() {
    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "get",
        "--file",
        "./__nonexistent_yaml.md",
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn yaml_get_missing_key_returns_null() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"```yaml
status: VERIFIED
```"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--key",
        "nonexistent_key",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn yaml_get_no_yaml_block_returns_null() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "# Just a markdown file\n\nNo YAML block here.").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn yaml_get_empty_file_returns_null() {
    let tmp = NamedTempFile::new().expect("temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("null\n");
}

// =============================================================================
// inv get - Extract inventory marker values
// =============================================================================

#[test]
fn inv_get_extracts_marker_value() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"# Inventory

- DEP_CI_SIGNAL: true
- DEP_LINT: false
- COUNT_TESTS: 42
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "inv",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--marker",
        "DEP_CI_SIGNAL",
    ]);

    cmd.assert().success().stdout("true\n");
}

#[test]
fn inv_get_missing_file_returns_null() {
    let mut cmd = demoswarm();
    cmd.args([
        "inv",
        "get",
        "--file",
        "./__nonexistent_inventory.md",
        "--marker",
        "DEP_CI_SIGNAL",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn inv_get_missing_marker_returns_null() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "- DEP_CI_SIGNAL: true").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "inv",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--marker",
        "NONEXISTENT_MARKER",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn inv_get_handles_special_characters_in_value() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "- MY_SPECIAL_MARKER: value with spaces").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "inv",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--marker",
        "MY_SPECIAL_MARKER",
    ]);

    cmd.assert().success().stdout("value with spaces\n");
}

// =============================================================================
// receipts count - Count existing receipts in a run directory
// =============================================================================

#[test]
fn receipts_count_returns_zero_for_empty_run_dir() {
    let tmp_dir = TempDir::new().expect("temp dir");

    let mut cmd = demoswarm();
    cmd.args([
        "receipts",
        "count",
        "--run-dir",
        tmp_dir.path().to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("0\n");
}

#[test]
fn receipts_count_counts_existing_receipts() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create signal and plan subdirectories with receipts
    let signal_dir = tmp_dir.path().join("signal");
    let plan_dir = tmp_dir.path().join("plan");
    fs::create_dir_all(&signal_dir).expect("create signal dir");
    fs::create_dir_all(&plan_dir).expect("create plan dir");

    fs::write(
        signal_dir.join("signal_receipt.json"),
        r#"{"status":"VERIFIED"}"#,
    )
    .expect("write signal receipt");
    fs::write(
        plan_dir.join("plan_receipt.json"),
        r#"{"status":"VERIFIED"}"#,
    )
    .expect("write plan receipt");

    let mut cmd = demoswarm();
    cmd.args([
        "receipts",
        "count",
        "--run-dir",
        tmp_dir.path().to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("2\n");
}

#[test]
fn receipts_count_missing_dir_returns_null() {
    let mut cmd = demoswarm();
    cmd.args(["receipts", "count", "--run-dir", "./__nonexistent_run_dir"]);

    cmd.assert().success().stdout("null\n");
}

// =============================================================================
// openapi count-paths - Count API paths in OpenAPI YAML
// =============================================================================

#[test]
fn openapi_count_paths_counts_api_paths() {
    let mut tmp = NamedTempFile::with_suffix(".yaml").expect("temp file");
    writeln!(
        tmp,
        r#"openapi: 3.0.0
info:
  title: Test API
paths:
  /users:
    get:
      summary: List users
  /users/{{id}}:
    get:
      summary: Get user
  /items:
    post:
      summary: Create item
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "openapi",
        "count-paths",
        "--file",
        tmp.path().to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("3\n");
}

#[test]
fn openapi_count_paths_missing_file_returns_null() {
    let mut cmd = demoswarm();
    cmd.args([
        "openapi",
        "count-paths",
        "--file",
        "./__nonexistent_openapi.yaml",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn openapi_count_paths_no_paths_section_returns_null() {
    let mut tmp = NamedTempFile::with_suffix(".yaml").expect("temp file");
    writeln!(
        tmp,
        r#"openapi: 3.0.0
info:
  title: Empty API
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "openapi",
        "count-paths",
        "--file",
        tmp.path().to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn openapi_count_paths_empty_paths_returns_zero() {
    let mut tmp = NamedTempFile::with_suffix(".yaml").expect("temp file");
    writeln!(
        tmp,
        r#"openapi: 3.0.0
info:
  title: Empty Paths API
paths:
components:
  schemas:
    User:
      type: object
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "openapi",
        "count-paths",
        "--file",
        tmp.path().to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("0\n");
}

// =============================================================================
// openq next-id - Generate next open question ID
// =============================================================================

#[test]
fn openq_next_id_returns_001_for_missing_file() {
    let mut cmd = demoswarm();
    cmd.args([
        "openq",
        "next-id",
        "--file",
        "./__nonexistent_openq.md",
        "--prefix",
        "SIGNAL",
    ]);

    cmd.assert().success().stdout("OQ-SIGNAL-001\n");
}

#[test]
fn openq_next_id_increments_existing_ids() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"# Open Questions

- QID: OQ-SIGNAL-001
  - Q: First question [OPEN]

- QID: OQ-SIGNAL-002
  - Q: Second question [OPEN]
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "openq",
        "next-id",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--prefix",
        "SIGNAL",
    ]);

    cmd.assert().success().stdout("OQ-SIGNAL-003\n");
}

#[test]
fn openq_next_id_different_prefix_starts_at_001() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"# Open Questions

- QID: OQ-SIGNAL-005
  - Q: A signal question
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "openq",
        "next-id",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--prefix",
        "BUILD",
    ]);

    cmd.assert().success().stdout("OQ-BUILD-001\n");
}

// =============================================================================
// openq append - Append an open question entry
// =============================================================================

#[test]
fn openq_append_creates_file_if_missing() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let file_path = tmp_dir.path().join("new_openq.md");

    let mut cmd = demoswarm();
    cmd.args([
        "openq",
        "append",
        "--file",
        file_path.to_str().expect("path utf8"),
        "--prefix",
        "TEST",
        "--question",
        "What is the expected behavior?",
        "--default",
        "Return empty array",
        "--impact",
        "May affect error handling",
    ]);

    cmd.assert().success().stdout("OQ-TEST-001\n");

    // Verify file was created with content
    let content = fs::read_to_string(&file_path).expect("read file");
    assert!(content.contains("OQ-TEST-001"));
    assert!(content.contains("What is the expected behavior?"));
}

#[test]
fn openq_append_increments_id_in_existing_file() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"# Open Questions

- QID: OQ-BUILD-001
  - Q: First question [OPEN]
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let file_path = tmp.path().to_str().expect("path utf8").to_string();

    let mut cmd = demoswarm();
    cmd.args([
        "openq",
        "append",
        "--file",
        &file_path,
        "--prefix",
        "BUILD",
        "--question",
        "Second question?",
        "--default",
        "Default answer",
        "--impact",
        "Low impact",
    ]);

    cmd.assert().success().stdout("OQ-BUILD-002\n");
}

// =============================================================================
// secrets scan - Scan for secrets (locations only) - basic tests
// =============================================================================

#[test]
fn secrets_scan_clean_file_reports_clean() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let scan_file = tmp_dir.path().join("clean.txt");
    let output_file = tmp_dir.path().join("findings.json");

    fs::write(&scan_file, "This is clean content with no secrets.").expect("write scan file");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        scan_file.to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("CLEAN\n");

    // Verify JSON output (note: output is pretty-printed with spaces)
    let json_content = fs::read_to_string(&output_file).expect("read output");
    assert!(json_content.contains(r#""status": "CLEAN""#));
    assert!(json_content.contains(r#""findings": []"#));
}

#[test]
fn secrets_scan_detects_github_token() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let scan_file = tmp_dir.path().join("secret.txt");
    let output_file = tmp_dir.path().join("findings.json");

    // Use a fake but pattern-matching GitHub token
    fs::write(
        &scan_file,
        "token = ghp_1234567890abcdef1234567890abcdef123456",
    )
    .expect("write");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        scan_file.to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("SECRETS_FOUND\n");

    // Verify findings contain the type but NOT the actual secret
    let json_content = fs::read_to_string(&output_file).expect("read output");
    assert!(json_content.contains("github-token"));
    // Secret content should not be in output
    assert!(!json_content.contains("ghp_1234567890"));
}

#[test]
fn secrets_scan_detects_aws_key() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let scan_file = tmp_dir.path().join("aws.txt");
    let output_file = tmp_dir.path().join("findings.json");

    // Fake AWS access key pattern
    fs::write(&scan_file, "AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE").expect("write");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        scan_file.to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("SECRETS_FOUND\n");

    let json_content = fs::read_to_string(&output_file).expect("read output");
    assert!(json_content.contains("aws-access-key"));
}

#[test]
fn secrets_scan_missing_path_reports_missing() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        "./__nonexistent_path",
        "--output",
        output_file.to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("SCAN_PATH_MISSING\n");
}

#[test]
fn secrets_scan_directory_recursive() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let sub_dir = tmp_dir.path().join("subdir");
    fs::create_dir_all(&sub_dir).expect("create subdir");

    let clean_file = tmp_dir.path().join("clean.txt");
    let secret_file = sub_dir.join("secret.txt");
    let output_file = tmp_dir.path().join("findings.json");

    fs::write(&clean_file, "Clean content").expect("write clean");
    fs::write(
        &secret_file,
        "key = ghp_abcdefghijklmnopqrstuvwxyz0123456789",
    )
    .expect("write secret");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        tmp_dir.path().to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("SECRETS_FOUND\n");
}

// =============================================================================
// secrets redact - Redact secrets in-place - basic tests
// =============================================================================

#[test]
fn secrets_redact_github_token() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "token = ghp_1234567890abcdef1234567890abcdef123456").expect("write");
    tmp.flush().expect("flush");

    let file_path = tmp.path().to_str().expect("path utf8").to_string();

    let mut cmd = demoswarm();
    cmd.current_dir(tmp.path().parent().unwrap());
    cmd.args([
        "secrets",
        "redact",
        "--file",
        &file_path,
        "--type",
        "github-token",
    ]);

    cmd.assert().success().stdout("ok\n");

    // Verify the token was redacted
    let content = fs::read_to_string(&file_path).expect("read");
    assert!(content.contains("[REDACTED:github-token]"));
    assert!(!content.contains("ghp_1234567890"));
}

#[test]
fn secrets_redact_missing_file_returns_not_found() {
    let mut cmd = demoswarm();
    cmd.args([
        "secrets",
        "redact",
        "--file",
        "./__nonexistent_file",
        "--type",
        "github-token",
    ]);

    cmd.assert().success().stdout("FILE_NOT_FOUND\n");
}

#[test]
fn secrets_redact_private_key() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"Config file
-----BEGIN RSA PRIVATE KEY-----
MIIEowIBAAKCAQEA0Z3VS5JJcds3xfn/ygWyf8pWvdA1
base64contenthere
-----END RSA PRIVATE KEY-----
More content"#
    )
    .expect("write");
    tmp.flush().expect("flush");

    let file_path = tmp.path().to_str().expect("path utf8").to_string();

    let mut cmd = demoswarm();
    cmd.current_dir(tmp.path().parent().unwrap());
    cmd.args([
        "secrets",
        "redact",
        "--file",
        &file_path,
        "--type",
        "private-key",
    ]);

    cmd.assert().success().stdout("ok\n");

    let content = fs::read_to_string(&file_path).expect("read");
    assert!(content.contains("[REDACTED:private-key]"));
    assert!(!content.contains("BEGIN RSA PRIVATE KEY"));
}

// =============================================================================
// index upsert-status - Update run status in index.json
// =============================================================================

#[test]
fn index_upsert_status_missing_index_returns_skipped() {
    let mut cmd = demoswarm();
    cmd.args([
        "index",
        "upsert-status",
        "--index",
        "./__nonexistent_index.json",
        "--run-id",
        "test-run",
        "--status",
        "VERIFIED",
        "--last-flow",
        "build",
    ]);

    cmd.assert().success().stdout("SKIPPED_MISSING_INDEX\n");
}

#[test]
fn index_upsert_status_updates_existing_entry() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let index_path = tmp_dir.path().join("index.json");

    // Create index with existing entry
    fs::write(
        &index_path,
        r#"{"runs":[{"run_id":"test-run","status":"UNVERIFIED","last_flow":"signal"}]}"#,
    )
    .expect("write index");

    let mut cmd = demoswarm();
    cmd.args([
        "index",
        "upsert-status",
        "--index",
        index_path.to_str().expect("path utf8"),
        "--run-id",
        "test-run",
        "--status",
        "VERIFIED",
        "--last-flow",
        "build",
    ]);

    cmd.assert().success().stdout("ok\n");

    // Verify the update (note: output is pretty-printed JSON with spaces)
    let content = fs::read_to_string(&index_path).expect("read");
    assert!(content.contains(r#""status": "VERIFIED""#));
    assert!(content.contains(r#""last_flow": "build""#));
}

#[test]
fn index_upsert_status_adds_new_entry() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let index_path = tmp_dir.path().join("index.json");

    // Create empty index
    fs::write(&index_path, r#"{"runs":[]}"#).expect("write index");

    let mut cmd = demoswarm();
    cmd.args([
        "index",
        "upsert-status",
        "--index",
        index_path.to_str().expect("path utf8"),
        "--run-id",
        "new-run",
        "--status",
        "UNVERIFIED",
        "--last-flow",
        "signal",
    ]);

    cmd.assert().success().stdout("ok\n");

    // Verify the new entry (note: output is pretty-printed JSON with spaces)
    let content = fs::read_to_string(&index_path).expect("read");
    assert!(content.contains(r#""run_id": "new-run""#));
}

#[test]
fn index_upsert_status_invalid_json_returns_null() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let index_path = tmp_dir.path().join("index.json");

    fs::write(&index_path, "not valid json {{{{").expect("write");

    let mut cmd = demoswarm();
    cmd.args([
        "index",
        "upsert-status",
        "--index",
        index_path.to_str().expect("path utf8"),
        "--run-id",
        "test-run",
        "--status",
        "VERIFIED",
        "--last-flow",
        "build",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn index_upsert_status_missing_runs_array_returns_null() {
    let tmp_dir = TempDir::new().expect("temp dir");
    let index_path = tmp_dir.path().join("index.json");

    fs::write(&index_path, r#"{"version": "1.0"}"#).expect("write");

    let mut cmd = demoswarm();
    cmd.args([
        "index",
        "upsert-status",
        "--index",
        index_path.to_str().expect("path utf8"),
        "--run-id",
        "test-run",
        "--status",
        "VERIFIED",
        "--last-flow",
        "build",
    ]);

    cmd.assert().success().stdout("null\n");
}

// =============================================================================
// time now - Get current UTC timestamp
// =============================================================================

#[test]
fn time_now_returns_iso8601_format() {
    let mut cmd = demoswarm();
    cmd.args(["time", "now"]);

    let output = cmd.assert().success().get_output().stdout.clone();
    let output_str = String::from_utf8(output).expect("utf8");
    let trimmed = output_str.trim();

    // Verify ISO8601 format: YYYY-MM-DDTHH:MM:SSZ
    assert!(
        trimmed.len() == 20,
        "Expected 20 chars for ISO8601 format, got: {}",
        trimmed.len()
    );
    assert!(trimmed.ends_with('Z'), "Expected Z suffix for UTC");
    assert!(trimmed.contains('T'), "Expected T separator");
}

// =============================================================================
// Edge cases and error handling
// =============================================================================

#[test]
fn unicode_content_handled_correctly() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"```yaml
message: "Hello, 世界!"
status: VERIFIED
```"#
    )
    .expect("write");
    tmp.flush().expect("flush");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("VERIFIED\n");
}

#[test]
fn count_bdd_scenarios_in_directory() {
    let tmp_dir = TempDir::new().expect("temp dir");

    // Create feature files
    fs::write(
        tmp_dir.path().join("auth.feature"),
        r#"Feature: Authentication

  Scenario: User logs in successfully
    Given a registered user
    When they enter valid credentials
    Then they are logged in

  Scenario: User login fails with wrong password
    Given a registered user
    When they enter wrong password
    Then they see an error
"#,
    )
    .expect("write auth.feature");

    fs::write(
        tmp_dir.path().join("items.feature"),
        r#"Feature: Items

  Scenario Outline: Create item
    Given a logged in user
    When they create an item with name "<name>"
    Then the item exists

    Examples:
      | name  |
      | Item1 |
      | Item2 |
"#,
    )
    .expect("write items.feature");

    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "bdd",
        "--dir",
        tmp_dir.path().to_str().expect("path utf8"),
    ]);

    // 2 scenarios + 1 scenario outline = 3
    cmd.assert().success().stdout("3\n");
}

#[test]
fn count_bdd_missing_dir_returns_null() {
    let mut cmd = demoswarm();
    cmd.args(["count", "bdd", "--dir", "./__nonexistent_features_dir"]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn count_bdd_empty_dir_returns_zero() {
    let tmp_dir = TempDir::new().expect("temp dir");

    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "bdd",
        "--dir",
        tmp_dir.path().to_str().expect("path utf8"),
    ]);

    cmd.assert().success().stdout("0\n");
}

// =============================================================================
// count pattern - Additional tests
// =============================================================================

#[test]
fn count_pattern_counts_matching_lines() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"IMPL_FILE_CHANGED: src/main.rs
IMPL_FILE_CHANGED: src/lib.rs
Some other line
IMPL_FILE_CHANGED: tests/test.rs
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "pattern",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--regex",
        "^IMPL_FILE_CHANGED:",
    ]);

    cmd.assert().success().stdout("3\n");
}

#[test]
fn count_pattern_missing_file_returns_null() {
    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "pattern",
        "--file",
        "./__nonexistent_file.md",
        "--regex",
        "^TEST",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn count_pattern_uses_fallback_regex_when_primary_is_zero() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"REQ-001: First requirement
REQ-002: Second requirement
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "pattern",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--regex",
        "^IMPL_FILE_CHANGED:", // Won't match
        "--fallback-regex",
        "^REQ-", // Will match
    ]);

    cmd.assert().success().stdout("2\n");
}

#[test]
fn count_pattern_returns_zero_when_no_matches() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "some content without markers").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "count",
        "pattern",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--regex",
        "^MARKER:",
    ]);

    cmd.assert().success().stdout("0\n");
}

// =============================================================================
// line get - Extract value from a line with a known prefix
// =============================================================================

#[test]
fn line_get_extracts_value() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"# Report

Mutation Score: 85.2%
Coverage: 92%
Other line: value
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "line",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--prefix",
        "Mutation Score:",
    ]);

    cmd.assert().success().stdout("85.2%\n");
}

#[test]
fn line_get_missing_file_returns_null() {
    let mut cmd = demoswarm();
    cmd.args([
        "line",
        "get",
        "--file",
        "./__nonexistent_file.md",
        "--prefix",
        "Status:",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn line_get_missing_prefix_returns_null() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "Status: VERIFIED\nCount: 42").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "line",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--prefix",
        "Nonexistent:",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn line_get_empty_value_returns_null() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "Status:\nCount: 42").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "line",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--prefix",
        "Status:",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn line_get_extracts_first_matching_line() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"Status: FIRST
Status: SECOND
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "line",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--prefix",
        "Status:",
    ]);

    cmd.assert().success().stdout("FIRST\n");
}

#[test]
fn line_get_preserves_entire_value_with_spaces() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "Description: This is a long description with spaces").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "line",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--prefix",
        "Description:",
    ]);

    cmd.assert()
        .success()
        .stdout("This is a long description with spaces\n");
}

// =============================================================================
// ms get - Machine Summary extraction - Additional tests
// =============================================================================

#[test]
fn ms_get_extracts_field_from_section() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"# Document

## Machine Summary

status: VERIFIED
run_id: test-001
test_count: 42

## Another Section

Other content here.
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "ms",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--section",
        "## Machine Summary",
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("VERIFIED\n");
}

#[test]
fn ms_get_missing_section_returns_null() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"# Document

## Different Section

status: VERIFIED
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "ms",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--section",
        "## Machine Summary",
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn ms_get_missing_key_returns_null() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"## Machine Summary

status: VERIFIED
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "ms",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--section",
        "## Machine Summary",
        "--key",
        "nonexistent_key",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn ms_get_returns_first_word_only() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"## Machine Summary

status: VERIFIED with additional info
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "ms",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--section",
        "## Machine Summary",
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("VERIFIED\n");
}

#[test]
fn ms_get_template_leak_guard_returns_null_for_pipe() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"## Machine Summary

status: | template value
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "ms",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--section",
        "## Machine Summary",
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn ms_get_template_leak_guard_returns_null_for_angle_bracket() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"## Machine Summary

status: <placeholder>
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "ms",
        "get",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--section",
        "## Machine Summary",
        "--key",
        "status",
    ]);

    cmd.assert().success().stdout("null\n");
}

// =============================================================================
// yaml count-items - Count items matching pattern in YAML block
// =============================================================================

#[test]
fn yaml_count_items_counts_matching_lines() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"# Document

```yaml
items:
  - REQ-001: First requirement
  - REQ-002: Second requirement
  - NFR-001: Non-functional requirement
  - REQ-003: Third requirement
```
"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "count-items",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--item-regex",
        "REQ-[0-9]+",
    ]);

    cmd.assert().success().stdout("3\n");
}

#[test]
fn yaml_count_items_missing_file_returns_null() {
    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "count-items",
        "--file",
        "./__nonexistent_file.md",
        "--item-regex",
        "REQ-",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn yaml_count_items_no_yaml_block_returns_null() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(tmp, "# Document\n\nNo YAML block here.").expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "count-items",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--item-regex",
        "REQ-",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn yaml_count_items_invalid_regex_returns_null() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"```yaml
items:
  - item1
```"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "count-items",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--item-regex",
        "[invalid(regex",
    ]);

    cmd.assert().success().stdout("null\n");
}

#[test]
fn yaml_count_items_returns_zero_when_no_matches() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"```yaml
items:
  - item1
  - item2
```"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "count-items",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--item-regex",
        "NONEXISTENT",
    ]);

    cmd.assert().success().stdout("0\n");
}

#[test]
fn yaml_count_items_handles_posix_character_class() {
    let mut tmp = NamedTempFile::new().expect("temp file");
    writeln!(
        tmp,
        r#"```yaml
items:
  - first item
  - second item
```"#
    )
    .expect("write");
    tmp.flush().expect("flush temp file");

    let mut cmd = demoswarm();
    cmd.args([
        "yaml",
        "count-items",
        "--file",
        tmp.path().to_str().expect("path utf8"),
        "--item-regex",
        "^\\s*-\\s+",
    ]);

    // Both "  - first item" and "  - second item" lines match
    cmd.assert().success().stdout("2\n");
}
