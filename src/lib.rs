use std::collections::HashMap;

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

    pub fn refresh(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("refresh");
        let url: &str = "https://b.hatena.ne.jp/entrylist/it.rss";
        let rt = Runtime::new()?;
        let response = rt.block_on(async {
            reqwest::get(url)
            .await?
            .text()
            .await
        })?;
        println!("response: {:#?}", response);
        Ok(())
    }
}
