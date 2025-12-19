use std::path::{Path, PathBuf};

use regex::Regex;
use walkdir::WalkDir;

use crate::ctx::Ctx;

#[derive(Debug, Clone)]
pub struct LineMatch {
    pub path: PathBuf,
    pub line_no: usize, // 1-based
    pub line: String,
}

pub fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(&needle.to_ascii_lowercase())
}

pub fn has_exact_line(content: &str, exact: &str) -> bool {
    content.lines().any(|l| l.trim_end() == exact)
}

pub fn has_line_starting_with(content: &str, prefix: &str) -> bool {
    content.lines().any(|l| l.starts_with(prefix))
}

pub fn find_matches_regex_recursive(
    ctx: &Ctx,
    roots: &[PathBuf],
    re: &Regex,
    ignore_file_names: &[&str],
) -> anyhow::Result<Vec<LineMatch>> {
    let mut out = Vec::new();

    for root in roots {
        if !root.exists() {
            continue;
        }

        for entry in WalkDir::new(root).follow_links(false) {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if should_ignore(path, ignore_file_names) {
                continue;
            }

            // Best-effort: skip unreadable/non-UTF8 files.
            let content = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(_) => continue,
            };

            for (i, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    out.push(LineMatch {
                        path: path.to_path_buf(),
                        line_no: i + 1,
                        line: line.to_string(),
                    });
                }
            }
        }
    }

    // Stable ordering (roughly grep-like)
    out.sort_by(|a, b| {
        ctx.rel(&a.path)
            .cmp(&ctx.rel(&b.path))
            .then(a.line_no.cmp(&b.line_no))
    });

    Ok(out)
}

pub fn find_files_containing_recursive(
    ctx: &Ctx,
    root: &Path,
    needle: &str,
    ignore_file_names: &[&str],
) -> anyhow::Result<Vec<PathBuf>> {
    let mut hits = Vec::new();

    if !root.exists() {
        return Ok(hits);
    }

    for entry in WalkDir::new(root).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if should_ignore(path, ignore_file_names) {
            continue;
        }

        let content = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        if content.contains(needle) {
            hits.push(path.to_path_buf());
        }
    }

    hits.sort_by_key(|a| ctx.rel(a));
    Ok(hits)
}

pub fn extract_frontmatter_name(content: &str) -> Option<String> {
    // Mimics:
    //   sed -n '/^---$/,/^---$/p' file | grep '^name:' | head -1

    let mut lines = content.lines();

    // Find first '---' line
    for line in lines.by_ref() {
        if line.trim_end() == "---" {
            break;
        }
    }

    for line in lines {
        if line.trim_end() == "---" {
            break;
        }
        if let Some(rest) = line.strip_prefix("name:") {
            let name = rest.trim().to_string();
            if !name.is_empty() {
                return Some(name);
            }
            break;
        }
    }

    None
}

