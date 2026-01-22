# Roadmap

> Where DemoSwarm is going and how we get there.

This roadmap reflects the current state of the pack, documented gaps, and the broader vision from the thesis and doctrine. Items are organized by timeline and confidence level.

---

## Near-term (Next Release)

Items that are clearly in progress or address documented gaps.

### Flow and Agent Polish

- **Flow 4/5 Flow Renumbering Alignment** -- Flow 4 was renamed from "Gate" to "Review" and Flow 5 from "Review" to "Gate" in recent refactors. Some documentation references may still use old numbering.

- **TodoWrite Clarity Improvements** -- Per `FLOW_CONSISTENCY_REPORT.md`, Flow 3 and Flow 4 TodoWrite templates could be more explicit about complex scenarios (severity: INFO, polish only).

- **Subtask Status Consistency** -- Several runs in `.runs/` show subtasks with `status: TODO` that may need completion or cleanup. The `context-loader` and `work-planner` agents handle the `TODO | DOING | DONE` lifecycle.

### Tooling Stability

- **Cross-platform Bash Compatibility** -- Some scripts rely on bash 4+ features. macOS default bash (3.x) may have issues. Document workarounds or add version checks in `pack-check.sh`.

- **demoswarm CLI Coverage** -- The Rust CLI in `tools/demoswarm-pack-check/` handles mechanical operations (counting, extraction). Continue migrating from raw shell utilities to the CLI for determinism.

### Documentation Alignment

