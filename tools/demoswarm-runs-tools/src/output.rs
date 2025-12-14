//! Output utilities for scalar stdout contract.
//!
//! All helpers print a single scalar: `null`, integer, or string.
//! Exit code is always 0 (errors expressed via `null` stdout).

/// Print a scalar value to stdout.
pub fn print_scalar<T: std::fmt::Display>(value: T) {
    println!("{value}");
}

/// Print null to stdout (represents missing/error).
pub fn print_null() {
    println!("null");
}

/// Print an integer count.
pub fn print_count(n: usize) {
    println!("{n}");
}

/// Print "ok" for successful operations.
pub fn print_ok() {
    println!("ok");
}
