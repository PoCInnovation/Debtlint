mod decode;
mod pairs;
mod replace;
mod sequence;
mod source;
mod train;
pub mod vocabulary;

pub use decode::{decode_sequence, decode_token};
pub use pairs::{TokenPair, count_pairs, most_common_pair};
pub use replace::replace_pair;
pub use sequence::text_to_sequence;
pub use source::SourceFile;
pub use train::{BpeTrainingResult, FileTokens, encode_corpus, train_bpe, train_corpus};
pub use vocabulary::{
    BASE_ALPHABET, BASE_VOCAB_SIZE, FileOccurrences, Token, UNK_TOKEN, VOCAB_EXPORT_VERSION,
    Vocabulary, VocabularyEntry, VocabularyExport,
};
