pub(crate) mod activation_logger;
mod cli;
pub(crate) mod scraper;
pub(crate) mod server;
pub(crate) mod shared;

use clap::Parser;
use std::error;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::cli::{Cli, Commands};

fn setup_logging() -> EnvFilter {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter.clone())
        .with(fmt::layer())
        .init();
    filter
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let filter = setup_logging();
    tracing::info!("Log level set to: {}", filter);
    let cli = Cli::parse();
    match cli.command {
        Commands::ActivationLogger(activation_logger_args) => {
            activation_logger::run(activation_logger_args).await;
        }
        Commands::Server(server_args) => {
            server::run(server_args).await?;
        }
        Commands::Scraper(scraper_args) => {
            scraper::run(scraper_args).await?;
        }
    }
    Ok(())
}
