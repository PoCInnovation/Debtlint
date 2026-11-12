use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, ErrorKind, Result};
use std::path::Path;

use crate::tokenizer::SourceFile;
use crate::tokenizer::vocabulary::{
    Token, VOCAB_EXPORT_VERSION, Vocabulary, VocabularyEntry, VocabularyExport,
};

/// On-disk vocabulary document format version.
pub const VOCAB_CACHE_FORMAT_VERSION: u32 = 1;
/// BPE algorithm version (incompatible bump forces retrain).
pub const ALGORITHM_VERSION: u32 = 1;

fn json_err(err: serde_json::Error) -> io::Error {
    io::Error::new(ErrorKind::InvalidData, err)
}

/// Corpus shape from ingested paths only (no extra I/O).
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

    /// Per-extension share as an integer percentage in `[0, 100]`.
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

    /// Min absolute growth (with doubling) that invalidates the cache.
    const FILE_COUNT_DELTA_LIMIT: usize = 100;
    /// Majority-language %-point shift that invalidates the cache.
    const MAJORITY_SHIFT_LIMIT: u32 = 50;

    /// True if file count doubled with ≥100 growth, or majority language shifted by >50 points / changed.
    pub fn requires_retrain(
        &self,
        cached_file_count: usize,
        cached_distribution: &BTreeMap<String, u32>,
    ) -> bool {
        if self.file_count >= cached_file_count.saturating_mul(2)
            && self.file_count.saturating_sub(cached_file_count) >= Self::FILE_COUNT_DELTA_LIMIT
        {
            return true;
        }

        let Some((majority_ext, &majority_pct)) =
            cached_distribution
                .iter()
                .max_by(|(ext_a, pct_a), (ext_b, pct_b)| {
                    pct_a.cmp(pct_b).then_with(|| ext_a.cmp(ext_b))
                })
        else {
            return !self.language_distribution().is_empty();
        };

        let current = self.language_distribution();
        match current.iter().max_by(|(ext_a, pct_a), (ext_b, pct_b)| {
            pct_a.cmp(pct_b).then_with(|| ext_a.cmp(ext_b))
        }) {
            Some((current_majority, _)) if current_majority != majority_ext => true,
            _ => {
                let current_pct = current.get(majority_ext).copied().unwrap_or(0);
                majority_pct.abs_diff(current_pct) > Self::MAJORITY_SHIFT_LIMIT
            }
        }
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
            && !profile.requires_retrain(self.file_count, &self.language_distribution)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn profile(file_count: usize, extension_counts: &[(&str, usize)]) -> CorpusProfile {
        CorpusProfile {
            file_count,
            extension_counts: extension_counts
                .iter()
                .map(|(ext, count)| ((*ext).to_string(), *count))
                .collect(),
        }
    }

    fn dist(entries: &[(&str, u32)]) -> BTreeMap<String, u32> {
        entries
            .iter()
            .map(|(ext, pct)| ((*ext).to_string(), *pct))
            .collect()
    }

    fn cached_doc(vocab_size: u32, min_frequency: usize,
        file_count: usize, language_distribution: BTreeMap<String, u32>)
        -> VocabularyDocument {
        VocabularyDocument {
            format_version: VOCAB_CACHE_FORMAT_VERSION,
            algorithm_version: ALGORITHM_VERSION,
            vocab_size,
            min_frequency,
            selected_extensions: language_distribution.keys().cloned().collect(),
            file_count,
            language_distribution,
            entries: vec![],
            merge_start_id: 0,
        }
    }

    #[test]
    fn retrain_when_file_count_doubles_with_large_delta() {
        let current = profile(200, &[("rs", 200)]);
        assert!(current.requires_retrain(80, &dist(&[("rs", 100)])));
    }

    #[test]
    fn keep_cache_when_growth_is_small() {
        let current = profile(60, &[("rs", 60)]);
        assert!(!current.requires_retrain(50, &dist(&[("rs", 100)])));
    }

    #[test]
    fn retrain_when_majority_language_changes() {
        let current = profile(10, &[("py", 9), ("rs", 1)]);
        assert!(current.requires_retrain(10, &dist(&[("rs", 90), ("py", 10)])));
    }

    #[test]
    fn retrain_when_majority_share_shifts_a_lot() {
        let current = profile(10, &[("rs", 3), ("py", 7)]);
        assert!(current.requires_retrain(10, &dist(&[("rs", 90), ("py", 10)])));
    }

    #[test]
    fn compatible_when_params_and_corpus_match() {
        let doc = cached_doc(1000, 2, 10, dist(&[("rs", 100)]));
        let current = profile(12, &[("rs", 12)]);
        assert!(doc.is_compatible(1000, 2, &current));
    }

    #[test]
    fn incompatible_when_vocab_size_differs() {
        let doc = cached_doc(1000, 2, 10, dist(&[("rs", 100)]));
        let current = profile(10, &[("rs", 10)]);
        assert!(!doc.is_compatible(2000, 2, &current));
    }

    #[test]
    fn incompatible_when_format_version_differs() {
        let mut doc = cached_doc(1000, 2, 10, dist(&[("rs", 100)]));
        doc.format_version = VOCAB_CACHE_FORMAT_VERSION + 1;
        let current = profile(10, &[("rs", 10)]);
        assert!(!doc.is_compatible(1000, 2, &current));
    }
}