- **Index vs. Agent Prompt Alignment** -- Per `INDEX_REVIEW_REPORT.md`, some agent index entries miss the primary job description (e.g., standards-enforcer's core mission is catching reward hacking via test deletion).

- **Evidence Freshness Documentation** -- Reference to `evidence-freshness.md` exists in rules but the actual staleness policies could be more explicit in practice.

---

## Medium-term

Features mentioned in docs but not fully implemented, improvements suggested in explanation docs, and known gaps between current state and stated goals.

### Provider Portability

- **GitLab/Azure DevOps/Bitbucket Adapters** -- Currently, `gh-*` agents are GitHub-specific. Per `adapt-to-non-github.md`, forking these for other providers is documented but not implemented:
  - `glab-issue-manager`, `glab-reporter`, `glab-researcher` for GitLab
  - Azure DevOps equivalents using `az boards` CLI
  - Bitbucket equivalents

- **CI/CD Adapter Generalization** -- `deploy-monitor` assumes GitHub Actions. Generalize to query other CI systems (Jenkins, CircleCI, GitLab CI) while preserving the same `verification_report.md` contract.

### Verification Depth

- **Mutation Testing Integration** -- The thesis and quality panel reference mutation scores (target: 95%), but mutation testing tooling (`mutation-auditor` agent exists) is not yet a default part of the verification stack. Make it opt-in initially, then default for high-risk changes.

- **Fuzz Testing Pipeline** -- `fuzz-triager` agent exists but the integration with actual fuzzing infrastructure is not documented. Connect to property-based testing or coverage-guided fuzzing tools.

- **Production Feedback Loop** -- Per `emergent-phenomena.md`, the quality panel should include production signals. Add observability hooks that flow production metrics back into the verification ceiling detection.

### Emergent Phenomena Countermeasures

Per `emergent-phenomena.md`, implement detection and countermeasures for:

- **Trust Decay Detection** -- Track time-to-approval. If trending toward seconds, trigger calibration defect injection. Automate the periodic calibration check.

- **Evidence Drift Tracking** -- Compare receipt SHA to current HEAD. Flag stale references. Implement automatic staleness labels in PR cockpit.

- **Wisdom Overfitting Prevention** -- Track prompt length over time. Implement constraint sunset: after N runs without a constraint triggering, flag for removal or re-justification.

- **Synthetic Dialect Monitoring** -- Track ratio of generated-to-human code. If exceeding 90%, surface warning about mechanical sympathy risk.

### Flywheel Automation

- **Cross-Run Pattern Analysis** -- `pattern-analyst` and `process-analyst` agents exist but cross-run analysis is not automated. Build tooling to aggregate insights across `.runs/` history.

- **Automatic Scent Trail Updates** -- The scent trail (`.runs/_wisdom/latest.md`) should auto-update with the most impactful learnings from recent runs.

- **Feedback Application Automation** -- `feedback-applier` produces diffs but human review is required. For low-risk changes (typo fixes, doc updates), consider auto-apply with audit trail.

### Developer Experience

- **Flow Studio Integration** -- [Flow Studio](https://github.com/EffortlessMetrics/flow-studio-swarm) renders `.runs/` into an operator view. Improve the handoff between pack and studio.

- **Pack Validation Improvements** -- `pack-check.sh` validates structure. Add semantic checks for common misconfigurations (e.g., skill commands that don't match the stack).

- **Onboarding Wizard** -- `/customize-pack` exists but could be more interactive for common stacks (React, Rust, Python, Go).

---

## Long-term Vision

Broader goals from the thesis and doctrine that represent the full realization of the AgOps vision.

### The Glass Cockpit

Per the thesis, the ultimate goal is that a 100,000-line PR is reviewable by monitoring telemetry, not reading diffs:

- **Full Sensor Panel** -- Coverage, mutation score, complexity delta, BDD alignment, secrets scan, and production signals all visible in one view.

- **Anomaly Detection** -- Automatic flagging when sensors disagree or when evidence freshness is suspect.

- **Hotspot Heatmaps** -- Visual representation of where verification is strongest/weakest across the change surface.

### Spec as Source

Per the thesis, the abstraction level is shifting:

- **Intent Artifacts as Primary Version Surface** -- BDD scenarios, ADRs, and requirements become the primary review surface. Implementation is compiled output.

- **Two-Way Sync with Implementation** -- When implementation drifts from spec, either flag it or regenerate. The spec is the source of truth.

- **Semantic Diff** -- Instead of line-by-line code diff, show intent diff (what requirements changed, what BDD scenarios added/removed).

### The Self-Healing Factory

Per the thesis and `the-flywheel.md`:

- **Automatic Prompt Refinement** -- When a pattern of failure is detected, propose and (with approval) apply prompt updates.

- **Calibration Injection** -- Periodically inject known defects to verify the gate catches them. Track calibration health over time.

- **Wisdom Database** -- Persist learnings in a queryable format. New runs can search past learnings for relevant guidance.

### Multi-Agent Parallelism

- **Concurrent Subtask Execution** -- Currently subtasks execute sequentially. For independent subtasks (no `depends_on` overlap), execute in parallel.

- **Agent Specialization by Codebase** -- Over time, agents develop implicit knowledge of specific codebases. Consider caching codebase-specific context.

### The Trust Manufacturing Line

The factory produces trust, not code:

- **Evidence Quality Metrics** -- Track not just whether evidence exists, but how trustworthy it is (freshness, completeness, relevance).

- **Review Time as KPI** -- Measure DevLT (developer lead time) per PR. The goal is O(1) review time regardless of change size.

- **Audit Trail Completeness** -- Every claim in the PR cockpit has a pointer to evidence. No silent assertions.

---

## Community Input Needed

Areas where design decisions benefit from user feedback or where optional features depend on use case.

### Provider Preferences

- **Which Git Providers Matter?** -- GitLab and Azure DevOps adapters are on the roadmap, but prioritization depends on demand. Bitbucket? Gitea? Self-hosted options?

- **CI/CD Ecosystem** -- Beyond GitHub Actions, which CI systems should `deploy-monitor` support first? Jenkins? CircleCI? GitLab CI? ArgoCD?

### Verification Depth Trade-offs

- **Mutation Testing Default** -- Should mutation testing be default or opt-in? It increases confidence but also machine time.

- **Fuzz Testing Scope** -- For which types of changes should fuzz testing be default? Security-sensitive only? All API endpoints?

### Wisdom Flow Preferences

- **Auto-Apply Learnings** -- How much automation is acceptable for applying learnings? Prompt updates? Doc updates? Structural pack changes?

- **Cross-Run Learning Scope** -- Should cross-run learning be per-repo or cross-repo (for teams running the pack on multiple repos)?

### Observability Trade-offs

- **Production Signal Integration** -- What production observability tools should the pack integrate with? Datadog? Grafana? CloudWatch? Generic webhook?

- **Alert Threshold Configuration** -- Should the pack suggest alert thresholds based on observed patterns, or is this always human-configured?

### Accessibility and UX

- **Color-Blind Friendly Defaults** -- The pack should not rely on color alone. Feedback on current UX accessibility would help prioritization.

- **Screen Reader Compatibility** -- For teams with accessibility requirements, feedback on PR cockpit readability with screen readers.

### Advanced Features

- **MCP Integration** -- Should the pack expose MCP (Model Context Protocol) servers for external tool integration?

- **Agent Extensibility** -- How should users add custom agents? Fork-and-modify? Plugin system? Agent marketplace?

---

## What This Roadmap Is Not

This roadmap is not a promise or a schedule. It is a map of the territory:

- **Near-term** items are things we know need doing and have clear paths.
- **Medium-term** items are things we want and know how to do, pending capacity.
- **Long-term** items are the vision toward which we orient.
- **Community Input** items are questions we cannot answer alone.

The pack evolves based on evidence (runs, failures, learnings). This roadmap evolves with it.

---

## See Also

- [docs/explanation/the-thesis.md](docs/explanation/the-thesis.md) -- The full vision
- [docs/explanation/architecture.md](docs/explanation/architecture.md) -- How the pack is built
- [docs/explanation/emergent-phenomena.md](docs/explanation/emergent-phenomena.md) -- What happens at scale
- [docs/explanation/the-flywheel.md](docs/explanation/the-flywheel.md) -- How the system learns
- [docs/explanation/economics.md](docs/explanation/economics.md) -- Why the math works
- [CLAUDE.md](CLAUDE.md) -- The pack contract
