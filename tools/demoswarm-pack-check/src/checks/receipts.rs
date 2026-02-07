//! Receipt schema validation checks.
//!
//! Checks: 56, 57, 58, 59

use super::{CheckCtx, CheckSpec};
use crate::reporter::Reporter;

pub fn checks() -> Vec<CheckSpec> {
    vec![
        CheckSpec {
            id: 56,
            title: "Validating receipt JSON...",
            run: check_receipt_json_validity,
        },
        CheckSpec {
            id: 57,
            title: "Validating receipt base schema...",
            run: check_receipt_base_schema,
        },
        CheckSpec {
            id: 58,
            title: "Validating receipt flow-directory consistency...",
            run: check_receipt_flow_consistency,
        },
        CheckSpec {
            id: 59,
            title: "Validating receipt flow-specific fields...",
            run: check_receipt_flow_fields,
        },
    ]
}

/// Check 56: Receipt JSON validity.
///
/// Parse each discovered receipt file. FAIL on malformed JSON.
fn check_receipt_json_validity(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if cx.inv.receipt_files.is_empty() {
        rep.pass("No receipt files found (skipped)");
        return Ok(());
    }

    let mut ok_count = 0u32;
    for rf in &cx.inv.receipt_files {
        let content = cx.ctx.read_utf8(&rf.path)?;
        match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(_) => ok_count += 1,
            Err(e) => rep.fail(format!(
                "{}: malformed JSON: {}",
                cx.ctx.rel(&rf.path),
                e
            )),
        }
    }

    if ok_count > 0 {
        rep.pass(format!("{ok_count} receipt(s) are valid JSON"));
    }
    Ok(())
}

/// Check 57: Receipt base schema.
///
/// Validate required base fields and enum values.
fn check_receipt_base_schema(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if cx.inv.receipt_files.is_empty() {
        rep.pass("No receipt files found (skipped)");
        return Ok(());
    }

    let required_fields: &[&str] = &[
        "run_id",
        "flow",
        "status",
        "recommended_action",
        "completed_at",
    ];

    let mut total_warnings = 0u32;
    let mut total_checked = 0u32;

    for rf in &cx.inv.receipt_files {
        let content = cx.ctx.read_utf8(&rf.path)?;
        let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) else {
            // Malformed JSON already reported by check 56
            continue;
        };
        let Some(obj) = val.as_object() else {
            rep.warn(format!("{}: receipt is not a JSON object", cx.ctx.rel(&rf.path)));
            total_warnings += 1;
            continue;
        };

        total_checked += 1;
        let rel = cx.ctx.rel(&rf.path);

        // Check required fields exist and are strings
        for &field in required_fields {
            match obj.get(field) {
                None => {
                    rep.warn(format!("{rel}: missing required field \"{field}\""));
                    total_warnings += 1;
                }
                Some(v) if !v.is_string() => {
                    rep.warn(format!("{rel}: field \"{field}\" should be a string"));
                    total_warnings += 1;
                }
                _ => {}
            }
        }

        // Validate status enum
        if let Some(status) = obj.get("status").and_then(|v| v.as_str()) {
            if !cx.c.receipt_statuses.contains(&status) {
                rep.warn(format!(
                    "{rel}: unknown status \"{status}\" (expected one of: {})",
                    cx.c.receipt_statuses.join(", ")
                ));
                total_warnings += 1;
            }
        }

        // Validate recommended_action enum
        if let Some(action) = obj.get("recommended_action").and_then(|v| v.as_str()) {
            if !cx.c.receipt_actions.contains(&action) {
                rep.warn(format!(
                    "{rel}: unknown recommended_action \"{action}\" (expected one of: {})",
                    cx.c.receipt_actions.join(", ")
                ));
                total_warnings += 1;
            }
        }
    }

    if total_warnings == 0 && total_checked > 0 {
        rep.pass(format!(
            "{total_checked} receipt(s) have valid base schema"
        ));
    }
    Ok(())
}

