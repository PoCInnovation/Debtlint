use crate::in_out::{load_vocabulary, save_vocabulary};
use crate::tokenizer::{BpeTrainingResult, SourceFile, encode_corpus, train_corpus};
use std::path::PathBuf;

pub enum VocabularySource {
    Load(PathBuf), // path of load enum is mandatory because it need do be here when -load-vocab is present.
    Train(Option<PathBuf>), // path of train is optional because we can train the vocab without saving it.
}

pub struct PipelineConfig {
    pub source: VocabularySource,
    pub output_encoded: Option<PathBuf>,
}

pub struct BpeConfig {
    pub vocab_size: u32,
    pub min_frequency: usize,
}

/// Train BPE from scratch, or encode with a pre-loaded vocabulary file.
pub fn run_bpe(
    files: &[SourceFile],
    bpe_config: BpeConfig,
    pipeline_config: PipelineConfig,
) -> std::io::Result<BpeTrainingResult> {
    match pipeline_config.source {
        VocabularySource::Load(path) => {
            let vocabulary = load_vocabulary(&path)?;
            println!("Vocabulary loaded from: {}", path.display());
            Ok(encode_corpus(files, vocabulary))
        }

        VocabularySource::Train(save_path) => {
            let result = train_corpus(files, bpe_config.vocab_size, bpe_config.min_frequency);
            if let Some(path) = save_path {
            println!("Vocabulary saved to: {}", path.display());
                save_vocabulary(&path, &result.vocabulary)?;
            }
            Ok(result)
        }
    }
}
