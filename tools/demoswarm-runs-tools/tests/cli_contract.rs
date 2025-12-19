use assert_cmd::{Command, cargo::cargo_bin_cmd};
use std::io::Write;
use tempfile::NamedTempFile;

fn demoswarm() -> Command {
    cargo_bin_cmd!("demoswarm")
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
