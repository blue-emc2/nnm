use clap::{Subcommand, Parser};
use nnm::App;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
}

fn main() {
    let cli = Cli::parse();
    let mut app: App = App::new();

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
                        return ;
                    }
                    if let Err(e) = app.screen_draw() {
                        println!("Error drawing screen: {:#?}", e);
                        return ;
                    }
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }

        }
    }
}
