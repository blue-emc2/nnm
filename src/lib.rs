pub mod parser;
mod entity;

use parser::Parser;
use tokio::runtime::Runtime;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

pub struct App {
    entities: Vec<entity::Entity>,
}

impl App {
    pub fn new() -> Self {
        App {
            entities: Vec::new(),
        }
    }

    pub fn fetch_all(&self) -> Result<String, reqwest::Error> {
        let url: &str = "https://b.hatena.ne.jp/entrylist/it.rss";
        // let url: &str = "https://rss.itmedia.co.jp/rss/2.0/netlab.xml"; // 2.0
        let rt = Runtime::new().unwrap();
        let response = rt.block_on(async {
            reqwest::get(url)
            .await?
            .text()
            .await
        })?;
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

    pub fn screen_draw(self) -> crossterm::Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        self.print_all();
        loop {
            if let Event::Key(event) = event::read()? {
                match event.code {
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
        execute!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn print_all(self) {
        for entity in self.entities {
            println!("----------------------------------------\n");
            println!("Title: \t\t\t{}", entity.title.unwrap_or_default());
            println!("URL: \t\t\t{}", entity.link.unwrap_or_default());
            println!("Description: \t\t{}", entity.description.unwrap_or_default());
            println!();
        }
    }
}
