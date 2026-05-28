mod decode;
mod pairs;
mod replace;
mod sequence;
mod source;
mod train;
mod vocabulary;

pub use decode::{decode_sequence, decode_token};
pub use pairs::{count_pairs, most_common_pair, TokenPair};
pub use replace::replace_pair;
pub use sequence::text_to_sequence;
pub use source::SourceFile;
pub use train::{train_bpe, BpeTrainingResult};
pub use vocabulary::{
    FileId, FileOccurrences, Token, Vocabulary, VocabularyEntry, BASE_VOCAB_SIZE,
};