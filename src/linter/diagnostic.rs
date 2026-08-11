use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Serialize, Debug)]
pub struct Range {
    pub source: String,
    pub start: Position,
    pub end: Position,
}

#[derive(Serialize, Debug)]
pub struct Diagnostic {
    pub severity: String,
    pub code: u16,
    pub ranges: Vec<Range>,
    pub description: String,
}
