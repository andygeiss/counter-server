use crate::templates::index::Index;
use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn handler() -> impl IntoResponse {
    let template = Index {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
