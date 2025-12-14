use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Human-friendly text output (default).
    Text,
    /// Machine-readable JSON summary.
    Json,
}

#[derive(Debug, Clone, Parser)]
#[command(
    name = "pack-check",
    about = "Validate a DemoSwarm .claude pack for structural + contract consistency",
    version,
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Repo root directory (the directory that contains .claude/).
    ///
    /// If omitted, pack-check walks up from the current working directory
    /// until it finds a .claude/ directory.
    #[arg(long)]
    pub repo_root: Option<PathBuf>,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    /// Disable ANSI colors in text output.
    #[arg(long)]
    pub no_color: bool,

    /// Treat warnings as errors for the process exit code.
    #[arg(long)]
    pub strict_warnings: bool,
}
