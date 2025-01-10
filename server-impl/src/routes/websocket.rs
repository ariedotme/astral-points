use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use axum::{Extension, extract::WebSocketUpgrade, response::IntoResponse};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;

pub async fn websocket_handler(
	ws: WebSocketUpgrade,
	Extension(tx): Extension<broadcast::Sender<String>>,
) -> impl IntoResponse {
	ws.on_upgrade(|socket| handle_socket(socket, tx))
}

async fn handle_socket(socket: WebSocket, tx: broadcast::Sender<String>) {
	let (mut sender, mut receiver) = socket.split();

	let mut rx = tx.subscribe();

	tokio::spawn(async move {
		while let Ok(msg) = rx.recv().await {
			if sender
				.send(Message::Text(Utf8Bytes::from(msg)))
				.await
				.is_err()
			{
				break;
			}
		}
	});

	while let Some(Ok(msg)) = receiver.next().await {
		if let Message::Text(text) = msg {
			tracing::info!("Received: {}", text);
			let _ = tx.send(text.to_string());
		}
	}
}
