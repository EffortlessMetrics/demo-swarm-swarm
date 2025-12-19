//! Pack contracts: canonical strings, regex patterns, and required lists.
//!
//! All "what to check" lives here. Checks reference these constants
//! instead of inventing their own literals.

use regex::{Regex, RegexBuilder};

/// Canonical headings used in pack artifacts.
pub mod headings {
    pub const MACHINE_SUMMARY_H2: &str = "## Machine Summary";
    pub const REPO_OPERATOR_RESULT_H2: &str = "## Repo Operator Result";
    pub const ORCHESTRATOR_KICKOFF_H2: &str = "## Orchestrator Kickoff";
    pub const TODOWRITE_H3: &str = "### TodoWrite (copy exactly)";
    pub const SMOKE_VERIFIER_RESULT_H2: &str = "## Smoke Verifier Result";
    pub const SAFE_OUTPUT_CONTRACT: &str = "Safe Output Contract";
    pub const ITERATION_CONTROL_H2: &str = "## Iteration Control";
    pub const ISSUE_FIRST_INVARIANT: &str = "Issue-First Invariant";
    pub const DECISION_SPINE: &str = "Decision Spine";
}

/// All pack contracts bundled for easy passing.
#[derive(Debug)]
pub struct Contracts {
    pub required_agents: &'static [&'static str],
    pub required_skills: &'static [&'static str],
    pub cleanup_agents: &'static [(&'static str, &'static str)],
    pub critics: &'static [&'static str],
    pub critic_and_verifier_agents: &'static [&'static str],
    pub gate_agents: &'static [&'static str],
    pub gh_agents: &'static [&'static str],
    pub banned_patterns: &'static [&'static str],
    pub sealing_markers: &'static [&'static str],
    pub gate_result_fields: &'static [&'static str],
    pub repo_operator_result_fields: &'static [&'static str],
    pub claude_md_sections: &'static [&'static str],
    pub flow_specific_patterns: &'static [&'static str],
    pub raw_git_patterns: &'static [(&'static str, &'static str)],
    pub reg_marker_literal: &'static str,
    // Skill ownership
    pub index_allowed_agents: &'static [&'static str],
    pub secrets_allowed_agents: &'static [&'static str],
    pub openq_allowed_agents: &'static [&'static str],
    // GH body hygiene
    pub gh_posting_agents: &'static [&'static str],
    pub gh_body_forbidden_patterns: &'static [&'static str],
    // Flow boundary enforcement (check 52)
    pub skill_cli_subcommands: &'static [&'static str],
    // OpenQ prefix validation (check 53)
    pub openq_flow_codes: &'static [&'static str],
}

impl Default for Contracts {
    fn default() -> Self {
        Self {
            required_agents: REQUIRED_AGENTS,
            required_skills: REQUIRED_SKILLS,
            cleanup_agents: CLEANUP_AGENTS,
            critics: CRITICS,
            critic_and_verifier_agents: CRITIC_AND_VERIFIER_AGENTS,
            gate_agents: GATE_AGENTS,
            gh_agents: GH_AGENTS,
            banned_patterns: BANNED_PATTERNS,
            sealing_markers: SEALING_MARKERS,
            gate_result_fields: GATE_RESULT_FIELDS,
            repo_operator_result_fields: REPO_OPERATOR_RESULT_FIELDS,
            claude_md_sections: CLAUDE_MD_SECTIONS,
            flow_specific_patterns: FLOW_SPECIFIC_PATTERNS,
            raw_git_patterns: RAW_GIT_PATTERNS,
            reg_marker_literal: REG_MARKER_LITERAL,
            index_allowed_agents: INDEX_ALLOWED_AGENTS,
            secrets_allowed_agents: SECRETS_ALLOWED_AGENTS,
            openq_allowed_agents: OPENQ_ALLOWED_AGENTS,
            gh_posting_agents: GH_POSTING_AGENTS,
            gh_body_forbidden_patterns: GH_BODY_FORBIDDEN_PATTERNS,
            skill_cli_subcommands: SKILL_CLI_SUBCOMMANDS,
            openq_flow_codes: OPENQ_FLOW_CODES,
        }
    }
}

/// Compiled regex patterns used across checks.
pub struct Regexes {
    // Machine Summary axis
    pub canon_status: Regex,
    pub canon_action: Regex,
    pub route_to_agent: Regex,
    pub route_to_flow: Regex,
    pub recommended_action_present: Regex,

    // Status validation
    pub blocked_status: Regex,