/// Check 58: Flow-directory consistency.
///
/// The `flow` field in the JSON must match the parent directory name.
fn check_receipt_flow_consistency(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if cx.inv.receipt_files.is_empty() {
        rep.pass("No receipt files found (skipped)");
        return Ok(());
    }

    let mut ok_count = 0u32;

    for rf in &cx.inv.receipt_files {
        let content = cx.ctx.read_utf8(&rf.path)?;
        let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) else {
            continue;
        };

        let Some(json_flow) = val.get("flow").and_then(|v| v.as_str()) else {
            // Missing flow field already reported by check 57
            continue;
        };

        if json_flow == rf.flow {
            ok_count += 1;
        } else {
            rep.fail(format!(
                "{}: flow field \"{}\" does not match directory \"{}\"",
                cx.ctx.rel(&rf.path),
                json_flow,
                rf.flow
            ));
        }
    }

    if ok_count > 0 {
        rep.pass(format!(
            "{ok_count} receipt(s) have consistent flow/directory"
        ));
    }
    Ok(())
}

/// Check 59: Flow-specific fields present.
///
/// Each flow type has additional required fields.
fn check_receipt_flow_fields(cx: &CheckCtx, rep: &mut Reporter) -> anyhow::Result<()> {
    if cx.inv.receipt_files.is_empty() {
        rep.pass("No receipt files found (skipped)");
        return Ok(());
    }

    let mut total_warnings = 0u32;
    let mut total_checked = 0u32;

    for rf in &cx.inv.receipt_files {
        let content = cx.ctx.read_utf8(&rf.path)?;
        let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) else {
            continue;
        };
        let Some(obj) = val.as_object() else {
            continue;
        };

        let flow = rf.flow.as_str();
        let expected: &[&str] = match flow {
            "signal" => &["counts"],
            "plan" => &["counts", "decision_spine"],
            "build" => &["counts", "tests"],
            "review" => &["feedback", "worklist"],
            "gate" => &["merge_verdict", "counts"],
            "deploy" => &["deployment_verdict", "counts"],
            "wisdom" => &["counts", "run_complete"],
            _ => {
                // Unknown flow directory — not an error, just skip
                continue;
            }
        };

        total_checked += 1;
        let rel = cx.ctx.rel(&rf.path);

        for &field in expected {
            if !obj.contains_key(field) {
                rep.warn(format!(
                    "{rel}: missing flow-specific field \"{field}\" (expected for {flow})"
                ));
                total_warnings += 1;
            }
        }
    }

    if total_warnings == 0 && total_checked > 0 {
        rep.pass(format!(
            "{total_checked} receipt(s) have expected flow-specific fields"
        ));
    }
    Ok(())
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::checks::CheckCtx;
    use crate::cli::OutputFormat;
    use crate::contracts::{Contracts, Regexes};
    use crate::ctx::Ctx;
    use crate::inventory::Inventory;
    use crate::reporter::Reporter;
    use tempfile::TempDir;

    /// Helper: create a minimal .claude directory and optionally receipt files.
    fn setup_env(receipts: &[(&str, &str, &str)]) -> (TempDir, Ctx, Inventory) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        // Minimal .claude structure
        std::fs::create_dir_all(root.join(".claude/agents")).unwrap();
        std::fs::create_dir_all(root.join(".claude/commands")).unwrap();
        std::fs::create_dir_all(root.join(".claude/skills")).unwrap();

        // Create receipt files: (run_id, flow, json_content)
        for &(run_id, flow, content) in receipts {
            let dir = root.join(".runs").join(run_id).join(flow);
            std::fs::create_dir_all(&dir).unwrap();
            let filename = format!("{flow}_receipt.json");
            std::fs::write(dir.join(&filename), content).unwrap();
        }

        let ctx = Ctx::discover(Some(root.to_path_buf())).unwrap();
        let inv = Inventory::from_ctx(&ctx).unwrap();
        (tmp, ctx, inv)
    }

    fn make_check_ctx<'a>(
        ctx: &'a Ctx,
        inv: &'a Inventory,
        re: &'a Regexes,
        c: &'a Contracts,
    ) -> CheckCtx<'a> {
        CheckCtx { ctx, inv, re, c }
    }

    // -------------------------------------------------------------------------
    // Check 56: JSON validity
    // -------------------------------------------------------------------------

    #[test]
    fn test_no_runs_dir_skips_gracefully() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude/agents")).unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude/commands")).unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude/skills")).unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();
        let inv = Inventory::from_ctx(&ctx).unwrap();
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_json_validity(&cx, &mut rep).unwrap();
        assert_eq!(rep.errors, 0);
        assert_eq!(rep.warnings, 0);
    }

    #[test]
    fn test_valid_receipt_json_passes() {
        let json = r#"{"run_id":"test","flow":"signal","status":"VERIFIED","recommended_action":"PROCEED","completed_at":"2025-01-01T00:00:00Z","counts":{}}"#;
        let (_tmp, ctx, inv) = setup_env(&[("test", "signal", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_json_validity(&cx, &mut rep).unwrap();
        assert_eq!(rep.errors, 0);
    }

    #[test]
    fn test_malformed_json_fails() {
        let (_tmp, ctx, inv) = setup_env(&[("test", "signal", "{ not valid json")]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_json_validity(&cx, &mut rep).unwrap();
        assert_eq!(rep.errors, 1, "Malformed JSON should produce a FAIL");
    }

    // -------------------------------------------------------------------------
    // Check 57: Base schema
    // -------------------------------------------------------------------------

    #[test]
    fn test_missing_base_fields_warns() {
        let json = r#"{"run_id":"test","flow":"signal"}"#;
        let (_tmp, ctx, inv) = setup_env(&[("test", "signal", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_base_schema(&cx, &mut rep).unwrap();
        // Missing: status, recommended_action, completed_at
        assert!(rep.warnings >= 3, "Should warn for each missing field, got {}", rep.warnings);
    }

    #[test]
    fn test_unknown_status_warns() {
        let json = r#"{"run_id":"t","flow":"signal","status":"MAGIC","recommended_action":"PROCEED","completed_at":"now"}"#;
        let (_tmp, ctx, inv) = setup_env(&[("t", "signal", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_base_schema(&cx, &mut rep).unwrap();
        assert!(rep.warnings >= 1, "Unknown status should produce a warning");
    }

    #[test]
    fn test_valid_base_schema_passes() {
        let json = r#"{"run_id":"t","flow":"signal","status":"VERIFIED","recommended_action":"PROCEED","completed_at":"now"}"#;
        let (_tmp, ctx, inv) = setup_env(&[("t", "signal", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_base_schema(&cx, &mut rep).unwrap();
        assert_eq!(rep.warnings, 0);
        assert_eq!(rep.errors, 0);
    }

    #[test]
    fn test_non_string_field_warns() {
        let json = r#"{"run_id":123,"flow":"signal","status":"VERIFIED","recommended_action":"PROCEED","completed_at":"now"}"#;
        let (_tmp, ctx, inv) = setup_env(&[("t", "signal", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_base_schema(&cx, &mut rep).unwrap();
        assert!(rep.warnings >= 1, "Non-string run_id should warn");
    }

    // -------------------------------------------------------------------------
    // Check 58: Flow-directory consistency
    // -------------------------------------------------------------------------

    #[test]
    fn test_flow_directory_match_passes() {
        let json = r#"{"flow":"signal"}"#;
        let (_tmp, ctx, inv) = setup_env(&[("t", "signal", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_flow_consistency(&cx, &mut rep).unwrap();
        assert_eq!(rep.errors, 0);
    }

    #[test]
    fn test_flow_directory_mismatch_fails() {
        let json = r#"{"flow":"build"}"#;
        let (_tmp, ctx, inv) = setup_env(&[("t", "signal", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_flow_consistency(&cx, &mut rep).unwrap();
        assert_eq!(rep.errors, 1, "flow/directory mismatch should FAIL");
    }

    // -------------------------------------------------------------------------
    // Check 59: Flow-specific fields
    // -------------------------------------------------------------------------

    #[test]
    fn test_missing_flow_specific_fields_warns() {
        // signal requires "counts" but it's missing
        let json = r#"{"run_id":"t","flow":"signal","status":"VERIFIED","recommended_action":"PROCEED","completed_at":"now"}"#;
        let (_tmp, ctx, inv) = setup_env(&[("t", "signal", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_flow_fields(&cx, &mut rep).unwrap();
        assert!(rep.warnings >= 1, "Missing flow-specific field should warn");
    }

    #[test]
    fn test_signal_with_counts_passes() {
        let json = r#"{"run_id":"t","flow":"signal","status":"VERIFIED","recommended_action":"PROCEED","completed_at":"now","counts":{"requirements":3}}"#;
        let (_tmp, ctx, inv) = setup_env(&[("t", "signal", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_flow_fields(&cx, &mut rep).unwrap();
        assert_eq!(rep.warnings, 0);
    }

    #[test]
    fn test_build_missing_tests_warns() {
        // build requires "counts" and "tests"
        let json = r#"{"run_id":"t","flow":"build","status":"VERIFIED","recommended_action":"PROCEED","completed_at":"now","counts":{}}"#;
        let (_tmp, ctx, inv) = setup_env(&[("t", "build", json)]);
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_flow_fields(&cx, &mut rep).unwrap();
        assert!(rep.warnings >= 1, "Missing 'tests' field for build should warn");
    }

    #[test]
    fn test_unknown_flow_dir_skipped() {
        // Unknown flow directory: no expected fields, so no warnings
        let json = r#"{"run_id":"t","flow":"custom","status":"VERIFIED"}"#;
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude/agents")).unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude/commands")).unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude/skills")).unwrap();
        let dir = tmp.path().join(".runs/t/custom");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("custom_receipt.json"), json).unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();
        let inv = Inventory::from_ctx(&ctx).unwrap();
        let re = Regexes::compile().unwrap();
        let c = Contracts::default();
        let cx = make_check_ctx(&ctx, &inv, &re, &c);

        let mut rep = Reporter::new(OutputFormat::Json, false, false);
        check_receipt_flow_fields(&cx, &mut rep).unwrap();
        assert_eq!(rep.warnings, 0, "Unknown flow dir should be skipped gracefully");
    }

    // -------------------------------------------------------------------------
    // Receipt discovery tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_receipt_discovery_finds_files() {
        let json = r#"{"flow":"signal"}"#;
        let (_tmp, _ctx, inv) = setup_env(&[("run1", "signal", json)]);
        assert_eq!(inv.receipt_files.len(), 1);
        assert_eq!(inv.receipt_files[0].run_id, "run1");
        assert_eq!(inv.receipt_files[0].flow, "signal");
        assert_eq!(inv.receipt_files[0].filename, "signal_receipt.json");
    }

    #[test]
    fn test_receipt_discovery_multiple_runs() {
        let json1 = r#"{"flow":"signal"}"#;
        let json2 = r#"{"flow":"build"}"#;
        let (_tmp, _ctx, inv) = setup_env(&[
            ("run1", "signal", json1),
            ("run1", "build", json2),
        ]);
        assert_eq!(inv.receipt_files.len(), 2);
    }

    #[test]
    fn test_receipt_discovery_no_runs_dir() {
        let tmp = TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude/agents")).unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude/commands")).unwrap();
        std::fs::create_dir_all(tmp.path().join(".claude/skills")).unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();
        let inv = Inventory::from_ctx(&ctx).unwrap();
        assert!(inv.receipt_files.is_empty());
    }
}
