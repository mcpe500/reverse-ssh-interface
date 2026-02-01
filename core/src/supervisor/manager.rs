use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{mpsc, RwLock};
use tokio::time::sleep;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::error::{CoreError, Result};
use crate::ssh::{SshInfo, detect_ssh, spawn_ssh};
use crate::types::{
    Event, EventReceiver, EventSender, Profile, Session, SessionHandle, SessionStatus,
    event_channel, new_session_handle,
};

use super::backoff::Backoff;
use super::monitor::{MonitorResult, SessionMonitor};

/// Command sent to the session manager
#[derive(Debug)]
pub enum ManagerCommand {
    /// Start a session for a profile
    Start(Profile, StartSessionOptions),
    /// Stop a session by ID
    Stop(Uuid),
    /// Stop all sessions
    StopAll,
    /// Get status of all sessions
    GetStatus,
    /// Shutdown the manager
    Shutdown,
}

/// Options that apply to a started session but are not persisted in the profile.
#[derive(Debug, Clone, Default)]
pub struct StartSessionOptions {
    /// Password for `AuthMethod::Password`.
    ///
    /// This value is kept in memory for the lifetime of the session and is never
    /// written into the profile configuration.
    pub password: Option<String>,
}

/// Response from the session manager
#[derive(Debug)]
pub enum ManagerResponse {
    /// Session started successfully
    Started(Uuid),
    /// Session stopped
    Stopped(Uuid),
    /// All sessions stopped
    AllStopped,
    /// Status of all sessions
    Status(Vec<Session>),
    /// Error occurred
    Error(String),
    /// Manager shutting down
    ShuttingDown,
}

/// Active session info for the manager
struct ActiveSession {
    handle: SessionHandle,
    profile: Profile,
    stop_tx: mpsc::Sender<()>,
}

/// The session manager - central controller for all SSH sessions
pub struct SessionManager {
    /// Application configuration
    config: AppConfig,
    /// Detected SSH binary info
    ssh_info: Option<SshInfo>,
    /// Active sessions
    sessions: Arc<RwLock<HashMap<Uuid, ActiveSession>>>,
    /// Event broadcaster
    event_tx: EventSender,
    /// Command receiver
    cmd_rx: mpsc::Receiver<(ManagerCommand, mpsc::Sender<ManagerResponse>)>,
    /// Command sender (kept for potential future use)
    #[allow(dead_code)]
    cmd_tx: mpsc::Sender<(ManagerCommand, mpsc::Sender<ManagerResponse>)>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(config: AppConfig) -> (Self, SessionManagerHandle) {
        let (event_tx, _) = event_channel(100);
        let (cmd_tx, cmd_rx) = mpsc::channel(32);

        let manager = Self {
            config,
            ssh_info: None,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            event_tx: event_tx.clone(),
            cmd_rx,
            cmd_tx: cmd_tx.clone(),
        };

        let handle = SessionManagerHandle {
            cmd_tx,
            event_tx,
        };

        (manager, handle)
    }

    /// Initialize the manager (detect SSH, etc.)
    pub async fn init(&mut self) -> Result<()> {
        // Detect SSH binary
        let ssh_path = self.config.ssh.binary_path.as_ref();
        self.ssh_info = Some(detect_ssh(ssh_path).await?);
        
        if let Some(ref info) = self.ssh_info {
            tracing::info!("Detected SSH: {:?} ({})", info.path, info.version.as_deref().unwrap_or("unknown version"));
            let _ = self.event_tx.send(Event::SshBinaryChanged {
                path: info.path.display().to_string(),
                version: info.version.clone(),
                timestamp: chrono::Utc::now(),
            });
        }

        Ok(())
    }

    /// Run the manager event loop
    pub async fn run(mut self) -> Result<()> {
        tracing::info!("Session manager started");

        while let Some((cmd, response_tx)) = self.cmd_rx.recv().await {
            let response = match cmd {
                ManagerCommand::Start(profile, options) => self.handle_start(profile, options).await,
                ManagerCommand::Stop(id) => self.handle_stop(id).await,
                ManagerCommand::StopAll => self.handle_stop_all().await,
                ManagerCommand::GetStatus => self.handle_get_status().await,
                ManagerCommand::Shutdown => {
                    let _ = self.handle_stop_all().await;
                    let _ = response_tx.send(ManagerResponse::ShuttingDown).await;
                    break;
                }
            };

            let _ = response_tx.send(response).await;
        }

        tracing::info!("Session manager stopped");
        Ok(())
    }