    // Gate/checkpoint
    pub checkpoint_mode_local: Regex,
    pub proceed_false: Regex,
    pub gh_agent: Regex,
    pub both_gates_same_line: Regex,

    // Taxonomy
    pub old_fr_id: Regex,
    pub old_bdd_tag: Regex,
    pub bare_nfr_id: Regex,

    // Decision spine
    pub spine_marker: Regex,
    pub status_enum: Regex,
    pub precedence: Regex,
    pub todo_status: Regex,

    // smoke-verifier
    pub smoke_signal: Regex,

    // Drift patterns
    pub ensure_branch_op: Regex,
    pub claude_sub: Regex,
    pub domain_verdict: Regex,

    // CANNOT_PROCEED
    pub cannot_proceed_sem: Regex,

    // Reseal
    pub reseal_pattern: Regex,
    pub reseal_guard: Regex,

    // jq paths
    pub jq_has_runs: Regex,
    pub jq_quote: Regex,

    // PR-first pattern
    pub pr_first: Regex,

    // issue drafts
    pub issue_drafts: Regex,

    // grep for REG markers
    pub grep_reg_marker: Regex,

    // Bespoke pipeline patterns (should use demoswarm.sh shim)
    pub bespoke_pipeline: Regex,

    // Skill ownership patterns
    pub index_upsert_cmd: Regex,
    pub secrets_cmd: Regex,
    pub openq_cmd: Regex,

    // Shim enforcement patterns
    pub shim_line_continuation: Regex,
    pub direct_demoswarm_invocation: Regex,

    // Structured extraction calls (cross-agent contract checks)
    pub ms_get_invocation: Regex,
    pub inv_get_invocation: Regex,

    // Boundary check patterns (checks 45-47)
    pub skill_names_in_prose: Regex,
    pub demoswarm_shim_ref: Regex,
    pub flow_output_arrow: Regex,

    // GH body hygiene patterns (check 50)
    pub gh_heredoc_pattern: Regex,
}

