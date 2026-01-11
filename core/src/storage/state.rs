use anyhow::{Context, Result};
use tokio::fs;
use crate::types::session::Session;
use crate::config::paths;

pub async fn save_session(session: &Session) -> Result<()> {
    let dir = paths::sessions_dir()?;
    fs::create_dir_all(&dir).await.context("Failed to create sessions directory")?;
    
    let path = dir.join(format!("{}.json", session.profile_id));
    let content = serde_json::to_string_pretty(session).context("Failed to serialize session")?;
    
    let _: () = fs::write(&path, content).await.context(format!("Failed to write session file to {:?}", path))?;
    Ok(())
}

pub async fn load_session(profile_id: &str) -> Result<Option<Session>> {
    let path = paths::sessions_dir()?.join(format!("{}.json", profile_id));
    if !path.exists() {
        return Ok(None);
    }
    
    let content = fs::read_to_string(&path).await.context("Failed to read session file")?;
    let session: Session = serde_json::from_str(&content).context("Failed to parse session file")?;
    Ok(Some(session))
}

pub async fn list_persisted_sessions() -> Result<Vec<Session>> {
    let dir = paths::sessions_dir()?;
    if !dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut sessions = Vec::new();
    let mut entries = fs::read_dir(dir).await?;
    
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = fs::read_to_string(&path).await?;
            if let Ok(session) = serde_json::from_str::<Session>(&content) {
                sessions.push(session);
            }
        }
    }
    
    Ok(sessions)
}

pub async fn remove_session(profile_id: &str) -> Result<()> {
    let path = paths::sessions_dir()?.join(format!("{}.json", profile_id));
    if path.exists() {
        fs::remove_file(path).await?;
    }
    Ok(())
}
