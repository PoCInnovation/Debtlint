mod cli;
mod ingestion;
mod config;

use clap::Parser;
use cli::Args;
use ingestion::ingest_codebase;
use config::get_config;
use debtlint::pipeline::{BpeConfig, run_bpe};


fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let cfg = get_config();
    let files = ingest_codebase(cfg);

    let _result = run_bpe(
        &files,
        BpeConfig {
            vocab_size: args.vocab_size,
            min_frequency: args.min_frequency,
        },
        args.load_vocab.as_deref(),
    )?;
    Ok(())
}
