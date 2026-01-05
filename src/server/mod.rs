mod api_authentication;
mod controller;
mod custom_error;
pub(crate) mod endpoint;
mod repository;
mod service;

use crate::cli::ServerArgs;
use crate::server::api_authentication::api_authentication;
use crate::shared::helper::read_api_key;
use std::{collections::HashMap, error, sync::Arc};

use axum::http::header;
use axum::middleware::from_fn_with_state;
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tera::{Tera, Value, try_get_value};
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;

use crate::server::{
    controller::{activation_controller, host_controller},
    custom_error::RetError,
    service::{
        activation_service::ActivationLogService, host_service::HostService,
        nix_git_link_service::NixGitLinkService,
    },
};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;

pub const TIME_ZONE_ENV_NAME: &str = "TIME_ZONE_HOSTMAP";

#[derive(Debug, Clone)]
struct ServerConfig {
    default_grouping_key: Option<String>,
    columns: Vec<String>,
    repo_url: String,
}
impl ServerConfig {
    fn new(default_grouping_key: Option<String>, columns: Vec<String>, repo_url: String) -> Self {
        Self {
            default_grouping_key,
            columns,
            repo_url,
        }
    }
}
#[derive(Debug, Clone)]
struct ServerState {
    tera: Arc<Tera>,
    server_config: ServerConfig,
    host_service: HostService,
    activation_log_service: ActivationLogService,
    nix_git_link_service: NixGitLinkService,
}

impl ServerState {
    fn new(
        tera: Arc<Tera>,
        server_config: ServerConfig,
        host_service: HostService,
        activation_log_service: ActivationLogService,
        nix_git_link_service: NixGitLinkService,
    ) -> Self {
        Self {
            tera,
            server_config,
            host_service,
            activation_log_service,
            nix_git_link_service,
        }
    }
}

async fn build_pool(database_url: String) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(8)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(Some(std::time::Duration::from_secs(300)))
        .connect(&database_url)
        .await
}

fn create_public_router() -> Router<ServerState> {
    Router::new()
        .route(
            endpoint::frontpage(),
            get(controller::frontpage::render_frontpage),
        )
        .route(
            endpoint::history(),
            get(controller::history::render_history_page),
        )
}
fn create_protected_router(api_key: String) -> Router<ServerState> {
    let sensitive_headers: Vec<header::HeaderName> = vec![header::AUTHORIZATION];

    Router::new()
        .route(endpoint::hosts_bulk(), post(host_controller::create_hosts))
        .route(
            endpoint::activations_bulk(),
            post(activation_controller::create_activation),
        )
        .route(
            endpoint::nix_git_link(),
            post(controller::nix_git_link_controller::create_link),
        )
        .route(
            endpoint::nix_git_link_bulk(),
            post(controller::nix_git_link_controller::create_links),
        )
        .layer(SetSensitiveRequestHeadersLayer::new(sensitive_headers))
        .layer(from_fn_with_state(api_key, api_authentication))
}

pub async fn run(
    ServerArgs {
        database_url,
        api_key_file,
        default_grouping_key,
        url,
        port,
        columns,
        repo_url,
    }: ServerArgs,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let api_key = read_api_key(&api_key_file);
    let pool = build_pool(database_url).await?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run database migrations");
    tracing::info!("database migrations applied");

    let templates_dir = std::env::var("HOSTMAP_TEMPLATES_DIR").expect("user must specificy the env variable HOSTMAP_TEMPLATES_DIR. This should not be something end user needs to specify, since the flake will wrap this.");
    tracing::info!("Using templates directory: {}", &templates_dir);
    let host_service = HostService::new(pool.clone());
    let log_service = ActivationLogService::new(pool.clone());
    let nix_git_link_service = NixGitLinkService::new(pool.clone());
    let tera = Arc::new(load_tera(&templates_dir).unwrap_or_else(|_| {
        panic!(
            "Failed to load templates from directory: {}",
            &templates_dir
        )
    }));
    let server_config =
        ServerConfig::new(default_grouping_key, columns.unwrap_or_default(), repo_url);
    let server_state = ServerState::new(
        tera,
        server_config,
        host_service,
        log_service,
        nix_git_link_service,
    );
    let router = create_public_router()
        .merge(create_protected_router(api_key))
        .fallback(custom_error::fallback)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(server_state);

    let bind_addr = format!("{}:{}", url, port);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Failed to bind to address {}, is the port already in use?",
                &bind_addr
            )
        });

    tracing::info!("Starting server at http://{}", &bind_addr);

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Error in the graceful shutdown, slightly ironic");
            tracing::info!("We are shutting the server down. :(");
        })
        .await
        .unwrap();
    Ok(())
}
fn load_tera(templates_dir: &str) -> Result<Tera, tera::Error> {
    let tera_pattern = format!("{}/**/*", templates_dir);
    let mut tera = Tera::new(&tera_pattern)?;
    tera.register_filter("nix_name", nix_name);
    tera.register_filter("format_utc_as_local", format_utc_as_local);
    tera.register_filter("format_utc_as_local_time", format_utc_as_local_time);
    Ok(tera)
}

