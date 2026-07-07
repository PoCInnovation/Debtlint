use crate::tokenizer::Token;
use std::collections::HashMap;

pub type TokenPair = (Token, Token); // type of a pair of token

pub fn count_pairs(sequence: &[Token]) -> HashMap<TokenPair, usize> // recup the token sequence and return a hashmap of pairs and their count of iteration
{
    let mut counts = HashMap::new(); // create the hashmap
    for window in sequence.windows(2) {
        // for each window of 2 tokens (window return each pair of 2)
        let pair = (window[0], window[1]); // create the pair
        *counts.entry(pair).or_insert(0) += 1; // add the pair to the hashmap and if igt not exist set to 0 and add 1
    }
    counts // return the hashmap
}

pub fn count_pairs_corpus(sequences: &[&[Token]]) -> HashMap<TokenPair, usize> {
    let mut counts = HashMap::new();
    for sequence in sequences {
        for window in sequence.windows(2) {
            let pair = (window[0], window[1]);
            *counts.entry(pair).or_insert(0) += 1; // add the pair to the hashmap and if igt not exist set to 0 and add 1
        }
    }
    counts
}

pub fn most_common_pair(counts: &HashMap<TokenPair, usize>) -> Option<(TokenPair, usize)> // recup the hashmap and return the pair with the highest count
{
    let mut best: Option<(TokenPair, usize)> = None; // create the best pair
    for (&pair, &count) in counts {
        // for each pair and count in the hashmap
        let replace = match best {
            None => true, // if the best pair is no set return true
            Some((best_pair, best_count)) => {
                if count > best_count {
                    true
                } else if count == best_count && pair < best_pair {
                    true
                } else {
                    false
                }
            }
        };
        if replace {
            best = Some((pair, count));
        }
    }
    best
}
