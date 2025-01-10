use axum::{Extension, Router, routing::get};
use common::models::context::ServerContext;
use std::sync::OnceLock;
use tracing_subscriber;

mod routes;
mod services;

static CONTEXT: OnceLock<ServerContext> = OnceLock::new();

#[tokio::main]
async fn main() {
	CONTEXT.get_or_init(|| ServerContext::new());

	tracing_subscriber::fmt::init();

	let (tx, _rx) = services::websocket::setup_broadcast_channel();

	let app = Router::new()
		.route("/ws", get(routes::websocket::websocket_handler))
		.layer(Extension(tx));

	let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
		.await
		.unwrap();
	tracing::info!("Listening on {}", listener.local_addr().unwrap());

	axum::serve(listener, app.into_make_service())
		.await
		.map(|_| tracing::info!("Server shutdown gracefully"))
		.map_err(|e| tracing::error!("Server shutdown with error: {}", e))
		.ok();
}
