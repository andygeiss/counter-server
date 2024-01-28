use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowHeaders, AllowOrigin, CorsLayer},
    services::{ServeDir, ServeFile},
};

use crate::{app_state::*, handlers::*};

mod app_state;
mod handlers;
mod templates;

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let router = Router::new()
        .fallback_service(
            ServeDir::new("./assets/").not_found_service(ServeFile::new("./assets/404.html")),
        )
        .route("/counter", get(counter_handler))
        .route("/", get(index_handler))
        .layer(
            CorsLayer::new()
                .allow_headers(AllowHeaders::any())
                .allow_origin(AllowOrigin::any()),
        )
        .layer(CompressionLayer::new())
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    println!(
        "listening on {}",
        listener.local_addr().expect("Failed to get local address")
    );

    axum::serve(listener, router)
        .await
        .expect("Failed to run server");
}
