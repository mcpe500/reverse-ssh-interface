//! API types for the web server.
//! These are separate from core types to allow utoipa schema derivation.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// API representation of a tunnel specification
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiTunnelSpec {
    /// Remote bind address
    #[schema(example = "localhost")]
    pub remote_bind: String,
    /// Remote port
    #[schema(example = 8080)]
    pub remote_port: u16,
    /// Local host
    #[schema(example = "localhost")]
    pub local_host: String,
    /// Local port
    #[schema(example = 3000)]
    pub local_port: u16,
}

/// API representation of authentication method
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ApiAuthMethod {
    /// Use SSH agent
    Agent,
    /// Use key file
    KeyFile { path: String },
    /// Use password (requires sshpass)
    Password,
}

/// API representation of a profile
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiProfile {
    /// Unique identifier
    pub id: Uuid,
    /// Profile name
    #[schema(example = "my-server")]
    pub name: String,
    /// SSH host
    #[schema(example = "example.com")]
    pub host: String,
    /// SSH port
    #[schema(example = 22)]
    pub port: u16,
    /// SSH user
    #[schema(example = "admin")]
    pub user: String,
    /// Authentication method
    pub auth: ApiAuthMethod,
    /// Tunnel specifications
    pub tunnels: Vec<ApiTunnelSpec>,
}

/// Request to create a new profile
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateProfileRequest {
    /// Profile name (must be unique)
    #[schema(example = "my-server")]
    pub name: String,
    /// SSH host
    #[schema(example = "example.com")]
    pub host: String,
    /// SSH port (default: 22)
    #[schema(example = 22)]
    pub port: Option<u16>,
    /// SSH user
    #[schema(example = "admin")]
    pub user: String,
    /// Authentication method (default: agent)
    pub auth: Option<ApiAuthMethod>,
    /// Tunnel specifications
    pub tunnels: Vec<ApiTunnelSpec>,
}

/// Request to update an existing profile
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateProfileRequest {
    /// New profile name (optional). If set, renames the profile.
    pub name: Option<String>,
    /// SSH host
    pub host: Option<String>,
    /// SSH port
    pub port: Option<u16>,
    /// SSH user
    pub user: Option<String>,
    /// Authentication method
    pub auth: Option<ApiAuthMethod>,
    /// Tunnel specifications (replaces existing when provided)
    pub tunnels: Option<Vec<ApiTunnelSpec>>,
}

/// Request to start a session.
///
/// If `password` is provided, it will be used for `AuthMethod::Password` without
/// requiring `SSHPASS` to be set on the web server process.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct StartSessionRequest {
    /// Password for password-based auth.
    ///
    /// This is not stored in profile configuration.
    pub password: Option<String>,
}

/// API representation of session status
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ApiSessionStatus {
    Starting,
    Connected,
    Reconnecting,
    Stopped,
    Failed,
}

/// API representation of a session
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiSession {
    /// Session ID
    pub id: Uuid,
    /// Profile name
    pub profile_name: String,
    /// Current status
    pub status: ApiSessionStatus,
    /// When the session started
    pub started_at: DateTime<Utc>,
    /// Process ID (if running)
    pub pid: Option<u32>,
    /// Last error message
    pub last_error: Option<String>,
}

// Conversion functions
impl From<reverse_ssh_core::types::TunnelSpec> for ApiTunnelSpec {
    fn from(t: reverse_ssh_core::types::TunnelSpec) -> Self {
        Self {
            remote_bind: t.remote_bind,
            remote_port: t.remote_port,
            local_host: t.local_host,
            local_port: t.local_port,
        }
    }
}

impl From<ApiTunnelSpec> for reverse_ssh_core::types::TunnelSpec {
    fn from(t: ApiTunnelSpec) -> Self {
        Self {
            remote_bind: t.remote_bind,
            remote_port: t.remote_port,
            local_host: t.local_host,
            local_port: t.local_port,
        }
    }
}

impl From<reverse_ssh_core::types::AuthMethod> for ApiAuthMethod {
    fn from(a: reverse_ssh_core::types::AuthMethod) -> Self {
        match a {
            reverse_ssh_core::types::AuthMethod::Agent => Self::Agent,
            reverse_ssh_core::types::AuthMethod::KeyFile { path } => Self::KeyFile { path },
            reverse_ssh_core::types::AuthMethod::Password => Self::Password,
        }
    }
}

impl From<ApiAuthMethod> for reverse_ssh_core::types::AuthMethod {
    fn from(a: ApiAuthMethod) -> Self {
        match a {
            ApiAuthMethod::Agent => Self::Agent,
            ApiAuthMethod::KeyFile { path } => Self::KeyFile { path },
            ApiAuthMethod::Password => Self::Password,
        }
    }
}

impl From<reverse_ssh_core::types::Profile> for ApiProfile {
    fn from(p: reverse_ssh_core::types::Profile) -> Self {
        Self {
            id: p.id,
            name: p.name,
            host: p.host,
            port: p.port,
            user: p.user,
            auth: p.auth.into(),
            tunnels: p.tunnels.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<reverse_ssh_core::types::SessionStatus> for ApiSessionStatus {
    fn from(s: reverse_ssh_core::types::SessionStatus) -> Self {
        match s {
            reverse_ssh_core::types::SessionStatus::Starting => Self::Starting,
            reverse_ssh_core::types::SessionStatus::Connected => Self::Connected,
            reverse_ssh_core::types::SessionStatus::Reconnecting => Self::Reconnecting,
            reverse_ssh_core::types::SessionStatus::Stopped => Self::Stopped,
            reverse_ssh_core::types::SessionStatus::Failed => Self::Failed,
        }
    }
}

impl From<reverse_ssh_core::types::Session> for ApiSession {
    fn from(s: reverse_ssh_core::types::Session) -> Self {
        Self {
            id: s.id,
            profile_name: s.profile_name,
            status: s.status.into(),
            started_at: s.started_at,
            pid: s.pid,
            last_error: s.last_error,
        }
    }
}
