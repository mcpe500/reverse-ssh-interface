use clap::Parser;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use reverse_ssh_core::{
    config::init_config,
    supervisor::SessionManager,
};
use reverse_ssh_web_server::{routes, state};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host to bind to
    #[arg(long, default_value = "127.0.0.1", env = "HOST")]
    host: String,

    /// Port to bind to
    #[arg(long, default_value = "3000", env = "PORT")]
    port: u16,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,reverse_ssh_web_server=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();

    // Initialize configuration
    let config = match init_config() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to initialize configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Create session manager
    let (mut manager, handle) = SessionManager::new(config);
    
    // Initialize manager (loads persisted state)
    if let Err(e) = manager.init().await {
        tracing::error!("Failed to initialize session manager: {}", e);
        std::process::exit(1);
    }

    // Run session manager in background
    tokio::spawn(async move {
        if let Err(e) = manager.run().await {
            tracing::error!("Session manager error: {}", e);
        }
    });

    let state = state::AppState::new(handle);

    let app = routes::create_routes(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr_str = format!("{}:{}", args.host, args.port);
    
    let addr: SocketAddr = addr_str.parse().unwrap_or_else(|e| {
        tracing::error!("Invalid bind address ({:?}): {}", addr_str, e);
        std::process::exit(1);
    });

    tracing::info!("listening on {}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui/", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
