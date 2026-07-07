use debtlint::in_out::{read_corpus, save_vocabulary, write_encoded_sequence_json};
use debtlint::pipeline::{run_bpe, BpeConfig};
use debtlint::tokenizer::{decode_sequence, SourceFile, BASE_VOCAB_SIZE};

use crate::cli::Args;

pub fn run(args: &Args) -> std::io::Result<()> {
    let content = read_corpus(&args.file)?;
    let files = vec![SourceFile {
        path: args.file.clone(),
        content: content.clone(),
    }];
    let result = run_bpe(&files,
        BpeConfig {
            vocab_size: args.vocab_size,
            min_frequency: args.min_frequency,
        },
        args.load_vocab.as_deref(),
    )?;
    if args.load_vocab.is_some() {
        println!(
            "vocabulary loaded from: {}",
            args.load_vocab.as_ref().unwrap().display()
        );
    }
    print_stats(&result, &content, args);
    if let Some(vocab_path) = &args.save_vocab {
        save_vocabulary(vocab_path, &result.vocabulary)?;
        println!("vocabulary saved to: {}", vocab_path.display());
    }
    verify_decode_roundtrip(args, &files, &result);
    Ok(())
}

fn print_stats(result: &debtlint::tokenizer::BpeTrainingResult, content: &str, args: &Args,)
{
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
}

fn verify_decode_roundtrip(args: &Args, files: &[SourceFile], result: &debtlint::tokenizer::BpeTrainingResult)
{
    let mut decode_ok = true;
    for (source, trained) in files.iter().zip(result.files.iter()) {
        let encoded_path = args
            .output_encoded
            .clone()
            .unwrap_or_else(|| source.path.with_extension("encoded.json"));
        write_encoded_sequence_json(&encoded_path, &trained.sequence)
            .expect("failed to write encoded sequence");
        println!("encoded sequence written to: {}", encoded_path.display());

        let decoded = decode_sequence(&trained.sequence, &result.vocabulary);
        let equal = decoded == source.content;
        decode_ok &= equal;
        println!("{}: decoded == content ok: {equal}", trained.path.display());
    }
    println!("all files decode ok: {decode_ok}");
}
