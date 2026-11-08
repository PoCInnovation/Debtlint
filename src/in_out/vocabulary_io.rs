use std::fs;
use std::io::{self, ErrorKind, Result};
use std::path::Path;

use crate::in_out::vocab_cache::VocabularyDocument;
use crate::tokenizer::vocabulary::{Vocabulary, VocabularyExport};

fn json_err(err: serde_json::Error) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, err)
}

pub fn save_vocabulary(path: &Path, vocabulary: &Vocabulary) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&vocabulary.to_export()).map_err(json_err)?;
    fs::write(path, json)
}

/// Load a cached vocab document or legacy export JSON.
pub fn load_vocabulary(path: &Path) -> Result<Vocabulary> {
    let json = fs::read_to_string(path)?;
    if let Ok(document) = serde_json::from_str::<VocabularyDocument>(&json) {
        return document.to_vocabulary();
    }
    let export: VocabularyExport = serde_json::from_str(&json).map_err(json_err)?;
    Vocabulary::from_export(export)
}
