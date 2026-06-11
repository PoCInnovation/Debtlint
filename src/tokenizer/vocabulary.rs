use std::path::PathBuf;
use crate::tokenizer::pairs::TokenPair;
pub type Token = u32;
/// Fixed alphabet with letters lower and upper digits space punctuation then UNK.
pub const BASE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \t\n{}()[];,.=+-*/<>!&|^~%#@_`'\"\\:?\u{FFFD}"; // \u{FFFD} is the replacement character
pub const BASE_VOCAB_SIZE: u32 = 97; // size of the base alphabet
pub const UNK_TOKEN: Token = BASE_VOCAB_SIZE - 1; // token for unknown characters

pub fn char_to_token(c: char) -> Token {
    BASE_ALPHABET.chars().position(|symbol| symbol == c).map(|index| index as Token).unwrap_or(UNK_TOKEN) // return the token for the character or UNK_TOKEN if not found
}

pub struct FileOccurrences {
    pub path: PathBuf,
    pub offsets: Vec<usize>,
}

impl FileOccurrences {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            offsets: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.offsets.len() // return the number of occurrences in the file
    }
}

pub enum VocabularyEntry { // enum to manage the vocabulary entrie
    Symbol(char),
    Merge {
        pair: TokenPair,
        occurrences: Vec<FileOccurrences>,
    },
}

impl VocabularyEntry {
    pub const fn symbol(ch: char) -> Self {
        Self::Symbol(ch)
    }

    pub fn merge(pair: TokenPair, occurrences: Vec<FileOccurrences>) -> Self {
        Self::Merge { pair, occurrences }
    }

    pub fn pair(&self) -> Option<TokenPair> { // getter for the pair
        match self {
            Self::Symbol(_) => None, // if the entry is a symbol return None
            Self::Merge { pair, .. } => Some(*pair), // if the entry is a merge return the pair
        }
    }

    pub fn occurrences(&self) -> &[FileOccurrences] { // getter for the occurrence
        match self {
            Self::Symbol(_) => &[],
            Self::Merge { occurrences, .. } => occurrences,
        }
    }
}

pub struct Vocabulary {
    pub entries: Vec<VocabularyEntry>, // vector of all vocab letter and pair
}

impl Vocabulary {
    pub fn init_base() -> Self {
        let entries = BASE_ALPHABET.chars().map(VocabularyEntry::symbol).collect(); // create the vector of entries
        Self { entries } // return the vocabulary
    }

    pub fn len(&self) -> usize {
        self.entries.len() // return the length of the vocab
    }

    pub fn get(&self, token: Token) -> Option<&VocabularyEntry> {
        self.entries.get(token as usize) // return the struct entry of the token
    }

    pub fn push_merge(&mut self, pair: TokenPair, occurrences: Vec<FileOccurrences>) -> Token {
        let id = self.entries.len() as Token;
        self.entries.push(VocabularyEntry::merge(pair, occurrences));
        id
    }
}