fn nix_name(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("nix_name", "value", String, value);
    Ok(Value::String(nix_name_fn(&s).unwrap_or(s)))
}
fn nix_name_fn(s: &str) -> Option<String> {
    let s = s.strip_prefix("/nix/store/")?.strip_suffix("pre-git")?;
    let (prefix, rest) = s.split_at(10);
    let (_, suffix) = rest.split_once("-nixos-system-")?;
    Some(format!("{}-{}", prefix, suffix))
}

pub fn format_utc_as_local_time_fn(rfc3339: &str, tz: &str) -> Option<String> {
    let tz: Tz = tz.parse().ok()?;
    let dt_fixed = DateTime::parse_from_rfc3339(rfc3339).ok()?;
    let dt_utc: DateTime<Utc> = dt_fixed.with_timezone(&Utc);
    Some(dt_utc.with_timezone(&tz).format("%H:%M:%S").to_string())
}

pub fn format_utc_as_local_time(
    value: &Value,
    _args: &HashMap<String, Value>,
) -> tera::Result<Value> {
    let s = try_get_value!("format_utc_as_local_time", "value", String, value);
    let tz = std::env::var(TIME_ZONE_ENV_NAME).unwrap_or_else(|_| "UTC".to_string());
    Ok(Value::String(
        format_utc_as_local_time_fn(&s, &tz).unwrap_or(s),
    ))
}

pub fn format_utc_as_local_fn(rfc3339: &str, tz: &str) -> Option<String> {
    let tz: Tz = tz.parse().ok()?;
    let dt_fixed = DateTime::parse_from_rfc3339(rfc3339).ok()?;
    let dt_utc: DateTime<Utc> = dt_fixed.with_timezone(&Utc);
    Some(
        dt_utc
            .with_timezone(&tz)
            .format("%Y-%m-%d %H:%M:%S %Z")
            .to_string(),
    )
}

pub fn format_utc_as_local(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("format_utc_as_local", "value", String, value);
    let tz = std::env::var(TIME_ZONE_ENV_NAME).unwrap_or_else(|_| "UTC".to_string());

    Ok(Value::String(format_utc_as_local_fn(&s, &tz).unwrap_or(s)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::Europe::Copenhagen;
    #[test]
    fn test_nix_name_filter() {
        let store_path =
            "/nix/store/4v0ykqdvvpgpw83ljfk32bzjl2bcblmk-nixos-system-hosts-p01-25.05pre-git";
        let shortened_store_path = nix_name_fn(store_path);
        let correct = "4v0ykqdvvp-hosts-p01-25.05".to_string();
        assert_eq!(shortened_store_path.unwrap(), correct);

        let store_path =
            "/nix/store/z79zirq3imbn761fv32q1pnb2xkpp8ma-nixos-system-ceph-mon-p101-25.05pre-git";
        let shortened_store_path = nix_name_fn(store_path);
        let correct = "z79zirq3im-ceph-mon-p101-25.05".to_string();
        assert_eq!(shortened_store_path.unwrap(), correct);
    }
    #[test]
    fn test_copenhagen_winter_timezone() {
        // Jan 5 is winter time (CET, UTC+1)
        let s = "2026-01-05T12:34:56Z";
        let out = format_utc_as_local_fn(s, &Copenhagen.to_string()).unwrap();
        assert_eq!(out, "2026-01-05 13:34:56 CET");
    }

    #[test]
    fn test_invalid_datetime() {
        assert_eq!(
            format_utc_as_local_fn("not-a-date", &Copenhagen.to_string()),
            None
        );
    }

    #[test]
    fn test_invalid_timezone() {
        let s = "2026-01-05T12:34:56Z";
        assert_eq!(format_utc_as_local_fn(s, "not-a-timezone/nowhere"), None);
    }
}
