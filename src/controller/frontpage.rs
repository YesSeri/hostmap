use std::collections::HashMap;

use crate::shared::dto::host::{CurrentHostDto, HostDto, HostWithLogsDto};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;
use tera::Context;

use crate::AppState;
#[derive(Debug, Clone, Serialize)]
struct FrontPageContext {
    hosts: Vec<CurrentHostDto>,
    // total_groups: usize,
    total_hosts: usize,
}

impl FrontPageContext {
    fn new(hosts: Vec<CurrentHostDto>) -> Self {
        let total_groups = hosts.len();
        let mut total_hosts = 0;
        for _ in &hosts {
            total_hosts += 1;
        }
        // for g in &host_groups {
        //     for _ in &g.hosts {
        //         total_hosts += 1;
        //     }
        // }
        Self {
            hosts,
            // total_groups,
            total_hosts,
        }
    }
}

#[axum::debug_handler]
pub async fn render_frontpage(
    State(AppState {
        tera,
        host_repo,
        activation_log_service,
    }): State<AppState>,
) -> axum::response::Result<impl IntoResponse> {
    // let host_group_models = host_repo.get_all_host_groups().await.unwrap();
    // let mut host_group_dtos = HashMap::new();
    // for group in host_group_models {
    //     let current_host_group_dto = HostGroupDto::from(group);
    //     for host_dto in current_host_group_dto.hosts.into_iter() {
    //         let log_entry_model = activation_log_service
    //             .latest_entry_for_host(host_dto.clone())
    //             .await?;
    //         let host_model = host_dto.clone().into();
    //         let host_dto: CurrentHostDto = CurrentHostDto::from((host_model, log_entry_model));
    //         host_group_dtos
    //             .entry(current_host_group_dto.host_group_name.clone())
    //             .or_insert_with(|| HostGroupDto {
    //                 host_group_name: current_host_group_dto.host_group_name.clone(),
    //                 hosts: Vec::new(),
    //             })
    //             .hosts
    //             .push(HostDto::from(host_dto));
    //     }
    // }
    // let host_group_dtos: Vec<HostGroupDto> = host_group_dtos.into_values().collect();

    let host_models = host_repo
        .get_all_hosts_with_latest_log_entry()
        .await
        .unwrap();
    let hosts = host_models
        .into_iter()
        .map(|hwl| CurrentHostDto::from((hwl.host, hwl.logs)))
        .collect::<Vec<CurrentHostDto>>();
    for host in hosts.iter() {
        tracing::info!("Host in frontpage: {:?}", host);
    }
    let fp_ctx = FrontPageContext::new(hosts);
    let mut ctx = Context::new();
    ctx.insert("title", "frontpage");
    ctx.insert("frontpage_ctx", &fp_ctx);
    tracing::debug!("frontpage with context: {:#?}", ctx);

    let output = tera.render("frontpage.html.tera", &ctx).unwrap();
    Ok(Html(output))
}
