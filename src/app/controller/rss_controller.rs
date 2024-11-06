use std::{collections::HashMap, io};

use crate::app::{config::Config, file::File};

pub struct RssController;

impl RssController {
    pub fn add_link(&self, url: &str) -> Result<String, io::Error> {
        let mut config: Config = Config::new().load_from_file()?;
        let links = config.links();
        if links.contains(&url.to_string()) {
            return Ok(url.to_string());
        }
        let links = config.mut_links();
        links.push(url.to_string());
        config.save_to_file(config.clone())?;
        Ok(url.to_string())
    }

    pub fn show(&self) -> Result<(), io::Error> {
        let config: Config = Config::new().load_from_file()?;
        for link in config.links() {
            println!("{}", link);
        }
        Ok(())
    }

    pub fn delete_in_prompt(&self) {
        // Implement the logic to delete rss in prompt
    }

    pub fn run(&self, options: HashMap<String, String>) {
        println!("Running RSS with options: {:#?}", options);
    }
}
