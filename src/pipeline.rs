use crate::in_out::{
    CorpusProfile, VocabularyDocument, load_vocabulary, load_vocabulary_document,
    save_vocabulary, save_vocabulary_document,
};
use crate::tokenizer::{BpeTrainingResult, SourceFile, encode_corpus, train_corpus};
use std::path::PathBuf;

pub enum VocabularySource {
    /// Load a vocabulary from an explicit path (`--load-vocab`).
    Load(PathBuf),
    /// Always train; optionally also write a legacy export to `save_path` (`--save-vocab`).
    Train {
        save_path: Option<PathBuf>,
        cache_path: PathBuf,
    },
    /// Load `.debtlint/debtlint.vocab.json` when compatible, otherwise train and persist it.
    Auto {
        cache_path: PathBuf,
        force_retrain: bool,
    },
}

pub struct PipelineConfig {
    pub source: VocabularySource,
    pub output_encoded: Option<PathBuf>,
}

pub struct BpeConfig {
    pub vocab_size: u32,
    pub min_frequency: usize,
}

fn train_and_cache(
    files: &[SourceFile],
    bpe_config: &BpeConfig,
    cache_path: &PathBuf,
    legacy_save: Option<&PathBuf>,
) -> std::io::Result<BpeTrainingResult> {
    let profile = CorpusProfile::from_files(files);
    let result = train_corpus(files, bpe_config.vocab_size, bpe_config.min_frequency);
    let document = VocabularyDocument::from_trained(
        &result.vocabulary,
        bpe_config.vocab_size,
        bpe_config.min_frequency,
        &profile,
    );
    save_vocabulary_document(cache_path, &document)?;
    eprintln!("Vocabulary saved to: {}", cache_path.display());
    if let Some(path) = legacy_save {
        save_vocabulary(path, &result.vocabulary)?;
        eprintln!("Vocabulary export saved to: {}", path.display());
    }
    Ok(result)
}

/// Train BPE from scratch, load a vocabulary file, or reuse the project cache when compatible.
pub fn run_bpe(
    files: &[SourceFile],
    bpe_config: BpeConfig,
    pipeline_config: PipelineConfig,
) -> std::io::Result<BpeTrainingResult> {
    match pipeline_config.source {
        VocabularySource::Load(path) => {
            let vocabulary = load_vocabulary(&path)?;
            eprintln!("Vocabulary loaded from: {}", path.display());
            Ok(encode_corpus(files, vocabulary))
        }

        VocabularySource::Train {
            save_path,
            cache_path,
        } => train_and_cache(files, &bpe_config, &cache_path, save_path.as_ref()),

        VocabularySource::Auto {
            cache_path,
            force_retrain,
        } => {
            let profile = CorpusProfile::from_files(files);
            if !force_retrain {
                if let Ok(document) = load_vocabulary_document(&cache_path) {
                    if document.is_compatible(
                        bpe_config.vocab_size,
                        bpe_config.min_frequency,
                        &profile,
                    ) {
                        let vocabulary = document.to_vocabulary()?;
                        eprintln!("Vocabulary loaded from: {}", cache_path.display());
                        return Ok(encode_corpus(files, vocabulary));
                    }
                }
            }
            train_and_cache(files, &bpe_config, &cache_path, None)
        }
    }
}
