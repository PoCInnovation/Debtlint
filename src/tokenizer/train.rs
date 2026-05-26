use std::collections::HashMap;

use crate::tokenizer::pairs::{count_pairs, most_common_pair};
use crate::tokenizer::replace::replace_pair;
use crate::tokenizer::{Token, VocabularyEntry, BASE_VOCAB_SIZE};

pub struct BpeTrainingResult { // struct to store the result of the training
    pub sequence: Vec<Token>, // sequence of token
    pub vocabulary: HashMap<Token, VocabularyEntry>, // vocab of the training
    pub merges: u32, // number of merges
}

pub fn train_bpe(mut sequence: Vec<Token>, target_vocab_size: u32,
    min_frequency: usize) -> BpeTrainingResult { // recup the sequence, the target vocab size and the min frequency and return the struct
    let mut vocabulary = HashMap::new();
    let mut next_id = BASE_VOCAB_SIZE; // 256

    while next_id < target_vocab_size { // 256 -> 1000 max
        let counts: HashMap<(u32, u32), usize> = count_pairs(&sequence);
        let Some((pair, frequency)) = most_common_pair(&counts) else { // return the pair with the highest frequency
            break; // if no pair break loop
        };
        if frequency < min_frequency {
            break;
        }
        vocabulary.insert(next_id, VocabularyEntry::merge(pair, vec![])); // merge the pair and vector of occurrences
        sequence = replace_pair(&sequence, pair, next_id); // replace the pair with the new token
        next_id += 1;
    }
    BpeTrainingResult {
        merges: next_id - BASE_VOCAB_SIZE,
        sequence,
        vocabulary,
    }
}