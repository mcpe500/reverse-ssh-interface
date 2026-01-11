use axum::Router;
use axum::routing::{get, post};
use crate::state::AppState;
use crate::static_files;

pub mod health;
pub mod profiles;
pub mod sessions;
pub mod ws;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(static_files::index))
        .route("/health", get(health::check))
        .route("/api/profiles", get(profiles::list_profiles).post(profiles::create_profile))
        .route("/api/profiles/:id", get(profiles::get_profile).delete(profiles::delete_profile))
        .route("/api/sessions", get(sessions::list_sessions))
        .route("/api/sessions/:id/start", post(sessions::start_session))
        .route("/api/sessions/:id/stop", post(sessions::stop_session))
        .route("/ws", get(ws::ws_handler))
        .with_state(state)
}