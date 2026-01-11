use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use anyhow::Result;
use chrono::Utc;

use crate::types::profile::Profile;
use crate::types::session::{Session, SessionStatus};
use crate::ssh::spawn;

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
        let state = self.state.lock().await;
        state.sessions.values().cloned().collect()
    }
    
    pub async fn get_session(&self, id: &str) -> Option<Session> {
        let state = self.state.lock().await;
        state.sessions.get(id).cloned()
    }

    pub async fn start(&self, profile: Profile) -> Result<()> {
        let mut state = self.state.lock().await;
        let id = profile.id.clone();

        // 1. Check if running
        if let Some(session) = state.sessions.get(&id) {
            if matches!(session.status, SessionStatus::Running | SessionStatus::Starting) {
                // If it's effectively running, return ok
                return Ok(());
            }
        }
        
        // 2. Spawn SSH
        // We clone profile because we need it for spawning, 
        // but we already cloned ID.
        let (mut child, _askpass) = spawn::spawn_session(&profile)?;
        let pid = child.id();
        
        // 3. Update Status
        let mut session = Session::new(id.clone());
        session.status = SessionStatus::Running;
        session.pid = pid;
        session.start_time = Some(Utc::now());
        state.sessions.insert(id.clone(), session);

        // 4. Spawn Monitor Task
        let state_clone = self.state.clone();
        let profile_id = id.clone();
        
        let task = tokio::spawn(async move {
            // Keep _askpass alive until the process exits
            let _keep = _askpass;

            // Wait for process to exit
            let result = child.wait().await;
            
            // On exit, update status
            let mut s = state_clone.lock().await;
            
            // Remove self from tasks map to avoid leak? 
            // Actually we do it at the end, but we need to handle the session update first.
            
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
            }
            
            // Remove the handle from the map since it's done
            s.tasks.remove(&profile_id); 
        });

        state.tasks.insert(id, task);

        Ok(())
    }

    pub async fn stop(&self, id: &str) -> Result<()> {
        let mut state = self.state.lock().await;
        
        if let Some(task) = state.tasks.remove(id) {
            // Aborting the task drops the `child` process handle.
            // Since `kill_on_drop(true)` is set, this kills the SSH process.
            task.abort(); 
        }

        if let Some(session) = state.sessions.get_mut(id) {
            session.status = SessionStatus::Stopped;
            session.pid = None;
            session.start_time = None;
        }

        Ok(())
    }
}
