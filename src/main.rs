use clap::{Parser};
use dict::{ Dict };
use file_manager::FileManager;
use std::collections::HashMap;
mod file_manager;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to read:
    #[arg(short, long)]
    file: String,

    /// Number of word we want in our vocabulary;
    #[arg(short, long)]
    size: usize
}

fn merge_same<T: PartialEq + Clone>(vec: &mut Vec<T>, a: &T, b: &T, merged: T)
{
    let mut i = 0;

    while i < (vec.len() - 1) {
        if vec[i] == *a && vec[i + 1] == *b {
            vec[i] = merged.clone();
            vec.remove(i + 1);
        } else {
            i += 1;
        }
    }
}

fn byte_pair_encoding(string: String, size: usize)
{
    let mut vocabulary: HashMap<usize, String> = HashMap::new();
    let mut encoded: Vec<String> = string.chars().map(|c| c.to_string()).collect();
    let mut best_pair: HashMap<(String, String), u32> = HashMap::new();

    while vocabulary.len() < size {
        best_pair.clear();
        for (index, element) in encoded.iter().enumerate() {
            if index + 1 >= encoded.len() {
                continue;
            }
            let key = (element.to_string(), encoded[index + 1].to_string());
            *best_pair.entry(key).or_insert(0) += 1;
        }
        let max_entry = best_pair.iter().max_by_key(|&(_, value)| value);
        match max_entry {
            Some((key, _)) => {
                let merged = key.0.clone() + &key.1;
                merge_same(&mut encoded, &key.0, &key.1, merged.clone());
                if !vocabulary.values().any(|v| v == &merged) {
                    vocabulary.insert(vocabulary.len() + 1, merged);
                }
            },
            None => break,
        }
    }
    println!("{:?}", vocabulary);
}

fn main() {
    let args = Args::parse();
    println!("Args: {}", args.file);
    let content = FileManager::read_file(args.file.clone()).unwrap();
    byte_pair_encoding(content, args.size);
}
