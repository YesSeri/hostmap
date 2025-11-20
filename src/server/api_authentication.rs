use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use axum::http::{HeaderMap, HeaderValue, StatusCode, header::AUTHORIZATION};
use subtle::ConstantTimeEq;

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
            let uri = request.uri().clone();
            tracing::info!(
                uri = %uri,
                "unauthorized request, missing or invalid api key"
            );
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

fn parse_api_key(h: &HeaderValue) -> Option<&str> {
    let full_header_value = h.to_str().ok()?;
    let client_api_key = full_header_value.strip_prefix("Api-Key")?.trim();
    if client_api_key.is_empty() {
        None
    } else {
        Some(client_api_key)
    }
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers.get(&AUTHORIZATION).and_then(parse_api_key)
}

fn token_is_valid(client_api_key: &str, server_api_key: &str) -> bool {
    bool::from(client_api_key.as_bytes().ct_eq(server_api_key.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_token_is_valid() {
        let server_api_key = "supersecretkey";
        let client_api_key = "supersecretkey";
        assert!(token_is_valid(client_api_key, server_api_key));
    }
    #[test]
    fn test_token_is_invalid() {
        let server_api_key = "supersecretkey";
        let invalid_client_api_key = "wrongkey";
        assert!(!token_is_valid(invalid_client_api_key, server_api_key));
        let invalid_client_api_key = " supersecretkey";
        assert!(!token_is_valid(invalid_client_api_key, server_api_key));
    }
    #[test]
    fn test_parse_api_key() {
        let header_value = HeaderValue::from_str("Api-Key mysecretkey").unwrap();
        let parsed_key = parse_api_key(&header_value);
        assert_eq!(parsed_key, Some("mysecretkey"));
        let invalid_type_value = HeaderValue::from_str("Bearer mysecretkey").unwrap();
        let parsed_key_invalid = parse_api_key(&invalid_type_value);
        assert_eq!(parsed_key_invalid, None);
    }
    #[test]
    fn test_get_token() {
        let mut map = HeaderMap::new();
        let result = get_token(&map);
        assert_eq!(result, None);
        let h_val = HeaderValue::from_str("Api-Key mysecretkey").unwrap();
        map.insert(AUTHORIZATION, h_val);
        let result = get_token(&map);
        assert_eq!(result, Some("mysecretkey"));
    }
    #[test]
    fn parse_api_key_with_extra_spaces() {
        let header_value = HeaderValue::from_str("Api-Key    mysecretkey  ").unwrap();
        let parsed = parse_api_key(&header_value);
        assert_eq!(parsed, Some("mysecretkey"));
    }

    #[test]
    fn parse_api_key_without_value_returns_none() {
        let header_value = HeaderValue::from_str("Api-Key").unwrap();
        let parsed = parse_api_key(&header_value);
        assert_eq!(parsed, None);
    }
}
