use anyhow::{Result, Context};
use reverse_ssh_core::{
    config::load,
    supervisor::manager::SessionManager,
};
use tokio::signal;
use std::time::Duration;
use tokio::time;

pub async fn run(id: String) -> Result<()> {
    let config = load::load_config().await?;
    let profile = config.get_profile(&id)
        .context(format!("Profile '{}' not found", id))?;

    println!("Starting profile '{}' ({})", id, profile.host);

    let manager = SessionManager::new();
    
    // Start session
    if let Err(e) = manager.start(profile.clone()).await {
        eprintln!("Failed to start session: {}", e);
        return Err(e);
    }
    
    println!("Session started. Press Ctrl+C to stop.");

    // Select between Ctrl+C and a status monitor loop
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("\nReceived Ctrl+C, stopping...");
                manager.stop(&id).await?;
                println!("Stopped.");
                break;
            }
            _ = time::sleep(Duration::from_secs(2)) => {
                // Periodically check if it is still running
                if let Some(session) = manager.get_session(&id).await {
                     match session.status {
                        reverse_ssh_core::types::session::SessionStatus::Failed(reason) => {
                            eprintln!("Session failed unexpectedly: {}", reason);
                            manager.stop(&id).await?;
                            break;
                        }
                        reverse_ssh_core::types::session::SessionStatus::Stopped => {
                            eprintln!("Session stopped unexpectedly.");
                            break;
                        }
                        _ => {} // Still running/starting
                     }
                } else {
                     eprintln!("Session state lost?");
                     break;
                }
            }
        }
    }

    Ok(())
}
