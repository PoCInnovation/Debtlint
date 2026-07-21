use serde::Serialize;



#[derive(Serialize, Debug)]
pub struct Position {
    pub line: u32,
    pub character: u32 
}

#[derive(Serialize, Debug)]
pub struct Range {
    pub start: Position,
    pub end: Position
}

#[derive(Serialize, Debug)]
pub struct Diagnostic {
    pub source: String,
    pub severity: String,
    pub code: u16,
    pub ranges: Vec<Range>,
    pub code_description: String
}