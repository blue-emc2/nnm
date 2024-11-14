pub mod config;
pub mod controller;
pub mod prompt;

mod screen;
mod entity;
mod parser;
mod table;
mod history;
mod file;

use controller::{
    rss_controller::RssController,
    bookmark_controller::BookmarkController,
    history_controller::HistoryController,
    config_controller::ConfigController,
};
use file::File;
use history::History;
use tokio::runtime::Runtime;
use std::collections::HashMap;
use std::io::{stdin, stdout, Error, Write};
use std::result::Result;
use config::Config;
use screen::Screen;
use entity::Entity;
use parser::Parser;

pub struct App {
    entities: Vec<Entity>,
    screen: Screen,
    pub rss: RssController,
    pub bookmark: BookmarkController,
    pub history: HistoryController,
    pub config: ConfigController,
}

impl App {
    pub fn new() -> Self {
        let app = App {
            entities: Vec::new(),
            screen: Screen::new(),
            rss: RssController,
            bookmark: BookmarkController,
            history: HistoryController,
            config: ConfigController,
        };
        app
    }

    pub fn run(&mut self, options: HashMap<String, String>) {
         if !self.is_config_exists() {
            eprintln!("設定ファイルが見つかりませんでした。\nnnm init で初期設定を行ってください。");
            return;
        };

        let config: Config = match Config::new().load_from_file() {
            Ok(config) => config,
            Err(_) => {
                eprintln!("設定ファイルが見つかりませんでした。\nnnm init で初期設定を行ってください。");
                return;
            }
        };

        let links = config.links().clone();
        let rt = Runtime::new().unwrap();

        let results = rt.block_on(async {
            let tasks = links.into_iter().map(|link| {
                tokio::spawn(async move {
                    #[cfg(debug_assertions)]
                    {
                        println!("- start fetch task {} : {:?}", link, std::thread::current().id());
                    }
                    let res = Self::fetch_rss(link.clone()).await;
                    #[cfg(debug_assertions)]
                    {
                        println!("- end fetch task {} : {:?}", link, std::thread::current().id());
                    }
                    res
                })
            }).collect::<Vec<_>>();

            let results = futures::future::join_all(tasks).await;

            let fetched_data: Vec<String> = results.into_iter()
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
            self.screen_draw(options);
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

    pub fn parse_xml(&mut self, bodys: Vec<String>, config: Config) -> Result<(), quick_xml::Error> {
        let parser = Parser::new();
        let chunk_size = config.chunk_size();

        for body in bodys {
            let ret = parser.parse(body);
            match ret {
                Ok(entities) => {
                    let mut chunks = entities.into_iter().take(chunk_size.try_into().unwrap()).collect();
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

    pub fn screen_draw(&mut self, options: HashMap<String, String>) {
        self.screen.draw(&self.entities, options);
    }

    fn is_config_exists(&self) -> bool {
        let config = Config::new();
        let exists = config.default_file_path().try_exists();
        match exists {
            Ok(true) => true,
            Ok(false) => false,
            Err(e) => {
                println!("load_config: {:?}", e);
                false
            }
        }
    }

    pub fn show_bookmarks(&self) {
        let config: Result<Config, Error> = Config::new().load_from_file();
        match config {
            Ok(config) => {
                let bookmarks = config.bookmarks();
                for bookmark in bookmarks {
                    println!("{}", bookmark);
                }
            }
            Err(e) => {
                // TODO: ここのエラーは精査する必要がありそう
                eprintln!("設定ファイルに異常が見つかりました。: {:?}", e);
            }
        }
    }

    // pub fn run_delete_prompt_rss(&self) {
    //     self.run_delete_prompt(|config: &Config| {
    //             config.links().clone()
    //         }, |config: &mut Config, url: &str| {
    //             match config.delete_link_and_save(&url) {
    //                 Ok(_) => {
    //                     println!("URLを削除しました: {}", url);
    //                 }
    //                 Err(e) => {
    //                     println!("削除に失敗しました: {:?}", e);
    //                 }
    //             }
    //         });
    // }

    // pub fn run_delete_prompt_bookmark(&self) {
    //     self.run_delete_prompt(|config: &Config| {
    //             config.bookmarks().clone()
    //         }, |config: &mut Config, url: &str| {
    //             match config.delete_bookmark_and_save(&url) {
    //                 Ok(_) => {
    //                     println!("ブックマークを削除しました: {}", url);
    //                 }
    //                 Err(e) => {
    //                     println!("削除に失敗しました: {:?}", e);
    //                 }
    //             }
    //         });
    // }

    pub fn show_history(&self) {
        let history: Result<History, Error> = History::new().load_from_file();
        match history {
            Ok(history) => {
                let entities = history.get_entities();
                self.screen.draw(&entities, HashMap::new());
            }
            Err(e) => {
                eprintln!("Error loading history: {:?}", e);
            }
        }
    }

    fn save_history(&self) -> Result<(), std::io::Error> {
        let history: Result<History, Error> = History::new().load_from_file();

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
            Err(e   ) => {
                eprintln!("履歴ファイルが見つかりませんでした。\nhistory.jsonを再作成します。");
                let history = History::new();
                history.save_to_file(history.clone())?;
                Err(e)
            }
        }
    }

    fn filter_new_entities(&mut self) -> u16 {
        let history: Result<History, Error> = History::new().load_from_file();
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
