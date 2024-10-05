use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::app::Entity;

use super::file::File;

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct History {
    last_fetched_date: String,
    entity: Vec<Entity>,
}

impl File for History {
    fn file_path(&self) -> PathBuf {
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut config_dir = PathBuf::from(home_dir);
        config_dir.push(".config/nnm/history.json");
        config_dir
    }
}

impl History {
    pub fn new() -> Self {
        History {
            last_fetched_date: "".to_string(),
            entity: Vec::new(),
        }
    }

    // メソッドをここに追加
}
