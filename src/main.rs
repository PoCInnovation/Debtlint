mod cli;
mod config;
mod debug_run;
mod ingestion;

use clap::Parser;
use cli::Args;

fn main() {
    let _ = Args::parse();
    if let Err(err) = debug_run::run_linter() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
