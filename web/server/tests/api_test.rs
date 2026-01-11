use axum_test::TestServer;
use reverse_ssh_web_server::{routes, state};
use serde_json::json;

#[tokio::test]
async fn test_health_check() {
    let state = state::AppState::new();
    let app = routes::create_routes(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/health").await;
    response.assert_status_ok();
    response.assert_json(&json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION") }));
}

#[tokio::test]
async fn test_list_profiles_empty() {
    let state = state::AppState::new();
    let app = routes::create_routes(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/profiles").await;
    response.assert_status_ok();
}

#[tokio::test]
async fn test_swagger_ui() {
    let state = state::AppState::new();
    let app = routes::create_routes(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/swagger-ui/").await;
    response.assert_status_ok();
}
