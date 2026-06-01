use crate::tokenizer::{Token, Vocabulary, VocabularyEntry};

pub fn decode_token(token: Token, vocabulary: &Vocabulary) -> Vec<u8> { // decode the token and return the bytes
    let Some(entry) = vocabulary.get(token) else { // get the entry from the vocab
        eprintln!("unknown token id: {token}"); // if not found
        return Vec::new();
    };
    match entry {
        VocabularyEntry::Letter(byte) => vec![*byte], // if the entry is a letter return the byte
        VocabularyEntry::Merge { pair, .. } => { // if the entry is a merge return the pair
            let (left, right) = *pair; // destructure the pair
            let mut bytes = Vec::new();
            bytes.extend(decode_token(left, vocabulary)); // decode the left token
            bytes.extend(decode_token(right, vocabulary)); // decode the right token
            bytes
        }
    }
}

pub fn decode_sequence(sequence: &[Token], vocabulary: &Vocabulary) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    for &token in sequence { // for each token in the sequence
        bytes.extend(decode_token(token, vocabulary)); // decode the token and add to the vector
    }
    match String::from_utf8(bytes) {
        Ok(text) => text, // if the string is valid return the string
        Err(_) => panic!("decoded bytes must be valid utf-8"),
    }
}
