use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{self, ErrorKind, Result};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::tokenizer::pairs::TokenPair;
use crate::tokenizer::SourceFile;

pub type Token = u32;
pub const VOCAB_EXPORT_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct VocabularyExport {
    pub format_version: u32,
    pub entries: Vec<VocabularyEntry>,
    pub merge_start_id: Token,
}

/// Fixed alphabet with letters lower and upper digits space punctuation then UNK.
pub const BASE_ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \t\n{}()[];,.=+-*/<>!&|^~%#@_`'\"\\:?\u{FFFD}"; // \u{FFFD} is the replacement character
pub const BASE_VOCAB_SIZE: u32 = 97; // size of the base alphabet
pub const UNK_TOKEN: Token = BASE_VOCAB_SIZE - 1; // token for unknown characters

#[derive(Clone, Serialize, Deserialize)] // serialize and deserialize the file occurrences
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

#[derive(Clone, Serialize, Deserialize)]
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
    char_to_id: HashMap<char, Token>, // hashmap to map the character to the token
    merge_start_id: Token, // token for the start of the merge
}

impl Vocabulary {
    pub fn init_base() -> Self {
        let entries = BASE_ALPHABET.chars().map(VocabularyEntry::symbol).collect(); // collect the char of the base alphabet and create the entries
        let mut vocabulary = Self { // create the vocabulary
            entries,
            char_to_id: HashMap::new(), // create the hashmap
            merge_start_id: 0, // set the merge start id to 0
        };
        vocabulary.rebuild_char_index(); // rebuild the char index
        vocabulary.merge_start_id = vocabulary.entries.len() as Token; // set the merge start id to len of entries
        return vocabulary;
    }

    pub fn extend_with_corpus_symbols(&mut self, files: &[SourceFile])
    {
        let fixed: HashSet<char> = BASE_ALPHABET.chars().collect(); // get the fixed alphabet
        let mut extra = BTreeSet::new(); // create the extra set -> BTreeSet is a container that stores unique elements in a sorted order (no duplicate)

        for file in files {
            for ch in file.content.chars() {
                if !fixed.contains(&ch) {
                    extra.insert(ch); // insert in extra only if not in fixed
                }
            }
        }
        for ch in extra {
            self.entries.push(VocabularyEntry::symbol(ch)); // push the symbol in the entries
        }
        self.rebuild_char_index();
        self.merge_start_id = self.entries.len() as Token;
    }

    // func to get the token for a character
    pub fn token_for_char(&self, ch: char) -> Option<Token> {
        self.char_to_id.get(&ch).copied()
    }

    // func to encode a character
    pub fn encode_char(&self, ch: char) -> Token {
        self.token_for_char(ch).unwrap_or(UNK_TOKEN)
    }

    // func to get the merge start id
    pub fn merge_start_id(&self) -> Token {
        self.merge_start_id
    }

    // func to get the dynamic symbol count
    pub fn dynamic_symbol_count(&self) -> u32 {
        self.merge_start_id.saturating_sub(BASE_VOCAB_SIZE)
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

    // create the export struct
    pub fn to_export(&self) -> VocabularyExport {
        VocabularyExport {
            format_version: VOCAB_EXPORT_VERSION,
            entries: self.entries.clone(),
            merge_start_id: self.merge_start_id,
        }
    }

    pub fn from_export(export: VocabularyExport) -> Result<Self> {
        if export.format_version != VOCAB_EXPORT_VERSION {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                format!("unsupported vocabulary export version {} (expected {})",
                    export.format_version, VOCAB_EXPORT_VERSION,
                ),
            ));
        }

        let mut vocabulary = Self {
            entries: export.entries,
            char_to_id: HashMap::new(),
            merge_start_id: export.merge_start_id,
        };
        vocabulary.rebuild_char_index();
        Ok(vocabulary)
    }

    fn rebuild_char_index(&mut self) {
        self.char_to_id.clear();
        for (id, entry) in self.entries.iter().enumerate() {
            if let VocabularyEntry::Symbol(ch) = entry {
                self.char_to_id.insert(*ch, id as Token); // insert the character and the id in the hashmap
            }
        }
    }
}
