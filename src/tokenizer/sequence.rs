use crate::tokenizer::vocabulary::char_to_token;
use crate::tokenizer::Token;

// Encodes text as one token per Unicode scalar value (`char`).
pub fn text_to_sequence(text: &str) -> Vec<Token> {
    text.chars().map(char_to_token).collect()
}
