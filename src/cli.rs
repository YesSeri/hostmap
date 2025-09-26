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
        #[arg(long)]
        database_url: String,
    },
    Scraper {
        #[arg(long)]
        hosts_file: PathBuf,
        #[arg(long)]
        scrape_interval: u64,
    },
}
