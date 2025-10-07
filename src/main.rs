// allow most linter things for now in development
// dont care about clippy
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

mod cli;
pub(crate) mod scraper;
pub(crate) mod server;
pub(crate) mod shared;

use clap::Parser;
use std::{env, error};

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::{Cli, Commands};

fn setup_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}
fn get_api_key() -> Result<String, env::VarError> {
    let api_key = env::var("HOSTMAP_API_KEY").inspect_err(|err| {
        match err {
            env::VarError::NotPresent => tracing::warn!("please supply a HOSTMAP_API_KEY env variable for your server or scraper"),
            env::VarError::NotUnicode(_) => tracing::warn!("please supply a valid HOSTMAP_API_KEY for your server or scraper, the current one is not unicode"),
        }
    })?;
    Ok(api_key)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    setup_logging();

    let cli = Cli::parse();
    let api_key = get_api_key()?;
    tracing::debug!("running with cli config: {:?}", cli);

    match cli.command {
        Commands::Server {
            database_url,
            default_grouping_key,
            url,
            port,
            columns,
        } => {
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
        } => {
            scraper::run(hosts_file, scrape_interval, &url, api_key).await?;
        }
    }
    Ok(())
}
