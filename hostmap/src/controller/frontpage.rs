use std::collections::HashMap;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;
use shared::dto::{host::{CurrentHostDto, HostDto, HostWithLogsDto}, host_group::HostGroupDto};
use tera::Context;

use crate::AppState;
#[derive(Debug, Clone, Serialize)]
struct FrontPageContext {
    host_groups: Vec<HostGroupDto>,
    total_groups: usize,
    total_hosts: usize,
}

impl FrontPageContext {
    fn new(host_groups: Vec<HostGroupDto>) -> Self {
        let total_groups = host_groups.len();
        let mut total_hosts = 0;
        for g in &host_groups {
            for _ in &g.host_dtos {
                total_hosts += 1;
            }
        }
        Self {
            host_groups,
            total_groups,
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
    let host_group_models = host_repo.get_all_host_groups().await.unwrap();
    let mut host_group_dtos = HashMap::new();
    for group in &host_group_models {
        for host in &group.hosts {
            let log_entry_model = activation_log_service
                .latest_entry_for_host(host.host_id)
                .await?;
            let host_dto: CurrentHostDto = CurrentHostDto::from((host.clone(), log_entry_model));
            let current_host_group_dto = HostGroupDto::from((group.clone(), host_dto.clone()));
            host_group_dtos
                .entry(current_host_group_dto.group_name.clone())
                .or_insert_with(|| HostGroupDto {
                    group_name: current_host_group_dto.group_name.clone(),
                    host_dtos: Vec::new(),
                    host_group_id: current_host_group_dto.host_group_id,
                })
                .host_dtos
                .push(host_dto);
        }
    }
    let host_group_dtos: Vec<HostGroupDto> = host_group_dtos.into_values().collect();

    let fp_ctx = FrontPageContext::new(host_group_dtos);
    let mut ctx = Context::new();
    ctx.insert("title", "frontpage");
    ctx.insert("frontpage_ctx", &fp_ctx);

    let output = tera.render("frontpage.html.tera", &ctx).unwrap();
    Ok(Html(output))
}
