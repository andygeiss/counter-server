use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::net::TcpListener;

mod handlers;
mod model;
mod templates;

#[tokio::main]
async fn main() {

    let app_state = Arc::new(model::AppState::new());

    let router = Router::new()
        .route("/counter", get(handlers::counter_handler))
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    println!("listening on {}", listener.local_addr()
        .expect("Failed to get local address"));

    axum::serve(listener, router)
        .await
        .expect("Failed to run server");
}
