use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(tag = "state", content = "details")]
pub enum SessionStatus {
    Stopped,
    Starting,
    Running,
    Failed(String), // Reason
    Retrying { attempt: u8, max: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Session {
    pub profile_id: String,
    pub status: SessionStatus,
    
    /// System Process ID (if running)
    pub pid: Option<u32>,
    
    /// When the current process started
    pub start_time: Option<DateTime<Utc>>,
    
    /// Accumulated restart count since last manual start
    pub restart_count: u64,
}

impl Session {
    pub fn new(profile_id: String) -> Self {
        Self {
            profile_id,
            status: SessionStatus::Stopped,
            pid: None,
            start_time: None,
            restart_count: 0,
        }
    }
}
