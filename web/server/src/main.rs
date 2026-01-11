use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;

pub mod routes;
pub mod state;
pub mod static_files;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,reverse_ssh_web_server=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = state::AppState::new();

    let app = routes::create_routes(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr_str = format!("{}:{}", host, port);
    
    let addr: SocketAddr = addr_str.parse().unwrap_or_else(|e| {
        tracing::error!("Invalid HOST or PORT ({:?}): {}", addr_str, e);
        std::process::exit(1);
    });

    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
