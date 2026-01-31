use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A tunnel specification for reverse port forwarding (-R)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TunnelSpec {
    /// Remote bind address (default: localhost)
    #[serde(default = "default_bind_address")]
    pub remote_bind: String,
    /// Remote port on the SSH server
    pub remote_port: u16,
    /// Local host to forward to (default: localhost)
    #[serde(default = "default_bind_address")]
    pub local_host: String,
    /// Local port to forward to
    pub local_port: u16,
}

fn default_bind_address() -> String {
    "localhost".to_string()
}

impl TunnelSpec {
    pub fn new(remote_port: u16, local_port: u16) -> Self {
        Self {
            remote_bind: default_bind_address(),
            remote_port,
            local_host: default_bind_address(),
            local_port,
        }
    }

    /// Format as SSH -R argument: [bind_address:]port:host:hostport
    pub fn to_ssh_arg(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.remote_bind, self.remote_port, self.local_host, self.local_port
        )
    }
}

/// SSH authentication method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum AuthMethod {
    /// Use SSH agent (recommended)
    #[default]
    Agent,
    /// Use a specific key file
    KeyFile {
        path: String,
    },
    /// Use password (not recommended, requires sshpass or similar)
    Password,
}

/// Connection profile for a reverse SSH tunnel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Unique identifier
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,
    /// Human-readable name
    pub name: String,
    /// SSH server hostname or IP
    pub host: String,
    /// SSH server port (default: 22)
    #[serde(default = "default_ssh_port")]
    pub port: u16,
    /// SSH username
    pub user: String,
    /// Authentication method
    #[serde(default)]
    pub auth: AuthMethod,
    /// Tunnels to establish
    pub tunnels: Vec<TunnelSpec>,
    /// Keep-alive interval in seconds (default: 20)
    #[serde(default = "default_keepalive_interval")]
    pub keepalive_interval: u32,
    /// Keep-alive max count before disconnect (default: 3)
    #[serde(default = "default_keepalive_count")]
    pub keepalive_count: u32,
    /// Auto-reconnect on failure
    #[serde(default = "default_true")]
    pub auto_reconnect: bool,
    /// Maximum reconnection attempts (0 = unlimited)
    #[serde(default)]
    pub max_reconnect_attempts: u32,
    /// Additional SSH options
    #[serde(default)]
    pub extra_options: HashMap<String, String>,
    /// Custom SSH binary path (uses detected if not set)
    pub ssh_path: Option<String>,
    /// Custom known_hosts file path
    pub known_hosts_file: Option<String>,
    /// Custom identity file path
    pub identity_file: Option<String>,
}

fn default_ssh_port() -> u16 {
    22
}

fn default_keepalive_interval() -> u32 {
    20
}

fn default_keepalive_count() -> u32 {
    3
}

fn default_true() -> bool {
    true
}

impl Profile {
    pub fn new(name: impl Into<String>, host: impl Into<String>, user: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            host: host.into(),
            port: default_ssh_port(),
            user: user.into(),
            auth: AuthMethod::default(),
            tunnels: Vec::new(),
            keepalive_interval: default_keepalive_interval(),
            keepalive_count: default_keepalive_count(),
            auto_reconnect: true,
            max_reconnect_attempts: 0,
            extra_options: HashMap::new(),
            ssh_path: None,
            known_hosts_file: None,
            identity_file: None,
        }
    }

    /// Add a tunnel to this profile
    pub fn with_tunnel(mut self, tunnel: TunnelSpec) -> Self {
        self.tunnels.push(tunnel);
        self
    }

    /// Get the SSH destination string (user@host)
    pub fn destination(&self) -> String {
        format!("{}@{}", self.user, self.host)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tunnel_spec_to_ssh_arg() {
        let tunnel = TunnelSpec::new(8080, 3000);
        assert_eq!(tunnel.to_ssh_arg(), "localhost:8080:localhost:3000");
    }

    #[test]
    fn test_profile_destination() {
        let profile = Profile::new("test", "example.com", "user");
        assert_eq!(profile.destination(), "user@example.com");
    }
}
