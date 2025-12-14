# Pull Request

## Description

<!-- Brief description of what this PR does -->

---

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Agent/command/skill change

---

## Checklist

### For all PRs:

- [ ] My changes follow the style guidelines of this project
- [ ] I have performed a self-review of my changes
- [ ] I have updated documentation if needed

### For agent changes:

- [ ] Pack validation passes (`bash .claude/scripts/pack-check.sh`)
- [ ] Color matches role family (see CONTRIBUTING.md)
- [ ] `name:` field matches filename

### For command changes:

- [ ] Command tested in a sandbox repo
- [ ] Documentation updated if behavior changed

### For skill changes:

- [ ] SKILL.md updated with any new behavior
- [ ] Tested with relevant agents

---

## Related Issues

<!-- Link to GitHub issues this PR addresses -->

Closes #<issue-number>

---

## Additional Context

<!-- Add any other context about the PR here -->

---

## Quick Links

- [CLAUDE.md](../CLAUDE.md) — Pack reference
- [CONTRIBUTING.md](../CONTRIBUTING.md) — Contribution guide
- [docs/CUSTOMIZATION.md](../docs/CUSTOMIZATION.md) — Stack adaptation
