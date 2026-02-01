use std::time::Duration;

use tokio::time::sleep;

use crate::error::Result;
use crate::ssh::{SshOutput, SshProcess};
use crate::types::{Event, EventSender, SessionHandle, SessionStatus};

use super::backoff::Backoff;

/// Monitor result indicating what happened
#[derive(Debug)]
pub enum MonitorResult {
    /// Process exited normally (code 0)
    ExitedNormally,
    /// Process exited with an error code
    ExitedWithError(i32, String),
    /// Process was killed by signal
    Killed,
    /// Monitor was stopped externally
    Stopped,
}

/// Monitor an SSH process and update session state
pub struct SessionMonitor {
    session: SessionHandle,
    process: SshProcess,
    event_tx: EventSender,
    #[allow(dead_code)]
    backoff: Backoff,
}

impl SessionMonitor {
    pub fn new(
        session: SessionHandle,
        process: SshProcess,
        event_tx: EventSender,
        backoff: Backoff,
    ) -> Self {
        Self {
            session,
            process,
            event_tx,
            backoff,
        }
    }

    /// Run the monitor loop
    /// Returns when the process exits or is stopped
    pub async fn run(&mut self) -> MonitorResult {
        let mut last_output = String::new();
        
        loop {
            tokio::select! {
                // Check for process output
                output = self.process.output_rx.recv() => {
                    match output {
                        Some(SshOutput::Stdout(line)) => {
                            self.handle_output(&line, false).await;
                            last_output = line;
                        }
                        Some(SshOutput::Stderr(line)) => {
                            self.handle_output(&line, true).await;
                            
                            // Check for connection established indicators
                            if self.is_connection_established(&line) {
                                self.mark_connected().await;
                            }
                            
                            last_output = line;
                        }
                        Some(SshOutput::Exit(code)) => {
                            return self.handle_exit(code, &last_output).await;
                        }
                        None => {
                            // Channel closed, process likely exited
                            return self.check_process_status(&last_output).await;
                        }
                    }
                }
                
                // Periodic health check
                _ = sleep(Duration::from_secs(30)) => {
                    let check_result = self.check_process_status(&last_output).await;
                    if let Some(result) = IntoOption::into(check_result) {
                        return result;
                    }
                }
            }
        }
    }

    async fn handle_output(&self, line: &str, is_stderr: bool) {
        // Log SSH debug/error output to help diagnose password auth failures
        if is_stderr {
            tracing::debug!("SSH stderr: {}", line);
        }

        let session = self.session.read().await;
        let _ = self.event_tx.send(Event::session_output(
            session.id,
            &session.profile_name,
            line,
            is_stderr,
        ));
    }

    fn is_connection_established(&self, line: &str) -> bool {
        // Common indicators that SSH tunnel is established
        let indicators = [
            "Authenticated to",
            "pledge: ",
            "debug1: Entering interactive session",
            "debug1: Remote connections from",
        ];
        
        indicators.iter().any(|i| line.contains(i))
    }

    async fn mark_connected(&self) {
        let mut session = self.session.write().await;
        let old_status = session.status;
        session.status = SessionStatus::Connected;
        session.connected_at = Some(chrono::Utc::now());
        
        let _ = self.event_tx.send(Event::session_status_changed(
            session.id,
            &session.profile_name,
            old_status,
            SessionStatus::Connected,
        ));
        
        let _ = self.event_tx.send(Event::session_connected(
            session.id,
            &session.profile_name,
        ));

        tracing::info!("Session {} connected", session.profile_name);
    }

    async fn handle_exit(&self, code: Option<i32>, last_output: &str) -> MonitorResult {
        match code {
            Some(0) => MonitorResult::ExitedNormally,
            Some(code) => MonitorResult::ExitedWithError(code, last_output.to_string()),
            None => MonitorResult::Killed,
        }
    }

    async fn check_process_status(&mut self, last_output: &str) -> MonitorResult {
        match self.process.try_wait() {
            Ok(Some(code)) => self.handle_exit(code, last_output).await,
            Ok(None) => {
                // Process still running, this shouldn't happen in the None branch
                // but we'll continue monitoring
                MonitorResult::Stopped
            }
            Err(_) => MonitorResult::ExitedWithError(-1, "Failed to check process status".to_string()),
        }
    }

    /// Stop the monitored process
    pub async fn stop(&mut self) -> Result<()> {
        self.process.kill().await
    }
}

/// Helper trait for converting check results
trait IntoOption<T> {
    fn into(self) -> Option<T>;
}

impl IntoOption<MonitorResult> for MonitorResult {
    fn into(self) -> Option<MonitorResult> {
        match self {
            MonitorResult::Stopped => None, // Continue monitoring
            other => Some(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_result() {
        let result = MonitorResult::ExitedNormally;
        assert!(matches!(result, MonitorResult::ExitedNormally));

        let result = MonitorResult::ExitedWithError(1, "error".to_string());
        assert!(matches!(result, MonitorResult::ExitedWithError(1, _)));
    }
}
