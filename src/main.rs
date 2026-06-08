use std::path::PathBuf; // for path to a file
use clap::Parser;
use debtlint::in_out::read_corpus; // for read the file
use debtlint::tokenizer::{decode_sequence, train_corpus, SourceFile, BASE_VOCAB_SIZE};

#[derive(Parser, Debug)]
#[command(
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

    let content = match read_corpus(&args.file) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("failed to read {}: {err}", args.file.display());
            std::process::exit(1);
        }
    };

    let files = vec![SourceFile {
        path: args.file.clone(),
        content: content.clone(),
    }];

    let result = train_corpus(&files, args.vocab_size, args.min_frequency);
    let initial_tokens = result.initial_token_count;
    let encoded_tokens = result.encoded_token_count();
    let compression = if initial_tokens > 0 {
        (1.0 - encoded_tokens as f64 / initial_tokens as f64) * 100.0
    } else {
        0.0
    };

    println!("source files: {}", result.files.len());
    println!("characters: {}", content.chars().count());
    println!("bytes: {}", content.len());
    println!("initial tokens: {initial_tokens}");
    println!("encoded tokens: {encoded_tokens}");
    println!("compression ratio: {compression:.1}%");
    println!("target vocab size: {}", args.vocab_size);
    println!("min pair frequency: {}", args.min_frequency);
    println!("merges performed: {}", result.merges);
    println!(
        "vocabulary size: {} ({} base + {} merged)",
        result.vocabulary.len(),
        BASE_VOCAB_SIZE,
        result.merges
    );

    let mut decode_ok = true;
    for (source, trained) in files.iter().zip(result.files.iter()) {
        let decoded = decode_sequence(&trained.sequence, &result.vocabulary);
        let equal = decoded == source.content;
        decode_ok &= equal;
        println!("{}: decoded == content ok: {equal}", trained.path.display());
    }
    println!("all files decode ok: {decode_ok}");
}