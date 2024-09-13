mod entity;
mod table;
pub mod parser;
pub mod screen;

use parser::Parser;
use screen::Screen;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub struct App {
    entities: Vec<entity::Entity>,
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
        let ret = self.screen.draw2(&self.entities, options);
        match ret {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("{:?}", e);
                Err(e)
            }
        }
    }
}
