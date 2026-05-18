use std::path::PathBuf; // for path to a file
use clap::Parser;
use debtlint::in_out::read_corpus; // for read the file
use debtlint::tokenizer::{text_to_sequence, train_bpe, BASE_VOCAB_SIZE};

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

    let initial_tokens: usize = text_to_sequence(&corpus).len(); // convert the text to a sequence of token and return the length of the sequence
    let result = train_bpe(text_to_sequence(&corpus), args.vocab_size, args.min_frequency); // train the BPE and return the result
    let compression = if initial_tokens > 0 {
        (1.0 - result.sequence.len() as f64 / initial_tokens as f64) * 100.0 // return the compression ratio
    } else {
        0.0 // if the initial tokens is 0 return 0
    };

    println!("corpus: {}", args.file.display());
    println!("characters: {}", corpus.chars().count());
    println!("bytes: {}", corpus.len());
    println!("initial tokens: {initial_tokens}");
    println!("encoded tokens: {}", result.sequence.len());
    println!("compression ratio: {compression:.1}%");
    println!("target vocab size: {}", args.vocab_size);
    println!("min pair frequency: {}", args.min_frequency);
    println!("merges performed: {}", result.merges);
    println!("vocabulary size: {} ({} base + {} merged)",
        BASE_VOCAB_SIZE + result.merges, BASE_VOCAB_SIZE, result.merges);
}