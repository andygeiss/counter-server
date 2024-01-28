use askama::Template;

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

use std::sync::Arc;

use crate::{app_state::*, templates::*};

pub async fn counter_handler(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let counter = app_state.increment_counter();
    let template = CounterTemplate { counter };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn index_handler() -> impl IntoResponse {
    let template = IndexTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
