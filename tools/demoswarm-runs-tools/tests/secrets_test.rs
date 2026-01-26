use assert_cmd::{Command, cargo::cargo_bin_cmd};
use std::fs;
use std::io::Write;
use tempfile::Builder;

fn demoswarm() -> Command {
    cargo_bin_cmd!("demoswarm")
}

#[test]
fn secrets_scan_finds_token_in_nested_dir() {
    let tmp_dir = Builder::new()
        .prefix("secrets_test")
        .tempdir()
        .expect("temp dir");
    let nested = tmp_dir.path().join("nested");
    fs::create_dir(&nested).expect("mkdir");

    let file_path = nested.join("secret.txt");
    let mut file = fs::File::create(&file_path).expect("create file");
    writeln!(file, "ghp_123456789012345678901234567890123456").expect("write secret");

    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path()); // Set CWD to temp dir
    cmd.args([
        "secrets",
        "scan",
        "--path",
        tmp_dir.path().to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("out utf8"),
    ]);

    cmd.assert().success();

    let content = fs::read_to_string(&output_file).expect("read output");
    assert!(content.contains("SECRETS_FOUND"));
    assert!(content.contains("github-token"));
}

#[test]
fn secrets_scan_handles_relative_paths_via_canonicalize() {
    let tmp_dir = Builder::new()
        .prefix("secrets_test_dot")
        .tempdir()
        .expect("temp dir");
    let file_path = tmp_dir.path().join("secret.txt");
    let mut file = fs::File::create(&file_path).expect("create file");
    writeln!(file, "ghp_123456789012345678901234567890123456").expect("write secret");

    let output_file = tmp_dir.path().join("findings.json");

    // Construct a path like "C:\tmp\foo\.\secret.txt"
    let funny_path = tmp_dir.path().join(".").join("secret.txt");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        funny_path.to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("out utf8"),
    ]);

    cmd.assert().success();
    let content = fs::read_to_string(&output_file).expect("read output");
    assert!(content.contains("SECRETS_FOUND"));
}

#[test]
fn secrets_scan_with_json_patterns_file() {
    let tmp_dir = Builder::new()
        .prefix("secrets_patterns_json")
        .tempdir()
        .expect("temp dir");

    // Create a file with a custom secret pattern (Slack bot token format)
    let file_path = tmp_dir.path().join("config.txt");
    let mut file = fs::File::create(&file_path).expect("create file");
    writeln!(
        file,
        "slack_token=xoxb-1234567890-1234567890-{}",
        "abcdefghijklmnopqrstuvwx"
    )
    .expect("write secret");

    // Create a JSON patterns file
    let patterns_file = tmp_dir.path().join("patterns.json");
    let patterns_content = r#"{
        "patterns": [
            {
                "pattern": "xoxb-[0-9]{10}-[0-9]{10}-[a-z]{24}",
                "type": "slack-bot-token"
            }
        ]
    }"#;
    fs::write(&patterns_file, patterns_content).expect("write patterns file");

    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path()); // Set CWD to temp dir
    cmd.args([
        "secrets",
        "scan",
        "--path",
        tmp_dir.path().to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("out utf8"),
        "--patterns-file",
        patterns_file.to_str().expect("patterns utf8"),
    ]);

    cmd.assert().success();

    let content = fs::read_to_string(&output_file).expect("read output");
    assert!(
        content.contains("SECRETS_FOUND"),
        "Expected SECRETS_FOUND in output: {}",
        content
    );
    assert!(
        content.contains("slack-bot-token"),
        "Expected slack-bot-token type in output: {}",
        content
    );
}

