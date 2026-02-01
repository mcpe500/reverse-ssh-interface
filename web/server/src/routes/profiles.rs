use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reverse_ssh_core::{
    config::{load_profiles, save_profile, delete_profile as core_delete_profile},
    types::Profile,
};
use serde_json::json;
use uuid::Uuid;
use std::collections::HashMap;

use super::types::{ApiProfile, CreateProfileRequest, UpdateProfileRequest};

#[utoipa::path(
    get,
    path = "/api/profiles",
    responses(
        (status = 200, description = "List all profiles", body = [ApiProfile]),
        (status = 500, description = "Internal server error")
    ),
    tag = "profiles"
)]
pub async fn list_profiles() -> impl IntoResponse {
    match load_profiles() {
        Ok(profiles) => {
            let api_profiles: Vec<ApiProfile> = profiles.into_iter().map(Into::into).collect();
            (StatusCode::OK, Json(api_profiles)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/profiles",
    request_body = CreateProfileRequest,
    responses(
        (status = 201, description = "Profile created successfully", body = ApiProfile),
        (status = 400, description = "Invalid request"),
        (status = 409, description = "Profile already exists"),
        (status = 500, description = "Internal server error")
    ),
    tag = "profiles"
)]
pub async fn create_profile(Json(req): Json<CreateProfileRequest>) -> impl IntoResponse {
    // Check if profile already exists
    match load_profiles() {
        Ok(profiles) => {
            if profiles.iter().any(|p| p.name == req.name) {
                return (
                    StatusCode::CONFLICT,
                    Json(json!({ "error": format!("Profile '{}' already exists", req.name) })),
                ).into_response();
            }
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ).into_response();
        }
    }

    if req.tunnels.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "At least one tunnel is required" })),
        ).into_response();
    }

    let profile = Profile {
        id: Uuid::new_v4(),
        name: req.name.clone(),
        host: req.host,
        port: req.port.unwrap_or(22),
        user: req.user,
        auth: req.auth.map(Into::into).unwrap_or_default(),
        tunnels: req.tunnels.into_iter().map(Into::into).collect(),
        keepalive_interval: 20,
        keepalive_count: 3,
        auto_reconnect: true,
        max_reconnect_attempts: 0,
        extra_options: HashMap::new(),
        ssh_path: None,
        known_hosts_file: None,
        identity_file: None,
        password: None,
    };

    if let Err(e) = save_profile(&profile) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to save profile: {}", e) })),
        ).into_response();
    }

    let api_profile: ApiProfile = profile.into();
    (StatusCode::CREATED, Json(api_profile)).into_response()
}

#[utoipa::path(
    get,
    path = "/api/profiles/{name}",
    params(
        ("name" = String, Path, description = "Profile name")
    ),
    responses(
        (status = 200, description = "Get profile by name", body = ApiProfile),
        (status = 404, description = "Profile not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "profiles"
)]
pub async fn get_profile(Path(name): Path<String>) -> impl IntoResponse {
    match load_profiles() {
        Ok(profiles) => {
            if let Some(profile) = profiles.into_iter().find(|p| p.name == name) {
                let api_profile: ApiProfile = profile.into();
                (StatusCode::OK, Json(api_profile)).into_response()
            } else {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Profile not found" }))).into_response()
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/profiles/{name}",
    params(
        ("name" = String, Path, description = "Profile name")
    ),
    responses(
        (status = 200, description = "Profile deleted successfully"),
        (status = 404, description = "Profile not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "profiles"
)]
pub async fn delete_profile(Path(name): Path<String>) -> impl IntoResponse {
    match load_profiles() {
        Ok(profiles) => {
            if let Some(profile) = profiles.iter().find(|p| p.name == name) {
                if let Err(e) = core_delete_profile(profile) {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "error": format!("Failed to delete profile: {}", e) })),
                    ).into_response();
                }
                (StatusCode::OK, Json(json!({ "status": "deleted" }))).into_response()
            } else {
                (StatusCode::NOT_FOUND, Json(json!({ "error": "Profile not found" }))).into_response()
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/profiles/{name}",
    params(
        ("name" = String, Path, description = "Existing profile name")
    ),
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = ApiProfile),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "Profile not found"),
        (status = 409, description = "Profile name already exists"),
        (status = 500, description = "Internal server error")
    ),
    tag = "profiles"
)]
pub async fn update_profile(
    Path(name): Path<String>,
    Json(req): Json<UpdateProfileRequest>,
) -> impl IntoResponse {
    let profiles = match load_profiles() {
        Ok(p) => p,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let existing = match profiles.iter().find(|p| p.name == name) {
        Some(p) => p.clone(),
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Profile not found" })),
            )
                .into_response();
        }
    };

    let mut updated = existing.clone();

    if let Some(new_name) = req.name {
        updated.name = new_name;
    }
    if let Some(host) = req.host {
        updated.host = host;
    }
    if let Some(port) = req.port {
        updated.port = port;
    }
    if let Some(user) = req.user {
        updated.user = user;
    }
    if let Some(auth) = req.auth {
        updated.auth = auth.into();
    }
    if let Some(tunnels) = req.tunnels {
        if tunnels.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "At least one tunnel is required" })),
            )
                .into_response();
        }
        updated.tunnels = tunnels.into_iter().map(Into::into).collect();
    }

    // Rename collision check
    if updated.name != name && profiles.iter().any(|p| p.name == updated.name) {
        return (
            StatusCode::CONFLICT,
            Json(json!({ "error": format!("Profile '{}' already exists", updated.name) })),
        )
            .into_response();
    }

    if let Err(e) = save_profile(&updated) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to save profile: {}", e) })),
        )
            .into_response();
    }

    // If renamed, delete old file
    if updated.name != name {
        let _ = core_delete_profile(&existing);
    }

    let api_profile: ApiProfile = updated.into();
    (StatusCode::OK, Json(api_profile)).into_response()
}
