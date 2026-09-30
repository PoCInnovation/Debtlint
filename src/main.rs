mod cli;
mod config;
mod ingestion;

use clap::Parser;
use cli::Args;
use config::get_config;
use debtlint::duplication::{DetectionParams, DuplicateGroup, detect_duplicates, group_duplicates};
use debtlint::pipeline::run_bpe;
use ingestion::ingest_codebase;

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
    let cfg = get_config();
    let files = ingest_codebase(cfg);
    let result = run_bpe(&files, args.get_bpe_config(), args.get_pipeline_config())?;

    let params = DetectionParams {
        shingle_size_k: args.shingle_size,
        window_size_w: args.window_size,
        min_tokens: args.min_tokens,
        ..DetectionParams::default()
    };
    let duplicates = detect_duplicates(&result, &params);
    print_groups(&group_duplicates(&duplicates, &files));
    Ok(())
}
