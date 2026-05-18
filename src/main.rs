use std::path::PathBuf; // for path to a file
use clap::Parser;
use debtlint::in_out::read_corpus; // for read the file
use debtlint::tokenizer::{count_pairs, most_common_pair, text_to_sequence, Token};

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
    let args: Args = Args::parse(); // fill Args

    let corpus: String = match read_corpus(&args.file) { // read the file
        Ok(content) => content, // if the file is readed return the content
        Err(err) => { // else error
            eprintln!("failed to read {}: {err}", args.file.display());
            std::process::exit(1); // exit the program with code 1 (clean !)
        }
    };

    let sequence: Vec<Token> = text_to_sequence(&corpus); // convert the text to a sequence of token

    println!("corpus: {}", args.file.display());
    println!("characters: {}", corpus.chars().count());
    println!("bytes: {}", corpus.len());
    println!("initial tokens: {}", sequence.len());
    println!("target vocab size: {}", args.vocab_size);
    println!("min pair frequency: {}", args.min_frequency);

    let pair_counts = count_pairs(&sequence);
    println!("distinct pairs: {}", pair_counts.len());
    match most_common_pair(&pair_counts) {
        Some(((left, right), frequency)) => {
            println!("top pair: ({left}, {right})");
            println!("top pair frequency: {frequency}");
            if frequency < args.min_frequency {
                println!("top pair below min frequency —>  no merge would start");
            }
        }
        None => println!("no adjacent pairs in sequence"),
    }
}