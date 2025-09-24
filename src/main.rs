#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

pub(crate) mod controller;
pub(crate) mod scraper;
pub(crate) mod shared;
pub(crate) mod repository;
pub(crate) mod service;
use std::{error, sync::Arc};

use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use tera::Tera;
use tower_http::services::ServeDir;

use crate::{
    repository::{
        activation_log_repository::ActivationLogRepository, host_repository::HostRepository,
    },
    service::activation_log_service::ActivationLogService,
};

#[derive(Debug, thiserror::Error)]
enum RetError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    #[error(transparent)]
    Other(Box<dyn error::Error + Send + Sync + 'static>),
    #[error("Not Found")]
    NotFound,
}

impl IntoResponse for RetError {
    fn into_response(self) -> axum::response::Response {
        match self {
            RetError::DbError(err) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                    }
            RetError::Other(err) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
                    }
            RetError::NotFound => (StatusCode::NOT_FOUND, "The thing you were looking for could not be found".to_string()).into_response(),
        }
    }
}
async fn fallback() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Could not find the thing you were looking for.")
}

#[derive(Debug, Clone)]
struct AppState {
    tera: Arc<Tera>,
    host_repo: HostRepository,
    activation_log_service: ActivationLogService,
}

impl AppState {
    fn new(
        tera: Arc<Tera>,
        host_repo: HostRepository,
        activation_log_service: ActivationLogService,
    ) -> Self {
        Self {
            tera,
            host_repo,
            activation_log_service,
        }
    }
}

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .or_else(|_| tracing_subscriber::EnvFilter::try_new(""))
                .unwrap(),
        )
        .init();
}
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing() {
    // Reads RUST_LOG if set; otherwise uses a sensible default
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
        // .unwrap_or_else(|_| EnvFilter::new("info,sqlx=info,sqlx::query=trace,axum=debug"));
    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}


#[tokio::main]
async fn main() -> Result<(), RetError> {
    init_tracing();
    let db_url =
        std::env::var("DATABASE_URL").expect("could not find database url as environment variable");
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    let host_repo = HostRepository::new(pool.clone());
    let log_service = ActivationLogService::new(ActivationLogRepository::new(pool.clone()));
    let tera = Arc::new(load_tera());
    let app_state = AppState::new(tera, host_repo, log_service);
    let bg_scraper_state = app_state.clone();
    let app = Router::new()
        .route(
            "/api/host_group/bulk",
            post(controller::host_controller::create_host_groups),
        )
        .route(
            "/api/log_entry/bulk",
            post(controller::log_entry_controller::create_log_entry),
        )
        .route("/", get(controller::frontpage::render_frontpage))
        .route(
            "/{host_group_name}/{host_name}",
            get(controller::history::render_history_page),
        )
        .fallback(fallback)
        .nest_service("/assets", ServeDir::new("assets"))
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

fn read_host_groups_from_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("could not read target list file")
}

fn load_tera() -> Tera {
    Tera::new("templates/**/*").unwrap()
}
