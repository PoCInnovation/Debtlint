pub type Token = u32;
pub const BASE_VOCAB_SIZE: u32 = 256;

pub struct VocabularyEntry { // struct to store the left and right token
    pub left: Token,
    pub right: Token,
}

impl VocabularyEntry {
    pub const fn new(left: Token, right: Token) -> Self { // constructor
        Self { left, right } // return the struct with the left and right token
    }
}