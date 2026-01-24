use assert_cmd::{Command, cargo::cargo_bin_cmd};
use std::fs;
use std::io::Write;
use tempfile::Builder;

fn demoswarm() -> Command {
    cargo_bin_cmd!("demoswarm")
}

#[test]
fn secrets_scan_finds_token_in_nested_dir() {
    let tmp_dir = Builder::new().prefix("secrets_test").tempdir().expect("temp dir");
    let nested = tmp_dir.path().join("nested");
    fs::create_dir(&nested).expect("mkdir");
    
    let file_path = nested.join("secret.txt");
    let mut file = fs::File::create(&file_path).expect("create file");
    writeln!(file, "ghp_123456789012345678901234567890123456").expect("write secret");

    let output_file = tmp_dir.path().join("findings.json");

    let mut cmd = demoswarm();
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
    let tmp_dir = Builder::new().prefix("secrets_test_dot").tempdir().expect("temp dir");
    let file_path = tmp_dir.path().join("secret.txt");
    let mut file = fs::File::create(&file_path).expect("create file");
    writeln!(file, "ghp_123456789012345678901234567890123456").expect("write secret");

    let output_file = tmp_dir.path().join("findings.json");

    // Construct a path like "C:\tmp\foo\.\secret.txt"
    let funny_path = tmp_dir.path().join(".").join("secret.txt");

    let mut cmd = demoswarm();
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
