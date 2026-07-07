use std::path::PathBuf;

use crate::tokenizer::vocabulary::FileOccurrences;
use crate::tokenizer::{Token, TokenPair};

// func to find the occurrences of a pair in a file sequence
pub fn find_pair_occurrences(
    file_sequences: &[(PathBuf, Vec<Token>)],
    pair: TokenPair,
) -> Vec<FileOccurrences> {
    let (left, right) = pair;
    let mut occurrences = Vec::new();

    for (path, sequence) in file_sequences {
        let mut offsets = Vec::new();
        let mut index = 0;
        while index + 1 < sequence.len() {
            if sequence[index] == left && sequence[index + 1] == right {
                offsets.push(index);
                index += 2;
            } else {
                index += 1;
            }
        }
        if !offsets.is_empty() {
            occurrences.push(FileOccurrences {
                path: path.clone(),
                offsets,
            });
        }
    }
    occurrences
}

pub fn replace_pair(sequence: &[Token], pair: TokenPair, new_token: Token) -> Vec<Token> {
    let (left, right) = pair;
    let mut result = Vec::with_capacity(sequence.len()); // create the vector with the exact same capacity of the sequence
    let mut index = 0; // head of lecture

    while index < sequence.len() {
        if index + 1 < sequence.len() && sequence[index] == left && sequence[index + 1] == right {
            result.push(new_token); // add the token and not the pair when we find it
            index += 2;
        } else {
            result.push(sequence[index]); // add token like basicly
            index += 1;
        }
    }
    result
}
