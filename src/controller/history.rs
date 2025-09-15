use std::collections::{BTreeMap, HashMap, HashSet};

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use chrono::NaiveDate;
use serde::Serialize;
use tera::Context;

use crate::{
    dto::host::{CurrentHostDto, CurrentHostGroupDto},
    model::log::{ExistingLogEntryModel, HostId},
    AppState,
};
#[derive(Debug, Clone, Serialize)]
struct HistoryPageContext {
    activations_by_date: BTreeMap<NaiveDate, Vec<ExistingLogEntryModel>>,
}

impl HistoryPageContext {
    fn new(activations_by_date: BTreeMap<NaiveDate, Vec<ExistingLogEntryModel>>) -> Self {
        Self {
            activations_by_date,
        }
    }
}

#[axum::debug_handler]
pub async fn render_history_page(
    State(AppState {
        tera,
        host_repo,
        activation_log_service,
    }): State<AppState>,
    Path(host_name): Path<String>,
) -> impl IntoResponse {
    let mut ctx = Context::new();
    ctx.insert("title", "Hostmap - History");
    println!("getting acti logs by date");
    let host_id = HostId(4);
    let logs = activation_log_service
        .activation_logs_by_date_for_host_name(host_id)
        .await
        .unwrap();
    let fp_ctx = HistoryPageContext::new(logs);

    ctx.insert("history_ctx", &fp_ctx);
    let output = tera.render("history.html.tera", &ctx).unwrap();
    Html(output)
}
