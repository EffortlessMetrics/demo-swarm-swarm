//! Time utilities.

use anyhow::Result;
use chrono::Utc;
use clap::{Args, Subcommand};

use crate::output::print_scalar;

#[derive(Args, Debug)]
pub struct TimeCommand {
    #[command(subcommand)]
    pub command: TimeSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum TimeSubcommand {
    /// Print current UTC timestamp in ISO8601 format
    Now,
}

pub fn run(cmd: TimeCommand) -> Result<()> {
    match cmd.command {
        TimeSubcommand::Now => {
            let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
            print_scalar(now);
            Ok(())
        }
    }
}
