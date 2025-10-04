use crate::{
    server::ServerState,
    shared::{dto::host::HostWithLogsDto, model::activation::NewActivation},
};
use axum::{Json, extract::State};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct LogContext {}

#[axum::debug_handler]
pub(crate) async fn create_activation(
    State(ServerState {
        tera,
        host_service,
        activation_log_service,
        ..
    }): State<ServerState>,
    Json(host_with_logs_dto): Json<HostWithLogsDto>,
) -> axum::response::Result<String> {
    let models: Vec<NewActivation> = host_with_logs_dto
        .logs
        .iter()
        .map(|dto| NewActivation::from((&host_with_logs_dto, dto.clone())))
        .collect();
    let i = activation_log_service
        .bulk_insert_log_records(models.as_ref())
        .await?;
    Ok(format!("Num log entries created: {}", i))
}
