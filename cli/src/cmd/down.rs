use anyhow::Result;
use reverse_ssh_core::supervisor::manager::SessionManager;

pub async fn run(id: String) -> Result<()> {
    let manager = SessionManager::new();
    println!("Stopping session '{}'...", id);
    
    match manager.stop(&id).await {
        Ok(_) => println!("Session stopped."),
        Err(e) => eprintln!("Failed to stop session: {}", e),
    }

    Ok(())
}
