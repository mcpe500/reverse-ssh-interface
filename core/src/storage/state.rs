use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::paths;
use crate::error::{CoreError, Result};
use crate::types::{Session, SessionStatus};

/// Persisted application state
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppState {
    /// Sessions that were running when the app last closed
    #[serde(default)]
    pub sessions: Vec<PersistedSession>,
    /// Last active profile ID
    pub last_active_profile: Option<Uuid>,
}

/// Minimal session info for persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedSession {
    pub profile_id: Uuid,
    pub profile_name: String,
    pub was_connected: bool,
}

impl From<&Session> for PersistedSession {
    fn from(session: &Session) -> Self {
        Self {
            profile_id: session.profile_id,
            profile_name: session.profile_name.clone(),
            was_connected: session.status == SessionStatus::Connected,
        }
    }
}

/// State manager for persisting application state
pub struct StateManager {
    path: std::path::PathBuf,
    state: AppState,
}

impl StateManager {
    /// Create a new state manager using the default path
    pub fn new() -> Self {
        Self {
            path: paths::state_file(),
            state: AppState::default(),
        }
    }

    /// Create a state manager with a custom path
    pub fn with_path(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            state: AppState::default(),
        }
    }

    /// Load state from disk
    pub fn load(&mut self) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let content = std::fs::read_to_string(&self.path)
            .map_err(|e| CoreError::StorageAccess(format!("Failed to read state file: {}", e)))?;

        self.state = serde_json::from_str(&content)
            .map_err(|e| CoreError::Deserialization(format!("Failed to parse state: {}", e)))?;

        Ok(())
    }

    /// Save state to disk
    pub fn save(&self) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| CoreError::StorageAccess(format!("Failed to create directory: {}", e)))?;
        }

        let content = serde_json::to_string_pretty(&self.state)
            .map_err(|e| CoreError::Serialization(format!("Failed to serialize state: {}", e)))?;

        std::fs::write(&self.path, content)
            .map_err(|e| CoreError::StorageAccess(format!("Failed to write state file: {}", e)))?;

        Ok(())
    }

    /// Get the current state
    pub fn state(&self) -> &AppState {
        &self.state
    }

    /// Get mutable state
    pub fn state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    /// Update sessions from current running sessions
    pub fn update_sessions(&mut self, sessions: &[Session]) {
        self.state.sessions = sessions.iter().map(PersistedSession::from).collect();
    }

    /// Set the last active profile
    pub fn set_last_active_profile(&mut self, profile_id: Option<Uuid>) {
        self.state.last_active_profile = profile_id;
    }

    /// Get profiles that should be auto-started
    pub fn get_auto_start_profiles(&self) -> Vec<Uuid> {
        self.state
            .sessions
            .iter()
            .filter(|s| s.was_connected)
            .map(|s| s.profile_id)
            .collect()
    }

    /// Clear all state
    pub fn clear(&mut self) {
        self.state = AppState::default();
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_state_save_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("state.json");

        let mut manager = StateManager::with_path(&path);
        manager.state.last_active_profile = Some(Uuid::new_v4());
        manager.state.sessions.push(PersistedSession {
            profile_id: Uuid::new_v4(),
            profile_name: "test".to_string(),
            was_connected: true,
        });

        manager.save().unwrap();

        let mut manager2 = StateManager::with_path(&path);
        manager2.load().unwrap();

        assert_eq!(
            manager.state.last_active_profile,
            manager2.state.last_active_profile
        );
        assert_eq!(manager.state.sessions.len(), manager2.state.sessions.len());
    }
}
