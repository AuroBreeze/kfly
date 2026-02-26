use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub settings: Settings,
    pub workflow: Vec<Task>,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub kernel_root: PathBuf,
    pub test_email: String,
}

#[derive(Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub interactive: bool,
    pub fail_fast: bool,
}

impl Config {
    /// Load and parse the kfly.toml file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
