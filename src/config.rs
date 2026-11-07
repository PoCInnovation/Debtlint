use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const DEBTLINT_DIR: &str = ".debtlint";
pub const CONFIG_FILE_NAME: &str = "debtlint.config.json";
pub const VOCAB_FILE_NAME: &str = "debtlint.vocab.json";

/// Legacy config path at the repository root (still accepted).
const LEGACY_CONFIG_FILE: &str = "debtlint.config.json";

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

/// Project analysis settings loaded from `.debtlint/debtlint.config.json` when present.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

pub fn debtlint_dir() -> PathBuf {
    PathBuf::from(DEBTLINT_DIR)
}

pub fn default_config_path() -> PathBuf {
    debtlint_dir().join(CONFIG_FILE_NAME)
}

pub fn default_vocab_path() -> PathBuf {
    debtlint_dir().join(VOCAB_FILE_NAME)
}

fn parse_config(path: &Path, content: &str) -> Config {
    serde_json::from_str(content).unwrap_or_else(|err| {
        eprintln!(
            "Warning: invalid {} ({err}); using defaults",
            path.display()
        );
        Config::default()
    })
}

fn write_config(path: &Path, config: &Config) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let json = serde_json::to_string_pretty(config)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    fs::write(path, format!("{json}\n"))
}

/// Load config from `.debtlint/debtlint.config.json`.
/// On first run, writes the default config there (migrating a legacy root file if present).
pub fn load_config() -> Config {
    let primary = default_config_path();
    if let Ok(content) = fs::read_to_string(&primary) {
        return parse_config(&primary, &content);
    }

    let legacy = Path::new(LEGACY_CONFIG_FILE);
    let config = if let Ok(content) = fs::read_to_string(legacy) {
        eprintln!(
            "Migrating legacy {} -> {}",
            legacy.display(),
            primary.display()
        );
        parse_config(legacy, &content)
    } else {
        Config::default()
    };

    if let Err(err) = write_config(&primary, &config) {
        eprintln!(
            "Warning: could not create {} ({err})",
            primary.display()
        );
    } else {
        eprintln!("Config written to: {}", primary.display());
    }
    config
}
