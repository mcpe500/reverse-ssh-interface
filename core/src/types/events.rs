use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::session::SessionStatus;

/// Event types for UI/CLI notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    /// Session status changed
    SessionStatusChanged {
        session_id: Uuid,
        profile_name: String,
        old_status: SessionStatus,
        new_status: SessionStatus,
        timestamp: DateTime<Utc>,
    },
    /// Session connected successfully
    SessionConnected {
        session_id: Uuid,
        profile_name: String,
        timestamp: DateTime<Utc>,
    },
    /// Session disconnected
    SessionDisconnected {
        session_id: Uuid,
        profile_name: String,
        reason: Option<String>,
        timestamp: DateTime<Utc>,
    },
    /// Session reconnecting
    SessionReconnecting {
        session_id: Uuid,
        profile_name: String,
        attempt: u32,
        max_attempts: u32,
        timestamp: DateTime<Utc>,
    },
    /// Session failed permanently
    SessionFailed {
        session_id: Uuid,
        profile_name: String,
        error: String,
        timestamp: DateTime<Utc>,
    },
    /// SSH process output (stdout/stderr)
    SessionOutput {
        session_id: Uuid,
        profile_name: String,
        output: String,
        is_stderr: bool,
        timestamp: DateTime<Utc>,
    },
    /// Profile created
    ProfileCreated {
        profile_id: Uuid,
        profile_name: String,
        timestamp: DateTime<Utc>,
    },
    /// Profile updated
    ProfileUpdated {
        profile_id: Uuid,
        profile_name: String,
        timestamp: DateTime<Utc>,
    },
    /// Profile deleted
    ProfileDeleted {
        profile_id: Uuid,
        profile_name: String,
        timestamp: DateTime<Utc>,
    },
    /// SSH binary detected/changed
    SshBinaryChanged {
        path: String,
        version: Option<String>,
        timestamp: DateTime<Utc>,
    },
    /// Error event
    Error {
        message: String,
        context: Option<String>,
        timestamp: DateTime<Utc>,
    },
}

impl Event {
    pub fn session_status_changed(
        session_id: Uuid,
        profile_name: impl Into<String>,
        old_status: SessionStatus,
        new_status: SessionStatus,
    ) -> Self {
        Self::SessionStatusChanged {
            session_id,
            profile_name: profile_name.into(),
            old_status,
            new_status,
            timestamp: Utc::now(),
        }
    }

    pub fn session_connected(session_id: Uuid, profile_name: impl Into<String>) -> Self {
        Self::SessionConnected {
            session_id,
            profile_name: profile_name.into(),
            timestamp: Utc::now(),
        }
    }

    pub fn session_disconnected(
        session_id: Uuid,
        profile_name: impl Into<String>,
        reason: Option<String>,
    ) -> Self {
        Self::SessionDisconnected {
            session_id,
            profile_name: profile_name.into(),
            reason,
            timestamp: Utc::now(),
        }
    }

    pub fn session_reconnecting(
        session_id: Uuid,
        profile_name: impl Into<String>,
        attempt: u32,
        max_attempts: u32,
    ) -> Self {
        Self::SessionReconnecting {
            session_id,
            profile_name: profile_name.into(),
            attempt,
            max_attempts,
            timestamp: Utc::now(),
        }
    }

    pub fn session_failed(
        session_id: Uuid,
        profile_name: impl Into<String>,
        error: impl Into<String>,
    ) -> Self {
        Self::SessionFailed {
            session_id,
            profile_name: profile_name.into(),
            error: error.into(),
            timestamp: Utc::now(),
        }
    }

    pub fn session_output(
        session_id: Uuid,
        profile_name: impl Into<String>,
        output: impl Into<String>,
        is_stderr: bool,
    ) -> Self {
        Self::SessionOutput {
            session_id,
            profile_name: profile_name.into(),
            output: output.into(),
            is_stderr,
            timestamp: Utc::now(),
        }
    }

    pub fn error(message: impl Into<String>, context: Option<String>) -> Self {
        Self::Error {
            message: message.into(),
            context,
            timestamp: Utc::now(),
        }
    }

    /// Get the timestamp of this event
    pub fn timestamp(&self) -> DateTime<Utc> {
        match self {
            Event::SessionStatusChanged { timestamp, .. } => *timestamp,
            Event::SessionConnected { timestamp, .. } => *timestamp,
            Event::SessionDisconnected { timestamp, .. } => *timestamp,
            Event::SessionReconnecting { timestamp, .. } => *timestamp,
            Event::SessionFailed { timestamp, .. } => *timestamp,
            Event::SessionOutput { timestamp, .. } => *timestamp,
            Event::ProfileCreated { timestamp, .. } => *timestamp,
            Event::ProfileUpdated { timestamp, .. } => *timestamp,
            Event::ProfileDeleted { timestamp, .. } => *timestamp,
            Event::SshBinaryChanged { timestamp, .. } => *timestamp,
            Event::Error { timestamp, .. } => *timestamp,
        }
    }
}

/// Event channel sender type
pub type EventSender = tokio::sync::broadcast::Sender<Event>;
/// Event channel receiver type
pub type EventReceiver = tokio::sync::broadcast::Receiver<Event>;

/// Create a new event channel with the specified capacity
pub fn event_channel(capacity: usize) -> (EventSender, EventReceiver) {
    tokio::sync::broadcast::channel(capacity)
}
