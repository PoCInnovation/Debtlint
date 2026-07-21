#[derive(Debug)]
pub struct Position {
    pub line: u32,
    pub character: u32 
}

#[derive(Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position
}

#[derive(Debug)]
pub struct Diagnostic {
    pub source: String,
    pub severity: String,
    pub code: u16,
    pub ranges: Vec<Range>,
    pub code_description: String
}