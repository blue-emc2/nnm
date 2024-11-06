use std::io;

use crate::app::{config::Config, file::File};

pub struct BookmarkController;

impl BookmarkController {
    pub fn add_link(&self, url: &str) -> Result<String, io::Error> {
        let mut config: Config = Config::new().load_from_file()?;
        let bookmarks = config.bookmarks();

        if bookmarks.contains(&url.to_string()) {
            return Ok(url.to_string());
        }
        let bookmarks = config.mut_bookmarks();
        bookmarks.push(url.to_string());
        config.save_to_file(config.clone())?;
        Ok(url.to_string())
    }

    pub fn show(&self) -> Result<(), io::Error> {
        let config: Config = Config::new().load_from_file()?;
        for link in config.bookmarks() {
            println!("{}", link);
        }
        Ok(())
    }

    pub fn delete_in_prompt(&self) {
        // Implement the logic to delete bookmark in prompt
    }
}
