use std::path::PathBuf;

use crate::tokenizer::SourceFile;
use crate::tokenizer::pairs::{count_pairs_corpus, most_common_pair};
use crate::tokenizer::replace::{find_pair_occurrences, replace_pair};
use crate::tokenizer::sequence::text_to_sequence;
use crate::tokenizer::{Token, Vocabulary, VocabularyEntry};

pub struct FileTokens {
    pub path: PathBuf,
    pub sequence: Vec<Token>,
}

pub struct BpeTrainingResult {
    // struct to store the result of the training
    pub files: Vec<FileTokens>,     // vector of FileTokens
    pub vocabulary: Vocabulary,     // vocabulary
    pub merges: u32,                // number of merges
    pub initial_token_count: usize, // number of initial tokens
}

impl BpeTrainingResult {
    pub fn encoded_token_count(&self) -> usize // func to count the number of encoded tokens
    {
        self.files.iter().map(|f| f.sequence.len()).sum() // sum of all the sequences tokens (all tokens in all files)
    }
}

pub fn train_corpus(
    files: &[SourceFile],
    target_vocab_size: u32,
    min_frequency: usize,
) -> BpeTrainingResult {
    let mut vocabulary = Vocabulary::init_base();
    vocabulary.extend_with_corpus_symbols(files); // extend the vocabulary with the corpus symbols

    let file_sequences: Vec<(PathBuf, Vec<Token>)> = files
        .iter()
        .map(|file| {
            (
                file.path.clone(),
                text_to_sequence(&file.content, &vocabulary),
            ) // create the file sequences
        })
        .collect();
    train_from_sequences(file_sequences, vocabulary, target_vocab_size, min_frequency) // train the sequences
}

// merge the tokens in the sequences with the vocabulary
pub fn encode_corpus(files: &[SourceFile], vocabulary: Vocabulary) -> BpeTrainingResult {
    let merge_start_id = vocabulary.merge_start_id();
    let mut file_sequences: Vec<(PathBuf, Vec<Token>)> = files
        .iter()
        .map(|file| {
            (
                file.path.clone(),
                text_to_sequence(&file.content, &vocabulary),
            )
        })
        .collect();
    let initial_token_count: usize = file_sequences
        .iter()
        .map(|(_, sequence)| sequence.len())
        .sum();

    for (id, entry) in vocabulary.entries.iter().enumerate() {
        let token_id = id as Token;
        if token_id < merge_start_id {
            continue;
        }
        let VocabularyEntry::Merge { pair, .. } = entry else {
            continue;
        };
        for (_, sequence) in &mut file_sequences {
            *sequence = replace_pair(sequence, *pair, token_id);
        }
    }
    let files: Vec<FileTokens> = file_sequences
        .into_iter()
        .map(|(path, sequence)| FileTokens { path, sequence })
        .collect();

    BpeTrainingResult {
        merges: vocabulary.len() as u32 - merge_start_id,
        files,
        vocabulary,
        initial_token_count,
    }
}

pub fn train_bpe(
    sequence: Vec<Token>,
    target_vocab_size: u32,
    min_frequency: usize,
) -> BpeTrainingResult {
    let vocabulary = Vocabulary::init_base();
    train_from_sequences(
        vec![(PathBuf::from("-"), sequence)],
        vocabulary,
        target_vocab_size,
        min_frequency,
    ) // from - means we are training from a sequence of tokens
}

fn train_from_sequences(
    mut file_sequences: Vec<(PathBuf, Vec<Token>)>,
    mut vocabulary: Vocabulary,
    target_vocab_size: u32,
    min_frequency: usize,
) -> BpeTrainingResult {
    let initial_token_count: usize = file_sequences
        .iter()
        .map(|(_, sequence)| sequence.len())
        .sum(); // sum of all the sequences tokens (all tokens in all files)
    let merge_start_id = vocabulary.merge_start_id(); // get the merge start id

    while (vocabulary.len() as u32) < target_vocab_size {
        // while the vocab didnt reach the max size we fixe
        let sequences: Vec<&[Token]> = file_sequences
            .iter()
            .map(|(_, sequence)| sequence.as_slice())
            .collect();
        let counts = count_pairs_corpus(&sequences); // count the pairs in sequences
        let Some((pair, frequency)) = most_common_pair(&counts) else {
            // get the most common pair and frequance
            break;
        };
        if frequency < min_frequency {
            break;
        }
        let occurrences = find_pair_occurrences(&file_sequences, pair); // find the occurrences of the pair in the file sequences
        let new_token = vocabulary.push_merge(pair, occurrences); // push the merge to the vocabulary
        for (_, sequence) in &mut file_sequences {
            *sequence = replace_pair(sequence, pair, new_token); // replace the pair with the new token
        }
    }
    let files: Vec<FileTokens> = file_sequences
        .into_iter()
        .map(|(path, sequence)| FileTokens { path, sequence })
        .collect(); // recup file squences (all sequences) and create the struct FileTokens with the path and the sequence
    BpeTrainingResult {
        merges: vocabulary.len() as u32 - merge_start_id,
        files,
        vocabulary,
        initial_token_count,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;
    use crate::tokenizer::decode_sequence;
    use crate::tokenizer::sequence::text_to_sequence;
    use crate::tokenizer::vocabulary::{VocabularyEntry};

    fn source_file(name: &str, content: &str) -> SourceFile {
        SourceFile {
            path: PathBuf::from(name),
            content: content.to_string(),
        }
    }

    #[test]
    fn round_trip_sample() {
        let content = include_str!("../../fixtures/sample.rs"); // get the content of the file
        let file = vec![source_file("fixtures/sample.rs", content)]; // create the file with the content
        let result = train_corpus(&file, 500, 2);
        let decoded = decode_sequence(&result.files[0].sequence, &result.vocabulary);
        assert_eq!(decoded, content); // verif if afeter train and decode the content is the same
    }

    #[test]
    fn cafe_becomes_unk() {
        let vocabulary = Vocabulary::init_base();
        let sequence = text_to_sequence("café", &vocabulary);
        let decoded = decode_sequence(&sequence, &vocabulary);
        assert_eq!(decoded, "caf\u{FFFD}");// é est bien UNK
    }

    #[test]
    fn merges_have_offsets() {
        let content = include_str!("../../fixtures/sample.rs");
        let files = vec![source_file("fixtures/sample.rs", content)];
        let result = train_corpus(&files, 500, 2);
        assert!(result.merges > 0);

        for entry in &result.vocabulary.entries {
            let VocabularyEntry::Merge { occurrences, .. } = entry else {
                continue;
            };
            assert!(
                !occurrences.is_empty(),
                "each merge must list at least one file"
            );
            assert!(
                occurrences.iter().any(|file| !file.offsets.is_empty()),
                "each merge must record at least one offset"
            );
        }
    }
}
