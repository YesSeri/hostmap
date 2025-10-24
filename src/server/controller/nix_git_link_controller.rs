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
    tracing::info!(
        "recieved {} relationships between nix and git",
        nix_git_link.len()
    );
    let i = nix_git_link_service.create_many(nix_git_link).await?;
    tracing::info!("inserted {i} relationships between nix and git");
    Ok((StatusCode::CREATED, format!("{i} links created")))
}

#[axum::debug_handler]
pub(crate) async fn create_link(
    State(ServerState {
        nix_git_link_service,
        ..
    }): State<ServerState>,
    Json(nix_git_link): Json<NixGitLinkDto>,
) -> axum::response::Result<impl IntoResponse> {
    tracing::info!("recieved one nix git relationship");
    nix_git_link_service.create(nix_git_link).await?;
    Ok(StatusCode::CREATED)
}
