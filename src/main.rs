use clap::{Subcommand, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            println!("Hello, world!");
        }
    }
}
