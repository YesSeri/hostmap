use crate::{
    server::ServerState,
    shared::{dto::host::HostWithLogsDto, model::log::CreateLogEntryModel},
};
use axum::{Json, extract::State};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct LogContext {}

#[axum::debug_handler]
pub(crate) async fn post_log_entry(
    State(ServerState {
        tera,
        host_service,
        activation_log_service,
        ..
    }): State<ServerState>,
    // post request in body
    Json(host_with_logs_dto): Json<HostWithLogsDto>,
) -> axum::response::Result<String> {
    tracing::debug!("received post request: {:?}", host_with_logs_dto);
    let models: Vec<CreateLogEntryModel> = host_with_logs_dto
        .logs
        .iter()
        .map(|dto| CreateLogEntryModel::from((&host_with_logs_dto, dto.clone())))
        .collect();
    let i = activation_log_service
        .bulk_insert_log_records(models.as_ref())
        .await?;
    tracing::debug!("recieved post successfully, inserted {i} log entries");
    Ok(format!("Num log entries created: {}", i))
}
