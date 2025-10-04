mod controller;
mod custom_error;
pub(crate) mod endpoint;
mod repository;
mod service;

use std::{error, sync::Arc};

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tera::Tera;
use tower_http::services::ServeDir;

use crate::server::{
    self,
    controller::{activation_controller, host_controller},
    custom_error::RetError,
    repository::{
        activation_repository::ActivationRepository, host_repository::HostRepository,
        nix_git_link_repository::NixGitLinkRepository,
    },
    service::{
        activation_log_service::ActivationLogService, host_service::HostService,
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
const MIGRATIONS_DIR_DEV: &str = "./migrations_dev";
const MIGRATIONS_DIR: &str = "./migrations";

async fn build_pool(database_url: String) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(8)
        .acquire_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(Some(std::time::Duration::from_secs(300)))
        .max_lifetime(Some(std::time::Duration::from_secs(3600)))
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                sqlx::query("SET application_name = 'hostmap'")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await
}

pub async fn run(
    database_url: String,
    default_grouping_key: Option<String>,
    url: &str,
    port: u16,
    columns: Option<Vec<String>>,
) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let pool = build_pool(database_url).await?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run database migrations");

    let templates_dir = std::env::var("HOSTMAP_TEMPLATES_DIR").expect("user must specificy the env variable HOSTMAP_TEMPLATES_DIR. This should not be something end user needs to specify, since the flake will wrap this.");
    tracing::info!("Using templates directory: {}", &templates_dir);
    let host_service = HostService::new(HostRepository::new(pool.clone()));
    let log_service = ActivationLogService::new(ActivationRepository::new(pool.clone()));
    let nix_git_link_service = NixGitLinkService::new(NixGitLinkRepository::new(pool.clone()));
    let tera = Arc::new(load_tera(&templates_dir).expect(&format!(
        "Failed to load templates from directory: {}",
        &templates_dir
    )));
    let server_config = ServerConfig::new(default_grouping_key, columns.unwrap_or_default());
    let server_state = ServerState::new(
        tera,
        server_config,
        host_service,
        log_service,
        nix_git_link_service,
    );
    let router = Router::new()
        .route(endpoint::hosts_bulk(), post(host_controller::create_hosts))
        .route(
            endpoint::log_entry_bulk(),
            post(activation_controller::create_activation),
        )
        .route(
            endpoint::frontpage(),
            get(controller::frontpage::render_frontpage),
        )
        .route(
            endpoint::link_entry(),
            get(controller::nix_git_link_controller::create_link),
        )
        .route("/{hostname}", get(controller::history::render_history_page))
        .fallback(custom_error::fallback)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(server_state);

    let bind_addr = format!("{}:{}", url, port);
    tracing::info!("Starting server at {}", &bind_addr);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.expect(
        format!(
            "Failed to bind to address {}, is the port already in use?",
            &bind_addr
        )
        .as_str(),
    );

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
    Tera::new(&tera_pattern)
}
