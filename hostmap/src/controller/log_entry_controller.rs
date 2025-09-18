use std::collections::HashMap;

use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;
use shared::{dto::{host::HostWithLogDto, log::LogEntryDto}, model::log::ExistingLogEntryModel};
use tera::Context;

use crate::AppState;
#[derive(Debug, Clone, Serialize)]
struct LogContext {}

#[axum::debug_handler]
pub(crate) async fn create_log_entry(
    State(AppState {
        tera,
        host_repo,
        activation_log_service,
    }): State<AppState>,
    // post request in body
    Json(payload): Json<Vec<HostWithLogDto>>,
) -> axum::response::Result<String> {
    let models = payload.into_iter().map(|el| LogEntryDto::from(el)).collect::<Vec<LogEntryDto>>();
    activation_log_service.bulk_insert_log_records(&models).await?;
    todo!();
}
