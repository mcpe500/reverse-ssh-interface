use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Global application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// General settings
    #[serde(default)]
    pub general: GeneralConfig,
    /// SSH-related settings
    #[serde(default)]
    pub ssh: SshConfig,
    /// Logging settings
    #[serde(default)]
    pub logging: LoggingConfig,
    /// Web server settings (if running web interface)
    #[serde(default)]
    pub web: WebConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            ssh: SshConfig::default(),
            logging: LoggingConfig::default(),
            web: WebConfig::default(),
        }
    }
}

/// General application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Whether to start minimized (GUI)
    #[serde(default)]
    pub start_minimized: bool,
    /// Auto-start sessions on app launch
    #[serde(default)]
    pub auto_start_sessions: bool,
    /// Default profile to start (by name or ID)
    pub default_profile: Option<String>,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            start_minimized: false,
            auto_start_sessions: false,
            default_profile: None,
        }
    }
}

/// SSH-related configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfig {
    /// Custom SSH binary path (auto-detect if not set)
    pub binary_path: Option<PathBuf>,
    /// Default keepalive interval in seconds
    #[serde(default = "default_keepalive_interval")]
    pub default_keepalive_interval: u32,
    /// Default keepalive count
    #[serde(default = "default_keepalive_count")]
    pub default_keepalive_count: u32,
    /// Default SSH options applied to all connections
    #[serde(default)]
    pub default_options: HashMap<String, String>,
    /// Strict host key checking mode
    #[serde(default)]
    pub strict_host_key_checking: StrictHostKeyChecking,
    /// Use app-managed known_hosts file
    #[serde(default = "default_true")]
    pub use_app_known_hosts: bool,
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

impl Default for SshConfig {
    fn default() -> Self {
        Self {
            binary_path: None,
            default_keepalive_interval: default_keepalive_interval(),
            default_keepalive_count: default_keepalive_count(),
            default_options: HashMap::new(),
            strict_host_key_checking: StrictHostKeyChecking::default(),
            use_app_known_hosts: true,
        }
    }
}

/// Strict host key checking modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum StrictHostKeyChecking {
    /// Always verify host keys (most secure)
    Yes,
    /// Accept new keys, reject changed keys
    #[default]
    AcceptNew,
    /// Never verify host keys (insecure, not recommended)
    No,
}

impl StrictHostKeyChecking {
    pub fn to_ssh_option(&self) -> &'static str {
        match self {
            StrictHostKeyChecking::Yes => "yes",
            StrictHostKeyChecking::AcceptNew => "accept-new",
            StrictHostKeyChecking::No => "no",
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,
    /// Log to file
    #[serde(default = "default_true")]
    pub file_logging: bool,
    /// Maximum log file size in MB before rotation
    #[serde(default = "default_max_log_size")]
    pub max_file_size_mb: u32,
    /// Number of rotated log files to keep
    #[serde(default = "default_max_log_files")]
    pub max_files: u32,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_max_log_size() -> u32 {
    10
}

fn default_max_log_files() -> u32 {
    5
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            file_logging: true,
            max_file_size_mb: default_max_log_size(),
            max_files: default_max_log_files(),
        }
    }
}

/// Web server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebConfig {
    /// Enable web server
    #[serde(default)]
    pub enabled: bool,
    /// Bind address
    #[serde(default = "default_bind_address")]
    pub bind_address: String,
    /// Port number
    #[serde(default = "default_web_port")]
    pub port: u16,
    /// Enable CORS for development
    #[serde(default)]
    pub cors_enabled: bool,
}

fn default_bind_address() -> String {
    "127.0.0.1".to_string()
}

fn default_web_port() -> u16 {
    3847
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            bind_address: default_bind_address(),
            port: default_web_port(),
            cors_enabled: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert!(!config.general.start_minimized);
        assert_eq!(config.ssh.default_keepalive_interval, 20);
        assert_eq!(config.logging.level, "info");
        assert_eq!(config.web.port, 3847);
    }

    #[test]
    fn test_strict_host_key_checking_to_option() {
        assert_eq!(StrictHostKeyChecking::Yes.to_ssh_option(), "yes");
        assert_eq!(StrictHostKeyChecking::AcceptNew.to_ssh_option(), "accept-new");
        assert_eq!(StrictHostKeyChecking::No.to_ssh_option(), "no");
    }
}
