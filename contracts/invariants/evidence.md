# Evidence and lifecycle invariants

1. **Missing is not zero.** An absent or unreadable measurement remains unknown.
2. **Completion and verification are different axes.** Work may be complete without
   sufficient mechanical proof, or partially complete with verified checkpoint evidence.
3. **Receipts record what happened.** Agent routing and next-action recommendations
   remain in native handoffs, not durable receipt control fields.
4. **Repository state outranks stale narration.** After the tree moves, evidence must be
   checked against the current repository state or explicitly marked stale.
5. **Publication requires two boundaries.** Content must be safe to publish and the
   operation must be authorized for the intended provider/repository.
6. **Security verification is mechanical.** A scanner that is absent, failed,
   uninterpretable, or not run yields `UNVERIFIED`, never an inferred clean result.
7. **Run identity is stable.** Aliases and provider bindings may accumulate, but native
   adapters must not silently create a second run for the same accepted identity.
8. **The manager is lifecycle infrastructure.** Normal flow execution cannot require the
   `demoswarm` executable, a shared wrapper, or a replacement orchestration daemon.
9. **Project mechanics are project-owned.** Test, lint, format, policy, path, and scanner
   configuration live outside managed canonical prompt/skill content.
10. **Unknown content is preserved.** Install, update, remove, and migration do not infer
    ownership from a path name alone.
