use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use crate::state::AppState;
use tokio::time::{self, Duration};
use serde_json::json;
use futures_util::{SinkExt, StreamExt};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut _receiver) = socket.split();
    let mut interval = time::interval(Duration::from_secs(2));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let sessions = state.session_manager.list_sessions().await;
                let msg = json!({ "type": "sessions_update", "data": sessions }).to_string();

                if sender.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
            // Keep the connection alive by reading (even if we ignore input)
            // If the client closes, this returns None and we break
            msg = _receiver.next() => {
                if msg.is_none() {
                    break;
                }
            }
        }
    }
}
