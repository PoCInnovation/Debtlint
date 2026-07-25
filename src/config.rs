use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum Output {
    #[default]
    Json,
    Table,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub src: String,
    pub excludes: Vec<String>,
    #[serde(default)]
    #[allow(dead_code)]
    output: Output,
}

pub fn get_config() -> Config {
    let content =
        fs::read_to_string("debtlint.config.json").expect("Cannot read debtlint.config.json");
    let config: Config = serde_json::from_str(&content).expect("invalid JSON");
    config
}
