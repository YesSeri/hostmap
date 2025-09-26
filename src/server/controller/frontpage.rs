use std::collections::{BTreeMap, HashMap};

use crate::{server::{custom_error::RetError, ServerState}, shared::dto::host::CurrentHostDto};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Debug, Clone, Serialize)]
struct FrontPageContext {
    hosts: Vec<CurrentHostDto>,
    // total_groups: usize,
    total_hosts: usize,
}

impl FrontPageContext {
    fn new(hosts: Vec<CurrentHostDto>) -> Self {
        // let total_groups = hosts.len();
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
#[derive(Debug, Clone, Deserialize)]
pub struct FrontPageQuery {
    sorting_key: Option<String>,
}

#[axum::debug_handler]
pub async fn render_frontpage(
    State(app_state): State<ServerState>,
    Query(params): Query<FrontPageQuery>,
) -> axum::response::Result<Html<String>, RetError> {
    tracing::debug!("Rendering frontpage with params: {:#?}", params);
    if let Some(sorting_key) = params.sorting_key {
        tracing::info!("Sorting key provided: {}", sorting_key);
        render_frontpage_by_group(sorting_key, app_state).await
    } else {
        tracing::info!("No sorting key provided, using default sorting.");
        render_frontpage_all_hosts(app_state).await
    }
}

async fn render_frontpage_all_hosts(
    ServerState {
        tera,
        host_service,
        activation_log_service,
    }: ServerState,
) -> axum::response::Result<Html<String>, RetError> {
    tracing::info!("No sorting key provided, using default sorting.");

    let host_models = host_service
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

#[derive(Debug, Clone, Serialize)]
struct FrontpageGroupedContext {
    groups: BTreeMap<String, Vec<CurrentHostDto>>,
    total_groups: usize,
    total_hosts: usize,
}

impl FrontpageGroupedContext {
    fn new(groups: BTreeMap<String, Vec<CurrentHostDto>>) -> Self {
        let total_groups = groups.len();
        let mut total_hosts = 0;
        for group in groups.values() {
            total_hosts += group.len();
        }
        Self {
            groups,
            total_groups,
            total_hosts,
        }
    }
}
async fn render_frontpage_by_group(
    grouping_key: String,
    ServerState {
        tera, host_service, ..
    }: ServerState,
) -> axum::response::Result<Html<String>, RetError> {
    let host_with_logs = host_service
        .get_all_hosts_with_latest_log_entry()
        .await
        .unwrap();

    let mut grouped_hosts: BTreeMap<String, Vec<CurrentHostDto>> = BTreeMap::new();

    for host_with_log in host_with_logs {
        let current_host_dto =
            CurrentHostDto::from((host_with_log.host.clone(), host_with_log.logs.clone()));
        let group_name = host_with_log
            .host
            .metadata
            .get(&grouping_key)
            .and_then(|v| v.as_str().map(|s| s.to_owned()))
            .unwrap_or_else(|| "Ungrouped".to_string());

        tracing::debug!("Hosts with logs: {:#?}", group_name);
        grouped_hosts
            .entry(group_name)
            .or_default()
            .push(current_host_dto);
    }
    tracing::debug!("Grouped hosts: {:#?}", grouped_hosts);

    let fp_ctx = FrontpageGroupedContext::new(grouped_hosts);
    let mut ctx = Context::new();
    ctx.insert("title", "frontpage");
    ctx.insert("grouped_frontpage_ctx", &fp_ctx);

    Ok(Html(
        tera.render("grouped_frontpage.html.tera", &ctx).unwrap(),
    ))
}
