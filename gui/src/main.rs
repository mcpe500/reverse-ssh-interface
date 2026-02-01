// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{
    AppHandle, Emitter, Manager,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tokio::sync::RwLock;
use uuid::Uuid;

use reverse_ssh_core::{
    config::{load_config, profiles_dir, load_profiles, save_profile, update_profile as core_update_profile, delete_profile as core_delete_profile},
    supervisor::{SessionManager, SessionManagerHandle, StartSessionOptions},
    types::{Profile, TunnelSpec, AuthMethod, Session, Event},
    error::CoreError,
};

// ============================================================================
// State Management
// ============================================================================

struct AppState {
    manager_handle: Arc<RwLock<Option<SessionManagerHandle>>>,
    sessions: Arc<RwLock<HashMap<Uuid, SessionInfo>>>,
}

#[derive(Debug, Clone, Serialize)]
struct SessionInfo {
    id: String,
    profile_name: String,
    status: String,
    started_at: String,
    pid: Option<u32>,
    reconnect_count: u32,
}

impl From<&Session> for SessionInfo {
    fn from(session: &Session) -> Self {
        Self {
            id: session.id.to_string(),
            profile_name: session.profile_name.clone(),
            status: format!("{:?}", session.status),
            started_at: session.started_at.to_rfc3339(),
            pid: session.pid,
            reconnect_count: session.reconnect_count,
        }
    }
}

