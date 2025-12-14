#![forbid(unsafe_code)]

mod checks;
mod cli;
mod contracts;
mod ctx;
mod inventory;
mod reporter;
mod runner;
mod util;

pub use cli::{Cli, OutputFormat};
pub use runner::run;
