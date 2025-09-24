#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

pub(crate) mod controller;
pub(crate) mod endpoints;
pub(crate) mod repository;
pub(crate) mod scraper;
pub(crate) mod service;
pub(crate) mod shared;
use std::{error, path::PathBuf, sync::Arc};

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

use crate::{
    controller::{host_controller, log_entry_controller},
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
use tracing_subscriber::{EnvFilter, fmt};

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
        host_group_file: PathBuf,
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
            let host_repo = HostRepository::new(pool.clone());
            let log_service = ActivationLogService::new(ActivationLogRepository::new(pool.clone()));
            let tera = Arc::new(load_tera());
            let app_state = AppState::new(tera, host_repo, log_service);
            let bg_scraper_state = app_state.clone();
            let app = Router::new()
                .route(endpoints::hosts_bulk(), post(host_controller::create_hosts))
                .route(
                    endpoints::log_entry_bulk(),
                    post(log_entry_controller::create_log_entry),
                )
                .route(
                    endpoints::frontpage(),
                    get(controller::frontpage::render_frontpage),
                )
                .route("/{hostname}", get(controller::history::render_history_page))
                .fallback(fallback)
                .nest_service(endpoints::assets_folder(), ServeDir::new("assets"))
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
            host_group_file,
            scrape_interval,
        } => {
            // systemd restarts scraper on fail
            tracing::info!(
                "Starting scraper with file: {:?} and interval: {}",
                host_group_file,
                scrape_interval
            );
            scraper::run(host_group_file, scrape_interval)
                .await
                .unwrap_or_else(|err| {
                    tracing::info!("scraping failed due to {err:?}");
                });
        }
    }
    Ok(())
}

fn read_host_groups_from_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("could not read target list file")
}

fn load_tera() -> Tera {
    Tera::new("templates/**/*").unwrap()
}

// use std::path::PathBuf;
// pub(crate) mod scraper;
// // pub(crate) mod controller;
// // pub(crate) mod repository;
// // pub(crate) mod service;
// pub(crate) mod shared;

// fn setup_logging() {
//     tracing_subscriber::fmt()
//         .with_env_filter(
//             tracing_subscriber::EnvFilter::try_from_default_env()
//                 .or_else(|_| tracing_subscriber::EnvFilter::try_new(""))
//                 .unwrap(),
//         )
//         .init();
// }

// #[tokio::main]
// async fn main() {
//     setup_logging();
//     scraper::run(PathBuf::from("./test-assets/minimalTargetList.json"), 3)
//         .await
//         .unwrap_or_else(|err| {
//             tracing::info!("scraping failed due to {err:?}");
//         });
// }
