use crate::tokenizer::Token;

pub fn text_to_sequence(text: &str) -> Vec<Token> {
    text.bytes().map(u32::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_yields_empty_sequence() {
        assert!(text_to_sequence("").is_empty());
    }

    #[test]
    fn banana_becomes_byte_tokens() {
        let seq = text_to_sequence("banana");
        assert_eq!(seq, vec![98, 97, 110, 97, 110, 97]);
    }

    #[test]
    fn multibyte_utf8_uses_raw_bytes() {
        let seq = text_to_sequence("é");
        assert_eq!(seq, vec![195, 169]);
    }
}
