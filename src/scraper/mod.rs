#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

use std::{env, error, sync::Arc};

use crate::{scraper, shared::{
    dto::host_group::CreateHostGroupsDto,
    model::host_group::{self, HostGroupModel},
}};


fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .or_else(|_| tracing_subscriber::EnvFilter::try_new(""))
                .unwrap(),
        )
        .init();
}

async fn run() -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    setup_logging();
    let args: Vec<String> = env::args().collect();
    let target_list = args
        .get(1)
        .expect("please provide target list file as first argument");
    let timeout = args
        .get(2)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(10);
    let create_host_group_dtos = parse_host_groups(target_list).await;
    // scraper::insert_host_groups(&create_host_group_dtos).await?;
    loop {
        tracing::info!("running background scraper");
        let create_host_group_dtos = create_host_group_dtos.clone();
        // scraper::run_scraper(&create_host_group_dtos, timeout)
        //     .await
        //     .unwrap_or_else(|err| {
        //         tracing::info!("scraping failed due to {err:?}");
        //     });
    }
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
