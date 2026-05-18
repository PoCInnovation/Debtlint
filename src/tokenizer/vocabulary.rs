pub type Token = u32;
pub const BASE_VOCAB_SIZE: u32 = 256;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VocabularyEntry {
    pub left: Token,
    pub right: Token,
}

impl VocabularyEntry {
    pub const fn new(left: Token, right: Token) -> Self {
        Self { left, right }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_stores_left_and_right() {
        let entry = VocabularyEntry::new(97, 110);
        assert_eq!(entry.left, 97);
        assert_eq!(entry.right, 110);
    }
}
