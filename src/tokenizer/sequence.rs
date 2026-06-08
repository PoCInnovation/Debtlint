use crate::tokenizer::Token;

/// Maps lowercase `a`–`z` to token ids `0`–`25`.
pub fn text_to_sequence(text: &str) -> Vec<Token> {
    text.bytes().map(|byte| {
            if !byte.is_ascii_lowercase() {
                panic!(
                    "text_to_sequence expects lowercase a-z only; got byte {byte} (preprocessing required)"
                );
            }
            u32::from(byte - b'a')
        })
        .collect()
}