#[test]
fn secrets_scan_with_yaml_patterns_file() {
    let tmp_dir = Builder::new()
        .prefix("secrets_patterns_yaml")
        .tempdir()
        .expect("temp dir");

    // Create a file with a custom secret pattern (22 chars after prefix)
    let file_path = tmp_dir.path().join("config.txt");
    let mut file = fs::File::create(&file_path).expect("create file");
    writeln!(file, "square_token=sq0atp-ABCDEFGHIJ1234567890{}", "AB").expect("write secret");

    // Create a YAML patterns file
    let patterns_file = tmp_dir.path().join("patterns.yaml");
    let patterns_content = r#"patterns:
  - pattern: "sq0atp-[A-Za-z0-9]{22}"
    type: square-access-token
"#;
    fs::write(&patterns_file, patterns_content).expect("write patterns file");

    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path()); // Set CWD to temp dir
    cmd.args([
        "secrets",
        "scan",
        "--path",
        tmp_dir.path().to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("out utf8"),
        "--patterns-file",
        patterns_file.to_str().expect("patterns utf8"),
    ]);

    cmd.assert().success();

    let content = fs::read_to_string(&output_file).expect("read output");
    assert!(
        content.contains("SECRETS_FOUND"),
        "Expected SECRETS_FOUND in output: {}",
        content
    );
    assert!(
        content.contains("square-access-token"),
        "Expected square-access-token type in output: {}",
        content
    );
}

#[test]
fn secrets_scan_invalid_regex_returns_pattern_error() {
    let tmp_dir = Builder::new()
        .prefix("secrets_invalid_regex")
        .tempdir()
        .expect("temp dir");

    // Create a file (content doesn't matter for this test)
    let file_path = tmp_dir.path().join("config.txt");
    fs::write(&file_path, "some content").expect("write file");

    // Create a patterns file with invalid regex
    let patterns_file = tmp_dir.path().join("patterns.json");
    let patterns_content = r#"{
        "patterns": [
            {
                "pattern": "[invalid(regex",
                "type": "bad-pattern"
            }
        ]
    }"#;
    fs::write(&patterns_file, patterns_content).expect("write patterns file");

    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path()); // Set CWD to temp dir
    cmd.args([
        "secrets",
        "scan",
        "--path",
        tmp_dir.path().to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("out utf8"),
        "--patterns-file",
        patterns_file.to_str().expect("patterns utf8"),
    ]);

    cmd.assert().success(); // Command succeeds but reports PATTERN_ERROR in output

    let content = fs::read_to_string(&output_file).expect("read output");
    assert!(
        content.contains("PATTERN_ERROR"),
        "Expected PATTERN_ERROR in output: {}",
        content
    );
    assert!(
        content.contains("error"),
        "Expected error field in output: {}",
        content
    );
}

#[test]
fn secrets_scan_merges_builtin_and_custom_patterns() {
    let tmp_dir = Builder::new()
        .prefix("secrets_merge_patterns")
        .tempdir()
        .expect("temp dir");

    // Create a file with both a built-in secret (GitHub token) and a custom secret
    let file_path = tmp_dir.path().join("config.txt");
    let mut file = fs::File::create(&file_path).expect("create file");
    writeln!(
        file,
        "github_token=ghp_123456789012345678901234567890123456"
    )
    .expect("write github secret");
    writeln!(file, "custom_key=CUSTOM_KEY_ABC123DEF456GHI789XYZ").expect("write custom secret");

    // Create a patterns file with a custom pattern (21 chars after CUSTOM_KEY_)
    let patterns_file = tmp_dir.path().join("patterns.json");
    let patterns_content = r#"{
        "patterns": [
            {
                "pattern": "CUSTOM_KEY_[A-Z0-9]{21}",
                "type": "custom-api-key"
            }
        ]
    }"#;
    fs::write(&patterns_file, patterns_content).expect("write patterns file");

    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path()); // Set CWD to temp dir
    cmd.args([
        "secrets",
        "scan",
        "--path",
        tmp_dir.path().to_str().expect("path utf8"),
        "--output",
        output_file.to_str().expect("out utf8"),
        "--patterns-file",
        patterns_file.to_str().expect("patterns utf8"),
    ]);

    cmd.assert().success();

    let content = fs::read_to_string(&output_file).expect("read output");
    assert!(
        content.contains("SECRETS_FOUND"),
        "Expected SECRETS_FOUND in output: {}",
        content
    );
    // Both built-in and custom patterns should be detected
    assert!(
        content.contains("github-token"),
        "Expected github-token in output: {}",
        content
    );
    assert!(
        content.contains("custom-api-key"),
        "Expected custom-api-key in output: {}",
        content
    );
}

