mod api_authentication;
mod controller;
mod custom_error;
pub(crate) mod endpoint;
mod repository;
mod service;

use crate::server::api_authentication::api_authentication;
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

#[derive(Debug, Clone)]
struct ServerConfig {
    default_grouping_key: Option<String>,
    columns: Vec<String>,
}
impl ServerConfig {
    fn new(default_grouping_key: Option<String>, columns: Vec<String>) -> Self {
        Self {
            default_grouping_key,
            columns,
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
    //api_key: String,
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
    let public_routes = Router::new()
        .route(
            endpoint::frontpage(),
            get(controller::frontpage::render_frontpage),
        )
        .route(
            endpoint::history(),
            get(controller::history::render_history_page),
        );
    public_routes
}
fn create_protected_router(api_key: String) -> Router<ServerState> {
    let sensitive_headers: Vec<header::HeaderName> = vec![header::AUTHORIZATION];
    let protected_routes = Router::new()
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
        .layer(from_fn_with_state(api_key, api_authentication));
    protected_routes
}

pub async fn run(
    database_url: String,
    default_grouping_key: Option<String>,
    url: &str,
    port: u16,
    columns: Option<Vec<String>>,
    api_key: String,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let pool = build_pool(database_url).await?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run database migrations");

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
    let server_config = ServerConfig::new(default_grouping_key, columns.unwrap_or_default());
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
    tracing::info!("Starting server at {}", &bind_addr);
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Failed to bind to address {}, is the port already in use?",
                &bind_addr
            )
        });

    tracing::info!("Creating server at http://{}", &bind_addr);

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
    let mut tera = Tera::new(&tera_pattern);
    if let Ok(t) = tera.as_mut() {
        t.register_filter("nix_name", nix_name);
        t.register_filter("background_color", background_color);
    }
    tera
}

fn nix_name(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("nix_name", "value", String, value);
    Ok(Value::String(
        nix_name_fn(&s).unwrap_or("unknown_path".into()),
    ))
}
fn nix_name_fn(s: &str) -> Option<String> {
    let s = s.strip_prefix("/nix/store/")?.strip_suffix("pre-git")?;
    let (prefix, rest) = s.split_at(10);
    let (_, suffix) = rest.split_once("-nixos-system-")?;
    Some(format!("{}-{}", prefix.to_string(), suffix))
}
const BACKGROUND_COLORS: [&str; 22] = [
    "#DDDDDD", "#FFFF00", "#FFCC00", "#FF9900", "#CCFF00", "#CCCC00", "#9999FF", "#FF55FF",
    "#FF5555", "#53FFFF", "#CCFFFF", "#FFE680", "#FFD1DC", "#FFB3E6", "#E6B3FF", "#B3E6FF",
    "#B3FFE6", "#D1FFB3", "#FFFFB3", "#FFCCFF", "#FFB3B3", "#FFF0B3",
];

fn background_color_fn(name: &str) -> String {
    if (name == "unknown") {
        "#FFFFFF".to_string()
    } else {
        let val = name.as_bytes().get(1).unwrap_or(&0);
        BACKGROUND_COLORS[*val as usize % BACKGROUND_COLORS.len()].into()
    }
}

fn background_color(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("background_color", "value", String, value);
    Ok(Value::String(background_color_fn(&s)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_color_filters() {
        let store_path =
            "/nix/store/4v0ykqdvvpgpw83ljfk32bzjl2bcblmk-nixos-system-hosts-p01-25.05pre-git";
        let background_color = background_color_fn(store_path);
        let correct_color = BACKGROUND_COLORS[0];
        assert_eq!(background_color, correct_color);
    }
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
}
