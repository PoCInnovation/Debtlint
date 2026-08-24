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
