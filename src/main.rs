use axum::{routing::get, Extension, Router};
use config::entity_template::{create_entity_from_template, load_entity_template};
use std::net::SocketAddr;
use tracing_subscriber;

mod config;
mod models;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (tx, _rx) = services::websocket::setup_broadcast_channel();

    let app = Router::new()
        .route("/ws", get(routes::websocket::websocket_handler))
        .layer(Extension(tx));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map(|_| tracing::info!("Server shutdown gracefully"))
        .map_err(|e| tracing::error!("Server shutdown with error: {}", e))
        .ok();
}
