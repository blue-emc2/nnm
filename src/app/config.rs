use serde::{Deserialize, Serialize};

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
            display_limit: 10,
        }
    }

    pub fn load_from_file() -> Result<Self, std::io::Error> {
        Ok(Config {
            display_limit: 10,
            bookmarks: Vec::new(),
            links: Vec::new(),
        })
    }
}
