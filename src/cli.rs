use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "debtlint",
    version,
    about = "Technical debt detection via BPE-inspired pattern analysis"
)]
pub struct Args {
    #[arg(value_name = "FILE")]
    pub file: PathBuf,
    #[arg(long, default_value_t = 1000)]
    pub vocab_size: u32,
    #[arg(long, default_value_t = 2)]
    pub min_frequency: usize,
    /// write the encoded token sequence in json before decode (default <FILE>.encoded.json)
    #[arg(long, value_name = "PATH")]
    pub output_encoded: Option<PathBuf>,
    /// save the trained vocabulary in json file
    #[arg(long, value_name = "PATH")]
    pub save_vocab: Option<PathBuf>,
    /// load a vocabulary from json and skip bpe training
    #[arg(long, value_name = "PATH", conflicts_with = "save_vocab")]
    pub load_vocab: Option<PathBuf>,
}
