use std::process::ExitCode;

use clap::Parser;

fn main() -> ExitCode {
    let cli = demoswarm_pack_check::Cli::parse();

    match demoswarm_pack_check::run(cli) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("Error: {err:#}");
            ExitCode::from(2)
        }
    }
}
