use std::path::PathBuf;

use crate::tokenizer::pairs::{count_pairs_corpus, most_common_pair};
use crate::tokenizer::replace::replace_pair;
use crate::tokenizer::sequence::text_to_sequence;
use crate::tokenizer::SourceFile;
use crate::tokenizer::{Token, Vocabulary, BASE_VOCAB_SIZE};

pub struct FileTokens {
    pub path: PathBuf,
    pub sequence: Vec<Token>,
}

pub struct BpeTrainingResult { // struct to store the result of the training
    pub files: Vec<FileTokens>, // vector of FileTokens
    pub vocabulary: Vocabulary, // vocabulary
    pub merges: u32, // number of merges
    pub initial_token_count: usize, // number of initial tokens
}

impl BpeTrainingResult {
    pub fn encoded_token_count(&self) -> usize { // func to count the number of encoded tokens
        self.files.iter().map(|f| f.sequence.len()).sum() // sum of all the sequences tokens (all tokens in all files)
    }
}

pub fn train_corpus(files: &[SourceFile], target_vocab_size: u32, min_frequency: usize,) -> BpeTrainingResult
{
    // map the files to a vector of pairs (path, sequence of tokens)
    let file_sequences: Vec<(PathBuf, Vec<Token>)> = files.iter().map(|file| (file.path.clone(), text_to_sequence(&file.content))).collect();
    // train the BPE from the file sequences
    train_from_sequences(file_sequences, target_vocab_size, min_frequency)
}

// func to train the BPE from a sequence of tokens
pub fn train_bpe(sequence: Vec<Token>, target_vocab_size: u32, min_frequency: usize,) -> BpeTrainingResult
{
    train_from_sequences(vec![(PathBuf::from("-"), sequence)], target_vocab_size, min_frequency) // from - means we are training from a sequence of tokens
}

// func to train from a vector of pairs
fn train_from_sequences(mut file_sequences: Vec<(PathBuf, Vec<Token>)>, target_vocab_size: u32, // vector of pairs (path, sequence of tokens)
    min_frequency: usize,) -> BpeTrainingResult
    {
    let initial_token_count: usize = file_sequences.iter().map(|(_, sequence)| sequence.len()).sum(); // sum of all the sequences tokens (all tokens in all files)
    let mut vocabulary = Vocabulary::init_base();

    while (vocabulary.len() as u32) < target_vocab_size { // while the vocab didnt reach the max size we fixe

        let sequences: Vec<&[Token]> = file_sequences.iter().map(|(_, sequence)| sequence.as_slice()).collect();
        let counts = count_pairs_corpus(&sequences); // count the pairs in sequences
        let Some((pair, frequency)) = most_common_pair(&counts) else { // get the most common pair and frequance
            break;
        };
        if frequency < min_frequency {
            break;
        }
        let new_token = vocabulary.push_merge(pair, vec![]); // push the merge to the vocabulary
        for (_, sequence) in &mut file_sequences {
            *sequence = replace_pair(sequence, pair, new_token); // replace the pair with the new token
        }
    }

    let files: Vec<FileTokens> = file_sequences.into_iter().map(|(path, sequence)| FileTokens { path, sequence }).collect(); // convert the vector of pairs to a vector of FileTokens

    BpeTrainingResult {
        merges: vocabulary.len() as u32 - BASE_VOCAB_SIZE,
        files,
        vocabulary,
        initial_token_count,
    }
}
