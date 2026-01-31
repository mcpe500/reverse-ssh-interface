use axum_test::TestServer;
use reverse_ssh_web_server::{routes, state};
use reverse_ssh_core::{
    config::init_config,
    supervisor::SessionManager,
};
use serde_json::json;

async fn create_test_state() -> state::AppState {
    let config = init_config().expect("Failed to init config");
    let (mut manager, handle) = SessionManager::new(config);
    manager.init().await.expect("Failed to init manager");
    
    // Run manager in background
    tokio::spawn(async move {
        let _ = manager.run().await;
    });
    
    state::AppState::new(handle)
}

#[tokio::test]
async fn test_health_check() {
    let state = create_test_state().await;
    let app = routes::create_routes(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/health").await;
    response.assert_status_ok();
    response.assert_json(&json!({ "status": "ok", "version": env!("CARGO_PKG_VERSION") }));
}

#[tokio::test]
async fn test_list_profiles() {
    let state = create_test_state().await;
    let app = routes::create_routes(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/profiles").await;
    response.assert_status_ok();
}

#[tokio::test]
async fn test_swagger_ui() {
    let state = create_test_state().await;
    let app = routes::create_routes(state);
    let server = TestServer::new(app).unwrap();

    let response = server.get("/swagger-ui/").await;
    response.assert_status_ok();
}
