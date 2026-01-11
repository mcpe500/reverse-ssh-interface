use axum::Router;
use axum::routing::{get, post};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::state::AppState;
use crate::static_files;
use reverse_ssh_core::types::{profile::*, session::*};

pub mod health;
pub mod profiles;
pub mod sessions;
pub mod ws;

#[derive(OpenApi)]
#[openapi(
    paths(
        health::check,
        profiles::list_profiles,
        profiles::create_profile,
        profiles::get_profile,
        profiles::delete_profile,
        sessions::list_sessions,
        sessions::start_session,
        sessions::stop_session,
    ),
    components(
        schemas(Profile, AuthMethod, ForwardRule, AdvancedOptions, Session, SessionStatus)
    ),
    tags(
        (name = "reverse-ssh", description = "Reverse SSH Interface API")
    )
)]
pub struct ApiDoc;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(static_files::index))
        .route("/health", get(health::check))
        .route("/api/profiles", get(profiles::list_profiles).post(profiles::create_profile))
        .route("/api/profiles/{id}", get(profiles::get_profile).delete(profiles::delete_profile))
        .route("/api/sessions", get(sessions::list_sessions))
        .route("/api/sessions/{id}/start", post(sessions::start_session))
        .route("/api/sessions/{id}/stop", post(sessions::stop_session))
        .route("/ws", get(ws::ws_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}