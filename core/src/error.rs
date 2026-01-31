use thiserror::Error;
use std::path::PathBuf;

/// Core error types for the reverse SSH interface
#[derive(Error, Debug)]
pub enum CoreError {
    // SSH-related errors
    #[error("SSH binary not found. Please ensure OpenSSH is installed.")]
    SshNotFound,

    #[error("SSH binary at '{0}' is not executable")]
    SshNotExecutable(PathBuf),

    #[error("Failed to detect SSH version: {0}")]
    SshVersionDetection(String),

    #[error("SSH process failed to start: {0}")]
    SshSpawnFailed(String),

    #[error("SSH process exited with code {code}: {message}")]
    SshExitError { code: i32, message: String },

    #[error("SSH process terminated by signal")]
    SshSignalTerminated,

    // Config-related errors
    #[error("Configuration file not found: {0}")]
    ConfigNotFound(PathBuf),

    #[error("Failed to parse configuration: {0}")]
    ConfigParse(String),

    #[error("Invalid configuration: {0}")]
    ConfigInvalid(String),

    #[error("Failed to write configuration: {0}")]
    ConfigWrite(String),

    // Profile-related errors
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("Profile already exists: {0}")]
    ProfileAlreadyExists(String),

    #[error("Invalid profile: {0}")]
    ProfileInvalid(String),

    // Session-related errors
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Session already running for profile: {0}")]
    SessionAlreadyRunning(String),

    #[error("Session not running: {0}")]
    SessionNotRunning(String),

    #[error("Maximum reconnection attempts reached")]
    MaxReconnectAttemptsReached,

    // Storage errors
    #[error("Failed to access storage: {0}")]
    StorageAccess(String),

    #[error("Failed to serialize data: {0}")]
    Serialization(String),

    #[error("Failed to deserialize data: {0}")]
    Deserialization(String),

    // IO errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // Generic errors
    #[error("Operation cancelled")]
    Cancelled,

    #[error("{0}")]
    Other(String),
}

impl CoreError {
    /// Create an "other" error from any error type
    pub fn other<E: std::error::Error>(err: E) -> Self {
        Self::Other(err.to_string())
    }
}

/// Result type alias using CoreError
pub type Result<T> = std::result::Result<T, CoreError>;
