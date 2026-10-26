mod duplication;
mod winnowing;

pub use duplication::{
    DetectionParams, Duplicate, DuplicateGroup, Instance, RawDuplicate, Region, TokenMatch,
    build_index, detect_duplicates, find_matches, group_duplicates, merge_matches,
    token_char_offsets,
};
pub use winnowing::{
    Fingerprint, hash_shingle, hash_shingles, winnow, winnow_corpus, winnow_hashes,
};
