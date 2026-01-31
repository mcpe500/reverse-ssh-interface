//! Prelude module - common re-exports for convenience

pub use crate::error::{CoreError, Result};
pub use crate::types::{
    AuthMethod, Event, EventReceiver, EventSender, Profile, Session, SessionHandle, SessionStatus,
    TunnelSpec, event_channel, new_session_handle,
};
