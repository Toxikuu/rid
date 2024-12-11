// config.rs
// responsible for parsing the config.toml

use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub behavior: BehaviorConfig,
    pub colors: ColorsConfig,
    pub linkval: LinkvalConfig,
    pub upstream: UpstreamConfig,
}

#[derive(Deserialize, Debug)]
pub struct LinkvalConfig {
    pub retry_count: u8,
    pub stack_size: usize,
    pub thread_count: usize,
}

#[derive(Deserialize, Debug)]
pub struct UpstreamConfig {
    pub retry_count: u8,
    pub stack_size: usize,
    pub thread_count: usize,
}

#[derive(Deserialize, Debug)]
pub struct ColorsConfig {
    pub danger: String,
    pub default: String,
    pub message: String,
    pub prompt: String,
    pub stderr: String,
    pub verbose: String,
}

#[derive(Deserialize, Debug)]
pub struct BehaviorConfig {
    pub remove_tarballs: bool,
    pub search_threshold: usize,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;

        Ok(config)
    }
}

lazy_static! {
    pub static ref CONFIG: Arc<Config> = Arc::new(
        Config::load("/rid/config.toml").expect("Failed to load config.toml")
    );
}
