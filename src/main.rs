mod app;
mod commands;

use std::collections::HashMap;

use clap::Parser;
use app::App;
use commands::{Actions, Commands};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// A numeric option
    #[arg(short, long, default_value_t = 10)]
    number: i32,
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
        Some(Commands::Rss { action } ) => {
            match action {
                Some(Actions::Add { url }) => {
                    if let Some(url) = url {
                        match app.add_link(url) {
                            Ok(url) => {
                                println!("{} を追加しました。", url);
                            }
                            Err(e) => {
                                println!("Error: {:#?}", e);
                            }
                        }
                    }
                }
                Some(Actions::Delete) => {
                    app.run_delete_prompt_rss();
                }
                None => {
                    todo!();
                }
            }
        },
        Some(Commands::Bookmark { action} ) => {
            match action {
                Some(Actions::Add { url }) => {
                    if let Some(url) = url {
                        app.add_link_to_bookmarks(url);
                    }
                }
                Some(Actions::Delete) => {
                    app.run_delete_prompt_bookmark();
                }
                None => {
                    app.show_bookmarks();
                }
            }
        },
        Some(Commands::History) => {
            app.show_history();
        }
        None => {
            app.run(options);
        }
    }
}
