use std::path::Path;
use crate::in_out::load_vocabulary;
use crate::tokenizer::{encode_corpus, train_corpus, BpeTrainingResult, SourceFile};

pub struct BpeConfig {
    pub vocab_size: u32,
    pub min_frequency: usize,
}

/// Train BPE from scratch, or encode with a pre-loaded vocabulary file.
pub fn run_bpe(files: &[SourceFile], config: BpeConfig, load_vocab: Option<&Path>,)
-> std::io::Result<BpeTrainingResult>
{
    if let Some(path) = load_vocab {
        let vocabulary = load_vocabulary(path)?;
        Ok(encode_corpus(files, vocabulary))
    } else {
        Ok(train_corpus(
            files,
            config.vocab_size,
            config.min_frequency,
        ))
    }
}
