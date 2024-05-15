mod parser;
mod entity;

use parser::Parser;
use tokio::runtime::Runtime;

pub struct App {
    // ...
}

impl App {
    pub fn new() -> Self {
        App {
            // ...
        }
    }

    fn init(&self) {
        // ...
    }

    pub fn fetch_all(&self) -> Result<String, reqwest::Error> {
        let url: &str = "https://b.hatena.ne.jp/entrylist/it.rss";
        let rt = Runtime::new().unwrap();
        let response = rt.block_on(async {
            reqwest::get(url)
            .await?
            .text()
            .await
        })?;
        Ok(response)
    }

    pub fn parse_xml(&self, body: String) {
        let parser = Parser::new();
        let entity = parser.parse(body);

        println!("entity: {:#?}", entity);
    }
}
