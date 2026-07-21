mod cli;
mod config;
mod ingestion;

use clap::Parser;
use cli::Args;
use config::get_config;
use debtlint::pipeline::{BpeConfig, run_bpe};
use ingestion::ingest_codebase;

fn main() {
    let _ = Args::parse();
    if let Err(err) = debug_run::run_linter() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
