use std::fs;
use std::io::{self, ErrorKind, Result};
use std::path::Path;

use crate::tokenizer::vocabulary::{Vocabulary, VocabularyExport};

// func to handle the json errors
fn json_err(err: serde_json::Error) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, err)
}

pub fn save_vocabulary(path: &Path, vocabulary: &Vocabulary) -> Result<()> {
    let json = serde_json::to_string_pretty(&vocabulary.to_export()).map_err(json_err)?; // serialize the vocabulary to a json string
    fs::write(path, json) // write the json string to the file
}

pub fn load_vocabulary(path: &Path) -> Result<Vocabulary> {
    let json = fs::read_to_string(path)?; // read the json string from the file wirh ? to handle the errors
    let export: VocabularyExport = serde_json::from_str(&json).map_err(json_err)?; // deserialize the json string to a VocabularyExport struct
    Vocabulary::from_export(export) // create a new Vocabulary from the VocabularyExport struct
}
