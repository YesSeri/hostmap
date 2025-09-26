#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

pub(crate) mod scraper;
use std::{env, error, path::PathBuf, sync::Arc};

use tracing_subscriber::EnvFilter;

use crate::shared::dto::host::CurrentHostDto;

fn create_client() -> Result<reqwest::Client, reqwest::Error> {
    let builder = reqwest::Client::builder().connect_timeout(std::time::Duration::from_secs(10));
    builder.build()
}

pub async fn run(
    hosts_file: PathBuf,
    scrape_interval: u64,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    tracing::info!(
        "Starting scraper with file: {:?} and interval: {}",
        hosts_file,
        scrape_interval
    );
    let create_host_dtos = parse_hosts(&hosts_file).await;
    let client = create_client()?;
    scraper::insert_hosts(&create_host_dtos, &client).await?;
    loop {
        tracing::info!("running background scraper");
        let create_host_dtos = create_host_dtos.clone();
        scraper::scrape_hosts(&create_host_dtos, scrape_interval, &client)
            .await
            .unwrap_or_else(|err| {
                tracing::info!("scraping failed due to {err:?}");
            });
    }
}

fn read_hosts_from_file(path: &PathBuf) -> String {
    std::fs::read_to_string(path).expect("could not read target list file")
}

async fn parse_hosts(host_file: &PathBuf) -> Vec<CurrentHostDto> {
    tracing::info!(
        "target list file with host groups and hosts: {}",
        host_file.display()
    );
    let content = read_hosts_from_file(host_file);
    let host_dtos: Vec<CurrentHostDto> =
        serde_json::from_str(&content).expect("could not parse target list file as json");

    tracing::info!("hosts len: {}", host_dtos.len());
    host_dtos
}
