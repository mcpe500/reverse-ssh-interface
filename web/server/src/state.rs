use reverse_ssh_core::supervisor::manager::SessionManager;

#[derive(Clone)]
pub struct AppState {
    pub session_manager: SessionManager,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            session_manager: SessionManager::new(),
        }
    }
}