    async fn handle_start(&self, profile: Profile, options: StartSessionOptions) -> ManagerResponse {
        let ssh_info = match &self.ssh_info {
            Some(info) => info,
            None => return ManagerResponse::Error("SSH not detected".to_string()),
        };

        // Check if session already exists for this profile
        {
            let sessions = self.sessions.read().await;
            for (_, active) in sessions.iter() {
                if active.profile.id == profile.id {
                    let session = active.handle.read().await;
                    if session.is_running() {
                        return ManagerResponse::Error(format!(
                            "Session already running for profile '{}'",
                            profile.name
                        ));
                    }
                }
            }
        }

        // Create session handle
        let session_handle = new_session_handle(&profile);
        let session_id = {
            let session = session_handle.read().await;
            session.id
        };

        // Create stop channel
        let (stop_tx, stop_rx) = mpsc::channel::<()>(1);

        // Spawn the session task
        let task_handle = session_handle.clone();
        let task_profile = profile.clone();
        let task_options = options.clone();
        let task_ssh_info = ssh_info.clone();
        let task_event_tx = self.event_tx.clone();
        let task_sessions = self.sessions.clone();
        let task_config = self.config.clone();

        tokio::spawn(async move {
            run_session_task(
                task_handle,
                task_profile,
                task_options,
                task_ssh_info,
                task_event_tx,
                task_sessions,
                task_config,
                stop_rx,
            )
            .await;
        });

        // Store active session
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id, ActiveSession {
                handle: session_handle,
                profile,
                stop_tx,
            });
        }

        ManagerResponse::Started(session_id)
    }

    async fn handle_stop(&self, session_id: Uuid) -> ManagerResponse {
        let mut sessions = self.sessions.write().await;
        
        if let Some(active) = sessions.remove(&session_id) {
            // Send stop signal
            let _ = active.stop_tx.send(()).await;
            
            // Update session status
            {
                let mut session = active.handle.write().await;
                let old_status = session.status;
                session.status = SessionStatus::Stopped;
                
                let _ = self.event_tx.send(Event::session_status_changed(
                    session.id,
                    &session.profile_name,
                    old_status,
                    SessionStatus::Stopped,
                ));
            }
            
            ManagerResponse::Stopped(session_id)
        } else {
            ManagerResponse::Error(format!("Session {} not found", session_id))
        }
    }

    async fn handle_stop_all(&self) -> ManagerResponse {
        let mut sessions = self.sessions.write().await;
        
        for (_, active) in sessions.drain() {
            let _ = active.stop_tx.send(()).await;
            
            let mut session = active.handle.write().await;
            session.status = SessionStatus::Stopped;
        }

        ManagerResponse::AllStopped
    }

    async fn handle_get_status(&self) -> ManagerResponse {
        let sessions = self.sessions.read().await;
        let mut status = Vec::new();

        for (_, active) in sessions.iter() {
            let session = active.handle.read().await;
            status.push(session.clone());
        }

        ManagerResponse::Status(status)
    }
}

/// Handle to interact with the session manager
#[derive(Clone)]
pub struct SessionManagerHandle {
    cmd_tx: mpsc::Sender<(ManagerCommand, mpsc::Sender<ManagerResponse>)>,
    event_tx: EventSender,
}

impl SessionManagerHandle {
    /// Send a command to the manager and wait for response
    async fn send_command(&self, cmd: ManagerCommand) -> Result<ManagerResponse> {
        let (response_tx, mut response_rx) = mpsc::channel(1);
        
        self.cmd_tx
            .send((cmd, response_tx))
            .await
            .map_err(|_| CoreError::Other("Manager channel closed".to_string()))?;

        response_rx
            .recv()
            .await
            .ok_or_else(|| CoreError::Other("No response from manager".to_string()))
    }

    /// Start a session for a profile
    pub async fn start(&self, profile: Profile) -> Result<Uuid> {
        self.start_with_options(profile, StartSessionOptions::default()).await
    }

    /// Start a session with non-persisted options (e.g. password auth).
    pub async fn start_with_options(&self, profile: Profile, options: StartSessionOptions) -> Result<Uuid> {
        match self.send_command(ManagerCommand::Start(profile, options)).await? {
            ManagerResponse::Started(id) => Ok(id),
            ManagerResponse::Error(e) => Err(CoreError::Other(e)),
            _ => Err(CoreError::Other("Unexpected response".to_string())),
        }
    }

    /// Stop a session
    pub async fn stop(&self, session_id: Uuid) -> Result<()> {
        match self.send_command(ManagerCommand::Stop(session_id)).await? {
            ManagerResponse::Stopped(_) => Ok(()),
            ManagerResponse::Error(e) => Err(CoreError::Other(e)),
            _ => Err(CoreError::Other("Unexpected response".to_string())),
        }
    }

    /// Stop all sessions
    pub async fn stop_all(&self) -> Result<()> {
        match self.send_command(ManagerCommand::StopAll).await? {
            ManagerResponse::AllStopped => Ok(()),
            ManagerResponse::Error(e) => Err(CoreError::Other(e)),
            _ => Err(CoreError::Other("Unexpected response".to_string())),
        }
    }

