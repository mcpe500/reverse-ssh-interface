use anyhow::{Result, Context};
use reverse_ssh_core::{
    config::init_config,
    supervisor::SessionManager,
};
use uuid::Uuid;

pub async fn run(session_id: String) -> Result<()> {
    // Parse session ID
    let id = Uuid::parse_str(&session_id)
        .context("Invalid session ID format")?;

    let config = init_config()?;

    // Create and initialize session manager
    let (mut manager, handle) = SessionManager::new(config);
    manager.init().await?;

    // Run manager in background to process commands
    tokio::spawn(async move {
        let _ = manager.run().await;
    });

    println!("Stopping session '{}'...", session_id);

    match handle.stop(id).await {
        Ok(_) => println!("Session stopped."),
        Err(e) => eprintln!("Failed to stop session: {}", e),
    }

    handle.shutdown().await?;
    Ok(())
}
