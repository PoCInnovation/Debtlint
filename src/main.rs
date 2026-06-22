use std::path::PathBuf;

use clap::Parser;
use debtlint::in_out::{load_vocabulary, read_corpus, save_vocabulary, write_encoded_sequence_json};
use debtlint::tokenizer::{decode_sequence, encode_corpus, train_corpus, SourceFile, BASE_VOCAB_SIZE};

#[derive(Parser, Debug)]
#[command(
    name = "debtlint",
    version,
    about = "Technical debt detection via BPE-inspired pattern analysis"
)]
struct Args {
    #[arg(value_name = "FILE")]
    file: PathBuf,
    #[arg(long, default_value_t = 1000)]
    vocab_size: u32,
    #[arg(long, default_value_t = 2)]
    min_frequency: usize,
    /// Write the encoded token sequence as JSON before decoding (default: <FILE>.encoded.json)
    #[arg(long, value_name = "PATH")]
    output_encoded: Option<PathBuf>,
    /// Save the trained vocabulary to a JSON file
    #[arg(long, value_name = "PATH")]
    save_vocab: Option<PathBuf>,
    /// Load a vocabulary from JSON and skip BPE training
    #[arg(long, value_name = "PATH", conflicts_with = "save_vocab")]
    load_vocab: Option<PathBuf>,
}

fn main() {
    let args: Args = Args::parse();
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

    let result = if let Some(vocab_path) = &args.load_vocab {
        let vocabulary = match load_vocabulary(vocab_path) {
            Ok(vocabulary) => vocabulary,
            Err(err) => {
                eprintln!("failed to load vocabulary from {}: {err}", vocab_path.display());
                std::process::exit(1);
            }
        };
        println!("vocabulary loaded from: {}", vocab_path.display());
        encode_corpus(&files, vocabulary)
    } else {
        train_corpus(&files, args.vocab_size, args.min_frequency)
    };

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
    if args.load_vocab.is_none() {
        println!("target vocab size: {}", args.vocab_size);
        println!("min pair frequency: {}", args.min_frequency);
    }
    println!("merges performed: {}", result.merges);
    println!(
        "vocabulary size: {} ({} fixed + {} dynamic + {} merged)",
        result.vocabulary.len(),
        BASE_VOCAB_SIZE,
        result.vocabulary.dynamic_symbol_count(),
        result.merges
    );

    if let Some(vocab_path) = &args.save_vocab {
        if let Err(err) = save_vocabulary(vocab_path, &result.vocabulary) {
            eprintln!(
                "failed to save vocabulary to {}: {err}",
                vocab_path.display()
            );
            std::process::exit(1);
        }
        println!("vocabulary saved to: {}", vocab_path.display());
    }

    let mut decode_ok = true;
    for (source, trained) in files.iter().zip(result.files.iter()) {
        let encoded_path = args
            .output_encoded
            .clone()
            .unwrap_or_else(|| source.path.with_extension("encoded.json"));
        if let Err(err) = write_encoded_sequence_json(&encoded_path, &trained.sequence) {
            eprintln!(
                "failed to write encoded sequence to {}: {err}",
                encoded_path.display()
            );
            std::process::exit(1);
        }
        println!("encoded sequence written to: {}", encoded_path.display());

        let decoded = decode_sequence(&trained.sequence, &result.vocabulary);
        let equal = decoded == source.content;
        decode_ok &= equal;
        println!("{}: decoded == content ok: {equal}", trained.path.display());
    }
    println!("all files decode ok: {decode_ok}");
}
