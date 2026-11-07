use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, ErrorKind, Result};
use std::path::Path;

use crate::tokenizer::SourceFile;
use crate::tokenizer::vocabulary::{
    Token, VOCAB_EXPORT_VERSION, Vocabulary, VocabularyEntry, VocabularyExport,
};

/// Bumps when the on-disk vocabulary document shape changes.
pub const VOCAB_CACHE_FORMAT_VERSION: u32 = 1;
/// Bumps when the BPE training / encoding algorithm changes incompatibly.
pub const ALGORITHM_VERSION: u32 = 1;

fn json_err(err: serde_json::Error) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, err)
}

/// Extension histogram of the ingested corpus (paths only — no extra I/O).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusProfile {
    pub file_count: usize,
    pub extension_counts: BTreeMap<String, usize>,
}

impl CorpusProfile {
    pub fn from_files(files: &[SourceFile]) -> Self {
        let mut extension_counts = BTreeMap::new();
        for file in files {
            let ext = file
                .path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            *extension_counts.entry(ext).or_insert(0) += 1;
        }
        Self {
            file_count: files.len(),
            extension_counts,
        }
    }

    pub fn selected_extensions(&self) -> Vec<String> {
        self.extension_counts.keys().cloned().collect()
    }

    /// Per-extension share of the corpus as an integer percentage in `[0, 100]`.
    pub fn language_distribution(&self) -> BTreeMap<String, u32> {
        if self.file_count == 0 {
            return BTreeMap::new();
        }
        let total = self.file_count;
        self.extension_counts
            .iter()
            .map(|(ext, count)| {
                let percent = (count * 100 + total / 2) / total;
                (ext.clone(), percent as u32)
            })
            .collect()
    }

    /// Same file count and same integer percentages per extension.
    pub fn matches_distribution(
        &self,
        file_count: usize,
        distribution: &BTreeMap<String, u32>,
    ) -> bool {
        self.file_count == file_count && &self.language_distribution() == distribution
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyDocument {
    pub format_version: u32,
    pub algorithm_version: u32,
    pub vocab_size: u32,
    pub min_frequency: usize,
    pub selected_extensions: Vec<String>,
    pub file_count: usize,
    pub language_distribution: BTreeMap<String, u32>,
    pub entries: Vec<VocabularyEntry>,
    pub merge_start_id: Token,
}

impl VocabularyDocument {
    pub fn from_trained(
        vocabulary: &Vocabulary,
        vocab_size: u32,
        min_frequency: usize,
        profile: &CorpusProfile,
    ) -> Self {
        Self {
            format_version: VOCAB_CACHE_FORMAT_VERSION,
            algorithm_version: ALGORITHM_VERSION,
            vocab_size,
            min_frequency,
            selected_extensions: profile.selected_extensions(),
            file_count: profile.file_count,
            language_distribution: profile.language_distribution(),
            entries: vocabulary.entries.clone(),
            merge_start_id: vocabulary.merge_start_id(),
        }
    }

    pub fn to_vocabulary(&self) -> Result<Vocabulary> {
        if self.format_version != VOCAB_CACHE_FORMAT_VERSION {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "unsupported vocabulary format version {} (expected {})",
                    self.format_version, VOCAB_CACHE_FORMAT_VERSION
                ),
            ));
        }
        if self.algorithm_version != ALGORITHM_VERSION {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!(
                    "unsupported algorithm version {} (expected {})",
                    self.algorithm_version, ALGORITHM_VERSION
                ),
            ));
        }
        Vocabulary::from_export(VocabularyExport {
            format_version: VOCAB_EXPORT_VERSION,
            entries: self.entries.clone(),
            merge_start_id: self.merge_start_id,
        })
    }

    pub fn is_compatible(
        &self,
        vocab_size: u32,
        min_frequency: usize,
        profile: &CorpusProfile,
    ) -> bool {
        self.format_version == VOCAB_CACHE_FORMAT_VERSION
            && self.algorithm_version == ALGORITHM_VERSION
            && self.vocab_size == vocab_size
            && self.min_frequency == min_frequency
            && self.selected_extensions == profile.selected_extensions()
            && profile.matches_distribution(self.file_count, &self.language_distribution)
    }
}

pub fn save_vocabulary_document(path: &Path, document: &VocabularyDocument) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(document).map_err(json_err)?;
    fs::write(path, json)
}

pub fn load_vocabulary_document(path: &Path) -> Result<VocabularyDocument> {
    let json = fs::read_to_string(path)?;
    serde_json::from_str(&json).map_err(json_err)
}
