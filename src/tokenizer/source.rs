use std::path::PathBuf;

/// Lines starting with one of these prefixes (after trimming) are dropped as comments.
const COMMENT_PREFIXES: [&str; 2] = ["#", "//"];

pub struct SourceFile {
    pub path: PathBuf,
    /// Normalized text seen by the BPE: one `\n`-terminated line per kept source line.
    pub content: String,
    /// `line_numbers[i]` is the 1-based line number in the original file of the i-th
    /// line of `content` (comment and blank lines are dropped, so numbers can skip).
    pub line_numbers: Vec<usize>,
}

impl SourceFile {
    /// Wrap already-normalized content: the i-th line of `content` is line `i + 1`.
    pub fn new(path: PathBuf, content: String) -> Self {
        let line_numbers = (1..=content.lines().count()).collect();
        Self {
            path,
            content,
            line_numbers,
        }
    }

    /// Build a source file from raw text: drop comment and blank lines, collapse the
    /// whitespace of the others, and remember their original line numbers.
    pub fn from_text(path: PathBuf, text: &str) -> Self {
        let mut content = String::new();
        let mut line_numbers = Vec::new();
        for (index, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || COMMENT_PREFIXES.iter().any(|p| trimmed.starts_with(p)) {
                continue;
            }
            content.push_str(&trimmed.split_whitespace().collect::<Vec<_>>().join(" "));
            content.push('\n');
            line_numbers.push(index + 1);
        }
        Self {
            path,
            content,
            line_numbers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_text_drops_comments_and_blank_lines_and_keeps_numbers() {
        let text = "// header\nfn a() {\n\n    let  x = 1;   \n    # note\n}\n";
        let file = SourceFile::from_text(PathBuf::from("a.rs"), text);
        assert_eq!(file.content, "fn a() {\nlet x = 1;\n}\n");
        assert_eq!(file.line_numbers, vec![2, 4, 6]);
    }

    #[test]
    fn from_text_handles_crlf_and_missing_final_newline() {
        let file = SourceFile::from_text(PathBuf::from("a.rs"), "a\r\nb");
        assert_eq!(file.content, "a\nb\n");
        assert_eq!(file.line_numbers, vec![1, 2]);
    }

    #[test]
    fn from_text_empty_input() {
        let file = SourceFile::from_text(PathBuf::from("a.rs"), "");
        assert!(file.content.is_empty());
        assert!(file.line_numbers.is_empty());
    }

    #[test]
    fn new_numbers_lines_from_one() {
        let file = SourceFile::new(PathBuf::from("a.rs"), "x\ny\n".to_string());
        assert_eq!(file.line_numbers, vec![1, 2]);
    }
}
