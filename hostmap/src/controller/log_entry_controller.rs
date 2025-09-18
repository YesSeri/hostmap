use std::collections::HashMap;

use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;
use shared::{dto::{host::HostWithLogsDto, log::LogEntryDto}, model::{host::HostModel, log::{CreateLogEntryModel, ExistingLogEntryModel}}};
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
    Json(payload): Json<HostWithLogsDto>,
) -> axum::response::Result<String> {
    let models: Vec<CreateLogEntryModel> = payload.logs.iter()
        .map(|dto| CreateLogEntryModel::from((payload.host_id, dto.clone())))
        .collect();
     activation_log_service
        .bulk_insert_log_records(models.as_ref())
        .await;
    todo!();
}
