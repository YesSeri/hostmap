use std::error;

use axum::response::IntoResponse;
use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub(super) enum RetError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error("Not Found")]
    NotFound,
}

impl IntoResponse for RetError {
    fn into_response(self) -> axum::response::Response {
        match self {
            RetError::DbError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
            RetError::NotFound => (
                StatusCode::NOT_FOUND,
                "The thing you were looking for could not be found".to_string(),
            )
                .into_response(),
        }
    }
}
pub(super) async fn fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "Could not find the thing you were looking for.",
    )
}
