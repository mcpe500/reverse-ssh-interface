use anyhow::{Result, Context};
use reverse_ssh_core::{
    config::{init_config, load_profiles},
    supervisor::SessionManager,
    types::SessionStatus,
};
use tokio::signal;
use std::time::Duration;
use tokio::time;

pub async fn run(name: String) -> Result<()> {
    // Initialize config and load profiles
    let config = init_config()?;
    let profiles = load_profiles()?;
    
    // Find profile by name
    let profile = profiles.iter()
        .find(|p| p.name == name)
        .context(format!("Profile '{}' not found", name))?
        .clone();

    println!("Starting profile '{}' ({})", name, profile.host);

    // Create and initialize session manager
    let (mut manager, handle) = SessionManager::new(config);
    manager.init().await?;
    
    // Run manager in background
    tokio::spawn(async move {
        if let Err(e) = manager.run().await {
            eprintln!("Session manager error: {}", e);
        }
    });

    // Start session
    let session_id = handle.start(profile).await?;
    println!("Session started (ID: {}). Press Ctrl+C to stop.", session_id);

    // Subscribe to events for monitoring
    let mut events = handle.subscribe();

    // Monitor loop
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("\nReceived Ctrl+C, stopping...");
                handle.stop(session_id).await?;
                println!("Stopped.");
                break;
            }
            event = events.recv() => {
                if let Ok(event) = event {
                    match event {
                        reverse_ssh_core::types::Event::SessionConnected { profile_name, .. } => {
                            println!("Session '{}' connected", profile_name);
                        }
                        reverse_ssh_core::types::Event::SessionDisconnected { profile_name, reason, .. } => {
                            println!("Session '{}' disconnected: {:?}", profile_name, reason);
                        }
                        reverse_ssh_core::types::Event::SessionFailed { profile_name, error, .. } => {
                            eprintln!("Session '{}' failed: {}", profile_name, error);
                            break;
                        }
                        reverse_ssh_core::types::Event::SessionReconnecting { profile_name, attempt, max_attempts, .. } => {
                            let max = if max_attempts == 0 { "unlimited".to_string() } else { max_attempts.to_string() };
                            println!("Session '{}' reconnecting (attempt {}/{})", profile_name, attempt, max);
                        }
                        _ => {}
                    }
                }
            }
            _ = time::sleep(Duration::from_secs(30)) => {
                // Periodic status check
                let sessions = handle.status().await?;
                if let Some(session) = sessions.iter().find(|s| s.id == session_id) {
                    if session.status == SessionStatus::Failed {
                        eprintln!("Session failed: {:?}", session.last_error);
                        break;
                    }
                }
            }
        }
    }

    // Cleanup
    handle.shutdown().await?;
    Ok(())
}
