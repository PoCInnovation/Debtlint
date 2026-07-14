mod cli;
mod config;
mod ingestion;

use clap::Parser;
use cli::Args;
use config::get_config;
use debtlint::pipeline::{BpeConfig, run_bpe};
use ingestion::ingest_codebase;

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
