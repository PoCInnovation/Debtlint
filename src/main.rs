mod cli;
mod config;
mod ingestion;

use clap::Parser;
use cli::Args;
use config::get_config;
use debtlint::pipeline::run_bpe;
use ingestion::ingest_codebase;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let cfg = get_config();
    let files = ingest_codebase(cfg);

    let _ = run_bpe(&files,
        args.get_bpe_config(), args.get_pipeline_config())?;
    Ok(())
}
