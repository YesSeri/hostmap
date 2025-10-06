use crate::{
    server::ServerState,
    shared::{
        dto::{host::HostWithLogsDto, nix_git_link::NixGitLinkDto},
        model::activation::NewActivation,
    },
};
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct LogContext {}

#[axum::debug_handler]
pub(crate) async fn create_links(
    State(ServerState {
        tera,
        nix_git_link_service,
        ..
    }): State<ServerState>,
    Json(nix_git_link): Json<Vec<NixGitLinkDto>>,
) -> axum::response::Result<impl IntoResponse> {
    let i = nix_git_link_service.create_many(nix_git_link).await?;
    Ok((StatusCode::CREATED, format!("Num links created: {}", i)))
}

#[axum::debug_handler]
pub(crate) async fn create_link(
    State(ServerState {
        tera,
        nix_git_link_service,
        activation_log_service,
        ..
    }): State<ServerState>,
    Json(nix_git_link): Json<NixGitLinkDto>,
) -> axum::response::Result<impl IntoResponse> {
    nix_git_link_service.create(nix_git_link).await?;
    Ok(StatusCode::CREATED)
}
