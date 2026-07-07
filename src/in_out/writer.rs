use std::fs;
use std::path::Path;

use crate::tokenizer::Token;

/// write encoded sequence as json
pub fn write_encoded_sequence_json(path: &Path, sequence: &[Token]) -> std::io::Result<()> {
    let body = sequence
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    fs::write(path, format!("[{body}]"))
}
