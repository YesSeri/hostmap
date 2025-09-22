#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

pub(crate) mod scraper;
use std::{env, error, sync::Arc};

use shared::{
    dto::host_group::CreateHostGroupsDto,
    model::host_group::{self, HostGroupModel},
};

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .or_else(|_| tracing_subscriber::EnvFilter::try_new(""))
                .unwrap(),
        )
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    setup_logging();
    let args: Vec<String> = env::args().collect();
    let target_list = args
        .get(1)
        .expect("please provide target list file as first argument");
    let create_host_group_dtos = parse_host_groups(target_list).await;
    println!("created hgdtos");
    let json = serde_json::to_string_pretty(&create_host_group_dtos);
    println!("json: {}", json.unwrap());
    scraper::insert_host_groups(create_host_group_dtos).await?;
    Ok(())

    // loop {
    //     tracing::info!("running background scraper");
    //     scraper::run_scraper(create_host_group_dtos.clone())
    //         .await
    //         .unwrap_or_else(|err| {
    //             log::info!("scraping failed due to {err:?}");
    //         });
    //     tokio::time::sleep(std::time::Duration::from_secs(300)).await;
    // }
}

fn read_host_groups_from_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("could not read target list file")
}

async fn parse_host_groups(target_list: &str) -> CreateHostGroupsDto {
    tracing::info!("target list file with host groups and hosts: {target_list}");
    let content = read_host_groups_from_file(target_list);
    let host_group_dtos: CreateHostGroupsDto =
        serde_json::from_str(&content).expect("could not parse target list file as json");
    host_group_dtos
}
