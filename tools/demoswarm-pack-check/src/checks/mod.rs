//! Check registry.
//!
//! Each check is a function that takes (CheckCtx, Reporter) and emits diagnostics.
//! The registry returns all checks in numeric order.

mod contracts;
mod control_plane;
mod drift;
mod flow;
mod structure;
mod wisdom;

use super::contracts::{Contracts, Regexes};
use crate::ctx::Ctx;
use crate::inventory::Inventory;
use crate::reporter::Reporter;

/// Shared context for all checks - avoids signature sprawl.
pub struct CheckCtx<'a> {
    pub ctx: &'a Ctx,
    pub inv: &'a Inventory,
    pub re: &'a Regexes,
    pub c: &'a Contracts,
}

/// Check function signature.
pub type CheckFn = fn(&CheckCtx, &mut Reporter) -> anyhow::Result<()>;

/// A single check specification.
pub struct CheckSpec {
    pub id: u32,
    pub title: &'static str,
    pub run: CheckFn,
}

/// Returns all checks in numeric order (1..53).
pub fn all() -> Vec<CheckSpec> {
    let mut checks = Vec::new();

    // Structure checks (1, 2, 6, 9, 10, 15)
    checks.extend(structure::checks());

    // Control-plane checks (3, 4, 16, 17, 18, 19, 20, 21, 28, 29, 31, 32, 33, 34, 35)
    checks.extend(control_plane::checks());

    // Drift checks (7, 8, 14, 23, 30, 38, 39, 40, 42, 45, 46, 47, 48, 49, 50, 52, 53)
    checks.extend(drift::checks());

    // Flow checks (5, 11, 12, 13, 22, 25, 26, 27, 37, 43, 44, 45, 46, 47, 48, 49, 50)
    checks.extend(flow::checks());

    // Wisdom checks (24, 36, 41)
    checks.extend(wisdom::checks());

    // Sort by ID to ensure consistent ordering
    checks.sort_by_key(|c| c.id);
    checks
}
