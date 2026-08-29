# DemoSwarm v3 shared contracts

This directory defines what DemoSwarm means independently of any one agent host.
Native adapters separately define how Claude Code, Codex, Gemini CLI, OpenClaw, and
future hosts express and execute these contracts.

## Authority

The shared contract owns:

- the seven flows and their order;
- role classes and control boundaries;
- evidence honesty and publication invariants;
- project configuration semantics;
- run identity and receipt schemas;
- support and capability maturity vocabulary.

It does not own host frontmatter, slash-command syntax, plugin layout, subagent
invocation syntax, permission files, or host-specific lifecycle commands.

## Version domains

Manager, pack, adapter, configuration, run, receipt, and capability schemas are
separate version domains. The root `demoswarm-pack.toml` records their compatibility
for one released pack.

## Runtime boundary

Normal Signal → Wisdom execution uses host-native tools and project commands. The
`demoswarm` manager installs, validates, migrates, and inspects the system but is not
required in the normal flow execution path.
