# How-To Guides

Task-oriented: pick the guide that matches the job you need to do.

| Guide | Use it for | Output |
|-------|------------|--------|
| [Configure Claude Code](configure-claude-code.md) | Set up Claude Code permissions and settings | Working Claude Code environment for the pack |
| [Adopt Fork Workflow](adopt-fork-workflow.md) | Set up fork-based swarm development | Fork topology + GitHub PR integration |
| [Run Topology](run-topology.md) | Set up clone-based swarm repo + branch model | Working branch layout for `run/<id>` workflows |
| [Upstream Export](upstream-export.md) | Move swarm work into a human-owned repo | Exported code/tests to origin |
| [Customize Pack](customize-pack.md) | Point tests/lint/policy commands at your stack | Updated `.claude/skills/*` commands and settings |
| [Work Without GitHub](work-without-github.md) | Operate locally with no `gh` | Local-only flow checklist and skip behavior |
| [Adapt to Non-GitHub](adapt-to-non-github.md) | Use GitLab/Azure/Bitbucket | Mapping of GitHub ops to your provider |
| [Create a Flow](create-a-flow.md) | Add a new flow to the pack | New flow command + cleanup agent |
| [Add an Agent](add-an-agent.md) | Add a new agent to existing flows | New agent file + flow integration |
| [Design Agents](design-agents.md) | Write effective agent prompts | Well-structured agent prompt |
| [Decompose Work](decompose-work.md) | Break features into SRP agent tasks | Task graph with clear ownership |
| [Working with Microloops](working-with-microloops.md) | Understand and use writer↔critic patterns | Effective adversarial iteration |
| [Orchestrator Decision Tree](orchestrator-decision-tree.md) | Decide routing as a flow orchestrator | Agent routing decisions |
| [Working with Receipts](working-with-receipts.md) | Read and interpret flow receipts | Understanding flow outputs and routing |
| [Handle Open Questions](handle-open-questions.md) | Resolve uncertainty without stopping flows | DEFAULTED and NEEDS_HUMAN protocols |
| [Review a Swarm PR](review-a-swarm-pr.md) | Decide yes/no on swarm-generated PRs | Bounded review procedure with hotspot selection |
| [Troubleshoot](troubleshoot.md) | Debug pack runs | Checklists for common failures and reseal loops |
| [Failure Recovery](failure-recovery.md) | Recover from failed runs | Nuclear delete/restart procedures |
| [Team Operations](team-operations.md) | Coordinate multi-person swarm usage | Collision avoidance + handoff procedures |
| [Maintain the Pack](maintain-the-pack.md) | Add or modify agents, flows, or skills | Pre-commit checklist for maintainers |
| [Documentation Governance](documentation-governance.md) | Prevent doc drift and duplication | Rules for canonical locations and cross-references |
