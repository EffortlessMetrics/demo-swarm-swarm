//! Shared clap helpers for compatibility flags.

use clap::Args;

/// Compatibility flag accepted by read-ish commands to avoid clap errors when
/// callers pass `--null-if-missing`. Semantics are handled upstream; we accept
/// the flag to keep the interface stable.
#[derive(Args, Debug, Default, Clone)]
pub struct CompatNullIfMissing {
    /// Optional compatibility flag; semantics are already null-safe.
    #[arg(long, default_value_t = false)]
    pub null_if_missing: bool,
}
