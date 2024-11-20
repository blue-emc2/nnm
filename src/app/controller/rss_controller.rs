use std::{collections::HashMap, io::{self, ErrorKind}};

use crate::app::{
    config::Config, entity::Entity, file::File, history::History, parser::Parser, prompt::Prompt,
    screen,
};
use tokio::runtime::Runtime;

pub struct RssController {
    entities: Vec<Entity>,
}

impl Default for RssController {
    fn default() -> Self {
        RssController {
            entities: Vec::new(),
        }
    }
}

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

    pub fn index(&mut self, options: HashMap<String, String>) {
        let config: Config = match Config::new().load_from_file() {
            Ok(config) => config,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                eprintln!(
                    "設定ファイルが見つかりませんでした。\nnnm init で初期設定を行ってください。"
                );
                return;
            }
            Err(e   ) => {
                eprintln!(
                    "エラーが発生しました。\n{}", e
                );
                return;
            }
        };

        let links = config.links().clone();
        let rt = Runtime::new().unwrap();

        let results = rt.block_on(async {
            let tasks = links
                .into_iter()
                .map(|link| {
                    tokio::spawn(async move {
                        #[cfg(debug_assertions)]
                        {
                            println!(
                                "- start fetch task {} : {:?}",
                                link,
                                std::thread::current().id()
                            );
                        }
                        let res = Self::fetch_rss(link.clone()).await;
                        #[cfg(debug_assertions)]
                        {
                            println!(
                                "- end fetch task {} : {:?}",
                                link,
                                std::thread::current().id()
                            );
                        }
                        res
                    })
                })
                .collect::<Vec<_>>();

            let results = futures::future::join_all(tasks).await;

            let fetched_data: Vec<String> = results
                .into_iter()
                .filter_map(|res| res.ok())
                .filter_map(|res| res.ok())
                .collect();

            fetched_data
        });

        if let Err(e) = self.parse_xml(results, config) {
            println!("Error parsing XML: {:#?}", e);
            return;
        }

        // 新しい記事のみを表示するためにここでフィルタリングするが、
        // filter_new_entitiesを常に呼ばないといけないのでなんだかいけてない
        let new_entity_size = self.filter_new_entities();
        if new_entity_size == 0 {
            println!("新しい記事はありません。");
            return;
        } else {
            let screen = screen::Screen::new();
            screen.draw(&self.entities, options);
        }

        if let Err(e) = self.save_history() {
            println!("Error saving history: {:#?}", e);
            return;
        }
    }

    async fn fetch_rss(url: String) -> Result<String, reqwest::Error> {
        // let mut file = File::open("tests/fixtures/sample.xml").unwrap();
        // let mut response = String::new();
        // file.read_to_string(&mut response).unwrap();

        // TODO: 後で引数とかで切り替えたい
        // let url: &str = "https://game.watch.impress.co.jp/data/rss/1.0/gmw/feed.rdf";
        // let url: &str = "https://b.hatena.ne.jp/entrylist/it.rss";
        // let url: &str = "https://rss.itmedia.co.jp/rss/2.0/netlab.xml"; // 2.0

        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        Ok(body)
    }

    pub fn parse_xml(
        &mut self,
        bodys: Vec<String>,
        config: Config,
    ) -> Result<(), quick_xml::Error> {
        let parser = Parser::new();
        let chunk_size = config.chunk_size();

        for body in bodys {
            let ret = parser.parse(body);
            match ret {
                Ok(entities) => {
                    let mut chunks = entities
                        .into_iter()
                        .take(chunk_size.try_into().unwrap())
                        .collect();
                    self.entities.append(&mut chunks);
                }
                Err(e) => {
                    println!("{:?}", e);
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    fn save_history(&self) -> Result<(), std::io::Error> {
        let history: Result<History, io::Error> = History::new().load_from_file();

        match history {
            Ok(mut history) => {
                for body in self.entities.iter() {
                    let entity = Entity {
                        entity_type: body.entity_type.clone(),
                        title: body.title.clone(),
                        link: body.link.clone(),
                        description: body.description.to_string(),
                        pub_date: None,
                    };
                    history.entity_push(entity);
                }

                history.update_last_fetched_date();
                history.save_to_file(history.clone())?;

                Ok(())
            }
            Err(e) => {
                eprintln!("履歴ファイルが見つかりませんでした。\nhistory.jsonを再作成します。");
                let history = History::new();
                history.save_to_file(history.clone())?;
                Err(e)
            }
        }
    }

    fn filter_new_entities(&mut self) -> u16 {
        let history: Result<History, io::Error> = History::new().load_from_file();
        match history {
            Ok(history) => {
                let mut new_entities: Vec<Entity> = Vec::new();
                for body in self.entities.iter() {
                    if !history.get_entities().iter().any(|h| h.link == body.link) {
                        new_entities.push(body.clone());
                    }
                }
                self.entities = new_entities;
                self.entities.len() as u16
            }
            Err(e) => {
                eprintln!("Error loading history: {:?}", e);
                0
            }
        }
    }
}
