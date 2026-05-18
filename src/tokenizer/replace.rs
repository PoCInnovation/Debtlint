use crate::tokenizer::{Token, TokenPair};

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
