mod cli;
mod config;
mod ingestion;

use clap::Parser;
use cli::{Args, ReportFormat};
use config::load_config;
use debtlint::algorithm::{DetectionParams, detect_duplicates, group_duplicates};
use debtlint::in_out::{format_groups_json, format_groups_text};
use debtlint::pipeline::run_bpe;
use ingestion::ingest_codebase;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let config = load_config();
    let root: PathBuf = args.dir.clone().unwrap_or_else(|| config.dir.clone());
    let files = ingest_codebase(&root, &config);
    let result = run_bpe(&files, args.get_bpe_config(), args.get_pipeline_config())?;

    let params = DetectionParams {
        shingle_size_k: args.shingle_size,
        window_size_w: args.window_size,
        min_tokens: args.min_tokens,
        ..DetectionParams::default()
    };
    let duplicates = detect_duplicates(&result, &params);
    let groups = group_duplicates(&duplicates, &files);
    match args.report_format {
        ReportFormat::Text => println!("{}", format_groups_text(&groups)),
        ReportFormat::Json => println!("{}", format_groups_json(&groups)?),
    }
    Ok(())
}
