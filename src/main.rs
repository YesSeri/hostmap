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

use std::error;
use clap::Parser;

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
        Commands::Server { database_url } => {
            server::run(database_url).await?;
        }
        Commands::Scraper {
            hosts_file,
            scrape_interval,
        } => {
            scraper::run(hosts_file, scrape_interval).await?;
        }
    }
    Ok(())
}
