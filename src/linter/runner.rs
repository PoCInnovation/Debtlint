use super::diagnostic::{Diagnostic, Position, Range};

pub fn get_duplicated() ->  Vec<Diagnostic> {
    vec![
    Diagnostic {
        code: 1,
        severity: String::from("low"),
        ranges: vec![
            Range {
                start: Position {
                    source: "src/linter/runner.rs".to_string(),
                    line: 10,
                    character: 1,
                },
                end: Position {
                    source: "src/linter/runner.rs".to_string(),
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
    },
    Diagnostic {
        code: 2,
        severity: String::from("high"),
        ranges: vec![
            Range {
                start: Position {
                    source: "src/tokenizer/decode.rs".to_string(),
                    line: 10,
                    character: 1,
                },
                end: Position {
                    source: "src/tokenizer/decode.rs".to_string(),
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
        code_description: String::from("Duplicate code"),
    }
    ]
}

pub fn run_linter() -> std::io::Result<()> {
    let diagnostic = get_duplicated();
    match serde_json::to_string(&diagnostic) {
        Ok(data) => {
            println!("{}", data);
            Ok(())
        }
        Err(err) => {
            println!("Error: {}", err);
            Err(std::io::Error::other(err))
        }
    }
}
