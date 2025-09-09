pub(crate) mod controller;
pub(crate) mod model;
pub(crate) mod repository;
pub(crate) mod viewmodel;
use std::{path::Path, sync::Arc};
use tower_http::
    services::ServeDir
;

use axum::{routing::get, Router};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tera::Tera;

#[derive(Debug, Clone)]
struct AppState {
    tera: Arc<Tera>,
    db: Pool<Postgres>,
}

impl AppState {
    fn new(tera: Arc<Tera>, db: Pool<Postgres>) -> Self {
        Self { tera, db }
    }
}

#[tokio::main]
async fn main() {
    let tera = Arc::new(load_tera());
    let db_url =
        std::env::var("DATABASE_URL").expect("could not find database url as environment variable");
    let db = PgPoolOptions::new()
        .max_connections(8)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");

    let app_state = AppState::new(tera, db);
    let app = Router::new()
        .route("/", get(controller::frontpage::render_frontpage))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(app_state);

	let bind_addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .unwrap();
	
	println!("Creating server at {bind_addr}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn load_tera() -> Tera {
    let mut tera = Tera::default();

    tera.add_template_file(
        Path::new("templates/base.html.tera"),
        Some("base.html.tera"),
    )
    .unwrap();
    tera.add_template_file(
        Path::new("templates/frontpage.html.tera"),
        Some("frontpage.html.tera"),
    )
    .unwrap();
    tera.add_template_file(
        Path::new("templates/group.html.tera"),
        Some("group.html.tera"),
    )
    .unwrap();

    tera
}
