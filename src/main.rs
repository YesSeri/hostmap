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
use std::{env, sync::Arc, time::Duration};

use axum::{routing::get, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tera::Tera;
use tokio::time::{self, interval};
use tower_http::services::ServeDir;

use crate::{
    dto::host::HostGroupsCreateDto,
    model::host::{ExistingHostGroupModel, NewHostGroupModel},
    repository::{
        activation_log_repository::ActivationLogRepository, host_repository::HostRepository,
    },
    service::activation_log_service::ActivationLogService,
};

type RetError = dyn std::error::Error + Send + Sync + 'static;
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

#[tokio::main]
async fn main() -> Result<(), Box<RetError>> {
    let db_url =
        std::env::var("DATABASE_URL").expect("could not find database url as environment variable");
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    let host_repo = HostRepository::new(pool.clone());
    // let log_repo = ActivationLogRepository::new(pool.clone());
    let log_service = ActivationLogService::new(ActivationLogRepository::new(pool.clone()));
    setup_host_groups(&host_repo).await;
    let tera = Arc::new(load_tera());
    let app_state = AppState::new(tera, host_repo, log_service);
    let bg_scraper_state = app_state.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            scraper::run_scraper(bg_scraper_state.clone())
                .await
                .unwrap_or_else(|err| {
                    println!("scraping failed due to {err}");
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
        .with_state(app_state);

    let bind_addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();

    println!("Creating server at http://{bind_addr}");

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Error in the graceful shutdown, slightly ironic");
            println!("We are shutting the server down. :(");
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
    println!("target list file with host groups and hosts: {target_list}");
    let content = read_host_groups_from_file(&target_list);
    let HostGroupsCreateDto(host_group_dtos): HostGroupsCreateDto =
        serde_json::from_str(&content).expect("could not parse target list file as json");
    for host_group_dto in host_group_dtos {
        let host_group = NewHostGroupModel::from(host_group_dto);
        // it will fail if host_group is already inserted
        let _ = repo.insert_group_hosts_with_hosts(&host_group).await;
    }
}

fn load_tera() -> Tera {
    let mut tera = Tera::new("templates/**/*").unwrap();
    tera
}
