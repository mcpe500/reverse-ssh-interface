//! Reverse SSH Interface Core Library
//!
//! This crate provides the shared logic for managing reverse SSH tunnels,
//! including SSH process spawning, session supervision, configuration management,
//! and event handling.
//!
//! # Architecture
//!
//! The core is organized into several modules:
//!
//! - [`config`]: Configuration loading, saving, and management
//! - [`ssh`]: SSH binary detection, argument building, and process spawning
//! - [`supervisor`]: Session management with reconnection logic
//! - [`storage`]: State persistence and optional keyring integration
//! - [`types`]: Core data types (profiles, sessions, events)
//! - [`error`]: Error types and result aliases
//! - [`util`]: Utility functions (redaction, etc.)
//!
//! # Example
//!
//! ```rust,no_run
//! use reverse_ssh_core::prelude::*;
//! use reverse_ssh_core::config::{init_config, load_profiles};
//! use reverse_ssh_core::supervisor::SessionManager;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Initialize configuration
//!     let config = init_config()?;
//!     
//!     // Load profiles
//!     let profiles = load_profiles()?;
//!     
//!     // Create session manager
//!     let (mut manager, handle) = SessionManager::new(config);
//!     manager.init().await?;
//!     
//!     // Start manager in background
//!     tokio::spawn(async move {
//!         manager.run().await.ok();
//!     });
//!     
//!     // Start a session
//!     if let Some(profile) = profiles.first() {
//!         handle.start(profile.clone()).await?;
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod prelude;
pub mod ssh;
pub mod storage;
pub mod supervisor;
pub mod types;
pub mod util;

// Re-export commonly used items at the crate root
pub use error::{CoreError, Result};
pub use types::{Profile, Session, SessionStatus, TunnelSpec};
