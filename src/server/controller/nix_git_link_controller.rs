use crate::{server::ServerState, shared::dto::nix_git_link::NixGitLinkDto};
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};

#[axum::debug_handler]
pub(crate) async fn create_links(
    State(ServerState {
        nix_git_link_service,
        ..
    }): State<ServerState>,
    Json(nix_git_link): Json<Vec<NixGitLinkDto>>,
) -> axum::response::Result<impl IntoResponse> {
    let i = nix_git_link_service.create_many(nix_git_link).await?;
    tracing::debug!("bulk inserted {i} nix git links");
    Ok((StatusCode::CREATED, format!("{i} links created")))
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
    tracing::debug!("created a nix git link");
    Ok(StatusCode::CREATED)
}
