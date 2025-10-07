mod controller;
mod custom_error;
pub(crate) mod endpoint;
mod repository;
mod service;

use std::{collections::HashMap, error, sync::Arc};

use axum::http::header;
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tera::{Tera, Value, try_get_value};

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
    let sensitive_headers: Vec<header::HeaderName> = vec![header::AUTHORIZATION];
    let server_state = ServerState::new(
        tera,
        server_config,
        host_service,
        log_service,
        nix_git_link_service,
    );
    let public_routes = Router::new()
        .route(
            endpoint::frontpage(),
            get(controller::frontpage::render_frontpage),
        )
        .route(
            endpoint::history(),
            get(controller::history::render_history_page),
        );
    let protected_routes = Router::new()
        .route(endpoint::hosts_bulk(), post(host_controller::create_hosts))
        .route(
            endpoint::activations_bulk(),
            post(activation_controller::create_activation),
        )
        .route(
            endpoint::nix_git_link(),
            get(controller::nix_git_link_controller::create_link),
        )
        .route(
            endpoint::nix_git_link_bulk(),
            get(controller::nix_git_link_controller::create_links),
        );
    let router = public_routes
        .merge(protected_routes)
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

fn nix_name(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("nix_name", "value", String, value);
    let s = s.strip_prefix("/nix/store").unwrap_or(&s);
    // remove from idx 10 to idx 20
    let s = if s.len() > 28 {
        format!("{}...{}", &s[..12], &s[28..])
    } else {
        s.to_string()
    };
    Ok(Value::String(s.to_string()))
}

// register once
fn load_tera(templates_dir: &str) -> Result<Tera, tera::Error> {
    let tera_pattern = format!("{}/**/*", templates_dir);
    let mut tera = Tera::new(&tera_pattern);
    if let Ok(t) = tera.as_mut() {
        t.register_filter("nix_name", nix_name)
    }
    tera
}
