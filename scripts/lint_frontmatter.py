#!/usr/bin/env python3
"""
Lint frontmatter in .claude/ agents, commands, and skills.

Checks:
- Agents: require name + description in YAML frontmatter
- Commands: require description in YAML frontmatter
- Skills: accept EITHER format:
    a) YAML frontmatter with name + description (tool skills)
    b) H1 heading (# Name) followed by a description paragraph (workflow skills)

Exit codes:
  0 - All checks pass
  1 - Found issues
"""

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CLAUDE_DIR = ROOT / ".claude"


def _read_lines(path: Path) -> list[str]:
    """Read a file's lines, handling BOM (utf-8-sig) for Windows compatibility."""
    text = path.read_text(encoding="utf-8-sig")
    return text.splitlines()


def extract_frontmatter(path: Path) -> list[str]:
    """Extract YAML frontmatter lines from a markdown file."""
    lines = _read_lines(path)
    if not lines or not lines[0].strip().startswith("---"):
        raise ValueError("missing starting ---")
    try:
        end = next(
            idx
            for idx, line in enumerate(lines[1:], start=1)
            if line.strip().startswith("---")
        )
    except StopIteration:
        raise ValueError("missing closing ---")
    return lines[1:end]


def has_key(frontmatter: list[str], key: str) -> bool:
    """Check if frontmatter contains a key."""
    prefix = f"{key}:"
    return any(line.lstrip().startswith(prefix) for line in frontmatter)


def extract_skill_h1_and_description(path: Path) -> tuple[str, str]:
    """Extract H1 title and first description paragraph from a skill file.

    Returns (name, description) or raises ValueError.
    """
    lines = _read_lines(path)

    # Find first non-empty line — must be an H1
    h1_line = None
    h1_idx = -1
    for idx, line in enumerate(lines):
        if line.strip():
            h1_line = line.strip()
            h1_idx = idx
            break

    if h1_line is None or not h1_line.startswith("# "):
        raise ValueError("first non-empty line is not an H1 heading (# Name)")

    name = h1_line[2:].strip()
    if not name:
        raise ValueError("H1 heading is empty")

    # Find the first non-empty line after H1 — that's the description
    description = None
    for line in lines[h1_idx + 1 :]:
        stripped = line.strip()
        if stripped:
            # A heading is not a description
            if stripped.startswith("#"):
                break
            description = stripped
            break

    if not description:
        raise ValueError("no description paragraph after H1 heading")

    return name, description


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
                errors.append(
                    f"{md.relative_to(ROOT)}: missing 'description:' in frontmatter"
                )

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
                errors.append(
                    f"{md.relative_to(ROOT)}: missing 'description:' in frontmatter"
                )

    # Skills: accept frontmatter OR H1+description
    skills_dir = CLAUDE_DIR / "skills"
    if skills_dir.exists():
        for skill_subdir in skills_dir.iterdir():
            if not skill_subdir.is_dir():
                continue
            skill_md = skill_subdir / "SKILL.md"
            if not skill_md.exists():
                errors.append(f"{skill_subdir.relative_to(ROOT)}: missing SKILL.md")
                continue

            rel = skill_md.relative_to(ROOT)

            # Determine format: frontmatter (starts with ---) or H1
            lines = _read_lines(skill_md)
            first_non_empty = ""
            for line in lines:
                if line.strip():
                    first_non_empty = line.strip()
                    break

            if first_non_empty.startswith("---"):
                # Frontmatter format — validate name + description keys
                try:
                    fm = extract_frontmatter(skill_md)
                except ValueError as e:
                    errors.append(f"{rel}: invalid frontmatter ({e})")
                    continue

                if not has_key(fm, "name"):
                    errors.append(f"{rel}: missing 'name:' in frontmatter")
                if not has_key(fm, "description"):
                    errors.append(f"{rel}: missing 'description:' in frontmatter")
            else:
                # H1+description format — validate heading and paragraph
                try:
                    extract_skill_h1_and_description(skill_md)
                except ValueError as e:
                    errors.append(f"{rel}: {e}")

    if errors:
        print("Frontmatter lint failed:", file=sys.stderr)
        for err in errors:
            print(f"  - {err}", file=sys.stderr)
        return 1

    print("Frontmatter lint passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
