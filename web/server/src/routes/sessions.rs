use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reverse_ssh_core::config::load;
use crate::state::AppState;
use serde_json::json;

pub async fn list_sessions(State(state): State<AppState>) -> impl IntoResponse {
    let sessions = state.session_manager.list_sessions().await;
    Json(sessions)
}

pub async fn start_session(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Load config to get profile
    let config = match load::load_config().await {
        Ok(c) => c,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))).into_response(),
    };

    if let Some(profile) = config.get_profile(&id) {
        match state.session_manager.start(profile.clone()).await {
            Ok(_) => (StatusCode::OK, Json(json!({ "status": "started", "id": id }))).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))).into_response(),
        }
    } else {
        (StatusCode::NOT_FOUND, Json(json!({ "error": "Profile not found" }))).into_response()
    }
}

pub async fn stop_session(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.session_manager.stop(&id).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "stopped", "id": id }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))).into_response(),
    }
}
