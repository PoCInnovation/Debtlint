use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE: &str = "debtlint.config.json";

fn default_dir() -> PathBuf {
    PathBuf::from(".")
}

fn default_excludes() -> Vec<String> {
    [
        "build",
        "target",
        "node_modules",
        "pycache",
        "__pycache__",
        "*.json",
        "*.jsonl",
        "*.lock",
        "*.csv",
        "*.md",
        "*.markdown",
        "*.txt",
        "*.min.js",
        "*.min.css",
        "*.min.html",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

/// Project analysis settings loaded from `debtlint.config.json` when present.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Config {
    /// Directory to analyze (same role as the CLI `DIR` argument).
    #[serde(default = "default_dir", alias = "src")]
    pub dir: PathBuf,
    /// Directory names and extension globs to skip (gitignore-style).
    #[serde(default = "default_excludes")]
    pub excludes: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dir: default_dir(),
            excludes: default_excludes(),
        }
    }
}

/// Load `debtlint.config.json` from the current directory, or the built-in defaults.
pub fn load_config() -> Config {
    match fs::read_to_string(CONFIG_FILE) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|err| {
            eprintln!("Warning: invalid {CONFIG_FILE} ({err}); using defaults");
            Config::default()
        }),
        Err(_) => Config::default(),
    }
}
