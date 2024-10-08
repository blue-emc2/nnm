use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Rss {
        #[command(subcommand)]
        action: Option<Actions>
    },
    Bookmark {
        #[command(subcommand)]
        action: Option<Actions>
    },
    History
}

#[derive(Subcommand, Debug, Clone)]
pub enum Actions {
    Add {
        url: Option<String>,
    },
    Delete,
}
