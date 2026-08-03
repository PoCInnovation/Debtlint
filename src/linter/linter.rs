use super::diagnostic::{Diagnostic, Position, Range};

pub fn get_duplicated() -> Diagnostic {
    Diagnostic {
        code: 1,
        severity: String::from("low"),
        ranges: vec![
            Range {
                start: Position {
                    source: "src/linter/linter.rs".to_string(),
                    line: 10,
                    character: 1,
                },
                end: Position {
                    source: "src/linter/linter.rs".to_string(),
                    line: 19,
                    character: 1,
                },
            },
            Range {
                start: Position {
                    source: "src/debug_run.rs".to_string(),
                    line: 10,
                    character: 1,
                },
                end: Position {
                    source: "src/debug_run.rs".to_string(),
                    line: 16,
                    character: 1,
                },
            },
        ],
        code_description: String::from("Duplicate"),
    }
}
