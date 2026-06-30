use crate::tokenizer::{Token, Vocabulary};

pub fn text_to_sequence(text: &str, vocabulary: &Vocabulary) -> Vec<Token>
{
    text.chars().map(|ch| vocabulary.encode_char(ch)).collect()
}
