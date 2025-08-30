use std::collections::HashSet;

use axum::{extract::State, response::Html};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{
    repository::{self, host_group_repository::fetch_all_host_group_names},
    viewmodel::host_group::HostGroupView,
    AppState,
};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct FilterParams {
    #[serde(default)]
    pub(crate) hosts: HashSet<String>,
    pub(crate) search: Option<String>,
}

pub async fn render_frontpage(
    Query(filter_params): Query<FilterParams>,
    State(AppState { tera, db }): State<AppState>,
) -> Html<String> {
    let mut context = Context::new();
    let host_groups = repository::host_group_repository::fetch_all_host_groups_by_filter_params(
        &db,
        &filter_params,
    )
    .await
    .unwrap();
    let host_options: Vec<String> = fetch_all_host_group_names(&db)
        .await
        .expect("could not get host group options");
    let total_groups = host_groups.len();
    let total_hosts = host_groups.iter().fold(0, |acc, hg| acc + hg.rows.len());

    let host_group_view: Vec<HostGroupView> =
        host_groups.into_iter().map(HostGroupView::from).collect();
    context.insert("title", "start page");
    context.insert("host_groups", &host_group_view);
    context.insert("total_groups", &total_groups);
    context.insert("total_hosts", &total_hosts);
    context.insert("host_options", &host_options);
    context.insert("params", &filter_params);

    let rendered = tera
        .render("frontpage.html.tera", &context)
        .expect("Template render failed");

    Html(rendered)
}
