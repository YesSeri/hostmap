use axum::{Json, extract::State};

use crate::{server::ServerState, shared::{dto::host::CurrentHostDto, model::host::HostModel}};
// #[derive(Debug, Clone, Serialize)]
// struct LogContext {}

#[axum::debug_handler]
pub(crate) async fn create_hosts(
    State(ServerState { host_service, .. }): State<ServerState>,
    Json(payload): Json<Vec<CurrentHostDto>>,
) -> axum::response::Result<String> {
    tracing::info!("Received payload: {:?}", payload);
    let hosts = payload
        .iter()
        .map(|dto| HostModel::from(dto.clone()))
        .collect::<Vec<HostModel>>();

    let num_hosts_inserted = host_service.bulk_insert_hosts(&hosts).await.unwrap();
    tracing::info!("Created {} hosts", num_hosts_inserted);

    Ok(num_hosts_inserted.to_string())
}
