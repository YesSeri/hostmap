mod controller;
pub(crate) mod endpoint;
mod repository;
mod service;
mod custom_error;

use std::{error, path::PathBuf, sync::Arc};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use clap::{Parser, Subcommand, command};
use sqlx::postgres::PgPoolOptions;
use tera::Tera;
use tower_http::services::ServeDir;

use crate::server::{
    controller::{host_controller, log_entry_controller}, custom_error::RetError, repository::{
        activation_log_repository::ActivationLogRepository, host_repository::HostRepository,
    }, service::{activation_log_service::ActivationLogService, host_service::HostService}
};

#[derive(Debug, Clone)]
struct ServerState {
    tera: Arc<Tera>,
    host_service: HostService,
    activation_log_service: ActivationLogService,
}

impl ServerState {
    fn new(
        tera: Arc<Tera>,
        host_service: HostService,
        activation_log_service: ActivationLogService,
    ) -> Self {
        Self {
            tera,
            host_service,
            activation_log_service,
        }
    }
}
pub async fn run(database_url: String) -> Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&database_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    let host_service = HostService::new(HostRepository::new(pool.clone()));
    let log_service = ActivationLogService::new(ActivationLogRepository::new(pool.clone()));
    let tera = Arc::new(load_tera());
    let app_state = ServerState::new(tera, host_service, log_service);
    let app = Router::new()
        .route(endpoint::hosts_bulk(), post(host_controller::create_hosts))
        .route(
            endpoint::log_entry_bulk(),
            post(log_entry_controller::create_log_entry),
        )
        .route(
            endpoint::frontpage(),
            get(controller::frontpage::render_frontpage),
        )
        .route("/{hostname}", get(controller::history::render_history_page))
        .fallback(custom_error::fallback)
        .nest_service(endpoint::assets_folder(), ServeDir::new("assets"))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(app_state);

    let bind_addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();

    tracing::info!("Creating server at http://{bind_addr}");

    axum::serve(listener, app.into_make_service())
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

fn load_tera() -> Tera {
    Tera::new("templates/**/*").unwrap()
}
