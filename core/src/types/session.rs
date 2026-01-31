use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use super::profile::Profile;

/// Current status of an SSH session
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    /// Session is starting up
    Starting,
    /// Session is connected and running
    Connected,
    /// Session is reconnecting after a failure
    Reconnecting,
    /// Session has been stopped (intentionally)
    Stopped,
    /// Session has failed and won't retry
    Failed,
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionStatus::Starting => write!(f, "starting"),
            SessionStatus::Connected => write!(f, "connected"),
            SessionStatus::Reconnecting => write!(f, "reconnecting"),
            SessionStatus::Stopped => write!(f, "stopped"),
            SessionStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Runtime state of an SSH tunnel session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session identifier
    pub id: Uuid,
    /// The profile this session is based on
    pub profile_id: Uuid,
    /// Profile name (cached for display)
    pub profile_name: String,
    /// Current status
    pub status: SessionStatus,
    /// Process ID of the SSH process (if running)
    #[serde(skip)]
    pub pid: Option<u32>,
    /// When the session was started
    pub started_at: DateTime<Utc>,
    /// When the session last connected
    pub connected_at: Option<DateTime<Utc>>,
    /// Number of reconnection attempts
    pub reconnect_count: u32,
    /// Last error message (if any)
    pub last_error: Option<String>,
}

impl Session {
    pub fn new(profile: &Profile) -> Self {
        Self {
            id: Uuid::new_v4(),
            profile_id: profile.id,
            profile_name: profile.name.clone(),
            status: SessionStatus::Starting,
            pid: None,
            started_at: Utc::now(),
            connected_at: None,
            reconnect_count: 0,
            last_error: None,
        }
    }

    /// Check if the session is in a running state
    pub fn is_running(&self) -> bool {
        matches!(
            self.status,
            SessionStatus::Starting | SessionStatus::Connected | SessionStatus::Reconnecting
        )
    }

    /// Check if the session is connected
    pub fn is_connected(&self) -> bool {
        self.status == SessionStatus::Connected
    }

    /// Get the uptime duration if connected
    pub fn uptime(&self) -> Option<chrono::Duration> {
        self.connected_at.map(|t| Utc::now() - t)
    }

    /// Format uptime as human-readable string
    pub fn uptime_string(&self) -> String {
        match self.uptime() {
            Some(duration) => {
                let secs = duration.num_seconds();
                if secs < 60 {
                    format!("{}s", secs)
                } else if secs < 3600 {
                    format!("{}m {}s", secs / 60, secs % 60)
                } else {
                    format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
                }
            }
            None => "-".to_string(),
        }
    }
}

/// Thread-safe session handle for the supervisor
pub type SessionHandle = Arc<RwLock<Session>>;

/// Create a new session handle
pub fn new_session_handle(profile: &Profile) -> SessionHandle {
    Arc::new(RwLock::new(Session::new(profile)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::profile::Profile;

    #[test]
    fn test_session_creation() {
        let profile = Profile::new("test", "example.com", "user");
        let session = Session::new(&profile);
        
        assert_eq!(session.profile_id, profile.id);
        assert_eq!(session.status, SessionStatus::Starting);
        assert!(session.is_running());
        assert!(!session.is_connected());
    }
}
