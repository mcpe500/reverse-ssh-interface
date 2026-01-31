use std::sync::Arc;
use reverse_ssh_core::supervisor::SessionManagerHandle;

#[derive(Clone)]
pub struct AppState {
    pub handle: Arc<SessionManagerHandle>,
}

impl AppState {
    pub fn new(handle: SessionManagerHandle) -> Self {
        Self {
            handle: Arc::new(handle),
        }
    }
}
