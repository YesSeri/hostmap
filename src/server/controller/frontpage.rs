use std::collections::BTreeMap;

use crate::{
    server::{ServerState, custom_error::RetError},
    shared::dto::host::CurrentHostDto,
};
use axum::{
    extract::{Query, State},
    response::Html,
};
use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Debug, Clone, Serialize)]
struct FrontPageContext {
    hosts: Vec<CurrentHostDto>,
    total_hosts: usize,
}

impl FrontPageContext {
    fn new(hosts: Vec<CurrentHostDto>) -> Self {
        let total_hosts = hosts.len();
        Self { hosts, total_hosts }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct FrontPageQuery {
    grouping_key: Option<String>,
}

#[axum::debug_handler]
pub async fn render_frontpage(
    State(server_state): State<ServerState>,
    Query(params): Query<FrontPageQuery>,
) -> axum::response::Result<Html<String>, RetError> {
    let grouping_key = params
        .grouping_key
        .as_ref()
        .or(server_state.default_grouping_key.as_ref())
        .cloned();
    tracing::debug!("params from query: {:#?}", params);
    tracing::debug!("Using grouping key: {:?}", grouping_key);

    if let Some(grouping_key) = grouping_key {
        render_frontpage_by_group(&grouping_key, server_state).await
    } else {
        render_frontpage_all_hosts(server_state).await
    }
}

async fn render_frontpage_all_hosts(
    ServerState {
        tera, host_service, ..
    }: ServerState,
) -> axum::response::Result<Html<String>, RetError> {
    let host_models = host_service
        .get_all_hosts_with_latest_log_entry()
        .await
        .unwrap();
    let hosts = host_models
        .into_iter()
        .map(|hwl| CurrentHostDto::from((hwl.host, hwl.logs)))
        .collect::<Vec<CurrentHostDto>>();
    let fp_ctx = FrontPageContext::new(hosts);
    let mut ctx = Context::new();
    ctx.insert("title", "frontpage");
    ctx.insert("frontpage_ctx", &fp_ctx);

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
    grouping_key: &str,
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
            .get(grouping_key)
            .and_then(|v| v.as_str().map(|s| s.to_owned()))
            .unwrap_or_else(|| "Ungrouped".to_string());

        grouped_hosts
            .entry(group_name)
            .or_default()
            .push(current_host_dto);
    }

    let fp_ctx = FrontpageGroupedContext::new(grouped_hosts);
    let mut ctx = Context::new();
    ctx.insert("title", "frontpage");
    ctx.insert("grouped_frontpage_ctx", &fp_ctx);

    Ok(Html(
        tera.render("grouped_frontpage.html.tera", &ctx).unwrap(),
    ))
}
