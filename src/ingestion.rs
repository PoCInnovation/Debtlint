use crate::config::Config;
use debtlint::tokenizer::SourceFile;
use ignore::WalkBuilder;
use ignore::overrides::OverrideBuilder;
use std::fs;
use std::path::{Path, PathBuf};

fn get_file(path: &Path) -> Option<SourceFile> {
    let text = fs::read_to_string(path).ok()?;
    Some(SourceFile::from_text(PathBuf::from(path), &text))
}

/// Turn a config exclude into an `ignore` override ignore-glob (`!…`).
fn to_ignore_glob(exclude: &str) -> Option<String> {
    let pattern = exclude.trim();
    if pattern.is_empty() || pattern.starts_with('#') {
        return None;
    }
    if pattern.starts_with('!') {
        return Some(pattern.to_string());
    }
    if pattern.contains('*') || pattern.contains('?') || pattern.contains('[') {
        return Some(format!("!{pattern}"));
    }
    let pattern = pattern.trim_matches('/');
    if pattern.is_empty() {
        return None;
    }
    // Bare names are treated as directories ignored anywhere in the tree.
    Some(format!("!{pattern}/"))
}

fn build_overrides(root: &Path, excludes: &[String]) -> ignore::overrides::Override {
    let mut builder = OverrideBuilder::new(root);
    for exclude in excludes {
        let Some(glob) = to_ignore_glob(exclude) else {
            continue;
        };
        if let Err(err) = builder.add(&glob) {
            eprintln!("Warning: invalid exclude `{exclude}` ({err})");
        }
    }
    builder.build().unwrap_or_else(|err| {
        eprintln!("Warning: could not build exclude overrides ({err})");
        OverrideBuilder::new(root)
            .build()
            .expect("empty override builder")
    })
}

pub fn ingest_codebase(root: &Path, config: &Config) -> Vec<SourceFile> {
    if !root.exists() {
        eprintln!("Error: {} doesn't exist.", root.display());
        return vec![];
    }

    let overrides = build_overrides(root, &config.excludes);

    WalkBuilder::new(root)
        .follow_links(false)
        .standard_filters(true)
        .overrides(overrides)
        .build()
        .filter_map(|result| {
            let entry = result.ok()?;
            let file_type = entry.file_type()?;
            if !file_type.is_file() {
                return None;
            }
            get_file(entry.path())
        })
        .collect()
}
