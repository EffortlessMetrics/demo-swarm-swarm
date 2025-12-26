use std::process::ExitCode;

use serde::Serialize;

use crate::cli::OutputFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub level: Level,
    pub check_id: u32,
    pub check_title: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PackCounts {
    pub agents: usize,
    pub commands: usize,
    pub skills: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct RunReport {
    pub schema_version: u32,
    pub repo_root: String,
    pub errors: usize,
    pub warnings: usize,
    pub counts: PackCounts,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug)]
pub struct Reporter {
    format: OutputFormat,
    color: bool,
    strict_warnings: bool,

    pub errors: usize,
    pub warnings: usize,

    current_check_id: u32,
    current_check_title: String,

    diagnostics: Vec<Diagnostic>,
}

impl Reporter {
    pub fn new(format: OutputFormat, color: bool, strict_warnings: bool) -> Self {
        Self {
            format,
            color,
            strict_warnings,
            errors: 0,
            warnings: 0,
            current_check_id: 0,
            current_check_title: String::new(),
            diagnostics: Vec::new(),
        }
    }

    pub fn print_banner(&self) {
        if self.format != OutputFormat::Text {
            return;
        }
        println!("==================================");
        println!("  DemoSwarm Pack Self-Check");
        println!("==================================");
        println!();
    }

    pub fn section(&mut self, id: u32, title: &str) {
        self.current_check_id = id;
        self.current_check_title = title.to_string();

        if self.format != OutputFormat::Text {
            return;
        }

        println!("{id}. {title}...");
    }

    pub fn blank_line(&self) {
        if self.format == OutputFormat::Text {
            println!();
        }
    }

    pub fn pass(&mut self, msg: impl AsRef<str>) {
        self.emit(Level::Pass, msg.as_ref());
    }

    pub fn warn(&mut self, msg: impl AsRef<str>) {
        self.warnings += 1;
        self.emit(Level::Warn, msg.as_ref());
    }

    pub fn fail(&mut self, msg: impl AsRef<str>) {
        self.errors += 1;
        self.emit(Level::Fail, msg.as_ref());
    }

    pub fn indent_lines<I, S>(&self, lines: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        if self.format != OutputFormat::Text {
            return;
        }
        for line in lines {
            println!("       {}", line.as_ref());
        }
    }

    pub fn print_summary_header(&self) {
        if self.format != OutputFormat::Text {
            return;
        }
        println!("==================================");
        println!("  Summary");
        println!("==================================");
        println!();
    }

    pub fn print_counts(&self, counts: &PackCounts) {
        if self.format != OutputFormat::Text {
            return;
        }
        println!("Pack contents:");
        println!("  Agents:   {}", counts.agents);
        println!("  Commands: {}", counts.commands);
        println!("  Skills:   {}", counts.skills);
        println!();
    }

    pub fn finish(self, repo_root: &str, counts: PackCounts) -> anyhow::Result<ExitCode> {
        match self.format {
            OutputFormat::Text => {
                let code = if self.errors == 0 && (!self.strict_warnings || self.warnings == 0) {
                    if self.warnings == 0 {
                        println!("{}", self.colorize(Level::Pass, "All checks passed!"));
                    } else {
                        println!(
                            "{}",
                            self.colorize(
                                Level::Warn,
                                &format!("Passed with {} warning(s)", self.warnings),
                            )
                        );
                    }
                    ExitCode::SUCCESS
                } else {
                    println!(
                        "{}",
                        self.colorize(
                            Level::Fail,
                            &format!(
                                "Failed with {} error(s) and {} warning(s)",
                                self.errors, self.warnings
                            ),
                        )
                    );
                    ExitCode::from(1)
                };

                Ok(code)
            }
            OutputFormat::Json => {
                let report = RunReport {
                    schema_version: 1,
                    repo_root: repo_root.to_string(),
                    errors: self.errors,
                    warnings: self.warnings,
                    counts,
                    diagnostics: self
                        .diagnostics
                        .into_iter()
                        .filter(|d| d.level != Level::Pass)
                        .collect(),
                };

                println!("{}", serde_json::to_string_pretty(&report)?);

                if report.errors == 0 && (!self.strict_warnings || report.warnings == 0) {
                    Ok(ExitCode::SUCCESS)
                } else {
                    Ok(ExitCode::from(1))
                }
            }
        }
    }

    fn emit(&mut self, level: Level, msg: &str) {
        // JSON mode: keep a structured record.
        if self.format == OutputFormat::Json {
            self.diagnostics.push(Diagnostic {
                level,
                check_id: self.current_check_id,
                check_title: self.current_check_title.clone(),
                message: msg.to_string(),
            });
            return;
        }

        // Text mode: print immediately.
        let prefix = match level {
            Level::Pass => self.colorize(Level::Pass, "✓"),
            Level::Warn => self.colorize(Level::Warn, "⚠"),
            Level::Fail => self.colorize(Level::Fail, "✗"),
        };
        println!("{prefix} {msg}");
    }