// ============================================================================
// API Types for Frontend
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub auth: String,
    pub tunnels: Vec<TunnelInfo>,
    pub auto_reconnect: bool,
    pub keepalive_interval: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelInfo {
    pub remote_bind: String,
    pub remote_port: u16,
    pub local_host: String,
    pub local_port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProfileRequest {
    pub name: String,
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub auth: Option<String>,
    pub key_path: Option<String>,
    pub tunnels: Vec<TunnelInfo>,
    pub auto_reconnect: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProfileRequest {
    pub existing_name: String,
    pub name: String,
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub auth: Option<String>,
    pub key_path: Option<String>,
    pub tunnels: Vec<TunnelInfo>,
    pub auto_reconnect: Option<bool>,
}

impl From<&Profile> for ProfileInfo {
    fn from(profile: &Profile) -> Self {
        Self {
            id: profile.id.to_string(),
            name: profile.name.clone(),
            host: profile.host.clone(),
            port: profile.port,
            user: profile.user.clone(),
            auth: match &profile.auth {
                AuthMethod::Agent => "agent".to_string(),
                AuthMethod::KeyFile { path } => format!("key:{}", path),
                AuthMethod::Password => "password".to_string(),
            },
            tunnels: profile.tunnels.iter().map(|t| TunnelInfo {
                remote_bind: t.remote_bind.clone(),
                remote_port: t.remote_port,
                local_host: t.local_host.clone(),
                local_port: t.local_port,
            }).collect(),
            auto_reconnect: profile.auto_reconnect,
            keepalive_interval: profile.keepalive_interval,
        }
    }
}

// ============================================================================
// Tauri Commands
// ============================================================================

fn load_profile_by_name(name: &str) -> Result<Profile, CoreError> {
    let profiles = load_profiles()?;
    profiles
        .into_iter()
        .find(|p| p.name == name)
        .ok_or_else(|| CoreError::ProfileNotFound(name.to_string()))
}

/// Get all profiles
#[tauri::command]
async fn get_profiles() -> Result<Vec<ProfileInfo>, String> {
    let profiles = load_profiles()
        .map_err(|e: CoreError| e.to_string())?;
    
    Ok(profiles.iter().map(ProfileInfo::from).collect())
}

/// Get a specific profile by name
#[tauri::command]
async fn get_profile(name: String) -> Result<ProfileInfo, String> {
    let profile = load_profile_by_name(&name)
        .map_err(|e: CoreError| e.to_string())?;
    
    Ok(ProfileInfo::from(&profile))
}

/// Create a new profile
#[tauri::command]
async fn create_profile(request: CreateProfileRequest) -> Result<ProfileInfo, String> {
    let auth = match request.auth.as_deref() {
        Some("password") => AuthMethod::Password,
        Some(s) if s.starts_with("key:") => AuthMethod::KeyFile { 
            path: s.strip_prefix("key:").unwrap().to_string() 
        },
        _ => {
            if let Some(key_path) = request.key_path {
                AuthMethod::KeyFile { path: key_path }
            } else {
                AuthMethod::Agent
            }
        }
    };

    let tunnels: Vec<TunnelSpec> = request.tunnels.iter().map(|t| {
        TunnelSpec {
            remote_bind: t.remote_bind.clone(),
            remote_port: t.remote_port,
            local_host: t.local_host.clone(),
            local_port: t.local_port,
        }
    }).collect();

    let profile = Profile {
        id: Uuid::new_v4(),
        name: request.name.clone(),
        host: request.host,
        port: request.port.unwrap_or(22),
        user: request.user,
        auth,
        tunnels,
        keepalive_interval: 20,
        keepalive_count: 3,
        auto_reconnect: request.auto_reconnect.unwrap_or(true),
        max_reconnect_attempts: 0,
        extra_options: HashMap::new(),
        ssh_path: None,
        known_hosts_file: None,
        identity_file: None,
    };

    save_profile(&profile)
        .map_err(|e: CoreError| e.to_string())?;

    Ok(ProfileInfo::from(&profile))
}

/// Update an existing profile (supports rename)
#[tauri::command]
async fn update_profile(request: UpdateProfileRequest) -> Result<ProfileInfo, String> {
    let mut profile = load_profile_by_name(&request.existing_name)
        .map_err(|e: CoreError| e.to_string())?;

    if request.tunnels.is_empty() {
        return Err("At least one tunnel is required".to_string());
    }

    let auth = match request.auth.as_deref() {
        Some("password") => AuthMethod::Password,
        Some(s) if s.starts_with("key:") => {
            let path = s.strip_prefix("key:").unwrap_or_default().to_string();
            if path.trim().is_empty() {
                return Err("Key file path is required for key auth".to_string());
            }
            AuthMethod::KeyFile { path }
        }
        _ => {
            if let Some(key_path) = request.key_path {
                if key_path.trim().is_empty() {
                    return Err("Key file path is required for key auth".to_string());
                }
                AuthMethod::KeyFile { path: key_path }
            } else {
                AuthMethod::Agent
            }
        }
    };

    let tunnels: Vec<TunnelSpec> = request
        .tunnels
        .iter()
        .map(|t| TunnelSpec {
            remote_bind: t.remote_bind.clone(),
            remote_port: t.remote_port,
            local_host: t.local_host.clone(),
            local_port: t.local_port,
        })
        .collect();

    profile.name = request.name;
    profile.host = request.host;
    profile.port = request.port.unwrap_or(22);
    profile.user = request.user;
    profile.auth = auth;
    profile.tunnels = tunnels;
    if let Some(auto_reconnect) = request.auto_reconnect {
        profile.auto_reconnect = auto_reconnect;
    }

    core_update_profile(&request.existing_name, &profile)
        .map_err(|e: CoreError| e.to_string())?;

    Ok(ProfileInfo::from(&profile))
}

/// Delete a profile
#[tauri::command]
async fn delete_profile(name: String) -> Result<(), String> {
    // Load the profile first to get its full data
    let profile = load_profile_by_name(&name)
        .map_err(|e: CoreError| e.to_string())?;
    
    core_delete_profile(&profile)
        .map_err(|e: CoreError| e.to_string())
}

/// Start a session for a profile
#[tauri::command]
async fn start_session(
    name: String,
    password: Option<String>,
    state: tauri::State<'_, Arc<AppState>>,
    app_handle: AppHandle,
) -> Result<SessionInfo, String> {
    // Load profile
    let profile = load_profile_by_name(&name)
        .map_err(|e: CoreError| e.to_string())?;

    let manager_handle = state.manager_handle.read().await;
    let handle = manager_handle.as_ref()
        .ok_or_else(|| "Session manager not initialized".to_string())?;

    let password = password.and_then(|p| {
        let trimmed = p.trim().to_string();
        if trimmed.is_empty() { None } else { Some(trimmed) }
    });

    let session_id = handle
        .start_with_options(profile, StartSessionOptions { password })
        .await
        .map_err(|e| e.to_string())?;

    let session_info = SessionInfo {
        id: session_id.to_string(),
        profile_name: name.clone(),
        status: "Starting".to_string(),
        started_at: chrono::Utc::now().to_rfc3339(),
        pid: None,
        reconnect_count: 0,
    };

    // Store session
    state.sessions.write().await.insert(session_id, session_info.clone());

    // Emit event to frontend
    let _ = app_handle.emit("session-started", &session_info);

    Ok(session_info)
}

/// Stop a session
#[tauri::command]
async fn stop_session(
    session_id: String,
    state: tauri::State<'_, Arc<AppState>>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let id = Uuid::parse_str(&session_id)
        .map_err(|e| e.to_string())?;

    let manager_handle = state.manager_handle.read().await;
    let handle = manager_handle.as_ref()
        .ok_or_else(|| "Session manager not initialized".to_string())?;

    handle.stop(id)
        .await
        .map_err(|e| e.to_string())?;

    state.sessions.write().await.remove(&id);
    let _ = app_handle.emit("session-stopped", session_id);
    
    Ok(())
}

/// Get all active sessions
#[tauri::command]
async fn get_sessions(state: tauri::State<'_, Arc<AppState>>) -> Result<Vec<SessionInfo>, String> {
    let manager_handle = state.manager_handle.read().await;
    
    if let Some(handle) = manager_handle.as_ref() {
        let sessions = handle.status()
            .await
            .map_err(|e| e.to_string())?;

        let infos: Vec<SessionInfo> = sessions.iter().map(SessionInfo::from).collect();
        
        // Update local cache
        let mut cache = state.sessions.write().await;
        cache.clear();
        for session in &sessions {
            cache.insert(session.id, SessionInfo::from(session));
        }
        
        Ok(infos)
    } else {
        Ok(state.sessions.read().await.values().cloned().collect())
    }
}

/// Stop all sessions
#[tauri::command]
async fn stop_all_sessions(
    state: tauri::State<'_, Arc<AppState>>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let manager_handle = state.manager_handle.read().await;
    let handle = manager_handle.as_ref()
        .ok_or_else(|| "Session manager not initialized".to_string())?;

    handle.stop_all()
        .await
        .map_err(|e| e.to_string())?;

    state.sessions.write().await.clear();
    let _ = app_handle.emit("all-sessions-stopped", ());
    
    Ok(())
}

/// Get app configuration
#[tauri::command]
async fn get_config() -> Result<serde_json::Value, String> {
    let config = load_config()
        .map_err(|e: CoreError| e.to_string())?;
    
    serde_json::to_value(&config)
        .map_err(|e| e.to_string())
}

/// Get profiles directory path
#[tauri::command]
async fn get_profiles_path() -> Result<String, String> {
    let path = profiles_dir();
    Ok(path.to_string_lossy().to_string())
}

// ============================================================================
// Event Listener
// ============================================================================

async fn setup_event_listener(
    app_handle: AppHandle,
    state: Arc<AppState>,
    mut event_rx: tokio::sync::broadcast::Receiver<Event>,
) {
    loop {
        match event_rx.recv().await {
            Ok(event) => {
                let event_data = serde_json::to_value(&event).unwrap_or_default();
                
                match &event {
                    Event::SessionConnected { session_id, .. } => {
                        if let Some(session) = state.sessions.write().await.get_mut(session_id) {
                            session.status = "Connected".to_string();
                        }
                        let _ = app_handle.emit("session-connected", event_data);
                    }
                    Event::SessionDisconnected { session_id, .. } => {
                        if let Some(session) = state.sessions.write().await.get_mut(session_id) {
                            session.status = "Disconnected".to_string();
                        }
                        let _ = app_handle.emit("session-disconnected", event_data);
                    }
                    Event::SessionReconnecting { session_id, attempt, .. } => {
                        if let Some(session) = state.sessions.write().await.get_mut(session_id) {
                            session.status = "Reconnecting".to_string();
                            session.reconnect_count = *attempt;
                        }
                        let _ = app_handle.emit("session-reconnecting", event_data);
                    }
                    Event::SessionFailed { session_id, .. } => {
                        if let Some(session) = state.sessions.write().await.get_mut(session_id) {
                            session.status = "Failed".to_string();
                        }
                        let _ = app_handle.emit("session-failed", event_data);
                    }
                    Event::SessionOutput { .. } => {
                        let _ = app_handle.emit("session-output", event_data);
                    }
                    _ => {}
                }
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                // Missed some events, continue
                continue;
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                break;
            }
        }
    }
}

// ============================================================================
// Main Application
// ============================================================================

fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("reverse_ssh_gui=info".parse().unwrap())
                .add_directive("reverse_ssh_core=info".parse().unwrap()),
        )
        .init();

    tracing::info!("Starting Reverse SSH Interface GUI");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Create app state
            let state = Arc::new(AppState {
                manager_handle: Arc::new(RwLock::new(None)),
                sessions: Arc::new(RwLock::new(HashMap::new())),
            });

            // Initialize session manager in background
            let state_clone = state.clone();
            let app_handle = app.handle().clone();
            
            tauri::async_runtime::spawn(async move {
                // Load config
                let config = match load_config() {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::error!("Failed to load config: {}", e);
                        return;
                    }
                };

                // Create session manager
                let (mut manager, handle) = SessionManager::new(config);
                
                // Store handle
                *state_clone.manager_handle.write().await = Some(handle.clone());

                // Subscribe to events
                let event_rx = handle.subscribe();
                let state_for_events = state_clone.clone();
                
                tauri::async_runtime::spawn(async move {
                    setup_event_listener(app_handle, state_for_events, event_rx).await;
                });

                // Initialize and run manager
                if let Err(e) = manager.init().await {
                    tracing::error!("Failed to initialize session manager: {}", e);
                    return;
                }

                tracing::info!("Session manager initialized");
                
                if let Err(e) = manager.run().await {
                    tracing::error!("Session manager error: {}", e);
                }
            });

            // Manage state
            app.manage(state);

            // Setup system tray
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Reverse SSH Interface")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_profiles,
            get_profile,
            create_profile,
            update_profile,
            delete_profile,
            start_session,
            stop_session,
            get_sessions,
            stop_all_sessions,
            get_config,
            get_profiles_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
