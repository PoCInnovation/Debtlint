use std::fs::{self, ReadDir, File};
use std::path::PathBuf;
use std::io::{self, BufRead};

pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
}

const CODE_EXTENSIONS: [&str; 13] = ["py", "js", "rs", "c", "cpp", "go", "java", "php", "lua", "ts", "rb", "jsx", "tsx"];

fn get_excluded_paths() -> Option<Vec<String>>
{
    let file = File::open(".gitignore").ok()?;
    let reader = io::BufReader::new(file);

    let mut path_list = Vec::new();

    for l in reader.lines(){
        let mut line = l.ok()?;

        if !line.is_empty() && !line.starts_with("#"){
            if line.starts_with('/'){
                line.remove(0);
            }
            let path = fs::canonicalize(std::env::current_dir().ok()?.join(&line)).ok()?;
            println!("ignoring {}", path.to_string_lossy().to_string());
            path_list.push(path.to_string_lossy().to_string());

        }
    }
    Some(path_list)
}

fn is_path_excluded(path: &str, excluded_paths: &Option<Vec<String>>) -> bool
{
    let Some(excluded) = excluded_paths else { return false };
    let Ok(absolute) = PathBuf::from(path).canonicalize() else { return false };

    excluded.contains(&absolute.to_string_lossy().to_string())
}

fn ingest_codebase(dir: ReadDir, excluded_paths: &Option<Vec<String>>) -> std::io::Result<()>
{
    for e in dir {
        let entry = e?;
        let is_dir = entry.file_type()?.is_dir();
        let file_name = entry.file_name();
        let file_path = entry.path();

        if let Some(name) = file_name.to_str(){
            if !is_path_excluded(file_path.to_str().unwrap(), &excluded_paths){
                if let Some(extension) = name.split('.').last(){
                    if CODE_EXTENSIONS.contains(&extension){
                        println!("ingesting {}", file_path.to_str().unwrap());
                    }

                }
                if is_dir && name.chars().next().unwrap() != '.' {
                    ingest_codebase(entry.path().read_dir()?, excluded_paths)?;
                }
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {

    let excluded_paths = get_excluded_paths();

    let dir = fs::read_dir("./")?;
    ingest_codebase(dir, &excluded_paths)?;
    Ok(())

}
