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

fn byte_pair_encoding(string: String, size: usize)
{
    let mut vocabulary: Vec<String> = string.chars().map(|c| c.to_string()).collect();
    let mut best_pair: HashMap<(String, String), u32> = HashMap::new();

    while vocabulary.len() < size {
        for (index, element) in vocabulary.iter().enumerate() {
            let key: (String, String) = (element.to_string(), vocabulary[index + 1].to_string()).clone();
            if best_pair.contains_key(&key) {
                match best_pair.get_mut(&key) {
                    Some(x) => { *x += 1; }
                    None => {}
                }
            }
        }
        let max_entry = best_pair.iter().max_by_key(|&(_, value)| value);
        match max_entry {
            Some((key, value)) => { vocabulary.push(key.0.clone() + &key.1.clone()); },
            None => break,
        }
        //merge les 2 key dans le vecteur de string
    }
}

fn main() {
    let args = Args::parse();
    println!("Args: {}", args.file);
    let content = FileManager::read_file(args.file.clone()).unwrap();
    byte_pair_encoding(content, args.size);
}
