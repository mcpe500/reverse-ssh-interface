use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reverse_ssh_core::{config::load, types::session::Session};
use crate::state::AppState;
use serde_json::json;

#[utoipa::path(
    get,
    path = "/api/sessions",
    responses(
        (status = 200, description = "List all active sessions", body = [Session])
    )
)]
pub async fn list_sessions(State(state): State<AppState>) -> impl IntoResponse {
    let sessions = state.session_manager.list_sessions().await;
    Json(sessions)
}

#[utoipa::path(
    post,
    path = "/api/sessions/{id}/start",
    params(
        ("id" = String, Path, description = "Profile ID to start session for")
    ),
    responses(
        (status = 200, description = "Session started successfully"),
        (status = 404, description = "Profile not found"),
        (status = 500, description = "Internal server error")
    )
)]
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

#[utoipa::path(
    post,
    path = "/api/sessions/{id}/stop",
    params(
        ("id" = String, Path, description = "Profile ID to stop session for")
    ),
    responses(
        (status = 200, description = "Session stopped successfully"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn stop_session(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.session_manager.stop(&id).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "stopped", "id": id }))).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))).into_response(),
    }
}
