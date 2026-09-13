use crate::in_out::load_vocabulary;
use crate::tokenizer::{BpeTrainingResult, SourceFile, encode_corpus, train_corpus};
use std::path::Path;

pub struct BpeConfig {
    pub vocab_size: u32,
    pub min_frequency: usize,
}

/// Train BPE from scratch, or encode with a pre-loaded vocabulary file.
pub fn run_bpe(
    files: &[SourceFile],
    config: BpeConfig,
    load_vocab: Option<&Path>,
) -> std::io::Result<BpeTrainingResult> {
    if let Some(path) = load_vocab {
        let vocabulary = load_vocabulary(path)?;
        Ok(encode_corpus(files, vocabulary))
    } else {
        Ok(train_corpus(files, config.vocab_size, config.min_frequency))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::in_out::save_vocabulary;
    use crate::tokenizer::SourceFile;

    #[test]
    fn load_same_as_train() {
        let files = vec![SourceFile {
            path: PathBuf::from("fixtures/sample.rs"),
            content: include_str!("../fixtures/sample.rs").to_string(),
        }];
        let config = BpeConfig {
            vocab_size: 500,
            min_frequency: 2,
        };

        let trained = run_bpe(
            &files,
            BpeConfig {
                vocab_size: config.vocab_size,
                min_frequency: config.min_frequency,
            },
            None,
        )
        .unwrap();

        let path = std::env::temp_dir().join("debtlint_pipeline_test.vocab.json");
        save_vocabulary(&path, &trained.vocabulary).unwrap();
        let loaded = run_bpe(&files, config, Some(&path)).unwrap();
        std::fs::remove_file(&path).ok();

        assert_eq!(loaded.merges, trained.merges);
        assert_eq!(loaded.files.len(), trained.files.len());
        assert_eq!(
            loaded.files[0].sequence,
            trained.files[0].sequence
        );
    }
}
