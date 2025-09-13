use std::collections::{HashMap, HashSet};

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;
use tera::Context;

use crate::{
    dto::host::{CurrentHostDto, CurrentHostGroupDto},
    AppState,
};
#[derive(Debug, Clone, Serialize)]
struct HistoryPageContext {}

impl HistoryPageContext {
    fn new() -> Self {
        Self {}
    }
}

// #[axum::debug_handler]
// pub async fn render_history_page(
//     State(AppState {
//         tera, host_repo, ..
//     }): State<AppState>,
// ) -> impl IntoResponse {
// }

#[axum::debug_handler]
pub async fn render_history_page(
    State(AppState {
        tera, host_repo, ..
    }): State<AppState>,
    Path(host_name): Path<String>,
) -> impl IntoResponse {
    let fp_ctx = HistoryPageContext::new();
    let mut ctx = Context::new();
    ctx.insert("history_ctx", &fp_ctx);
    ctx.insert("title", "Hostmap - History");

    let output = tera.render("history.html.tera", &ctx).unwrap();
    Html(output)
}
