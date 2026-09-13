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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::tokenizer::{SourceFile, train_corpus};

    #[test]
    fn save_load_merges() {
        let files = vec![SourceFile {
            path: PathBuf::from("fixtures/sample.rs"),
            content: include_str!("../../fixtures/sample.rs").to_string(),
        }];
        let trained = train_corpus(&files, 500, 2);

        let path = std::env::temp_dir().join("debtlint_vocab_io_test.json");
        save_vocabulary(&path, &trained.vocabulary).unwrap();
        let loaded = load_vocabulary(&path).unwrap();
        std::fs::remove_file(&path).ok();

        assert_eq!(loaded.len(), trained.vocabulary.len());
        assert_eq!(loaded.merge_start_id(), trained.vocabulary.merge_start_id());
        assert_eq!(
            loaded.len() as u32 - loaded.merge_start_id(),
            trained.merges
        );
    }
}
