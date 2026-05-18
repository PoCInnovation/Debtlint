use crate::tokenizer::Token; // import Token from vocab

pub fn text_to_sequence(text: &str) -> Vec<Token> { // recup the text and return a vector of Token
    text.bytes().map(u32::from).collect() // convert the text to a vector of Token
}
