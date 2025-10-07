use std::path::PathBuf;

use clap::{Parser, Subcommand, command};

#[derive(Parser, Debug)]
#[command(name = "app")]
#[command(version, about = "Server/Scraper runner")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Server {
        #[arg(long, help = "The database URL to connect to")]
        database_url: String,
        #[arg(
            long = "grouping-key",
            help = "Default key from metadata to group by on the frontpage"
        )]
        default_grouping_key: Option<String>,
        #[arg(long, default_value = "127.0.0.1", help = "url run the server on")]
        url: String,
        #[arg(long, default_value = "3000", help = "port to run server on")]
        port: u16,
        #[arg(long, help = "metadata columns to show in the frontpage table")]
        columns: Option<Vec<String>>,
    },
    Scraper {
        #[arg(long)]
        hosts_file: PathBuf,
        #[arg(long)]
        scrape_interval: u64,
        #[arg(
            long,
            default_value = "http://localhost:3000",
            help = "url of server to send scraped activations to(the server you started with `hostmap server ...`"
        )]
        url: String,
    },
}
