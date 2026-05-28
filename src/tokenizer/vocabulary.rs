use crate::tokenizer::pairs::TokenPair;

pub type Token = u32;
pub type FileId = u32;

/// fixe alphabet a -> 0 & z -> 25, real first id is 26
pub const BASE_VOCAB_SIZE: u32 = 26;

const BASE_LETTERS: &[u8] = b"abcdefghijklmnopqrstuvwxyz"; // b -> byte / str to utf-8

pub struct FileOccurrences {
    pub file_id: FileId,
    pub offsets: Vec<usize>, // vector of offsets all the occurences of a token in a single file
}

// function to manage the file occurrences
impl FileOccurrences {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            offsets: Vec::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.offsets.len() // return the number of occurrences in the file
    }
}

pub enum VocabularyEntry { // enum to manage the vocabulary entry, not struct bc how we manage the first 26 letters in a pair -> a letter is not a pair
    Letter(u8), // letter entry
    Merge {
        pair: TokenPair, // pair entry
        occurrences: Vec<FileOccurrences>,
    },
}

// function to manage the vocabulary entry
impl VocabularyEntry {
    pub const fn letter(byte: u8) -> Self {
        Self::Letter(byte) // getter for the letter
    }

    pub fn merge(pair: TokenPair, occurrences: Vec<FileOccurrences>) -> Self {
        Self::Merge {
            pair,
            occurrences,
        } // add the pair and the occurrences
    }

    pub fn pair(&self) -> Option<TokenPair> { // getter for the pair
        match self {
            Self::Letter(_) => None,
            Self::Merge { pair, .. } => Some(*pair), // if the entry is a existing pair return the pair
        }
    }

    pub fn occurrences(&self) -> &[FileOccurrences] { // getter for the occurrences
        match self {
            Self::Letter(_) => &[], // empty vector
            Self::Merge { occurrences, .. } => occurrences, // if the entry is a existing pair return the occurrences
        }
    }
}

pub struct Vocabulary {
    pub entries: Vec<VocabularyEntry>, // vector of all vocab letter and pair
}

impl Vocabulary {
    pub fn init_base() -> Self {
        let entries = BASE_LETTERS.iter().copied().map(VocabularyEntry::letter).collect(); // collect the letters in a vector
        Self { entries } // return the vocab with the letters
    }

    pub fn len(&self) -> usize {
        self.entries.len() // return the length of the vocab
    }

    pub fn get(&self, token: Token) -> Option<&VocabularyEntry> {
        self.entries.get(token as usize) // return the struct entry of the token
    }

    pub fn push_merge(&mut self, pair: TokenPair, occurrences: Vec<FileOccurrences>) -> Token {
        let id = self.entries.len() as Token; // recup the id of the new pair
        self.entries.push(VocabularyEntry::merge(pair, occurrences)); // add the new pair to the vocab
        id // return the id of the new pair
    }
}
