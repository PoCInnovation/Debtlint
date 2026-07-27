use super::diagnostic::{Diagnostic, Range, Position};

pub fn get_duplicated() -> Diagnostic {
    Diagnostic {
        code: 1,
        severity: String::from("low"),
        ranges: vec![
            Range {
                start: Position { source: "src/ingestion.rs".to_string(), line: 10, character: 1 },
                end: Position { source: "src/ingestion.rs".to_string(), line: 26, character: 1 },
            },
            Range {
                start: Position { source: "src/pipeline.rs".to_string(), line: 12, character: 1 },
                end: Position { source: "src/pipeline.rs".to_string(), line: 23, character: 1 }
            }
        ],
        code_description: String::from("Duplicate"),
    }
}