impl Regexes {
    pub fn compile() -> anyhow::Result<Self> {
        // Helper for line-anchored patterns (need multiline mode)
        let ml = |pattern: &str| -> anyhow::Result<Regex> {
            Ok(RegexBuilder::new(pattern).multi_line(true).build()?)
        };
        let dotall = |pattern: &str| -> anyhow::Result<Regex> {
            Ok(RegexBuilder::new(pattern)
                .dot_matches_new_line(true)
                .multi_line(true)
                .build()?)
        };

        Ok(Self {
            // Machine Summary canonical axis lines (line-anchored, need multiline)
            canon_status: ml(
                r"^\s*status:\s*VERIFIED\s*\|\s*UNVERIFIED\s*\|\s*CANNOT_PROCEED\s*$",
            )?,
            canon_action: ml(
                r"^\s*recommended_action:\s*PROCEED\s*\|\s*RERUN\s*\|\s*BOUNCE\s*\|\s*FIX_ENV\s*$",
            )?,
            route_to_agent: ml(r"^\s*route_to_agent:")?,
            route_to_flow: ml(r"^\s*route_to_flow:")?,
            recommended_action_present: ml(r"^\s*recommended_action:")?,

            // Status validation
            blocked_status: Regex::new(r"status:.*BLOCKED[^_]|status:.*BLOCKED$")?,

            // Gate/checkpoint
            checkpoint_mode_local: Regex::new(r"checkpoint_mode.*local_only")?,
            proceed_false: Regex::new(r"proceed_to_github_ops.*false")?,
            gh_agent: Regex::new(r"(?i)(gh-issue-manager|gh-reporter)")?,
            both_gates_same_line: Regex::new(
                r"(safe_to_publish.*proceed_to_github_ops)|(proceed_to_github_ops.*safe_to_publish)",
            )?,

            // Taxonomy
            old_fr_id: Regex::new(r"(^|[^A-Za-z0-9_])FR-[0-9]{1,3}([^A-Za-z0-9_]|$)")?,
            old_bdd_tag: Regex::new(r"@FR-")?,
            bare_nfr_id: Regex::new(r"(^|[^A-Za-z0-9_])NFR-[0-9]{1,3}([^A-Za-z0-9_]|$)")?,

            // Decision spine
            spine_marker: Regex::new(r"ADR_CHOSEN_OPTION|ADR_DRIVER|DRIVER:")?,
            status_enum: Regex::new(r"status:\s*(TODO|DOING|DONE)")?,
            precedence: Regex::new(r"resolution_source.*subtask_index")?,
            todo_status: Regex::new(r"status:\s*TODO")?,

            // smoke-verifier (line-anchored)
            smoke_signal: ml(r"^\s*smoke_signal:\s*STABLE\s*\|\s*INVESTIGATE\s*\|\s*ROLLBACK\s*$")?,

            // Drift patterns
            ensure_branch_op: Regex::new(r"operation:\s*ensure_branch|operation.*ensure_branch")?,
            claude_sub: Regex::new(r"See.*CLAUDE\.md.*>")?,
            domain_verdict: ml(r"^\s*recommended_action:.*\b(ROLLBACK|INVESTIGATE)\b")?,

            // CANNOT_PROCEED semantics
            cannot_proceed_sem: Regex::new(r"(?i)IO|permissions|tool(ing)?|mechanical")?,

            // Reseal patterns
            reseal_pattern: Regex::new(
                r"(?i)(modified_files: true|reseal|cleanup.*↔.*secrets-sanitizer|secrets-sanitizer.*modified_files)",
            )?,
            reseal_guard: Regex::new(
                r"(?i)(reseal.*(2|two|twice)|modified_files.*persists|non-convergent|reseal.*loop)",
            )?,

            // jq paths
            jq_has_runs: Regex::new(r#"jq.*"\.runs/"#)?,
            jq_quote: Regex::new(r#"jq[^#]*""#)?,

            // PR-first pattern (anti-pattern)
            pr_first: Regex::new(r"((post|report).*summary.*to.*PR)|(to PR/issue)|(PR/issue)")?,

            // issue drafts (legacy filename)
            issue_drafts: Regex::new(r"issue_drafts\.md")?,

            // grep for REG markers
            grep_reg_marker: Regex::new(r"grep.*\^### REG-\[0-9\]\{3\}:")?,

            // Bespoke pipeline patterns (should use demoswarm.sh shim)
            // Matches common shell pipelines for counting/extracting in cleanup agents
            bespoke_pipeline: Regex::new(
                r#"(grep\s+-[cEoP]|grep.*\|.*wc|sed\s+-[nEe]|awk\s+['"]|jq\s+['".])"#,
            )?,

            // Skill ownership patterns (match actual demoswarm.sh invocations)
            index_upsert_cmd: Regex::new(r"demoswarm\.sh\s+index\s+upsert-status")?,
            secrets_cmd: Regex::new(r"demoswarm\.sh\s+secrets\s+(scan|redact)")?,
            openq_cmd: Regex::new(r"demoswarm\.sh\s+openq\s+(next-id|append)")?,

            // Shim enforcement patterns
            shim_line_continuation: Regex::new(r"demoswarm\.sh\s*\\$")?,
            direct_demoswarm_invocation: Regex::new(
                r"(^|\s)demoswarm\s+(count|ms|yaml|inv|line|receipt|receipts|openapi|time|index|openq|secrets)",
            )?,

            // Structured extraction calls (cross-agent contract checks)
            ms_get_invocation: dotall(
                r###"(?m)^\s*bash\s+\.claude/scripts/demoswarm\.sh\s+ms\s+get.*?--file\s+"([^"]+)".*?--section\s+"## Machine Summary".*?--key\s+"([^"]+)""###,
            )?,
            inv_get_invocation: dotall(
                r###"(?m)^\s*bash\s+\.claude/scripts/demoswarm\.sh\s+inv\s+get.*?--file\s+"([^"]+)".*?--marker\s+"([^"]+)""###,
            )?,

            // Boundary check patterns (checks 45-47)
            // Check 45: Skill names that should not appear in flow commands
            skill_names_in_prose: Regex::new(
                r"\b(runs-derive|runs-index|openq-tools|secrets-tools|test-runner|auto-linter|policy-runner)\b",
            )?,
            // Check 45: CLI shim reference
            demoswarm_shim_ref: Regex::new(r"demoswarm\.sh")?,
            // Check 47: Flow output arrows (agent -> file or agent -> .runs/)
            flow_output_arrow: Regex::new(
                r"(agent|cleanup|author|critic|analyzer|designer|planner)\s*[-→>]+\s*\.?runs/",
            )?,

            // GH body hygiene (check 50): heredoc pattern for safe body passing
            // Matches: -f body="$(cat <<'EOF' or --body "$(cat <<'EOF'
            gh_heredoc_pattern: Regex::new(r#"(-f\s+body=|--body\s+)"\$\(cat\s+<<'EOF'"#)?,
        })
    }
}

/// Sentinel markers for contract blocks.
pub mod sentinels {
    pub const GATE_RESULT_START: &str = "PACK-CONTRACT: GATE_RESULT_V1 START";
    pub const GATE_RESULT_END: &str = "PACK-CONTRACT: GATE_RESULT_V1 END";
}

/// Required agents (must exist in `.claude/agents/`).
pub const REQUIRED_AGENTS: &[&str] = &[
    // Cleanup agents (all 6 flows)
    "signal-cleanup",
    "plan-cleanup",
    "build-cleanup",
    "gate-cleanup",
    "deploy-cleanup",
    "wisdom-cleanup",
    // Prep + infra
    "signal-run-prep",
    "run-prep",
    "repo-operator",
    "secrets-sanitizer",
    // Flow 1 domain
    "gh-researcher",
    "signal-normalizer",
    "problem-framer",
    "clarifier",
    "requirements-author",
    "requirements-critic",
    "bdd-author",
    "bdd-critic",
    "scope-assessor",
    "risk-analyst",
    // Flow 2 domain
    "impact-analyzer",
    "design-optioneer",
    "adr-author",
    "interface-designer",
    "contract-critic",
    "observability-designer",
    "observability-critic",
    "test-strategist",
    "work-planner",
    "design-critic",
    "policy-analyst",
    // Flow 3 domain
    "context-loader",
    "test-author",
    "test-critic",
    "code-implementer",
    "code-critic",
    "mutator",
    "fixer",
    "lint-executor",
    "test-executor",
    "doc-writer",
    "doc-critic",
    "self-reviewer",
    // Flow 4 domain
    "receipt-checker",
    "contract-enforcer",
    "security-scanner",
    "coverage-enforcer",
    "gate-fixer",
    "fix-forward-runner",
    "traceability-auditor",
    "merge-decider",
    // Flow 5 domain
    "deploy-monitor",
    "smoke-verifier",
    "deploy-decider",
    // Flow 6 domain
    "artifact-auditor",
    "regression-analyst",
    "flow-historian",
    "learning-synthesizer",
    "feedback-applier",
    // GitHub integration
    "gh-issue-manager",
    "gh-reporter",
];

/// Required skills (must exist in `.claude/skills/<name>/SKILL.md`).
pub const REQUIRED_SKILLS: &[&str] = &[
    "test-runner",
    "auto-linter",
    "policy-runner",
    "runs-derive",
    "runs-index",
    "openq-tools",
    "secrets-tools",
];

/// Cleanup agents with their expected receipt filenames.
pub const CLEANUP_AGENTS: &[(&str, &str)] = &[
    ("signal-cleanup", "signal_receipt.json"),
    ("plan-cleanup", "plan_receipt.json"),
    ("build-cleanup", "build_receipt.json"),
    ("gate-cleanup", "gate_receipt.json"),
    ("deploy-cleanup", "deploy_receipt.json"),
    ("wisdom-cleanup", "wisdom_receipt.json"),
];

/// Critic agents (must have can_further_iteration_help).
pub const CRITICS: &[&str] = &[
    "requirements-critic",
    "bdd-critic",
    "design-critic",
    "contract-critic",
    "observability-critic",
    "code-critic",
    "test-critic",
    "doc-critic",
];

/// Critics and verifiers (must have Machine Summary with canonical axis).
pub const CRITIC_AND_VERIFIER_AGENTS: &[&str] = &[
    "requirements-critic",
    "bdd-critic",
    "design-critic",
    "contract-critic",
    "observability-critic",
    "code-critic",
    "test-critic",
    "doc-critic",
    "contract-enforcer",
    "coverage-enforcer",
    "artifact-auditor",
    "receipt-checker",
    "security-scanner",
    "deploy-monitor",
    "smoke-verifier",
    "traceability-auditor",
    "fix-forward-runner",
];

/// Gate agents (must use unified recommended_action).
pub const GATE_AGENTS: &[&str] = &["contract-enforcer", "coverage-enforcer"];

/// GH agents (must enforce two gates).
pub const GH_AGENTS: &[&str] = &["gh-issue-manager", "gh-reporter"];

/// Banned patterns (removed/deprecated concepts).
pub const BANNED_PATTERNS: &[&str] = &[
    "Flow Studio",
    "harness.py",
    "run-cleanup",
    "profiles/",
    "profile.yaml",
    "orchestrator.py",
    "swarm_runtime",
];

/// Sealing sequence markers (flow commands should reference all).
pub const SEALING_MARKERS: &[&str] = &[
    "cleanup",
    "secrets-sanitizer",
    "repo-operator",
    "gh-issue-manager",
    "gh-reporter",
];

/// Required Gate Result fields.
pub const GATE_RESULT_FIELDS: &[&str] = &[
    "safe_to_commit",
    "safe_to_publish",
    "modified_files",
    "needs_upstream_fix",
    "route_to_agent",
    "route_to_flow",
    "recommended_action",
];

/// Required Repo Operator Result fields.
pub const REPO_OPERATOR_RESULT_FIELDS: &[&str] = &[
    "operation:",
    "status:",
    "proceed_to_github_ops:",
    "commit_sha:",
    "publish_surface:",
    "anomaly_paths:",
];

/// CLAUDE.md key sections.
pub const CLAUDE_MD_SECTIONS: &[&str] = &[
    ".runs/<run-id>",
    "run_meta.json",
    "index.json",
    "Six Flows",
    "Receipt",
    "secrets-sanitizer",
];

/// Flow-specific action patterns (should not exist).
pub const FLOW_SPECIFIC_PATTERNS: &[&str] =
    &["RERUN_FLOW_", "BOUNCE_TO_", "BOUNCE_BUILD", "BOUNCE_PLAN"];

/// Raw git patterns that should not appear in flow commands.
pub const RAW_GIT_PATTERNS: &[(&str, &str)] = &[
    ("git diff --name-only", "git diff --name-only"),
    ("git ls-files --others", "git ls-files --others"),
    ("git add .", "git add \\."),
    ("git reset --hard", "git reset --hard"),
    ("git clean -fd", "git clean -fd"),
];

/// Regression marker literal (heading-based).
pub const REG_MARKER_LITERAL: &str = "^### REG-[0-9]{3}:";

/// Skill ownership: agents allowed to use `index upsert-status`.
pub const INDEX_ALLOWED_AGENTS: &[&str] = &[
    "signal-cleanup",
    "plan-cleanup",
    "build-cleanup",
    "gate-cleanup",
    "deploy-cleanup",
    "wisdom-cleanup",
    "run-prep",
    "signal-run-prep",
];

/// Skill ownership: agents allowed to use `secrets scan` / `secrets redact`.
pub const SECRETS_ALLOWED_AGENTS: &[&str] = &["secrets-sanitizer"];

/// Skill ownership: agents allowed to use `openq next-id` / `openq append`.
pub const OPENQ_ALLOWED_AGENTS: &[&str] = &["clarifier"];

/// GitHub-posting agents that must follow GH body hygiene rules.
pub const GH_POSTING_AGENTS: &[&str] = &["gh-reporter", "gh-issue-manager", "gh-issue-resolver"];

/// Dangerous patterns that must NOT appear in GH agent body handling.
/// These patterns indicate temp files, absolute paths, or placeholders that will fail.
pub const GH_BODY_FORBIDDEN_PATTERNS: &[&str] = &[
    "--body-file",          // Temp file paths break on Windows
    "@/",                   // File reference that will fail
    "C:\\",                 // Windows absolute path
    "C:/",                  // Windows absolute path (forward slash)
    "/tmp/",                // Unix temp directory
    "/var/",                // Unix system directory
    "/home/",               // Unix home directory
    "/Users/",              // macOS home directory
    "AppData\\Local\\Temp", // Windows temp directory
    "<updated_body>",       // Placeholder that should be replaced
    "comment content here", // Template placeholder
];

/// Skill CLI subcommands that should NOT appear in flow commands (check 52).
/// Flow commands delegate to agents; agents use skills. Flow commands should not
/// contain direct skill-layer CLI syntax.
pub const SKILL_CLI_SUBCOMMANDS: &[&str] = &[
    "count", "ms", "yaml", "index", "receipt", "receipts", "openapi", "line", "inv", "time",
    "openq", "secrets",
];

/// Canonical OpenQ flow codes (check 53).
/// QID format: OQ-<FLOW>-<NNN> where FLOW is one of these.
pub const OPENQ_FLOW_CODES: &[&str] = &[
    "SIG", // Signal (Flow 1)
    "PLN", // Plan (Flow 2)
    "BLD", // Build (Flow 3)
    "GAT", // Gate (Flow 4)
    "DEP", // Deploy (Flow 5)
    "WIS", // Wisdom (Flow 6)
];
