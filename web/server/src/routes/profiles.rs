use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use reverse_ssh_core::{
    config::load,
    types::profile::Profile,
};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/api/profiles",
    responses(
        (status = 200, description = "List all profiles", body = [Profile]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_profiles() -> impl IntoResponse {
    match load::load_config().await {
        Ok(config) => {
            // Convert HashMap to Vec for easier frontend consumption
            let profiles: Vec<Profile> = config.profiles.into_values().collect();
            (StatusCode::OK, Json(profiles)).into_response()
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
    request_body = Profile,
    responses(
        (status = 201, description = "Profile created successfully", body = Profile),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_profile(Json(profile): Json<Profile>) -> impl IntoResponse {
    match load::load_config().await {
        Ok(mut config) => {
            config.add_profile(profile.clone());
            if let Err(e) = load::save_config(&config).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    Json(json!({ "error": format!("Failed to save config: {}", e) }))
                ).into_response();
            }
            (StatusCode::CREATED, Json(profile)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to load config: {}", e) })),
        ).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/profiles/{id}",
    params(
        ("id" = String, Path, description = "Profile ID")
    ),
    responses(
        (status = 200, description = "Get profile by ID", body = Profile),
        (status = 404, description = "Profile not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_profile(Path(id): Path<String>) -> impl IntoResponse {
    match load::load_config().await {
        Ok(config) => {
            if let Some(profile) = config.get_profile(&id) {
                (StatusCode::OK, Json(profile)).into_response()
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
    path = "/api/profiles/{id}",
    params(
        ("id" = String, Path, description = "Profile ID")
    ),
    responses(
        (status = 200, description = "Profile deleted successfully"),
        (status = 404, description = "Profile not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_profile(Path(id): Path<String>) -> impl IntoResponse {
    match load::load_config().await {
        Ok(mut config) => {
            if config.remove_profile(&id).is_some() {
                if let Err(e) = load::save_config(&config).await {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR, 
                        Json(json!({ "error": format!("Failed to save config: {}", e) }))
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
