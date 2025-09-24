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

pub async fn run(
    host_group_file: PathBuf,
    scrape_interval: u64,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let create_host_group_dtos = parse_hosts(&host_group_file).await;
    scraper::insert_host_groups(&create_host_group_dtos).await?;

    loop {
        tracing::info!("running background scraper");
        let create_host_group_dtos = create_host_group_dtos.clone();
        scraper::scrape_hosts(&create_host_group_dtos, scrape_interval)
            .await
            .unwrap_or_else(|err| {
                tracing::info!("scraping failed due to {err:?}");
            });
    }
}

fn read_host_groups_from_file(path: &PathBuf) -> String {
    std::fs::read_to_string(path).expect("could not read target list file")
}

async fn parse_hosts(host_group_file: &PathBuf) -> Vec<CurrentHostDto> {
    tracing::info!(
        "target list file with host groups and hosts: {}",
        host_group_file.display()
    );
    let content = read_host_groups_from_file(host_group_file);
    let host_group_dtos: Vec<CurrentHostDto> =
        serde_json::from_str(&content).expect("could not parse target list file as json");

    // pretty print the parsed host groups
    for host in &host_group_dtos {
        tracing::info!("parsed host: {:?}", host);
    }
    host_group_dtos
}
