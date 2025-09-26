#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

pub(crate) mod scraper;
pub(crate) mod server;
pub(crate) mod shared;

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
    controller::{self, host_controller, log_entry_controller},
    endpoint,
    repository::{
        activation_log_repository::ActivationLogRepository, host_repository::HostRepository,
    },
    service::{activation_log_service::ActivationLogService, host_service::HostService},
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
            RetError::NotFound => (
                StatusCode::NOT_FOUND,
                "The thing you were looking for could not be found".to_string(),
            )
                .into_response(),
        }
    }
}
async fn fallback() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "Could not find the thing you were looking for.",
    )
}

#[derive(Debug, Clone)]
struct AppState {
    tera: Arc<Tera>,
    host_service: HostService,
    activation_log_service: ActivationLogService,
}

impl AppState {
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
fn setup_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
}

#[derive(Parser, Debug)]
#[command(name = "app")]
#[command(version, about = "Server/Scraper runner")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Server {
        #[arg(long)]
        database_url: String,
    },
    Scraper {
        #[arg(long)]
        hosts_file: PathBuf,
        #[arg(long)]
        scrape_interval: u64,
    },
}
#[tokio::main]
async fn main() -> Result<(), RetError> {
    let cli = Cli::parse();
    setup_logging();
    match cli.command {
        Commands::Server { database_url } => {
            let pool = PgPoolOptions::new()
                .max_connections(8)
                .connect(&database_url)
                .await
                .expect("failed to connect to DATABASE_URL");
            let host_service = HostService::new(HostRepository::new(pool.clone()));
            let log_service = ActivationLogService::new(ActivationLogRepository::new(pool.clone()));
            let tera = Arc::new(load_tera());
            let app_state = AppState::new(tera, host_service, log_service);
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
                .fallback(fallback)
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
        }
        Commands::Scraper {
            hosts_file,
            scrape_interval,
        } => {
            // systemd restarts scraper on fail
            tracing::info!(
                "Starting scraper with file: {:?} and interval: {}",
                hosts_file,
                scrape_interval
            );
            scraper::run(hosts_file, scrape_interval)
                .await
                .unwrap_or_else(|err| {
                    tracing::info!("scraping failed due to {err:?}");
                });
        }
    }
    Ok(())
}

fn load_tera() -> Tera {
    Tera::new("templates/**/*").unwrap()
}
