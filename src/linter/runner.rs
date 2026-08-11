use super::diagnostic::{Diagnostic, Position, Range};

pub fn get_duplicated() -> Vec<Diagnostic> {
    vec![
        Diagnostic {
            code: 1,
            severity: String::from("low"),
            ranges: vec![
                Range {
                    source: "src/Core.cpp".to_string(),
                    start: Position {
                        line: 50,
                        character: 1,
                    },
                    end: Position {
                        line: 54,
                        character: 1,
                    },
                },
                Range {
                    source: "src/Kitchen.cpp".to_string(),
                    start: Position {
                        line: 31,
                        character: 1,
                    },
                    end: Position {
                        line: 64,
                        character: 1,
                    },
                },
            ],
            description: String::from("Duplicate"),
        },
        Diagnostic {
            code: 2,
            severity: String::from("high"),
            ranges: vec![
                Range {
                    source: "src/Pizza.cpp".to_string(),
                    start: Position {
                        line: 3,
                        character: 1,
                    },
                    end: Position {
                        line: 8,
                        character: 1,
                    },
                },
            ],
            description: String::from("Duplicate code"),
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
