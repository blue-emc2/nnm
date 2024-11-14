use std::{collections::HashMap, io};

use crate::app::{config::Config, file::File, prompt::Prompt};

pub struct RssController;

impl Prompt for RssController {
    fn exec_delete_link(&self, url: &str) {
        // 本当はFileManager的な構造体を使うときれいかも？
        // let config = FileManager::load(config);
        // FileManager::save(config);
        let mut config: Config = Config::new().load_from_file().unwrap();
        let links = config.mut_links();
        let index = links.iter().position(|x| x == url).unwrap();
        links.remove(index);
        config.save_to_file(config.clone()).unwrap();
    }
}

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

    pub fn delete_link(&self) -> Result<(), io::Error> {
        let mut config: Config = Config::new().load_from_file()?;
        self.delete_prompt(config.mut_links());
        Ok(())
    }

    pub fn show(&self) -> Result<(), io::Error> {
        let config: Config = Config::new().load_from_file()?;
        for link in config.links() {
            println!("{}", link);
        }
        Ok(())
    }

    pub fn run(&self, options: HashMap<String, String>) {
        println!("Running RSS with options: {:#?}", options);
    }
}
