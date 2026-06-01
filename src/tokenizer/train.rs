use std::collections::HashMap;

use crate::tokenizer::pairs::{count_pairs, most_common_pair};
use crate::tokenizer::replace::replace_pair;
use crate::tokenizer::{Token, Vocabulary, BASE_VOCAB_SIZE};

pub struct BpeTrainingResult { // struct to store the result of the training
    pub sequence: Vec<Token>, // sequence of token
    pub vocabulary: Vocabulary, // vocab of the training
    pub merges: u32, // number of merges
}

pub fn train_bpe( // recup the sequence the target vocab size and the min frequency occurrences
    mut sequence: Vec<Token>,
    target_vocab_size: u32, // nb of token in the vocab
    min_frequency: usize, // min frequency of a pair to be merged
) -> BpeTrainingResult {
    let mut vocabulary = Vocabulary::init_base(); // init the vocab with the base letters

    while (vocabulary.len() as u32) < target_vocab_size { // while the vocab didnt reach the max size we fixe
        let counts: HashMap<(u32, u32), usize> = count_pairs(&sequence);
        let Some((pair, frequency)) = most_common_pair(&counts) else { // return the pair with the highest frequence
            break;
        };
        if frequency < min_frequency {
            break;
        }
        let new_token = vocabulary.push_merge(pair, vec![]); // merge the pair and vector of occurrences
        sequence = replace_pair(&sequence, pair, new_token); // replace the pair with the new token
    }

    BpeTrainingResult {
        merges: vocabulary.len() as u32 - BASE_VOCAB_SIZE, // len - les 26
        sequence,
        vocabulary,
    }
}
