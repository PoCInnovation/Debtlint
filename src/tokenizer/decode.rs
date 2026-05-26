use std::collections::HashMap;

use crate::tokenizer::{Token, VocabularyEntry, BASE_VOCAB_SIZE};

const BASE_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz"; // base letters for the token before vector

pub fn decode_token(token: Token, vocabulary: &HashMap<Token, VocabularyEntry>) -> Vec<u8> {
    if token < BASE_VOCAB_SIZE { // if the token is less than 26 return the letter as a vector of u8
        let index = token as usize;
        return vec![BASE_LETTERS[index]]; // return the letter
    }
    let Some((left, right)) = vocabulary.get(&token).and_then(VocabularyEntry::pair) // return the pair of the token
    else {
        eprintln!("unknown token id: {token}");
        return Vec::new();
    };
    let mut bytes = Vec::new();
    bytes.extend(decode_token(left, vocabulary)); // decode the left token
    bytes.extend(decode_token(right, vocabulary)); // decode the right token
    bytes
}

pub fn decode_sequence(sequence: &[Token], vocabulary: &HashMap<Token, VocabularyEntry>) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    for &token in sequence { // for each token in the sequence
        bytes.extend(decode_token(token, vocabulary)); // decode the token and add it to the vector
    }
    match String::from_utf8(bytes) {
        Ok(text) => text, // if the string is valid return the string
        Err(_) => panic!("decoded bytes must be valid utf-8"),
    }
}