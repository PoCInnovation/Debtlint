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
        // Build artifacts and dependencies
        "target",
        "node_modules",
        "vendor",
        "bower_components",
        "jspm_packages",
        "dist",
        "build",
        // Debtlint
        ".debtlint",
        // Python environments and caches
        ".venv",
        "venv",
        "__pycache__",
        ".pytest_cache",
        ".mypy_cache",
        ".ruff_cache",
        ".tox",
        ".nox",
        // JavaScript / frontend frameworks
        ".next",
        ".nuxt",
        ".output",
        ".svelte-kit",
        ".angular",
        ".parcel-cache",
        ".turbo",
        ".vercel",
        "storybook-static",
        // Generic caches and generated reports
        ".cache",
        "coverage",
        "htmlcov",
        // Other build systems
        ".gradle",
        "Pods",
        "Carthage",
        "DerivedData",
        // Data and configuration files
        "*.json",
        "*.jsonl",
        "*.yaml",
        "*.yml",
        "*.ipynb",
        "*.csv",
        "*.tsv",
        "*.lock",
        // Documentation
        "*.md",
        "*.markdown",
        "*.rst",
        "*.adoc",
        "*.txt",
        // Generated and minified files
        "*.min.js",
        "*.min.css",
        "*.min.html",
        "*.map",
        // Logs and compiled Python files
        "*.log",
        "*.pyc",
        "*.pyo",
        // Binary files and datasets
        "*.svg",
        "*.png",
        "*.jpg",
        "*.jpeg",
        "*.gif",
        "*.webp",
        "*.ico",
        "*.pdf",
        "*.parquet",
        "*.db",
        "*.sqlite",
        "*.sqlite3",
        "*.wasm",
        "*.zip",
        "*.gz",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

/// Project settings from `.debtlint/debtlint.config.json`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    /// Scan root (same role as CLI `DIR`).
    #[serde(default = "default_dir", alias = "src")]
    pub dir: PathBuf,
    /// Directories and globs to skip.
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

fn config_path_in(root: &Path) -> PathBuf {
    root.join(DEBTLINT_DIR).join(CONFIG_FILE_NAME)
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
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(config)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    fs::write(path, format!("{json}\n"))
}

/// Load config relative to `root` (creates `.debtlint/` on first run).
pub fn load_config_from(root: &Path) -> Config {
    let primary = if root == Path::new(".") {
        default_config_path()
    } else {
        config_path_in(root)
    };
    if let Ok(content) = fs::read_to_string(&primary) {
        return parse_config(&primary, &content);
    }

    let legacy = root.join(LEGACY_CONFIG_FILE);
    let config = if let Ok(content) = fs::read_to_string(&legacy) {
        eprintln!(
            "Migrating legacy {} -> {}",
            legacy.display(),
            primary.display()
        );
        parse_config(&legacy, &content)
    } else {
        Config::default()
    };

    if let Err(err) = write_config(&primary, &config) {
        eprintln!("Warning: could not create {} ({err})", primary.display());
    } else {
        eprintln!("Config written to: {}", primary.display());
    }
    config
}

/// Load `.debtlint/debtlint.config.json`, creating defaults on first run.
pub fn load_config() -> Config {
    load_config_from(Path::new("."))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("debtlint-{label}-{nanos}"));
        fs::create_dir_all(&root).expect("temp root");
        root
    }

    #[test]
    fn migrates_legacy_config_into_debtlint_dir() {
        let root = temp_root("migrate");
        let legacy = root.join(LEGACY_CONFIG_FILE);
        fs::write(
            &legacy,
            r#"{ "dir": "src", "excludes": ["target", "*.lock"] }"#,
        )
        .expect("write legacy");

        let config = load_config_from(&root);
        let migrated = root.join(DEBTLINT_DIR).join(CONFIG_FILE_NAME);

        assert_eq!(config.dir, PathBuf::from("src"));
        assert_eq!(
            config.excludes,
            vec!["target".to_string(), "*.lock".to_string()]
        );
        assert!(migrated.is_file());

        let _ = fs::remove_dir_all(&root);
    }
}
