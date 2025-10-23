use axum::{Json, extract::State};

use crate::{
    server::ServerState,
    shared::{dto::host::CurrentHostDto, model::host::HostModel},
};

#[axum::debug_handler]
pub(crate) async fn create_hosts(
    State(ServerState { host_service, .. }): State<ServerState>,
    Json(payload): Json<Vec<CurrentHostDto>>,
) -> axum::response::Result<String> {
    let i = payload.len();
    tracing::info!("Recieved {i} host dtos");
    let hosts = payload
        .iter()
        .map(|dto| HostModel::from(dto.clone()))
        .collect::<Vec<HostModel>>();

    let num_inserted = host_service.create_many(&hosts).await.unwrap();
    tracing::info!("Created {} hosts", num_inserted);

    Ok(num_inserted.to_string())
}
