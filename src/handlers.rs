use askama::Template;
use axum::{extract::State, response::{Html, IntoResponse}, http::StatusCode};
use std::sync::Arc;

use crate::model::{AppState, ResourceAccess};
use crate::templates::CounterTemplate;

pub async fn counter_handler(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    app_state.increment_counter();
    let counter = app_state.get_counter();
    let template = CounterTemplate { counter };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