    fn colorize(&self, level: Level, s: &str) -> String {
        if !self.color {
            return s.to_string();
        }

        let code = match level {
            Level::Pass => "\x1b[0;32m", // green
            Level::Warn => "\x1b[1;33m", // yellow
            Level::Fail => "\x1b[0;31m", // red
        };
        format!("{code}{s}\x1b[0m")
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Level enum tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_level_debug_and_clone() {
        let pass = Level::Pass;
        let warn = Level::Warn;
        let fail = Level::Fail;

        assert_eq!(pass, pass.clone());
        assert_eq!(warn, warn.clone());
        assert_eq!(fail, fail.clone());

        // Ensure Debug works (just call it, no assertion on format)
        let _ = format!("{:?}", pass);
        let _ = format!("{:?}", warn);
        let _ = format!("{:?}", fail);
    }

    #[test]
    fn test_level_serialize() {
        assert_eq!(serde_json::to_string(&Level::Pass).unwrap(), "\"pass\"");
        assert_eq!(serde_json::to_string(&Level::Warn).unwrap(), "\"warn\"");
        assert_eq!(serde_json::to_string(&Level::Fail).unwrap(), "\"fail\"");
    }

    // -------------------------------------------------------------------------
    // Diagnostic struct tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_diagnostic_debug_and_clone() {
        let diag = Diagnostic {
            level: Level::Fail,
            check_id: 1,
            check_title: "Test Check".to_string(),
            message: "Test message".to_string(),
        };

        let cloned = diag.clone();
        assert_eq!(diag.level, cloned.level);
        assert_eq!(diag.check_id, cloned.check_id);
        assert_eq!(diag.check_title, cloned.check_title);
        assert_eq!(diag.message, cloned.message);

        // Ensure Debug works
        let _ = format!("{:?}", diag);
    }

    #[test]
    fn test_diagnostic_serialize() {
        let diag = Diagnostic {
            level: Level::Warn,
            check_id: 42,
            check_title: "Title".to_string(),
            message: "Msg".to_string(),
        };

        let json = serde_json::to_string(&diag).unwrap();
        assert!(json.contains("\"level\":\"warn\""));
        assert!(json.contains("\"check_id\":42"));
        assert!(json.contains("\"check_title\":\"Title\""));
        assert!(json.contains("\"message\":\"Msg\""));
    }

    // -------------------------------------------------------------------------
    // PackCounts struct tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_pack_counts_debug_clone_serialize() {
        let counts = PackCounts {
            agents: 10,
            commands: 5,
            skills: 3,
        };

        let cloned = counts.clone();
        assert_eq!(counts.agents, cloned.agents);
        assert_eq!(counts.commands, cloned.commands);
        assert_eq!(counts.skills, cloned.skills);

        let _ = format!("{:?}", counts);

