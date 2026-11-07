use crate::config::default_vocab_path;
use debtlint::pipeline::{BpeConfig, PipelineConfig, VocabularySource};
use std::path::PathBuf;

use clap::{Parser, ValueEnum};

const DEFAULT_VOCAB_SIZE: u32 = 1000;
const DEFAULT_MIN_FREQUENCY: usize = 2;

/// Duplicate report format for stdout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum ReportFormat {
    /// Human-readable report.
    #[default]
    Text,
    /// JSON for CI / GitHub Action.
    Json,
}

#[derive(Parser, Debug)]
#[command(
    name = "debtlint",
    version,
    about = "Technical debt detection via BPE-inspired pattern analysis"
)]
pub struct Args {
    /// Directory to analyze (overrides config `dir`; default `.`)
    #[arg(value_name = "DIR")]
    pub dir: Option<PathBuf>,
    /// Max vocabulary size; forces retrain when set
    #[arg(long, value_name = "N")]
    pub vocab_size: Option<u32>,
    /// Min pair frequency for a merge
    #[arg(long, default_value_t = DEFAULT_MIN_FREQUENCY)]
    pub min_frequency: usize,
    /// Write encoded token sequence as JSON
    #[arg(long, value_name = "PATH")]
    pub output_encoded: Option<PathBuf>,
    /// Force retrain and write a legacy vocab export to PATH
    #[arg(long, value_name = "PATH")]
    pub save_vocab: Option<PathBuf>,
    /// Shingle size in tokens (Winnowing)
    #[arg(long, default_value_t = 50)]
    pub shingle_size: usize,
    /// Sliding window size in shingles (Winnowing)
    #[arg(long, default_value_t = 3)]
    pub window_size: usize,
    /// Minimum duplicated block length in tokens
    #[arg(long, default_value_t = 20)]
    pub min_tokens: usize,
    /// Load a vocabulary JSON and skip training / cache
    #[arg(long, value_name = "PATH", conflicts_with = "save_vocab")]
    pub load_vocab: Option<PathBuf>,
    /// Report format: `text` or `json`
    #[arg(long, value_enum, default_value_t = ReportFormat::Text, value_name = "FORMAT")]
    pub report_format: ReportFormat,
}

impl Args {
    pub fn effective_vocab_size(&self) -> u32 {
        self.vocab_size.unwrap_or(DEFAULT_VOCAB_SIZE)
    }

    pub fn get_pipeline_config(&self) -> PipelineConfig {
        let cache_path = default_vocab_path();
        let source = if let Some(path) = &self.load_vocab {
            VocabularySource::Load(path.clone())
        } else if self.save_vocab.is_some() || self.vocab_size.is_some() {
            VocabularySource::Train {
                save_path: self.save_vocab.clone(),
                cache_path,
            }
        } else {
            VocabularySource::Auto {
                cache_path,
                force_retrain: false,
            }
        };
        PipelineConfig {
            source,
            output_encoded: self.output_encoded.clone(),
        }
    }

    pub fn get_bpe_config(&self) -> BpeConfig {
        BpeConfig {
            vocab_size: self.effective_vocab_size(),
            min_frequency: self.min_frequency,
        }
    }
}
