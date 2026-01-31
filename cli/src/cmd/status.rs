use anyhow::Result;
use reverse_ssh_core::{
    config::init_config,
    supervisor::SessionManager,
    types::SessionStatus,
};
use uuid::Uuid;

use crate::output::OutputFormat;

pub async fn run(session_id: Option<String>, format: OutputFormat) -> Result<()> {
    let config = init_config()?;

    let (mut manager, handle) = SessionManager::new(config);
    manager.init().await?;

    // Run manager briefly
    tokio::spawn(async move {
        let _ = manager.run().await;
    });

    let sessions = handle.status().await?;

    if let Some(id_str) = session_id {
        let id = Uuid::parse_str(&id_str)?;
        if let Some(session) = sessions.iter().find(|s| s.id == id) {
            print_session(session, &format);
        } else {
            println!("Session not found: {}", id_str);
        }
    } else {
        if sessions.is_empty() {
            println!("No active sessions.");
        } else {
            for session in &sessions {
                print_session(session, &format);
                println!();
            }
        }
    }

    handle.shutdown().await?;
    Ok(())
}

fn print_session(session: &reverse_ssh_core::types::Session, format: &OutputFormat) {
    match format {
        OutputFormat::Human => {
            println!("Session ID: {}", session.id);
            println!("Profile:    {}", session.profile_name);
            println!("Status:     {}", format_status(&session.status));
            println!("Started:    {}", session.started_at.format("%Y-%m-%d %H:%M:%S"));
            if let Some(pid) = session.pid {
                println!("PID:        {}", pid);
            }
        }
        OutputFormat::Json => {
            let json = serde_json::json!({
                "id": session.id.to_string(),
                "profile": session.profile_name,
                "status": format_status(&session.status),
                "started_at": session.started_at.to_rfc3339(),
                "pid": session.pid,
            });
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
    }
}

fn format_status(status: &SessionStatus) -> &'static str {
    match status {
        SessionStatus::Starting => "starting",
        SessionStatus::Connected => "connected",
        SessionStatus::Reconnecting => "reconnecting",
        SessionStatus::Stopped => "stopped",
        SessionStatus::Failed => "failed",
    }
}
