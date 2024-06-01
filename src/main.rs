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
                    app.parse_xml(body);
                    app.print_all();
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }
        }
    }
}
