use clap::Parser;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
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
    let state = state::AppState::new();

    let app = routes::create_routes(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let addr_str = format!("{}:{}", args.host, args.port);
    
    let addr: SocketAddr = addr_str.parse().unwrap_or_else(|e| {
        tracing::error!("Invalid bind address ({:?}): {}", addr_str, e);
        std::process::exit(1);
    });

    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
