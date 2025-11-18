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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ret_error_db() {
        let db_err = RetError::DbError(sqlx::Error::RowNotFound);
        let response = db_err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
    #[test]
    fn test_ret_error_not_found() {
        let not_found_err = RetError::NotFound;
        let response = not_found_err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