        let json = serde_json::to_string(&counts).unwrap();
        assert!(json.contains("\"agents\":10"));
        assert!(json.contains("\"commands\":5"));
        assert!(json.contains("\"skills\":3"));
    }

    // -------------------------------------------------------------------------
    // RunReport struct tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_run_report_debug_clone_serialize() {
        let report = RunReport {
            schema_version: 1,
            repo_root: "/test/repo".to_string(),
            errors: 2,
            warnings: 3,
            counts: PackCounts {
                agents: 1,
                commands: 2,
                skills: 3,
            },
            diagnostics: vec![Diagnostic {
                level: Level::Fail,
                check_id: 1,
                check_title: "Check".to_string(),
                message: "Error".to_string(),
            }],
        };

        let cloned = report.clone();
        assert_eq!(report.schema_version, cloned.schema_version);
        assert_eq!(report.repo_root, cloned.repo_root);
        assert_eq!(report.errors, cloned.errors);
        assert_eq!(report.warnings, cloned.warnings);

        let _ = format!("{:?}", report);

        let json = serde_json::to_string(&report).unwrap();
        assert!(json.contains("\"schema_version\":1"));
        assert!(json.contains("\"repo_root\":\"/test/repo\""));
    }

    // -------------------------------------------------------------------------
    // Reporter::new tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_new_text_format() {
        let rep = Reporter::new(OutputFormat::Text, true, false);
        assert_eq!(rep.format, OutputFormat::Text);
        assert!(rep.color);
        assert!(!rep.strict_warnings);
        assert_eq!(rep.errors, 0);
        assert_eq!(rep.warnings, 0);
    }

    #[test]
    fn test_reporter_new_json_format() {
        let rep = Reporter::new(OutputFormat::Json, false, true);
        assert_eq!(rep.format, OutputFormat::Json);
        assert!(!rep.color);
        assert!(rep.strict_warnings);
        assert_eq!(rep.errors, 0);
        assert_eq!(rep.warnings, 0);
    }

    // -------------------------------------------------------------------------
    // Reporter::pass, warn, fail tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_pass_does_not_increment_counts() {
        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        rep.pass("passed check");
        assert_eq!(rep.errors, 0);
        assert_eq!(rep.warnings, 0);
    }

    #[test]
    fn test_reporter_warn_increments_warnings() {
        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        rep.warn("warning message");
        assert_eq!(rep.warnings, 1);
        assert_eq!(rep.errors, 0);
    }

    #[test]
    fn test_reporter_fail_increments_errors() {
        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        rep.fail("error message");
        assert_eq!(rep.errors, 1);
        assert_eq!(rep.warnings, 0);
    }

    #[test]
    fn test_reporter_multiple_diagnostics() {
        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        rep.section(1, "Check 1");
        rep.pass("ok");
        rep.warn("warn1");
        rep.section(2, "Check 2");
        rep.fail("fail1");
        rep.warn("warn2");

        assert_eq!(rep.errors, 1);
        assert_eq!(rep.warnings, 2);
    }

    // -------------------------------------------------------------------------
    // Reporter::section tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_section_sets_current_check() {
        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        rep.section(42, "Test Section");
        assert_eq!(rep.current_check_id, 42);
        assert_eq!(rep.current_check_title, "Test Section");
    }

    // -------------------------------------------------------------------------
    // Reporter::print_banner tests (JSON mode skips)
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_print_banner_json_returns_early() {
        // In JSON mode, print_banner should return early and not print
        let rep = Reporter::new(OutputFormat::Json, false, false);
        rep.print_banner(); // Should not panic, returns early
    }

    // -------------------------------------------------------------------------
    // Reporter::blank_line tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_blank_line_json_mode() {
        let rep = Reporter::new(OutputFormat::Json, false, false);
        rep.blank_line(); // Should not panic, skips in JSON mode
    }

    // -------------------------------------------------------------------------
    // Reporter::indent_lines tests (JSON mode skips)
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_indent_lines_json_returns_early() {
        let rep = Reporter::new(OutputFormat::Json, false, false);
        rep.indent_lines(vec!["line1", "line2"]); // Should not panic
    }

    // -------------------------------------------------------------------------
    // Reporter::print_summary_header tests (JSON mode skips)
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_print_summary_header_json_returns_early() {
        let rep = Reporter::new(OutputFormat::Json, false, false);
        rep.print_summary_header(); // Should not panic
    }

    // -------------------------------------------------------------------------
    // Reporter::print_counts tests (JSON mode skips)
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_print_counts_json_returns_early() {
        let rep = Reporter::new(OutputFormat::Json, false, false);
        let counts = PackCounts {
            agents: 10,
            commands: 5,
            skills: 3,
        };
        rep.print_counts(&counts); // Should not panic
    }

    // -------------------------------------------------------------------------
    // Reporter::finish tests - JSON format
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_finish_json_success() {
        let rep = Reporter::new(OutputFormat::Json, false, false);
        let counts = PackCounts {
            agents: 1,
            commands: 2,
            skills: 3,
        };
        let result = rep.finish("/test/repo", counts);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), std::process::ExitCode::SUCCESS);
    }

    #[test]
    fn test_reporter_finish_json_with_errors() {
        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        rep.section(1, "Check");
        rep.fail("error");
        let counts = PackCounts {
            agents: 1,
            commands: 2,
            skills: 3,
        };
        let result = rep.finish("/test/repo", counts);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), std::process::ExitCode::from(1));
    }

    #[test]
    fn test_reporter_finish_json_with_warnings_strict() {
        let mut rep = Reporter::new(OutputFormat::Json, false, true);
        rep.section(1, "Check");
        rep.warn("warning");
        let counts = PackCounts {
            agents: 1,
            commands: 2,
            skills: 3,
        };
        let result = rep.finish("/test/repo", counts);
        assert!(result.is_ok());
        // With strict_warnings, warnings cause failure
        assert_eq!(result.unwrap(), std::process::ExitCode::from(1));
    }

    #[test]
    fn test_reporter_finish_json_with_warnings_not_strict() {
        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        rep.section(1, "Check");
        rep.warn("warning");
        let counts = PackCounts {
            agents: 1,
            commands: 2,
            skills: 3,
        };
        let result = rep.finish("/test/repo", counts);
        assert!(result.is_ok());
        // Without strict_warnings, warnings don't cause failure
        assert_eq!(result.unwrap(), std::process::ExitCode::SUCCESS);
    }

    #[test]
    fn test_reporter_finish_json_filters_pass_diagnostics() {
        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        rep.section(1, "Check");
        rep.pass("ok");
        rep.warn("warning");
        // Pass diagnostics should be filtered out in JSON output
        // Only warn/fail should remain in diagnostics
        let counts = PackCounts {
            agents: 1,
            commands: 2,
            skills: 3,
        };
        let _ = rep.finish("/test/repo", counts);
    }

    // -------------------------------------------------------------------------
    // Reporter::finish tests - Text format
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_finish_text_all_passed() {
        let rep = Reporter::new(OutputFormat::Text, false, false);
        let counts = PackCounts {
            agents: 1,
            commands: 2,
            skills: 3,
        };
        let result = rep.finish("/test/repo", counts);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), std::process::ExitCode::SUCCESS);
    }

    #[test]
    fn test_reporter_finish_text_with_warnings() {
        let mut rep = Reporter::new(OutputFormat::Text, false, false);
        rep.section(1, "Check");
        rep.warn("warning");
        let counts = PackCounts {
            agents: 1,
            commands: 2,
            skills: 3,
        };
        let result = rep.finish("/test/repo", counts);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), std::process::ExitCode::SUCCESS);
    }

    #[test]
    fn test_reporter_finish_text_with_errors() {
        let mut rep = Reporter::new(OutputFormat::Text, false, false);
        rep.section(1, "Check");
        rep.fail("error");
        let counts = PackCounts {
            agents: 1,
            commands: 2,
            skills: 3,
        };
        let result = rep.finish("/test/repo", counts);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), std::process::ExitCode::from(1));
    }

    #[test]
    fn test_reporter_finish_text_strict_warnings() {
        let mut rep = Reporter::new(OutputFormat::Text, false, true);
        rep.section(1, "Check");
        rep.warn("warning");
        let counts = PackCounts {
            agents: 1,
            commands: 2,
            skills: 3,
        };
        let result = rep.finish("/test/repo", counts);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), std::process::ExitCode::from(1));
    }

    // -------------------------------------------------------------------------
    // Reporter::emit tests (via pass/warn/fail in JSON mode)
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_emit_stores_diagnostics_json() {
        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        rep.section(5, "Test Section");
        rep.pass("pass msg");
        rep.warn("warn msg");
        rep.fail("fail msg");

        // Check that diagnostics are stored
        assert_eq!(rep.diagnostics.len(), 3);

        assert_eq!(rep.diagnostics[0].level, Level::Pass);
        assert_eq!(rep.diagnostics[0].check_id, 5);
        assert_eq!(rep.diagnostics[0].check_title, "Test Section");
        assert_eq!(rep.diagnostics[0].message, "pass msg");

        assert_eq!(rep.diagnostics[1].level, Level::Warn);
        assert_eq!(rep.diagnostics[1].message, "warn msg");

        assert_eq!(rep.diagnostics[2].level, Level::Fail);
        assert_eq!(rep.diagnostics[2].message, "fail msg");
    }

    // -------------------------------------------------------------------------
    // Reporter::colorize tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_colorize_no_color() {
        let rep = Reporter::new(OutputFormat::Text, false, false);
        let result = rep.colorize(Level::Pass, "test");
        assert_eq!(result, "test");
    }

    #[test]
    fn test_reporter_colorize_with_color_pass() {
        let rep = Reporter::new(OutputFormat::Text, true, false);
        let result = rep.colorize(Level::Pass, "test");
        assert!(result.contains("\x1b[0;32m")); // green
        assert!(result.contains("test"));
        assert!(result.contains("\x1b[0m")); // reset
    }

    #[test]
    fn test_reporter_colorize_with_color_warn() {
        let rep = Reporter::new(OutputFormat::Text, true, false);
        let result = rep.colorize(Level::Warn, "test");
        assert!(result.contains("\x1b[1;33m")); // yellow
        assert!(result.contains("test"));
        assert!(result.contains("\x1b[0m")); // reset
    }

    #[test]
    fn test_reporter_colorize_with_color_fail() {
        let rep = Reporter::new(OutputFormat::Text, true, false);
        let result = rep.colorize(Level::Fail, "test");
        assert!(result.contains("\x1b[0;31m")); // red
        assert!(result.contains("test"));
        assert!(result.contains("\x1b[0m")); // reset
    }

    // -------------------------------------------------------------------------
    // Reporter debug test
    // -------------------------------------------------------------------------

    #[test]
    fn test_reporter_debug() {
        let rep = Reporter::new(OutputFormat::Text, true, false);
        let debug_str = format!("{:?}", rep);
        assert!(debug_str.contains("Reporter"));
    }
}
