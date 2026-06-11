use crate::tokenizer::{Token, Vocabulary, VocabularyEntry};

pub fn decode_token(token: Token, vocabulary: &Vocabulary) -> String { // decode the token and return the string
    let Some(entry) = vocabulary.get(token) else { // get the entry from the vocab
        eprintln!("unknown token id: {token}"); // if not found
        return String::new();
    };
    match entry {
        VocabularyEntry::Symbol(ch) => ch.to_string(), // if the entry is a symbol return the string
        VocabularyEntry::Merge { pair, .. } => { // if the entry is a merge return the pair
            let (left, right) = *pair; // destructure the pair
            let mut text = decode_token(left, vocabulary); // decode the left token
            text.push_str(&decode_token(right, vocabulary)); // decode the right token and add to the string
            text
        }
    } // return the string
}

pub fn decode_sequence(sequence: &[Token], vocabulary: &Vocabulary) -> String {
    sequence.iter().map(|&token| decode_token(token, vocabulary)).collect() // decode the sequence and return the string
}
