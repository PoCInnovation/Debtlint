use std::collections::HashMap;

use crate::tokenizer::{Token, VocabularyEntry, BASE_VOCAB_SIZE};

pub fn decode_token(token: Token, vocabulary: &HashMap<Token, VocabularyEntry>) -> Vec<u8> {
    if token < BASE_VOCAB_SIZE { // if the token is less than 256 return the token as a vector of u8
        return vec![token as u8];
    }
    let entry = match vocabulary.get(&token) {
        Some(entry) => entry,
        None => {
            eprintln!("unknown token id: {token}");
            return Vec::new();
        }
    };
    let mut bytes: Vec<u8> = Vec::new(); // create the vector of u8
    bytes.extend(decode_token(entry.left, vocabulary)); // decode the left token
    bytes.extend(decode_token(entry.right, vocabulary)); // decode the right token
    bytes // return the vector of u8
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