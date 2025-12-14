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
