use std::collections::BTreeMap;

use std::collections::HashMap;
use std::collections::HashSet;

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
}

impl FrontPageContext {
    fn new(hosts: Vec<CurrentHostDto>) -> Self {
        Self { hosts }
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
        .or(server_state.server_config.default_grouping_key.as_ref())
        .cloned();

    let mut ctx = Context::new();
    ctx.insert("columns", &server_state.server_config.columns);
    ctx.insert("repo_url", &server_state.server_config.repo_url);

    if let Some(grouping_key) = grouping_key {
        render_frontpage_by_group(&grouping_key, server_state, ctx).await
    } else {
        render_frontpage_all_hosts(server_state, ctx).await
    }
}

async fn render_frontpage_all_hosts(
    ServerState {
        tera, host_service, ..
    }: ServerState,
    mut ctx: Context,
) -> axum::response::Result<Html<String>, RetError> {
    let host_models = host_service
        .get_all_with_latest_log()
        .await
        .expect("Failed to fetch hosts");
    let hosts = host_models
        .into_iter()
        .map(|hwl| CurrentHostDto::from((hwl.host, hwl.logs)))
        .collect::<Vec<CurrentHostDto>>();

    let commit_hashes: HashSet<String> = hosts
        .iter()
        .filter_map(|h| {
            h.logs
                .as_ref()
                .and_then(|l| l.revision.as_ref())
                .map(|r| r.commit_hash.clone())
        })
        .collect();

    let color_map = build_color_map_for_hashes(commit_hashes);
    let fp_ctx = FrontPageContext::new(hosts);
    ctx.insert("title", "frontpage");
    ctx.insert("frontpage_ctx", &fp_ctx);
    ctx.insert("color_map", &color_map);

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
    mut ctx: Context,
) -> axum::response::Result<Html<String>, RetError> {
    let host_with_logs = host_service.get_all_with_latest_log().await.unwrap();

    let mut grouped_hosts: BTreeMap<String, Vec<CurrentHostDto>> = BTreeMap::new();

    for host_with_log in host_with_logs {
        let current_host_dto =
            CurrentHostDto::from((host_with_log.host.clone(), host_with_log.logs.clone()));
        let group_name = host_with_log
            .host
            .metadata
            .get(grouping_key)
            .map(|v| v.to_owned())
            .unwrap_or_else(|| "Ungrouped".to_string());

        grouped_hosts
            .entry(group_name)
            .or_default()
            .push(current_host_dto);
    }
    let commit_set: HashSet<String> = grouped_hosts
        .values()
        .flatten()
        .filter_map(|h| {
            h.logs
                .as_ref()
                .and_then(|l| l.revision.as_ref())
                .map(|r| r.commit_hash.clone())
        })
        .collect();

    let color_map = build_color_map_for_hashes(commit_set);

    let fp_ctx = FrontpageGroupedContext::new(grouped_hosts);
    ctx.insert("title", &format!("frontpage by group: {}", grouping_key));
    ctx.insert("grouped_frontpage_ctx", &fp_ctx);
    ctx.insert("color_map", &color_map);

    Ok(Html(
        tera.render("grouped_frontpage.html.tera", &ctx).unwrap(),
    ))
}

const BACKGROUND_COLORS: [&str; 11] = [
    "#C0C0C0", "#FFFF00", "#FFCC00", "#FF9900", "#CCFF00", "#CCCC00", "#CC99FF", "#FF00FF",
    "#FF0000", "#33FFFF", "#CCFFFF",
];
pub(crate) fn build_color_map_for_hashes(
    set_of_commit_hashes: HashSet<String>,
) -> HashMap<String, String> {
    let mut color_map: HashMap<String, String> = HashMap::new();
    let mut next_idx = 0usize;
    let colors_len = BACKGROUND_COLORS.len();

    for h in set_of_commit_hashes {
        if !color_map.contains_key(&h) {
            let color = BACKGROUND_COLORS[next_idx % colors_len].to_string();
            color_map.insert(h.to_owned(), color);
            next_idx += 1;
        }
    }
    color_map
}
