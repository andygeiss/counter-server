use axum::{routing::get, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowHeaders, AllowOrigin, CorsLayer},
    services::{ServeDir, ServeFile},
};

use crate::{
    app_state::AppState,
    errors::Error,
    handlers::{counter, index},
    prelude::Result,
};

mod app_state;
mod errors;
mod handlers;
mod prelude;
mod templates;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let server_address = "127.0.0.1:3000";
    let state = Arc::new(AppState::new());

    let router = Router::new()
        .fallback_service(
            ServeDir::new("./assets/").not_found_service(ServeFile::new("./assets/404.html")),
        )
        .route("/counter", get(counter::handler))
        .route("/", get(index::handler))
        .layer(
            CorsLayer::new()
                .allow_headers(AllowHeaders::any())
                .allow_origin(AllowOrigin::any()),
        )
        .layer(CompressionLayer::new())
        .with_state(state);

    let listener = TcpListener::bind(server_address)
        .await
        .map_err(|err| Error::Bind(err))?;

    println!(
        "listening on {}",
        listener
            .local_addr()
            .map_err(|err| Error::Listen(err, server_address))?
    );

    axum::serve(listener, router)
        .await
        .map_err(|err| Error::Serve(err))?;

    Ok(())
}
