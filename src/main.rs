mod cli;
mod config;
mod ingestion;

use clap::Parser;
use cli::Args;
use config::get_config;
use debtlint::duplication::{DetectionParams, DuplicateGroup, detect_duplicates, group_duplicates};
use debtlint::pipeline::run_bpe;
use ingestion::ingest_codebase;
use std::io::{Error, ErrorKind};

fn print_groups(groups: &[DuplicateGroup]) {
    println!("{} duplicated block(s) found", groups.len());
    for (number, group) in groups.iter().enumerate() {
        println!(
            "\n#{} - {} copies (up to {} chars)",
            number + 1,
            group.instances.len(),
            group.max_chars()
        );
        for instance in &group.instances {
            match &instance.lines {
                Some(lines) => println!(
                    "  {}:{}-{}",
                    instance.path.display(),
                    lines.start(),
                    lines.end()
                ),
                None => println!(
                    "  {} (chars {}..{})",
                    instance.path.display(),
                    instance.chars.start,
                    instance.chars.end
                ),
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    if args.k == 0 || args.w == 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "--k and --w must be at least 1",
        ));
    }
    let cfg = get_config();
    let files = ingest_codebase(cfg);

    let result = run_bpe(&files, args.get_bpe_config(), args.get_pipeline_config())?;

    let params = DetectionParams {
        k: args.k,
        w: args.w,
        min_tokens: args.min_tokens,
        ..DetectionParams::default()
    };
    let duplicates = detect_duplicates(&result, &params);
    print_groups(&group_duplicates(&duplicates, &files));
    Ok(())
}