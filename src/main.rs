use std::sync::{Arc, Mutex};

use axum::{extract, response, routing::get, Router};

#[derive(Debug, Clone)]
struct AppState {
    counter: Arc<Mutex<isize>>,
}

impl AppState {
    fn new(counter: Arc<Mutex<isize>>) -> Self {
        Self { counter }
    }
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let app_state = AppState::new(counter);
    let app = Router::new()
        .route("/", get(homepage))
        .route("/increment", get(increment))
        .route("/decrement", get(decrement))
        .with_state(app_state);

    let bind_addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();

    println!("Creating server at http://{bind_addr}");

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn homepage(
    extract::State(AppState { counter }): extract::State<AppState>,
) -> response::Html<String> {
    let val: isize = *counter.lock().unwrap();
    response::Html(format!(include_str!("../index.html"), val))
}

async fn increment(
    extract::State(AppState { counter }): extract::State<AppState>,
) -> impl response::IntoResponse {
    let f = |x: isize| x.saturating_add(1);
    change_counter(counter, f)
}

async fn decrement(
    extract::State(AppState { counter }): extract::State<AppState>,
) -> impl response::IntoResponse {
    let f = |x: isize| x.saturating_sub(1);
    change_counter(counter, f)
}

fn change_counter(
    counter: Arc<Mutex<isize>>,
    f: fn(isize) -> isize,
) -> impl response::IntoResponse {
    let mut val_ref = counter.lock().unwrap();
    *val_ref = f(*val_ref);
    response::Redirect::permanent("/")
}
