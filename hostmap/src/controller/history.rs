use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use chrono::NaiveDate;
use serde::Serialize;
use shared::{
    dto::{
        host::{CurrentHostDto, HostDto, HostDtoNoLogs},
        log::LogHistoryDto,
    },
    model::log::{HostName, LogEntryModel},
};
use tera::Context;

use crate::AppState;
#[derive(Debug, Clone, Serialize)]
struct HistoryPageContext {
    host: HostDtoNoLogs,
    activations_by_date: Vec<(NaiveDate, Vec<LogHistoryDto>)>,
}

impl HistoryPageContext {
    fn new(host: HostDtoNoLogs, activations_by_date: Vec<(NaiveDate, Vec<LogHistoryDto>)>) -> Self {
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
    Path(host_tuple): Path<(String, String)>,
) -> impl IntoResponse {
    let host = host_repo
        .get_host_from_tuple(host_tuple)
        .await
        .unwrap()
        .unwrap();
    let mut ctx = Context::new();
    let date_map = activation_log_service
        .host_with_logs_by_host_name(&host.host_name)
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

    let host_dto = HostDtoNoLogs::from(host.clone());
    let fp_ctx = HistoryPageContext::new(host_dto, date_dto_vec);

    ctx.insert("title", format!("History for {}", host.host_name).as_str());
    ctx.insert("history_ctx", &fp_ctx);
    let output = tera.render("history.html.tera", &ctx).unwrap();
    Html(output)
}
