mod decode;
mod pairs;
mod replace;
mod sequence;
mod source;
mod train;
pub mod vocabulary;

pub use decode::{decode_sequence, decode_token};
pub use pairs::{count_pairs, most_common_pair, TokenPair};
pub use replace::replace_pair;
pub use sequence::text_to_sequence;
pub use source::SourceFile;
pub use train::{encode_corpus, train_bpe, train_corpus, BpeTrainingResult, FileTokens};
pub use vocabulary::{
    FileOccurrences, Token, Vocabulary, VocabularyEntry, VocabularyExport, BASE_ALPHABET,
    BASE_VOCAB_SIZE, UNK_TOKEN, VOCAB_EXPORT_VERSION,
};