    /// Get status of all sessions
    pub async fn status(&self) -> Result<Vec<Session>> {
        match self.send_command(ManagerCommand::GetStatus).await? {
            ManagerResponse::Status(sessions) => Ok(sessions),
            ManagerResponse::Error(e) => Err(CoreError::Other(e)),
            _ => Err(CoreError::Other("Unexpected response".to_string())),
        }
    }

    /// Shutdown the manager
    pub async fn shutdown(&self) -> Result<()> {
        let _ = self.send_command(ManagerCommand::Shutdown).await;
        Ok(())
    }

    /// Subscribe to events
    pub fn subscribe(&self) -> EventReceiver {
        self.event_tx.subscribe()
    }
}

/// Background task that manages a single session with reconnection logic
async fn run_session_task(
    session_handle: SessionHandle,
    profile: Profile,
    options: StartSessionOptions,
    ssh_info: SshInfo,
    event_tx: EventSender,
    sessions: Arc<RwLock<HashMap<Uuid, ActiveSession>>>,
    _config: AppConfig,
    mut stop_rx: mpsc::Receiver<()>,
) {
    let session_id = {
        let session = session_handle.read().await;
        session.id
    };

    let mut backoff = Backoff::new()
        .with_initial_delay(Duration::from_secs(1))
        .with_max_delay(Duration::from_secs(300))
        .with_max_attempts(profile.max_reconnect_attempts);

    loop {
        // Spawn SSH process
        let process = match spawn_ssh(&ssh_info, &profile, options.password.as_deref()).await {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("Failed to spawn SSH for '{}': {}", profile.name, e);
                
                let mut session = session_handle.write().await;
                session.last_error = Some(e.to_string());
                
                // Check if we should retry
                if !profile.auto_reconnect || backoff.is_exhausted() {
                    session.status = SessionStatus::Failed;
                    let _ = event_tx.send(Event::session_failed(
                        session.id,
                        &session.profile_name,
                        e.to_string(),
                    ));
                    break;
                }
                
                // Wait and retry
                if let Some(delay) = backoff.next_delay() {
                    session.status = SessionStatus::Reconnecting;
                    session.reconnect_count += 1;
                    
                    let _ = event_tx.send(Event::session_reconnecting(
                        session.id,
                        &session.profile_name,
                        session.reconnect_count,
                        profile.max_reconnect_attempts,
                    ));
                    
                    drop(session);
                    
                    tokio::select! {
                        _ = sleep(delay) => continue,
                        _ = stop_rx.recv() => break,
                    }
                }
                continue;
            }
        };

        // Update session with PID
        {
            let mut session = session_handle.write().await;
            session.pid = Some(process.pid);
            session.status = SessionStatus::Starting;
        }

        // Create and run monitor
        let mut monitor = SessionMonitor::new(
            session_handle.clone(),
            process,
            event_tx.clone(),
            backoff.clone(),
        );

        // Run monitor with stop signal handling
        let result = tokio::select! {
            result = monitor.run() => result,
            _ = stop_rx.recv() => {
                let _ = monitor.stop().await;
                break;
            }
        };

        // Handle result
        match result {
            MonitorResult::ExitedNormally => {
                tracing::info!("Session '{}' exited normally", profile.name);
                backoff.reset();
                
                if !profile.auto_reconnect {
                    let mut session = session_handle.write().await;
                    session.status = SessionStatus::Stopped;
                    break;
                }
            }
            MonitorResult::ExitedWithError(code, msg) => {
                tracing::warn!("Session '{}' exited with code {}: {}", profile.name, code, msg);
                
                let mut session = session_handle.write().await;
                session.last_error = Some(msg.clone());
                
                let _ = event_tx.send(Event::session_disconnected(
                    session.id,
                    &session.profile_name,
                    Some(msg),
                ));
                
                if !profile.auto_reconnect || backoff.is_exhausted() {
                    session.status = SessionStatus::Failed;
                    break;
                }
            }
            MonitorResult::Killed => {
                tracing::warn!("Session '{}' was killed", profile.name);
                break;
            }
            MonitorResult::Stopped => {
                break;
            }
        }

        // Reconnect delay
        if let Some(delay) = backoff.next_delay() {
            let mut session = session_handle.write().await;
            session.status = SessionStatus::Reconnecting;
            session.reconnect_count += 1;
            
            let _ = event_tx.send(Event::session_reconnecting(
                session.id,
                &session.profile_name,
                session.reconnect_count,
                profile.max_reconnect_attempts,
            ));
            
            drop(session);
            
            tokio::select! {
                _ = sleep(delay) => {},
                _ = stop_rx.recv() => break,
            }
        } else {
            // Max attempts reached
            let mut session = session_handle.write().await;
            session.status = SessionStatus::Failed;
            
            let _ = event_tx.send(Event::session_failed(
                session.id,
                &session.profile_name,
                "Maximum reconnection attempts reached",
            ));
            break;
        }
    }

    // Remove from active sessions
    let mut sessions_guard = sessions.write().await;
    sessions_guard.remove(&session_id);
}
