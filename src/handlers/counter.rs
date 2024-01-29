use crate::{app_state::AppState, app_state::CounterResourceAccess, templates::counter::Counter};
use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

pub async fn handler(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let counter = app_state.increment_counter();
    let template = Counter { counter };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
