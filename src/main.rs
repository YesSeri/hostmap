#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

mod cli;
pub(crate) mod scraper;
pub(crate) mod server;
pub(crate) mod shared;

use clap::Parser;
use std::error;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::{Cli, Commands};

fn setup_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let cli = Cli::parse();
    setup_logging();
    match cli.command {
        Commands::Server {
            database_url,
            default_grouping_key,
            url,
            port,
        } => {
            server::run(database_url, default_grouping_key, &url, port).await?;
        }
        Commands::Scraper {
            hosts_file,
            scrape_interval,
            url,
        } => {
            scraper::run(hosts_file, scrape_interval, &url).await?;
        }
    }
    Ok(())
}
