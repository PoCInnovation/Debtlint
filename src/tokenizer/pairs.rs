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
                count > best_count || count == best_count && pair < best_pair
            }
        };
        if replace {
            best = Some((pair, count));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_count() {
        let sequence = [1, 2, 1, 2, 1, 2];
        let counts = count_pairs(&sequence);
        assert_eq!(counts[&(1, 2)], 3); // verif the count pair (1, 2) is 3
        assert_eq!(counts[&(2, 1)], 2); // verif the count pair (2, 1) is 2
        assert_eq!(counts.len(), 2);
    }

    #[test]
    fn pairs_most_common() {
        let counts: HashMap<(u32, u32), usize> = HashMap::from([((1, 2), 3), ((2, 1), 2)]);
        assert_eq!(most_common_pair(&counts), Some(((1, 2), 3)));
    }

    #[test]
    fn tie_break() {
        let counts = HashMap::from([((2, 3), 2), ((1, 2), 2)]);
        assert_eq!(most_common_pair(&counts), Some(((1, 2), 2)));
    }
}
