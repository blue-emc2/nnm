use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use std::io::Write;

const DEFAULT_DISPLAY_LIMIT: i32 = 10;
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    display_limit: i32,
    bookmarks: Vec<String>, // お気に入り一覧
    links: Vec<String>,     // rssのリンク一覧
}

impl Config {
    pub fn new() -> Self {
        Config {
            bookmarks: Vec::new(),
            links: Vec::new(),
            display_limit: DEFAULT_DISPLAY_LIMIT,
        }
    }

    pub fn default_config_path() -> PathBuf {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut config_dir = PathBuf::from(home_dir);
        config_dir.push(".config/nnm/config.json");
        config_dir
    }

    pub fn load_from_file() -> Result<Self, std::io::Error> {
        let path = Config::default_config_path();
        let config = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        Ok(config)
    }

    pub fn push_link(&mut self, url: &str) -> Result<String, std::io::Error> {
        if !self.links.contains(&url.to_string()) {
            self.links.push(url.to_string());
            self.save_to_file()?;
        }
        Ok(url.to_string())
    }

    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let path = Config::default_config_path();
        let config_json = serde_json::to_string_pretty(&self)?;
        let mut file = std::fs::File::create(path)?;
        write!(file, "{}", config_json)?;
        Ok(())
    }
}
