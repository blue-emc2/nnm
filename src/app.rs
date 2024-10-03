mod screen;
mod entity;
mod parser;
mod table;
mod config;
mod history;

use history::History;
use tokio::runtime::Runtime;
use std::collections::HashMap;
use std::{env, io};
use std::fs::File;
use std::path::PathBuf;
use std::result::Result;
use std::io::Write;
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

    pub fn run(&mut self, options: HashMap<String, String>) {
        let Some(config) = self.load_config() else {
            eprintln!("設定ファイルが見つかりませんでした。\nnnm init で初期設定を行ってください。");
            return;
        };

        let links = config.links();
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
        if let Err(e) = self.screen_draw(options) {
            println!("Error drawing screen: {:#?}", e);
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

    pub fn screen_draw(&mut self, options: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
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

        let history = History::new();
        let history_json = serde_json::to_string_pretty(&history)?;
        let mut file = File::create(config_dir.join("history.json"))?;
        write!(file, "{}", history_json)?;

        Ok(config_file_path.into_os_string().into_string().unwrap())
    }

    pub fn load_config(&self) -> Option<Config> {
        let exists = Config::default_config_path().try_exists();
        match exists {
            Ok(true) => {
                match Config::load_from_file() {
                    Ok(config) => {
                        return Some(config);
                    }
                    Err(e) => {
                        eprintln!("エラーが発生しました: {}", e);
                        return None;
                    }
                }
            }
            Ok(false) => {
                None
            }
            Err(e) => {
                println!("load_config: {:?}", e);
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

    pub fn delete_link_prompt(&self) {
        println!("削除したいURLまたは番号を入力してください。");
        println!("q, quit, exit で終了します。");
        let config = Config::load_from_file().unwrap();
        let links = config.links();
        let link_itretor = links.iter().enumerate();
        for (i, link) in link_itretor {
            println!("{}: {}", i, link);
        }

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "q" || input == "quit" || input == "exit" {
                break;
            }

            format!("入力された内容: {}", input);

            if let Ok(index) = input.parse::<usize>() {
                if index < links.len() {
                    let url = &links[index];
                    self.delete_link(url);
                    println!("URLを削除しました: {}", url);
                    break;
                } else {
                    println!("無効な番号です。もう一度入力してください。");
                }
            } else {
                // 入力がURLの場合
                if links.contains(&input.to_string()) {
                    self.delete_link(input);
                    println!("URLを削除しました: {}", input);
                    break;
                } else {
                    println!("URLが見つかりません。もう一度入力してください。");
                }
            }
        }
    }

    pub fn delete_link(&self, url: &str) {
        let mut config = Config::load_from_file().unwrap();
        let _result = config.delete_link(url);
    }
}
