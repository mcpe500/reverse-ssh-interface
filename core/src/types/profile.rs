use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(tag = "type", content = "value")]
pub enum AuthMethod {
    /// Use SSH Agent (default)
    Agent,
    /// Use a specific private key file
    IdentityFile(String),
    /// Use a plain password (handled via SSH_ASKPASS)
    Password(String),
}

impl std::fmt::Debug for AuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Agent => write!(f, "Agent"),
            Self::IdentityFile(path) => write!(f, "IdentityFile({:?})", path),
            Self::Password(_) => write!(f, "Password([REDACTED])"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct Profile {
    /// Unique identifier for the profile (e.g., "prod-db", "my-vps")
    pub id: String,

    /// SSH Host
    pub host: String,

    /// SSH Port (default 22)
    #[serde(default = "default_port")]
    pub port: u16,

    /// SSH User
    pub user: String,

    /// Authentication Method
    #[serde(default = "default_auth")]
    pub auth: AuthMethod,

    /// List of reverse forwards
    #[serde(default)]
    pub forwards: Vec<ForwardRule>,

    /// Advanced options
    #[serde(default)]
    pub advanced: AdvancedOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct ForwardRule {
    /// The port on the REMOTE server to open
    pub remote_port: u16,

    /// The interface on the remote server to bind to (default "127.0.0.1")
    #[serde(default = "default_bind")]
    pub remote_bind: String,

    /// The LOCAL destination to forward to (default "localhost")
    #[serde(default = "default_host")]
    pub local_host: String,

    /// The LOCAL port to forward to
    pub local_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct AdvancedOptions {
    #[serde(default = "default_alive_interval")]
    pub server_alive_interval: u64,

    #[serde(default = "default_alive_count")]
    pub server_alive_count_max: u64,

    /// Additional raw arguments to pass to SSH (use with caution)
    pub custom_args: Option<Vec<String>>,
}

impl Default for AdvancedOptions {
    fn default() -> Self {
        Self {
            server_alive_interval: default_alive_interval(),
            server_alive_count_max: default_alive_count(),
            custom_args: None,
        }
    }
}

// -- Defaults --

fn default_port() -> u16 {
    22
}

fn default_bind() -> String {
    "127.0.0.1".to_string()
}

fn default_host() -> String {
    "localhost".to_string()
}

fn default_alive_interval() -> u64 {
    20
}

fn default_alive_count() -> u64 {
    3
}

fn default_auth() -> AuthMethod {
    AuthMethod::Agent
}

impl Profile {
    pub fn new(id: impl Into<String>, host: impl Into<String>, user: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            host: host.into(),
            port: default_port(),
            user: user.into(),
            auth: AuthMethod::Agent,
            forwards: Vec::new(),
            advanced: AdvancedOptions::default(),
        }
    }
}

impl ForwardRule {
    /// Returns the string representation for -R flag: [bind_address:]port:host:hostport
    pub fn to_arg_string(&self) -> String {
        // format: [bind_address:]remote_port:local_host:local_port
        format!("{}:{}:{}:{}", self.remote_bind, self.remote_port, self.local_host, self.local_port)
    }
}
