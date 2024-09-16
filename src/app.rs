mod screen;
mod entity;
mod parser;
mod table;
mod config;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use config::Config;
use screen::Screen;
use entity::Entity;
use parser::Parser;

pub struct App {
    entities: Vec<Entity>,
    screen: Screen,
}

impl App {
    pub fn new() -> Self {
        let app = App {
            entities: Vec::new(),
            screen: Screen::new(),
        };
        app
    }

    pub fn fetch_all(&self) -> Result<String, reqwest::Error> {
        let mut file = File::open("tests/fixtures/sample.xml").unwrap();
        let mut response = String::new();
        file.read_to_string(&mut response).unwrap();

        // TODO: 後で引数とかで切り替えたい
        // let url: &str = "https://game.watch.impress.co.jp/data/rss/1.0/gmw/feed.rdf";
        // let url: &str = "https://b.hatena.ne.jp/entrylist/it.rss";
        // let url: &str = "https://rss.itmedia.co.jp/rss/2.0/netlab.xml"; // 2.0
        // let rt = Runtime::new().unwrap();
        // let response = rt.block_on(async {
        //     reqwest::get(url)
        //     .await?
        //     .text()
        //     .await
        // })?;
        Ok(response)
    }

    pub fn parse_xml(&mut self, body: String) -> Result<(), quick_xml::Error> {
        let parser = Parser::new();
        let ret = parser.parse(body);

        match ret {
            Ok(entities) => {
                self.entities = entities;
                Ok(())
            }
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
        }
    }

    pub fn screen_draw(self, options: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let ret = self.screen.draw(&self.entities, options);
        match ret {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
        }
    }

    pub fn init_config(&self) -> Result<String, std::io::Error> {
        let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut config_dir = PathBuf::from(home_dir);
        config_dir.push(".config/nnm");

        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)?;
        }

        let config_file_path = config_dir.join("config.json");
        let config = Config::new();
        let config_json = serde_json::to_string_pretty(&config)?;

        let mut file = File::create(config_file_path.clone())?;
        write!(file, "{}", config_json)?;

        Ok(config_file_path.into_os_string().into_string().unwrap())
    }

    pub fn load_config(&self) -> Option<Config> {
        let exists = Config::default_config_path().try_exists();
        match exists {
            Ok(true) => {
                let config = Config::load_from_file().unwrap();
                Some(config)
            }
            Ok(false) => {
                println!("設定ファイルが見つかりませんでした。\nnnm init で初期設定を行ってください。");
                None
            }
            Err(e) => {
                println!("{:?}", e);
                None
            }
        }
    }

    pub fn add_link(&self, url: &str) -> Result<String, std::io::Error> {
        let mut config = Config::load_from_file()?;
        let ret = config.push_link(url);

        match ret {
            Ok(url) => {
                Ok(url)
            }
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
        }
    }
}
