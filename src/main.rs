mod app;
mod commands;

use std::collections::HashMap;

use app::config::ConfigMessage;
use app::prompt::Prompt;
use app::App;
use clap::Parser;
use commands::{Actions, Commands};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[clap(name = "nnm", version = "1.0.1", about = "コンソールで読むRSSリーダー")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// A numeric option
    #[arg(short, long, default_value_t = 10)]
    number: i32,
}

fn main() {
    let cli = Cli::parse();
    let app: App = App::new();
    let number = cli.number;
    let mut options = HashMap::new();
    options.insert("head".to_string(), number.to_string());

    match &cli.command {
        Some(Commands::Init) => match app.config.create() {
            Ok(ConfigMessage::Success(path)) => {
                println!("設定ファイルを作成しました。{}", path);
                println!("nnm rss add \"{{url}}\" でRSSのURLを追加しましょう。");
            }
            Ok(ConfigMessage::ExistsConfig) => {
                println!("設定ファイルはすでに存在します。");
            }
            Err(e) => {
                println!("Error: {:#?}", e);
            }
        },
        Some(Commands::Rss { action }) => match action {
            Some(Actions::Add { url }) => {
                if let Some(url) = url {
                    match app.rss.add_link(url) {
                        Ok(url) => {
                            println!("{} を追加しました。", url);
                        }
                        Err(e) => {
                            println!("Error: {:#?}", e);
                        }
                    }
                }
            }
            Some(Actions::Delete) => match app.rss.delete_link() {
                Ok(()) => {
                    println!("URLを削除しました");
                }
                Err(e) => {
                    println!("削除に失敗しました: {:?}", e);
                }
            },
            None => {
                app.rss.show();
            }
        },
        Some(Commands::Bookmark { action }) => match action {
            Some(Actions::Add { url }) => {
                if let Some(url) = url {
                    app.bookmark.add_link(url);
                }
            }
            Some(Actions::Delete) => match app.bookmark.delete_link() {
                Ok(()) => {
                    println!("URLを削除しました");
                }
                Err(e) => {
                    println!("削除に失敗しました: {:?}", e);
                }
            },
            None => {
                app.bookmark.show();
            }
        },
        Some(Commands::History) => {
            app.history.show();
        }
        None => {
            app.rss.run(options);
        }
    }
}
