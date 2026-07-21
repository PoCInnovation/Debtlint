use super::diagnostic::{Diagnostic, Range, Position};

pub fn get_duplicated() -> Diagnostic {
    Diagnostic {
        source: String::from("src/main.rs"),
        severity: String::from("low"),
        code: 1,
        ranges: vec![Range {
            start: Position { line: 8, character: 8 },
            end: Position { line: 8, character: 12 },
        }],
        code_description: String::from("No typing"),
    }
}