fn should_ignore(path: &Path, ignore_file_names: &[&str]) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    ignore_file_names.contains(&name)
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // contains_ignore_ascii_case tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_contains_ignore_case_exact_match() {
        assert!(contains_ignore_ascii_case("Hello World", "Hello World"));
    }

    #[test]
    fn test_contains_ignore_case_lower() {
        assert!(contains_ignore_ascii_case("HELLO WORLD", "hello world"));
    }

    #[test]
    fn test_contains_ignore_case_upper() {
        assert!(contains_ignore_ascii_case("hello world", "HELLO WORLD"));
    }

    #[test]
    fn test_contains_ignore_case_mixed() {
        assert!(contains_ignore_ascii_case("HeLLo WoRLD", "hElLO wOrLd"));
    }

    #[test]
    fn test_contains_ignore_case_substring() {
        assert!(contains_ignore_ascii_case("The quick brown fox", "QUICK"));
    }

    #[test]
    fn test_contains_ignore_case_not_found() {
        assert!(!contains_ignore_ascii_case("Hello World", "Goodbye"));
    }

    #[test]
    fn test_contains_ignore_case_empty_needle() {
        assert!(contains_ignore_ascii_case("Hello World", ""));
    }

    #[test]
    fn test_contains_ignore_case_empty_haystack() {
        assert!(!contains_ignore_ascii_case("", "Hello"));
    }

    #[test]
    fn test_contains_ignore_case_both_empty() {
        assert!(contains_ignore_ascii_case("", ""));
    }

    // -------------------------------------------------------------------------
    // has_exact_line tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_has_exact_line_found() {
        let content = "line1\nline2\nline3";
        assert!(has_exact_line(content, "line2"));
    }

    #[test]
    fn test_has_exact_line_not_found() {
        let content = "line1\nline2\nline3";
        assert!(!has_exact_line(content, "line4"));
    }

    #[test]
    fn test_has_exact_line_with_trailing_whitespace() {
        let content = "line1  \nline2\nline3\t";
        assert!(has_exact_line(content, "line1"));
        assert!(has_exact_line(content, "line3"));
    }

    #[test]
    fn test_has_exact_line_partial_match_fails() {
        let content = "line123\nline2\nline3";
        assert!(!has_exact_line(content, "line1"));
    }

    #[test]
    fn test_has_exact_line_empty_content() {
        assert!(!has_exact_line("", "line"));
    }

    #[test]
    fn test_has_exact_line_empty_target() {
        let content = "line1\n\nline3";
        assert!(has_exact_line(content, ""));
    }

    #[test]
    fn test_has_exact_line_single_line() {
        assert!(has_exact_line("only line", "only line"));
    }

    #[test]
    fn test_has_exact_line_first_line() {
        let content = "target\nother\nstuff";
        assert!(has_exact_line(content, "target"));
    }

    #[test]
    fn test_has_exact_line_last_line() {
        let content = "other\nstuff\ntarget";
        assert!(has_exact_line(content, "target"));
    }

    // -------------------------------------------------------------------------
    // has_line_starting_with tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_has_line_starting_with_found() {
        let content = "## Header\nsome content\n## Another";
        assert!(has_line_starting_with(content, "## Header"));
    }

    #[test]
    fn test_has_line_starting_with_not_found() {
        let content = "## Header\nsome content";
        assert!(!has_line_starting_with(content, "## Missing"));
    }

    #[test]
    fn test_has_line_starting_with_prefix_only() {
        let content = "## Machine Summary\nstatus: VERIFIED";
        assert!(has_line_starting_with(content, "## Machine"));
    }

    #[test]
    fn test_has_line_starting_with_not_at_start() {
        let content = "prefix ## Machine Summary";
        assert!(!has_line_starting_with(content, "## Machine"));
    }

    #[test]
    fn test_has_line_starting_with_empty_prefix() {
        let content = "any content";
        assert!(has_line_starting_with(content, ""));
    }

    #[test]
    fn test_has_line_starting_with_empty_content() {
        assert!(!has_line_starting_with("", "prefix"));
    }

    #[test]
    fn test_has_line_starting_with_multiline() {
        let content = "line1\nline2\nline3";
        assert!(has_line_starting_with(content, "line"));
    }

    // -------------------------------------------------------------------------
    // extract_frontmatter_name tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_extract_frontmatter_basic() {
        let content = "---\nname: test-agent\n---\n# Content";
        assert_eq!(
            extract_frontmatter_name(content),
            Some("test-agent".to_string())
        );
    }

    #[test]
    fn test_extract_frontmatter_with_other_fields() {
        let content = "---\ndescription: Some description\nname: my-agent\nversion: 1.0\n---";
        assert_eq!(
            extract_frontmatter_name(content),
            Some("my-agent".to_string())
        );
    }

    #[test]
    fn test_extract_frontmatter_missing_name() {
        let content = "---\ndescription: Some description\n---";
        assert_eq!(extract_frontmatter_name(content), None);
    }

    #[test]
    fn test_extract_frontmatter_empty_name() {
        let content = "---\nname:\n---";
        assert_eq!(extract_frontmatter_name(content), None);
    }

    #[test]
    fn test_extract_frontmatter_whitespace_name() {
        let content = "---\nname:   \n---";
        assert_eq!(extract_frontmatter_name(content), None);
    }

    #[test]
    fn test_extract_frontmatter_no_frontmatter() {
        let content = "# Just a markdown file\n\nNo frontmatter here.";
        assert_eq!(extract_frontmatter_name(content), None);
    }

    #[test]
    fn test_extract_frontmatter_unclosed() {
        let content = "---\nname: unclosed\nNo closing delimiter";
        assert_eq!(
            extract_frontmatter_name(content),
            Some("unclosed".to_string())
        );
    }

    #[test]
    fn test_extract_frontmatter_name_with_spaces() {
        let content = "---\nname:   spaced-name  \n---";
        assert_eq!(
            extract_frontmatter_name(content),
            Some("spaced-name".to_string())
        );
    }

    // -------------------------------------------------------------------------
    // should_ignore tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_should_ignore_match() {
        let path = Path::new("/some/path/CLAUDE.md");
        assert!(should_ignore(path, &["CLAUDE.md", "README.md"]));
    }

    #[test]
    fn test_should_ignore_no_match() {
        let path = Path::new("/some/path/other.md");
        assert!(!should_ignore(path, &["CLAUDE.md", "README.md"]));
    }

    #[test]
    fn test_should_ignore_empty_list() {
        let path = Path::new("/some/path/any.md");
        assert!(!should_ignore(path, &[]));
    }

    #[test]
    fn test_should_ignore_nested_path() {
        let path = Path::new("/deep/nested/path/target.md");
        assert!(should_ignore(path, &["target.md"]));
    }

    #[test]
    fn test_should_ignore_case_sensitive() {
        let path = Path::new("/path/CLAUDE.md");
        assert!(!should_ignore(path, &["claude.md"]));
    }

    // -------------------------------------------------------------------------
    // LineMatch struct tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_line_match_creation() {
        let lm = LineMatch {
            path: PathBuf::from("/test/path.md"),
            line_no: 42,
            line: "test content".to_string(),
        };
        assert_eq!(lm.path, PathBuf::from("/test/path.md"));
        assert_eq!(lm.line_no, 42);
        assert_eq!(lm.line, "test content");
    }

    #[test]
    fn test_line_match_clone() {
        let lm = LineMatch {
            path: PathBuf::from("/test.md"),
            line_no: 1,
            line: "line".to_string(),
        };
        let cloned = lm.clone();
        assert_eq!(lm.path, cloned.path);
        assert_eq!(lm.line_no, cloned.line_no);
        assert_eq!(lm.line, cloned.line);
    }

    // -------------------------------------------------------------------------
    // extract_frontmatter_name additional tests (covering lines 141-142, 145)
    // -------------------------------------------------------------------------

    /// Test extract_frontmatter_name with name: followed by only whitespace.
    /// Covers lines 141-142: name value is trimmed and checked for empty.
    #[test]
    fn test_extract_frontmatter_name_whitespace_only_value() {
        let content = "---\nname:     \t\n---";
        assert_eq!(extract_frontmatter_name(content), None);
    }

    /// Test extract_frontmatter_name returns None after empty name value.
    /// Covers line 145: break after finding name: with empty value.
    #[test]
    fn test_extract_frontmatter_name_empty_then_breaks() {
        // When name: has empty value, it should break out, not continue
        let content = "---\nname:\ndescription: test\n---";
        assert_eq!(extract_frontmatter_name(content), None);
    }

    /// Test extract_frontmatter_name with content before first delimiter.
    #[test]
    fn test_extract_frontmatter_name_content_before_delimiter() {
        let content = "Some preamble text\n---\nname: valid-name\n---";
        assert_eq!(
            extract_frontmatter_name(content),
            Some("valid-name".to_string())
        );
    }

    /// Test extract_frontmatter_name with trailing content after name.
    #[test]
    fn test_extract_frontmatter_name_with_trailing_content() {
        let content = "---\nname: agent-name  # comment\n---";
        assert_eq!(
            extract_frontmatter_name(content),
            Some("agent-name  # comment".to_string())
        );
    }

    /// Test extract_frontmatter_name where name appears outside frontmatter.
    #[test]
    fn test_extract_frontmatter_name_outside_frontmatter() {
        let content = "---\ntitle: My Doc\n---\n\nname: not-in-frontmatter";
        // Should return None because name: isn't between the --- delimiters
        assert_eq!(extract_frontmatter_name(content), None);
    }

    // -------------------------------------------------------------------------
    // find_files_containing_recursive additional tests (covering lines 93, 98, 101-102, 110, 114-115)
    // -------------------------------------------------------------------------

    /// Test find_files_containing_recursive with non-existent root.
    /// Covers line 93: when root doesn't exist.
    #[test]
    fn test_find_files_containing_nonexistent_root() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let nonexistent = PathBuf::from("/definitely/not/a/real/path/abc123");
        let result = find_files_containing_recursive(&ctx, &nonexistent, "needle", &[]).unwrap();

        assert!(result.is_empty(), "Should return empty vec for nonexistent root");
    }

    /// Test find_files_containing_recursive finds files with matching content.
    /// Covers lines 114-115: content.contains and push.
    #[test]
    fn test_find_files_containing_finds_matches() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        // Create test files
        let search_dir = tmp.path().join("search");
        std::fs::create_dir(&search_dir).unwrap();

        std::fs::write(search_dir.join("has_needle.txt"), "This file has needle in it").unwrap();
        std::fs::write(search_dir.join("no_match.txt"), "This file does not match").unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let result = find_files_containing_recursive(&ctx, &search_dir, "needle", &[]).unwrap();

        assert_eq!(result.len(), 1);
        assert!(result[0].file_name().unwrap().to_str().unwrap().contains("has_needle"));
    }

    /// Test find_files_containing_recursive ignores specified files.
    #[test]
    fn test_find_files_containing_respects_ignore_list() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let search_dir = tmp.path().join("search");
        std::fs::create_dir(&search_dir).unwrap();

        std::fs::write(search_dir.join("include.txt"), "has needle").unwrap();
        std::fs::write(search_dir.join("IGNORE.md"), "has needle").unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let result =
            find_files_containing_recursive(&ctx, &search_dir, "needle", &["IGNORE.md"]).unwrap();

        assert_eq!(result.len(), 1);
        assert!(result[0].file_name().unwrap().to_str().unwrap() == "include.txt");
    }

    /// Test find_files_containing_recursive skips directories.
    /// Covers lines 101-102: path.is_file() check.
    #[test]
    fn test_find_files_containing_skips_directories() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let search_dir = tmp.path().join("search");
        std::fs::create_dir(&search_dir).unwrap();

        // Create a file with needle
        std::fs::write(search_dir.join("file.txt"), "needle").unwrap();
        // Create a subdirectory (can't contain needle directly)
        std::fs::create_dir(search_dir.join("subdir")).unwrap();
        std::fs::write(search_dir.join("subdir").join("nested.txt"), "needle").unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let result = find_files_containing_recursive(&ctx, &search_dir, "needle", &[]).unwrap();

        // Should find files but not count directories as matches
        assert_eq!(result.len(), 2); // file.txt and nested.txt
        for path in &result {
            assert!(path.is_file(), "Should only return files");
        }
    }

    // -------------------------------------------------------------------------
    // find_matches_regex_recursive additional tests (covering line 59)
    // -------------------------------------------------------------------------

    /// Test find_matches_regex_recursive with a root that doesn't exist.
    /// Covers line 38-39: when root doesn't exist.
    #[test]
    fn test_find_matches_regex_nonexistent_root() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let re = Regex::new(r"pattern").unwrap();
        let nonexistent = PathBuf::from("/definitely/not/real/xyz");

        let result = find_matches_regex_recursive(&ctx, &[nonexistent], &re, &[]).unwrap();

        assert!(result.is_empty(), "Should return empty for nonexistent roots");
    }

    /// Test find_matches_regex_recursive finds matching lines.
    #[test]
    fn test_find_matches_regex_finds_lines() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let search_dir = tmp.path().join("search");
        std::fs::create_dir(&search_dir).unwrap();

        std::fs::write(
            search_dir.join("test.txt"),
            "line1\nmatches HERE\nline3\nmore HERE stuff\n",
        )
        .unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let re = Regex::new(r"HERE").unwrap();
        let result = find_matches_regex_recursive(&ctx, &[search_dir], &re, &[]).unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].line_no, 2);
        assert_eq!(result[1].line_no, 4);
    }

    /// Test find_matches_regex_recursive respects ignore list.
    #[test]
    fn test_find_matches_regex_respects_ignore() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let search_dir = tmp.path().join("search");
        std::fs::create_dir(&search_dir).unwrap();

        std::fs::write(search_dir.join("include.txt"), "pattern match").unwrap();
        std::fs::write(search_dir.join("IGNORE.md"), "pattern match").unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let re = Regex::new(r"pattern").unwrap();
        let result =
            find_matches_regex_recursive(&ctx, &[search_dir], &re, &["IGNORE.md"]).unwrap();

        assert_eq!(result.len(), 1);
        assert!(result[0].path.file_name().unwrap().to_str().unwrap() == "include.txt");
    }

    /// Test find_matches_regex_recursive handles multiple roots.
    #[test]
    fn test_find_matches_regex_multiple_roots() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let dir1 = tmp.path().join("dir1");
        let dir2 = tmp.path().join("dir2");
        std::fs::create_dir(&dir1).unwrap();
        std::fs::create_dir(&dir2).unwrap();

        std::fs::write(dir1.join("file1.txt"), "pattern").unwrap();
        std::fs::write(dir2.join("file2.txt"), "pattern").unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let re = Regex::new(r"pattern").unwrap();
        let result = find_matches_regex_recursive(&ctx, &[dir1, dir2], &re, &[]).unwrap();

        assert_eq!(result.len(), 2);
    }

    /// Test find_matches_regex_recursive sorts results.
    #[test]
    fn test_find_matches_regex_sorted_results() {
        use tempfile::TempDir;

        let tmp = TempDir::new().unwrap();
        let claude_dir = tmp.path().join(".claude");
        std::fs::create_dir(&claude_dir).unwrap();

        let search_dir = tmp.path().join("search");
        std::fs::create_dir(&search_dir).unwrap();

        // Create files in reverse alphabetical order
        std::fs::write(search_dir.join("z_file.txt"), "pattern").unwrap();
        std::fs::write(search_dir.join("a_file.txt"), "pattern").unwrap();

        let ctx = Ctx::discover(Some(tmp.path().to_path_buf())).unwrap();

        let re = Regex::new(r"pattern").unwrap();
        let result = find_matches_regex_recursive(&ctx, &[search_dir], &re, &[]).unwrap();

        assert_eq!(result.len(), 2);
        // Results should be sorted by path
        let path0 = ctx.rel(&result[0].path);
        let path1 = ctx.rel(&result[1].path);
        assert!(path0 < path1, "Results should be sorted: {} < {}", path0, path1);
    }
}