#[test]
fn secrets_redact_with_custom_pattern() {
    let tmp_dir = Builder::new()
        .prefix("secrets_redact_custom")
        .tempdir()
        .expect("temp dir");

    // Create a file with a custom secret (21 chars after CUSTOM_KEY_)
    let file_path = tmp_dir.path().join("config.txt");
    let secret_content = "custom_key=CUSTOM_KEY_ABC123DEF456GHI789XYZ\n";
    fs::write(&file_path, secret_content).expect("write file");

    // Create a patterns file with the custom pattern
    let patterns_file = tmp_dir.path().join("patterns.json");
    let patterns_content = r#"{
        "patterns": [
            {
                "pattern": "CUSTOM_KEY_[A-Z0-9]{21}",
                "type": "custom-api-key"
            }
        ]
    }"#;
    fs::write(&patterns_file, patterns_content).expect("write patterns file");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path()); // Set CWD to temp dir so absolute paths are valid
    cmd.args([
        "secrets",
        "redact",
        "--file",
        file_path.to_str().expect("path utf8"),
        "--type",
        "custom-api-key",
        "--patterns-file",
        patterns_file.to_str().expect("patterns utf8"),
    ]);

    cmd.assert().success();

    let redacted_content = fs::read_to_string(&file_path).expect("read redacted file");
    assert!(
        redacted_content.contains("[REDACTED:custom-api-key]"),
        "Expected redacted content, got: {}",
        redacted_content
    );
    assert!(
        !redacted_content.contains("CUSTOM_KEY_ABC123DEF456GHI789XYZ"),
        "Secret should be redacted, got: {}",
        redacted_content
    );
}

// ==================== Path Traversal Security Tests ====================
//
// These tests verify that relative path traversal attacks (../../../etc/passwd)
// are blocked, while absolute paths are allowed (explicit user intent).

#[test]
fn secrets_scan_rejects_relative_path_traversal() {
    // Test that relative paths with ../ that escape the CWD are rejected.
    // This is the primary attack vector - crafted inputs like "../../../etc/passwd"
    let tmp_dir = Builder::new()
        .prefix("secrets_traversal_test")
        .tempdir()
        .expect("temp dir");

    let output_file = tmp_dir.path().join("findings.json");

    // Use a RELATIVE path that tries to escape (this is the attack pattern)
    // We need to be in a directory where ../../.. would escape
    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path()); // Set CWD to temp dir (simulates repo root)
    cmd.args([
        "secrets",
        "scan",
        "--path",
        "../../..", // Relative traversal - the attack vector
        "--output",
        "findings.json", // Output within CWD (valid)
    ]);

    // The command should succeed but output PATH_BOUNDARY_ERROR
    cmd.assert().success();

    let content = fs::read_to_string(&output_file).expect("read output");
    assert!(
        content.contains("PATH_BOUNDARY_ERROR"),
        "Expected PATH_BOUNDARY_ERROR for relative path traversal, got: {}",
        content
    );
}

#[test]
fn secrets_scan_allows_absolute_paths() {
    // Absolute paths are allowed IF they are within the repository (CWD).
    // Absolute paths outside the repo are now blocked for security consistency.
    let tmp_dir = Builder::new()
        .prefix("secrets_absolute_test")
        .tempdir()
        .expect("temp dir");

    // Create a subdirectory for scanning
    let scan_dir = tmp_dir.path().join("scan_target");
    fs::create_dir(&scan_dir).expect("mkdir");

    // Create a file with a secret
    let secret_file = scan_dir.join("secret.txt");
    fs::write(&secret_file, "ghp_123456789012345678901234567890123456").expect("write file");

    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        scan_dir.to_str().expect("path utf8"), // Absolute path inside CWD - allowed
        "--output",
        output_file.to_str().expect("out utf8"), // Absolute output inside CWD - allowed
    ]);

    cmd.assert().success();

    let content = fs::read_to_string(&output_file).expect("read output");
    assert!(
        content.contains("SECRETS_FOUND"),
        "Expected SECRETS_FOUND with valid absolute path, got: {}",
        content
    );
    assert!(
        content.contains("github-token"),
        "Expected github-token in output: {}",
        content
    );
}

