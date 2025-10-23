mod cli;
pub(crate) mod scraper;
pub(crate) mod server;
pub(crate) mod shared;

use clap::Parser;
use std::{error, path::PathBuf};

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::{Cli, Commands};

fn setup_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}
fn read_api_key(path: PathBuf) -> String {
    std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Could not read api_key from api_key_file {path:?}"))
        .trim()
        .to_owned()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    setup_logging();
    let cli = Cli::parse();
    match cli.command {
        Commands::Server {
            database_url,
            default_grouping_key,
            url,
            port,
            columns,
            api_key_file,
        } => {
            let api_key = read_api_key(api_key_file);
            server::run(
                database_url,
                default_grouping_key,
                &url,
                port,
                columns,
                api_key,
            )
            .await?;
        }
        Commands::Scraper {
            hosts_file,
            scrape_interval,
            url,
            api_key_file,
        } => {
            let api_key = read_api_key(api_key_file);
            scraper::run(hosts_file, scrape_interval, &url, api_key).await?;
        }
    }
    Ok(())
}
