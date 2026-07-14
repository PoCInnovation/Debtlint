use crate::config::Config;
use debtlint::tokenizer::SourceFile;
use std::fs::{self, File, ReadDir};
use std::io::{self, BufRead};
use std::path::Path;
use std::path::PathBuf;

const COMMENT_CHARS: [&str; 2] = ["#", "//"];

fn get_file(path: &str) -> Option<SourceFile> {
    let file = File::open(path).ok()?;
    let reader = io::BufReader::new(file);
    let mut content = String::from("");
    for l in reader.lines() {
        let line = l.ok()?;
        if COMMENT_CHARS.iter().any(|c| line.trim().starts_with(c)) {
            continue;
        };
        let normalized = line.split_whitespace().collect::<Vec<_>>().join(" ");
        content += &normalized;
    }
    Some(SourceFile {
        path: PathBuf::from(path),
        content: content,
    })
}

fn get_excluded_paths(excluded: Vec<String>) -> Vec<String> {
    let Ok(cwd) = std::env::current_dir() else {
        return Vec::new();
    };
    excluded
        .iter()
        .filter_map(|l| {
            let canonical = fs::canonicalize(cwd.join(l)).ok()?;
            Some(canonical.to_string_lossy().to_string())
        })
        .collect()
}

fn is_path_excluded(path: &str, excluded_paths: &Vec<String>) -> bool {
    let Ok(absolute) = PathBuf::from(path).canonicalize() else {
        return false;
    };
    excluded_paths.contains(&absolute.to_string_lossy().to_string())
}

fn is_inside_folder(path: &Path, folder: &Path) -> bool {
    let (Ok(canonical_path), Ok(canonical_folder)) = (path.canonicalize(), folder.canonicalize())
    else {
        return false;
    };
    canonical_path.starts_with(&canonical_folder)
}

fn collect_source_files(src: String, excluded: Vec<String>) -> Vec<SourceFile> {
    let mut codebase: Vec<SourceFile> = vec![];
    let Ok(output) = std::process::Command::new("git").arg("ls-files").output() else {
        return vec![];
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .for_each(|path| {
            if is_path_excluded(path, &excluded) {
                return;
            }
            if !is_inside_folder(Path::new(path), Path::new(&src)) {
                return;
            }
            let Some(source_file) = get_file(path) else {
                return;
            };
            codebase.push(source_file);
        });
    codebase
}

fn collect_source_files_fallback(
    dir: ReadDir,
    excluded_paths: &Vec<String>,
    codebase: &mut Vec<SourceFile>,
) {
    for entry in dir.flatten() {
        let file_path = entry.path();
        let file_name = entry.file_name();
        let Some(file_path_str) = file_path.to_str() else {
            continue;
        };
        let Some(file_name_str) = file_name.to_str() else {
            continue;
        };
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if is_path_excluded(file_path_str, excluded_paths) {
            continue;
        };
        if file_type.is_dir() && file_name_str.chars().next().unwrap() != '.' {
            let Ok(dir) = entry.path().read_dir() else {
                continue;
            };
            collect_source_files_fallback(dir, excluded_paths, codebase);
            continue;
        }
        let Some(source_file) = get_file(file_path_str) else {
            continue;
        };
        codebase.push(source_file);
    }
}

pub fn ingest_codebase(cfg: Config) -> Vec<SourceFile> {
    let mut codebase: Vec<SourceFile> = vec![];
    let is_git_repo = std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let excluded_paths = get_excluded_paths(cfg.excludes);
    if is_git_repo {
        codebase = collect_source_files(cfg.src, excluded_paths);
    } else {
        let Ok(dir) = fs::read_dir(cfg.src) else {
            return vec![];
        };
        collect_source_files_fallback(dir, &excluded_paths, &mut codebase);
    }
    codebase
}
