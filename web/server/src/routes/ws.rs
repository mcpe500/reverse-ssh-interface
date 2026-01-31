use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use crate::state::AppState;
use tokio::time::{self, Duration};
use serde_json::json;
use futures_util::{SinkExt, StreamExt};

use super::types::ApiSession;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut interval = time::interval(Duration::from_secs(2));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let sessions_result = state.handle.status().await;
                let msg = match sessions_result {
                    Ok(sessions) => {
                        let api_sessions: Vec<ApiSession> = sessions.into_iter().map(Into::into).collect();
                        json!({ "type": "sessions_update", "data": api_sessions }).to_string()
                    }
                    Err(e) => {
                        json!({ "type": "error", "message": e.to_string() }).to_string()
                    }
                };

                if sender.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
            // Keep the connection alive by reading (even if we ignore input)
            // If the client closes, this returns None and we break
            msg = receiver.next() => {
                if msg.is_none() {
                    break;
                }
            }
        }
    }
}
