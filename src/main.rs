#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

pub(crate) mod controller;
pub(crate) mod dto;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod scraper;
pub(crate) mod service;
use std::{env, error, sync::Arc};

use axum::{
    http::{status, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use sqlx::postgres::{PgPoolOptions, PgQueryResult};
use tera::Tera;
use tower_http::services::ServeDir;

use crate::{
    dto::host_group::CreateHostGroupsDto,
    model::host_group::CreateHostGroupModel,
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
        }
    }
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

#[tokio::main]
async fn main() -> Result<(), RetError> {
    setup_logging();
    let db_url =
        std::env::var("DATABASE_URL").expect("could not find database url as environment variable");
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    let host_repo = HostRepository::new(pool.clone());
    let log_service = ActivationLogService::new(ActivationLogRepository::new(pool.clone()));
    setup_host_groups(&host_repo).await;
    let tera = Arc::new(load_tera());
    let app_state = AppState::new(tera, host_repo, log_service);
    let bg_scraper_state = app_state.clone();
    tokio::spawn(async move {
        loop {
            tracing::info!("running background scraper");
            scraper::run_scraper(bg_scraper_state.clone())
                .await
                .unwrap_or_else(|err| {
                    log::info!("scraping failed due to {err:?}");
                });
        }
    });
    let app = Router::new()
        .route("/", get(controller::frontpage::render_frontpage))
        .route(
            "/host/{host_name}",
            get(controller::history::render_history_page),
        )
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(app_state);

    let bind_addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();

    log::info!("Creating server at http://{bind_addr}");

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Error in the graceful shutdown, slightly ironic");
            log::info!("We are shutting the server down. :(");
        })
        .await
        .unwrap();

    Ok(())
}

fn read_host_groups_from_file(path: &str) -> String {
    std::fs::read_to_string(path).expect("could not read target list file")
}

async fn setup_host_groups(repo: &HostRepository) {
    let args: Vec<String> = env::args().collect();
    let target_list = args
        .get(1)
        .expect("please provide a target list file as first argument");
    log::info!("target list file with host groups and hosts: {target_list}");
    let content = read_host_groups_from_file(target_list);
    let CreateHostGroupsDto(host_group_dtos): CreateHostGroupsDto =
        serde_json::from_str(&content).expect("could not parse target list file as json");
    for host_group_dto in host_group_dtos {
        let host_group = CreateHostGroupModel::from(host_group_dto);
        let _ = repo.insert_group_hosts_with_hosts(&host_group).await;
    }
}

fn load_tera() -> Tera {
    Tera::new("templates/**/*").unwrap()
}
