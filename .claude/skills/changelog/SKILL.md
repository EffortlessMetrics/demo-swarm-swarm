# Changelog Skill

Generate changelog entries from git history.

## Gather History

```bash
# Find last version tag
git describe --tags --abbrev=0

# Get commits since last tag
git log --oneline $(git describe --tags --abbrev=0)..HEAD

# Get detailed commits with bodies
git log --format="%h %s%n%b" $(git describe --tags --abbrev=0)..HEAD
```

## Categorize Changes

Group commits into Keep a Changelog categories:

| Category | Description | Commit Patterns |
|----------|-------------|-----------------|
| **Added** | New features | `feat:`, `add:`, "add", "new" |
| **Changed** | Changes to existing functionality | `refactor:`, `change:`, "update" |
| **Deprecated** | Soon-to-be removed features | `deprecate:` |
| **Removed** | Removed features | `remove:`, "delete" |
| **Fixed** | Bug fixes | `fix:`, "bug", "patch" |
| **Security** | Vulnerability fixes | `security:`, "CVE" |

## Changelog Format

```markdown
## [Unreleased]

### Added
- Description of new feature (#PR)

### Changed
- Description of change (#PR)

### Fixed
- Description of fix (#PR)
```

## Cross-Reference

When available, link to:
- PR/MR numbers
- Issue numbers
- Commit hashes (abbreviated)

## Workflow

1. **Gather** - Get all commits since last tag
2. **Categorize** - Sort into changelog categories
3. **Deduplicate** - Merge related commits into single entries
4. **Humanize** - Rewrite commit messages for users, not developers
5. **Link** - Add PR/issue references
6. **Insert** - Add to CHANGELOG.md under `[Unreleased]`

## Output Format

```markdown
## Changelog Update

### Commits Analyzed
<count> commits since <last-tag>

### Categorized Changes

#### Added
- <entry>

#### Changed
- <entry>

#### Fixed
- <entry>

### Skipped
- <merge commits, version bumps, etc.>
```

## Rules
- Does not bump versions, create tags, or release
- Conventional commit format aligns with the `/commit` skill
- User-facing language, not developer jargon
- One line per logical change (merge related commits)
- Always include reference links when available
- Skip merge commits and CI-only changes
- Match existing CHANGELOG.md style
