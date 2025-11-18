use axum::{
    Router,
    extract::Request,
    http::{
        HeaderValue,
        header::{CACHE_CONTROL, CONTENT_TYPE, EXPIRES, PRAGMA},
    },
    middleware::{Next, from_fn},
    response::Response,
};
use std::path::PathBuf;

use tower_http::services::ServeFile;

use crate::cli::ActivationLoggerArgs;

pub(crate) async fn run(
    ActivationLoggerArgs {
        activation_log_file,
        url_path,
        server_ip,
        port,
    }: ActivationLoggerArgs,
) {
    let bind_addr = format!("{}:{}", server_ip, port);
    tracing::info!("Starting server at http://{}", &bind_addr);
    tracing::info!(
        "Serving csv log file, {:?}, at http://{}{}",
        &activation_log_file.clone(),
        &bind_addr,
        &url_path
    );
    let router = serve_activation_log_file(&url_path, activation_log_file)
        .layer(from_fn(set_no_cache_headers));

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Failed to bind to address {}, is the port already in use?",
                &bind_addr
            )
        });

    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Error in the graceful shutdown, slightly ironic");
            tracing::info!("We are shutting the server down. :(");
        })
        .await
        .unwrap();
}

fn serve_activation_log_file(url_path: &str, activation_log_file: PathBuf) -> Router {
    Router::new().route_service(url_path, ServeFile::new(activation_log_file))
}
async fn set_no_cache_headers(req: Request, next: Next) -> Response {
    let mut res = next.run(req).await;
    let h = res.headers_mut();
    h.insert(CONTENT_TYPE, HeaderValue::from_static("text/csv"));
    h.insert(
        CACHE_CONTROL,
        HeaderValue::from_static("no-cache, no-store, must-revalidate"),
    );
    h.insert(PRAGMA, HeaderValue::from_static("no-cache"));
    h.insert(EXPIRES, HeaderValue::from_static("0"));
    res
}
