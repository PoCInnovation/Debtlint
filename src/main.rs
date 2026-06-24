mod cli;
mod debug_run;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();
    if let Err(err) = debug_run::run(&args) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
