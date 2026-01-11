use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use anyhow::Result;
use chrono::Utc;

use crate::types::profile::Profile;
use crate::types::session::{Session, SessionStatus};
use crate::ssh::spawn;
use crate::storage::state;

#[derive(Clone)]
pub struct SessionManager {
    state: Arc<Mutex<State>>,
}

struct State {
    sessions: HashMap<String, Session>,
    tasks: HashMap<String, JoinHandle<()>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(State {
                sessions: HashMap::new(),
                tasks: HashMap::new(),
            })),
        }
    }

    pub async fn list_sessions(&self) -> Vec<Session> {
        // First, load from disk to see sessions from other processes
        let mut persisted = state::list_persisted_sessions().await.unwrap_or_default();
        let state = self.state.lock().await;
        
        // Merge with local state (local state is more up-to-date for local tasks)
        for session in persisted.iter_mut() {
            if let Some(local) = state.sessions.get(&session.profile_id) {
                *session = local.clone();
            } else {
                // If it's not local, verify if PID is still alive
                if let Some(pid) = session.pid {
                    if !is_pid_alive(pid) {
                        session.status = SessionStatus::Stopped;
                        session.pid = None;
                        // Clean up dead session from disk?
                        let _ = state::remove_session(&session.profile_id).await;
                    }
                }
            }
        }
        
        // Add local sessions that might not be in persisted yet (unlikely but safe)
        for (id, local) in &state.sessions {
            if !persisted.iter().any(|s| s.profile_id == *id) {
                persisted.push(local.clone());
            }
        }

        persisted
    }
    
    pub async fn get_session(&self, id: &str) -> Option<Session> {
        let state = self.state.lock().await;
        if let Some(local) = state.sessions.get(id) {
            return Some(local.clone());
        }
        
        // Try disk
        if let Ok(Some(mut persisted)) = state::load_session(id).await {
            if let Some(pid) = persisted.pid {
                if !is_pid_alive(pid) {
                    persisted.status = SessionStatus::Stopped;
                    persisted.pid = None;
                    let _ = state::remove_session(id).await;
                }
            }
            return Some(persisted);
        }
        
        None
    }

    pub async fn start(&self, profile: Profile) -> Result<()> {
        let mut state = self.state.lock().await;
        let id = profile.id.clone();

        // 1. Check if running
        if let Some(session) = state.sessions.get(&id) {
            if matches!(session.status, SessionStatus::Running | SessionStatus::Starting) {
                return Ok(());
            }
        }
        
        // Also check disk/PID for other processes
        if let Ok(Some(persisted)) = state::load_session(&id).await {
            if let Some(pid) = persisted.pid {
                if is_pid_alive(pid) {
                    return Ok(());
                }
            }
        }
        
        // 2. Spawn SSH
        let (mut child, _askpass) = spawn::spawn_session(&profile)?;
        let pid = child.id();
        
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        // 3. Update Status
        let mut session = Session::new(id.clone());
        session.status = SessionStatus::Running;
        session.pid = pid;
        session.start_time = Some(Utc::now());
        
        // Persist
        state::save_session(&session).await?;
        state.sessions.insert(id.clone(), session);

        // 4. Spawn Monitor Task
        let state_clone = self.state.clone();
        let profile_id = id.clone();
        let log_path = crate::config::paths::logs_dir()?.join(format!("{}.log", profile_id));
        
        let task = tokio::spawn(async move {
            let _keep = _askpass;

            // Redirect logs
            if let (Some(mut out), Some(mut err)) = (stdout, stderr) {
                if let Ok(log_file) = tokio::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_path)
                    .await {
                        if let Ok(mut log_file_err) = log_file.try_clone().await {
                            tokio::spawn(async move {
                                let _ = tokio::io::copy(&mut out, &mut {log_file}).await;
                            });
                            tokio::spawn(async move {
                                let _ = tokio::io::copy(&mut err, &mut log_file_err).await;
                            });
                        } else {
                            // Fallback to sequential if clone fails (unlikely)
                            tokio::spawn(async move {
                                let mut log_file = log_file;
                                let _ = tokio::io::copy(&mut out, &mut log_file).await;
                                let _ = tokio::io::copy(&mut err, &mut log_file).await;
                            });
                        }
                    }
            }

            let result = child.wait().await;
            
            let mut s = state_clone.lock().await;
            if let Some(sess) = s.sessions.get_mut(&profile_id) {
                match result {
                    Ok(status) if status.success() => {
                        sess.status = SessionStatus::Stopped;
                    }
                    Ok(status) => {
                         sess.status = SessionStatus::Failed(format!("Exited with code: {:?}", status.code()));
                    }
                    Err(e) => {
                        sess.status = SessionStatus::Failed(format!("IO Error: {}", e));
                    }
                }
                sess.pid = None;
                sess.start_time = None;
                
                // Update persisted state
                let _ = state::save_session(sess).await;
            }
            s.tasks.remove(&profile_id); 
        });

        state.tasks.insert(id, task);

        Ok(())
    }

    pub async fn stop(&self, id: &str) -> Result<()> {
        let mut state_lock = self.state.lock().await;
        
        // 1. Stop local task if exists
        if let Some(task) = state_lock.tasks.remove(id) {
            task.abort(); 
        }

        // 2. Kill PID if exists (handles other processes too)
        let session = if let Some(sess) = state_lock.sessions.get_mut(id) {
            Some(sess)
        } else {
            None // We'll check disk below
        };

        if let Some(sess) = session {
            if let Some(pid) = sess.pid {
                kill_pid(pid);
            }
            sess.status = SessionStatus::Stopped;
            sess.pid = None;
            sess.start_time = None;
            state::save_session(sess).await?;
        } else if let Ok(Some(mut persisted)) = state::load_session(id).await {
            if let Some(pid) = persisted.pid {
                kill_pid(pid);
            }
            persisted.status = SessionStatus::Stopped;
            persisted.pid = None;
            persisted.start_time = None;
            state::save_session(&persisted).await?;
        }

        Ok(())
    }
}

fn is_pid_alive(pid: u32) -> bool {
    unsafe { libc::kill(pid as libc::pid_t, 0) == 0 }
}

fn kill_pid(pid: u32) {
    unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
}
