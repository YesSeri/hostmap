use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use chrono::NaiveDate;
use serde::Serialize;
use tera::Context;

use crate::{
    dto::{host::HostDto, log::LogHistoryDto},
    model::log::LogEntryModel,
    AppState,
};
#[derive(Debug, Clone, Serialize)]
struct HistoryPageContext {
    host: HostDto,
    activations_by_date: Vec<(NaiveDate, Vec<LogHistoryDto>)>,
}

impl HistoryPageContext {
    fn new(host: HostDto, activations_by_date: Vec<(NaiveDate, Vec<LogHistoryDto>)>) -> Self {
        Self {
            host,
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
    Path(h_name): Path<String>,
) -> impl IntoResponse {
    let host = host_repo
        .get_host_from_hostname(h_name)
        .await
        .unwrap()
        .unwrap();
    let mut ctx = Context::new();
    let date_map = activation_log_service
        .host_with_logs_by_host_id(host.host_id)
        .await
        .unwrap();
    let mut date_dto_vec = Vec::new();
    for (date, entries) in date_map {
        let mut dto_vec = Vec::new();
        for entry in entries {
            let log_entry_model: LogEntryModel<i64> = entry.into();
            let dto = LogHistoryDto::from(log_entry_model);
            dto_vec.push(dto);
        }
        date_dto_vec.push((date, dto_vec));
    }

    let fp_ctx = HistoryPageContext::new(HostDto::from(host.clone()), date_dto_vec);

    ctx.insert("title", format!("History for {}", host.name).as_str());
    ctx.insert("history_ctx", &fp_ctx);
    let output = tera.render("history.html.tera", &ctx).unwrap();
    Html(output)
}
