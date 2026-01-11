use anyhow::Result;
use reverse_ssh_core::supervisor::manager::SessionManager;
use reverse_ssh_core::types::session::SessionStatus;

pub async fn run() -> Result<()> {
    let manager = SessionManager::new();
    let sessions = manager.list_sessions().await;

    if sessions.is_empty() {
        println!("No active or recent sessions.");
        return Ok(());
    }

    println!("{:<20} {:<15} {:<10} {:<20}", "Profile ID", "Status", "PID", "Started At");
    println!("{}", "-".repeat(65));

    for sess in sessions {
        let status_str = match &sess.status {
            SessionStatus::Running => "Running",
            SessionStatus::Starting => "Starting",
            SessionStatus::Stopped => "Stopped",
            SessionStatus::Failed(_) => "Failed",
            SessionStatus::Retrying { .. } => "Retrying",
        };

        let pid_str = sess.pid.map(|p| p.to_string()).unwrap_or_else(|| "-".to_string());
        let start_str = sess.start_time
            .map(|t| t.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "-".to_string());

        println!("{:<20} {:<15} {:<10} {:<20}", sess.profile_id, status_str, pid_str, start_str);
        
        if let SessionStatus::Failed(reason) = &sess.status {
            println!("  Error: {}", reason);
        }
    }

    Ok(())
}
