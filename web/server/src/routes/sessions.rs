use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reverse_ssh_core::config::load_profiles;
use reverse_ssh_core::supervisor::StartSessionOptions;
use crate::state::AppState;
use serde_json::json;
use uuid::Uuid;

use super::types::{ApiSession, StartSessionRequest};

#[utoipa::path(
    get,
    path = "/api/sessions",
    responses(
        (status = 200, description = "List all active sessions", body = [ApiSession])
    ),
    tag = "sessions"
)]
pub async fn list_sessions(State(state): State<AppState>) -> impl IntoResponse {
    match state.handle.status().await {
        Ok(sessions) => {
            let api_sessions: Vec<ApiSession> = sessions.into_iter().map(Into::into).collect();
            Json(api_sessions).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/sessions/{name}/start",
    params(
        ("name" = String, Path, description = "Profile name to start session for")
    ),
    request_body = StartSessionRequest,
    responses(
        (status = 200, description = "Session started successfully"),
        (status = 404, description = "Profile not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "sessions"
)]
pub async fn start_session(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(req): Json<StartSessionRequest>,
) -> impl IntoResponse {
    // Load profiles to find the one to start
    let profiles = match load_profiles() {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(json!({ "error": e.to_string() }))
            ).into_response();
        }
    };

    if let Some(profile) = profiles.into_iter().find(|p| p.name == name) {
        let password = req.password.and_then(|p| {
            let trimmed = p.trim().to_string();
            if trimmed.is_empty() { None } else { Some(trimmed) }
        });

        let sshpass_path = req.sshpass_path.and_then(|p| {
            let trimmed = p.trim().to_string();
            if trimmed.is_empty() { None } else { Some(trimmed) }
        });

        match state
            .handle
            .start_with_options(profile, StartSessionOptions { password, sshpass_path })
            .await
        {
            Ok(session_id) => (
                StatusCode::OK, 
                Json(json!({ "status": "started", "session_id": session_id.to_string() }))
            ).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(json!({ "error": e.to_string() }))
            ).into_response(),
        }
    } else {
        (
            StatusCode::NOT_FOUND, 
            Json(json!({ "error": "Profile not found" }))
        ).into_response()
    }
}

#[utoipa::path(
    post,
    path = "/api/sessions/{id}/stop",
    params(
        ("id" = String, Path, description = "Session ID to stop")
    ),
    responses(
        (status = 200, description = "Session stopped successfully"),
        (status = 400, description = "Invalid session ID"),
        (status = 500, description = "Internal server error")
    ),
    tag = "sessions"
)]
pub async fn stop_session(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let session_id = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid session ID format" })),
            ).into_response();
        }
    };

    match state.handle.stop(session_id).await {
        Ok(_) => (
            StatusCode::OK, 
            Json(json!({ "status": "stopped", "id": id }))
        ).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR, 
            Json(json!({ "error": e.to_string() }))
        ).into_response(),
    }
}
