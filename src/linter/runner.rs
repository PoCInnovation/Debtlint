use super::diagnostic::{Diagnostic, Position, Range};

pub fn get_duplicated() -> Vec<Diagnostic> {
    vec![
        Diagnostic {
            code: 1,
            severity: String::from("low"),
            ranges: vec![
                Range {
                    source: "src/linter/runner.rs".to_string(),
                    start: Position {
                        line: 10,
                        character: 1,
                    },
                    end: Position {
                        line: 19,
                        character: 1,
                    },
                },
                Range {
                    source: "src/debug_run.rs".to_string(),
                    start: Position {
                        line: 10,
                        character: 1,
                    },
                    end: Position {
                        line: 16,
                        character: 1,
                    },
                },
            ],
            description: String::from("This code can be refactorised."),
        },
        Diagnostic {
            code: 2,
            severity: String::from("high"),
            ranges: vec![
                Range {
                    source: "src/tokenizer/decode.rs".to_string(),
                    start: Position {
                        line: 10,
                        character: 1,
                    },
                    end: Position {
                        line: 19,
                        character: 1,
                    },
                },
                Range {
                    source: "src/debug_run.rs".to_string(),
                    start: Position {
                        line: 10,
                        character: 1,
                    },
                    end: Position {
                        line: 16,
                        character: 1,
                    },
                },
            ],
            description: String::from("This code is duplicated."),
        },
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
