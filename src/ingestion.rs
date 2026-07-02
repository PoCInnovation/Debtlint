use std::fs::{self, ReadDir, File};
use std::path::{PathBuf};
use std::io::{self, BufRead};
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
}
const COMMENT_CHARS: [&str; 2] = ["#", "//"];


fn get_file(path: &str) -> Option<SourceFile>
{
    let file = File::open(path).ok()?;
    let reader = io::BufReader::new(file);

    let mut content = String::from("");
    for l in reader.lines(){
        let line = l.ok()?;

        if COMMENT_CHARS.iter().any(|c| line.trim().starts_with(c)){continue};
        let normalized = line.split_whitespace().collect::<Vec<_>>().join(" ");
        content += &normalized;
    }
    Some(SourceFile{
        path: PathBuf::from(path),
        content: content
    })
}
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

fn collect_source_files() -> Vec<SourceFile>
{
    let mut codebase:  Vec<SourceFile> = vec![];

    let Ok(output) = std::process::Command::new("git")
    .arg("ls-files")
    .output() else {return vec![]};

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .for_each(|path| {
            let Some(source_file) = get_file(path) else {return};
            codebase.push(source_file);

        });
    codebase
}

fn collect_source_files_fallback(dir: ReadDir, excluded_paths: &Option<Vec<String>>,  codebase:  &mut Vec<SourceFile>)
{
    for entry in dir.flatten() {
        let file_path = entry.path();
        let file_name = entry.file_name();


        let Some(file_path_str) = file_path.to_str() else {continue};
        let Some(file_name_str) = file_name.to_str() else {continue};

        let Ok(file_type) = entry.file_type() else {continue};

        if is_path_excluded(file_path_str, &excluded_paths) {continue};


        if file_type.is_dir() && file_name_str.chars().next().unwrap() != '.' {
            let Ok(dir) = entry.path().read_dir() else {continue};
            collect_source_files_fallback(dir, excluded_paths, codebase);
        }
        let Some(source_file) = get_file(file_path_str) else {continue};
        codebase.push(source_file);
    }
}

pub fn ingest_codebase() -> Vec<SourceFile>{

    let mut codebase:  Vec<SourceFile> = vec![];
    let is_git_repo = std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if is_git_repo{
        codebase = collect_source_files();
    } else {
        let excluded_paths = get_excluded_paths();
        let Ok(dir) = fs::read_dir("./") else {return vec![]};
        collect_source_files_fallback(dir, &excluded_paths, &mut codebase);
    }
    codebase
}
