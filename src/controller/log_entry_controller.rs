use std::collections::HashMap;

use crate::shared::{
    dto::{host::HostWithLogsDto, log::LogEntryDto},
    model::{
        host::HostModel,
        log::{CreateLogEntryModel, ExistingLogEntryModel},
    },
};
use axum::{
    Json,
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;
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
    Json(host_with_logs_dto): Json<HostWithLogsDto>,
) -> axum::response::Result<String> {
    tracing::debug!("Received log entry DTO: {:?}", host_with_logs_dto);
    let models: Vec<CreateLogEntryModel> = host_with_logs_dto
        .logs
        .iter()
        .map(|dto| CreateLogEntryModel::from((&host_with_logs_dto, dto.clone())))
        .collect();
    let i = activation_log_service
        .bulk_insert_log_records(models.as_ref())
        .await?;
    Ok(format!("Num log entries created: {}", i))
}
