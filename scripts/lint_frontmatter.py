#!/usr/bin/env python3
"""
Lint frontmatter in .claude/ agents, commands, and skills.

Checks:
- Agents: require name + description in frontmatter
- Commands: require description in frontmatter
- Skills: require name + description in SKILL.md

Exit codes:
  0 - All checks pass
  1 - Found issues
"""

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CLAUDE_DIR = ROOT / ".claude"


def extract_frontmatter(path: Path) -> list[str]:
    """Extract YAML frontmatter lines from a markdown file."""
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines()
    if not lines or not lines[0].strip().startswith("---"):
        raise ValueError("missing starting ---")
    try:
        end = next(
            idx for idx, line in enumerate(lines[1:], start=1)
            if line.strip().startswith("---")
        )
    except StopIteration:
        raise ValueError("missing closing ---")
    return lines[1:end]


def has_key(frontmatter: list[str], key: str) -> bool:
    """Check if frontmatter contains a key."""
    prefix = f"{key}:"
    return any(line.lstrip().startswith(prefix) for line in frontmatter)


def main() -> int:
    if not CLAUDE_DIR.exists():
        print("ERROR: .claude/ directory not found at repo root", file=sys.stderr)
        return 1

    errors: list[str] = []

    # Agents: require name + description
    agents_dir = CLAUDE_DIR / "agents"
    if agents_dir.exists():
        for md in agents_dir.glob("*.md"):
            try:
                fm = extract_frontmatter(md)
            except ValueError as e:
                errors.append(f"{md.relative_to(ROOT)}: invalid frontmatter ({e})")
                continue

            if not has_key(fm, "name"):
                errors.append(f"{md.relative_to(ROOT)}: missing 'name:' in frontmatter")
            if not has_key(fm, "description"):
                errors.append(f"{md.relative_to(ROOT)}: missing 'description:' in frontmatter")

    # Commands: require description
    commands_dir = CLAUDE_DIR / "commands"
    if commands_dir.exists():
        for md in commands_dir.glob("*.md"):
            try:
                fm = extract_frontmatter(md)
            except ValueError as e:
                errors.append(f"{md.relative_to(ROOT)}: invalid frontmatter ({e})")
                continue

            if not has_key(fm, "description"):
                errors.append(f"{md.relative_to(ROOT)}: missing 'description:' in frontmatter")

    # Skills: require name + description in SKILL.md
    skills_dir = CLAUDE_DIR / "skills"
    if skills_dir.exists():
        for skill_subdir in skills_dir.iterdir():
            if not skill_subdir.is_dir():
                continue
            skill_md = skill_subdir / "SKILL.md"
            if not skill_md.exists():
                errors.append(f"{skill_subdir.relative_to(ROOT)}: missing SKILL.md")
                continue

            try:
                fm = extract_frontmatter(skill_md)
            except ValueError as e:
                errors.append(f"{skill_md.relative_to(ROOT)}: invalid frontmatter ({e})")
                continue

            if not has_key(fm, "name"):
                errors.append(f"{skill_md.relative_to(ROOT)}: missing 'name:' in frontmatter")
            if not has_key(fm, "description"):
                errors.append(f"{skill_md.relative_to(ROOT)}: missing 'description:' in frontmatter")

    if errors:
        print("Frontmatter lint failed:", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    print("Frontmatter lint passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
