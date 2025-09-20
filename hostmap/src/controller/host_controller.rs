use axum::{Json, extract::State};
use serde::Serialize;
use shared::{dto::host_group::CreateHostGroupsDto, model::host_group::HostGroupModel};

use crate::AppState;
#[derive(Debug, Clone, Serialize)]
struct LogContext {}

#[axum::debug_handler]
pub(crate) async fn create_host_groups(
    State(AppState { host_repo, .. }): State<AppState>,
    Json(payload): Json<CreateHostGroupsDto>,
) -> axum::response::Result<String> {
    tracing::info!("Received payload: {:?}", payload);
    let host_groups: Vec<HostGroupModel> = payload
        .0
        .into_iter()
        .map(|dto| HostGroupModel::from(dto))
        .collect();
    let num_host_groups_inserted = host_repo
        .bulk_insert_group_hosts(host_groups.as_ref())
        .await
        .unwrap();
    tracing::info!("Created {} host groups", num_host_groups_inserted);

    let num_hosts_inserted = host_repo.bulk_insert_hosts(&host_groups).await.unwrap();
    tracing::info!("Created {} hosts", num_hosts_inserted);

    Ok(num_hosts_inserted.to_string())
}
