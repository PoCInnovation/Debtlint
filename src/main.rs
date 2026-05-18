use std::path::PathBuf; // for path to a file
use clap::Parser;

#[derive(Parser, Debug)]
#[command( // binary data to the user
    name = "debtlint",
    version,
    about = "Technical debt detection via BPE-inspired pattern analysis"
)]
struct Args { // struct to recup user input
    #[arg(value_name = "FILE")] // the name of the argument is FILE
    file: PathBuf, // PathBuf is a path to a file (mandatory)
    #[arg(long, default_value_t = 1000)] // arg long with def value 1000
    vocab_size: u32, // arg is a u32 int -> for later -> size of the vocabulary
    #[arg(long, default_value_t = 2)] // arg long with def 2
    min_frequency: usize, // arg is a usize int -> for later -> frequency min for a paire to be merged
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.vocab_size {
        println!("Hello !");
    }
}