use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use axum::http::{HeaderMap, HeaderValue, StatusCode, header::AUTHORIZATION};

pub async fn api_authentication(
    State(api_key): State<String>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    match get_token(headers) {
        Some(client_api_key) if token_is_valid(client_api_key, &api_key) => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => {
            tracing::warn!("unauthorized request, missing or invalid api key");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

fn parse_api_key(h: &HeaderValue) -> Option<&str> {
    let full_header_value = h.to_str().ok()?;
    let client_api_key = full_header_value.strip_prefix("Api-Key")?.trim().into();
    client_api_key
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers.get(&AUTHORIZATION).and_then(parse_api_key)
}

fn token_is_valid(client_api_key: &str, server_api_key: &str) -> bool {
    client_api_key == server_api_key
}
