use crate::{
    server::ServerState,
    shared::{dto::host::HostWithLogsDto, model::activation::NewActivation},
};
use axum::{Json, extract::State};

#[axum::debug_handler]
pub(crate) async fn create_activation(
    State(ServerState {
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
    tracing::debug!(count = models.len(), "recieved activations");
    let i = activation_log_service
        .bulk_insert_log_records(models.as_ref())
        .await?;
    if i > 0 {
        tracing::info!(count = i, "inserted activations");
    }
    Ok(format!("{i} log entries created"))
}
