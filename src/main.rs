mod app;

use std::collections::HashMap;

use clap::{Parser, Subcommand};
use app::App;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// A numeric option
    #[arg(short, long, default_value_t = 10)]
    number: i32,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Add {
        url: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut app: App = App::new();
    let number = cli.number;
    let mut options = HashMap::new();
    options.insert("head".to_string(), number.to_string());

    match &cli.command {
        Some(Commands::Init) => {
            match app.init_config() {
                Ok(path) => {
                    println!("設定ファイルを作成しました。{}", path);
                    println!("nnm add {{url}} でRSSのURLを追加しましょう。");
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }
        }
        Some(Commands::Add { url }) => {
            match app.add_link(url) {
                Ok(url) => {
                    println!("{} を追加しました。", url);
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }
        }
        None => {
            if let Some(config) = app.load_config() {
                println!("Config: {:#?}", config);
            } else {
                return;
            }

            let response = app.fetch_all();
            match response {
                Ok(body) => {
                    if let Err(e) = app.parse_xml(body) {
                        println!("Error parsing XML: {:#?}", e);
                        return;
                    }
                    if let Err(e) = app.screen_draw(options) {
                        println!("Error drawing screen: {:#?}", e);
                        return;
                    }
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }
        }
    }
}
