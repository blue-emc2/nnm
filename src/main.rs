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
    let app: App = App::new();

    match &cli.command {
        Some(Commands::Init) => {
            println!("Hello, world!");
        }
        None => {
            app.refresh();
            println!("Get xml!");
        }
    }
}
