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
}

fn main() {
    let cli = Cli::parse();
    let mut app: App = App::new();
    let number = cli.number;
    let mut options = HashMap::new();
    options.insert("head".to_string(), number.to_string());

    match &cli.command {
        Some(Commands::Init) => {
            println!("Hello, world!");
        }
        None => {
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