#[test]
fn secrets_scan_blocks_absolute_paths_outside_repo() {
    // Test that absolute paths pointing outside the repo are blocked
    let tmp_dir = Builder::new()
        .prefix("secrets_absolute_outside_test")
        .tempdir()
        .expect("temp dir");

    // Create a separate directory outside CWD
    let outside_dir = Builder::new()
        .prefix("outside_target")
        .tempdir()
        .expect("outside dir");
    
    let outside_file = outside_dir.path().join("secret.txt");
    fs::write(&outside_file, "ghp_123456789012345678901234567890123456").expect("write file");

    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        outside_file.to_str().expect("path utf8"), // Absolute path outside CWD - blocked
        "--output",
        output_file.to_str().expect("out utf8"),
    ]);

    let output = cmd.assert().success();
    let stdout = String::from_utf8_lossy(&output.get_output().stdout);

    assert!(
        stdout.contains("PATH_BOUNDARY_ERROR"),
        "Expected PATH_BOUNDARY_ERROR for absolute path outside repo, got: {}",
        stdout
    );
}

#[test]
fn secrets_scan_allows_valid_relative_path_within_repo() {
    // Test that relative paths that stay within CWD are allowed
    let tmp_dir = Builder::new()
        .prefix("secrets_valid_path")
        .tempdir()
        .expect("temp dir");

    // Create a nested directory with a file
    let nested = tmp_dir.path().join("subdir").join("nested");
    fs::create_dir_all(&nested).expect("mkdir -p");

    let file_path = nested.join("config.txt");
    fs::write(&file_path, "some content without secrets").expect("write file");

    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path()); // Set CWD to temp dir
    cmd.args([
        "secrets",
        "scan",
        "--path",
        "subdir/nested", // Relative path within repo - allowed
        "--output",
        "findings.json", // Relative output within repo - allowed
    ]);

    cmd.assert().success();

    let content = fs::read_to_string(&output_file).expect("read output");
    assert!(
        content.contains("CLEAN"),
        "Expected CLEAN for valid relative path, got: {}",
        content
    );
}

#[test]
fn secrets_redact_rejects_relative_traversal() {
    // Test that redact also blocks relative path traversal
    let tmp_dir = Builder::new()
        .prefix("secrets_redact_traversal")
        .tempdir()
        .expect("temp dir");

    // Create a parent directory to traverse to
    let parent = tmp_dir.path().parent().expect("has parent");

    // Create a secret file in the parent (simulating outside repo)
    let external_file = parent.join("traversal_target.txt");
    fs::write(&external_file, "ghp_123456789012345678901234567890123456").expect("write file");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "redact",
        "--file",
        "../traversal_target.txt", // Relative traversal - blocked
        "--type",
        "github-token",
    ]);

    let output = cmd.assert().success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("PATH_BOUNDARY_ERROR"),
        "Expected PATH_BOUNDARY_ERROR for relative traversal, got: {}",
        stdout
    );

    // Verify the file was NOT modified (security check)
    let content = fs::read_to_string(&external_file).expect("read file");
    assert!(
        content.contains("ghp_"),
        "File should NOT be modified via traversal, got: {}",
        content
    );

    // Cleanup
    let _ = fs::remove_file(&external_file);
}

#[test]
fn secrets_scan_relative_output_traversal_blocked() {
    // Test that relative output paths that escape are blocked
    let tmp_dir = Builder::new()
        .prefix("secrets_output_traversal")
        .tempdir()
        .expect("temp dir");

    // Create a file to scan
    let file_path = tmp_dir.path().join("clean.txt");
    fs::write(&file_path, "no secrets here").expect("write file");

    let mut cmd = demoswarm();
    cmd.current_dir(tmp_dir.path());
    cmd.args([
        "secrets",
        "scan",
        "--path",
        "clean.txt",
        "--output",
        "../escape_output.json", // Relative traversal for output - blocked
    ]);

    let output = cmd.assert().success();

    let stdout = String::from_utf8_lossy(&output.get_output().stdout);
    assert!(
        stdout.contains("PATH_BOUNDARY_ERROR"),
        "Expected PATH_BOUNDARY_ERROR for relative output traversal, got: {}",
        stdout
    );

    // Verify no file was created outside
    let escaped_path = tmp_dir.path().parent().unwrap().join("escape_output.json");
    assert!(
        !escaped_path.exists(),
        "Output file should NOT be created via traversal"
    );
}
