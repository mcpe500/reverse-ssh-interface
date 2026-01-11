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
