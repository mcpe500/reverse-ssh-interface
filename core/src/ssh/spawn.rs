use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

use crate::error::{CoreError, Result};
use crate::types::Profile;

use super::args::{validate_args, SshArgs};
use super::detect::SshInfo;

/// Output from the SSH process
#[derive(Debug, Clone)]
pub enum SshOutput {
    Stdout(String),
    Stderr(String),
    Exit(Option<i32>),
}

/// Handle to a spawned SSH process
pub struct SshProcess {
    /// The child process
    child: Child,
    /// Process ID
    pub pid: u32,
    /// Channel for receiving output
    pub output_rx: mpsc::Receiver<SshOutput>,
}

impl SshProcess {
    /// Wait for the process to exit
    pub async fn wait(&mut self) -> Result<Option<i32>> {
        let status = self.child.wait().await?;
        Ok(status.code())
    }

    /// Kill the process
    pub async fn kill(&mut self) -> Result<()> {
        self.child.kill().await?;
        Ok(())
    }

    /// Check if the process is still running
    pub fn try_wait(&mut self) -> Result<Option<Option<i32>>> {
        match self.child.try_wait()? {
            Some(status) => Ok(Some(status.code())),
            None => Ok(None),
        }
    }
}

/// Spawn an SSH process for the given profile
pub async fn spawn_ssh(ssh_info: &SshInfo, profile: &Profile) -> Result<SshProcess> {
    let args = SshArgs::from_profile(profile).build_tunnel_mode();
    spawn_ssh_with_args(ssh_info, args).await
}

/// Spawn an SSH process with custom arguments
pub async fn spawn_ssh_with_args(ssh_info: &SshInfo, args: Vec<String>) -> Result<SshProcess> {
    // Validate arguments before spawning
    validate_args(&args).map_err(|e| CoreError::SshSpawnFailed(e))?;

    tracing::debug!("Spawning SSH with args: {:?}", args);

    let mut child = Command::new(&ssh_info.path)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| CoreError::SshSpawnFailed(e.to_string()))?;

    let pid = child.id().ok_or_else(|| {
        CoreError::SshSpawnFailed("Failed to get process ID".to_string())
    })?;

    // Create channel for output
    let (tx, rx) = mpsc::channel(100);

    // Spawn task to read stdout
    let stdout = child.stdout.take();
    let tx_stdout = tx.clone();
    if let Some(stdout) = stdout {
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if tx_stdout.send(SshOutput::Stdout(line)).await.is_err() {
                    break;
                }
            }
        });
    }

    // Spawn task to read stderr
    let stderr = child.stderr.take();
    let tx_stderr = tx.clone();
    if let Some(stderr) = stderr {
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if tx_stderr.send(SshOutput::Stderr(line)).await.is_err() {
                    break;
                }
            }
        });
    }

    tracing::info!("Spawned SSH process with PID {}", pid);

    Ok(SshProcess {
        child,
        pid,
        output_rx: rx,
    })
}

/// Test SSH connection without establishing tunnels
/// Returns Ok(()) if connection succeeds, Err otherwise
pub async fn test_connection(ssh_info: &SshInfo, profile: &Profile) -> Result<()> {
    let args = SshArgs::new()
        .option("ServerAliveInterval", &profile.keepalive_interval.to_string())
        .option("ServerAliveCountMax", "1")
        .option("ConnectTimeout", "10")
        .option("BatchMode", "yes")
        .no_tty()
        .port(profile.port)
        .destination(&profile.destination())
        .build();

    // Add "exit" command to just test connection
    let mut full_args = args;
    full_args.push("exit".to_string());

    let output = Command::new(&ssh_info.path)
        .args(&full_args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| CoreError::SshSpawnFailed(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(CoreError::SshExitError {
            code: output.status.code().unwrap_or(-1),
            message: stderr.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would require actual SSH setup
    // These are placeholder tests

    #[test]
    fn test_ssh_output_enum() {
        let stdout = SshOutput::Stdout("test".to_string());
        let stderr = SshOutput::Stderr("error".to_string());
        let exit = SshOutput::Exit(Some(0));

        match stdout {
            SshOutput::Stdout(s) => assert_eq!(s, "test"),
            _ => panic!("Expected stdout"),
        }

        match stderr {
            SshOutput::Stderr(s) => assert_eq!(s, "error"),
            _ => panic!("Expected stderr"),
        }

        match exit {
            SshOutput::Exit(code) => assert_eq!(code, Some(0)),
            _ => panic!("Expected exit"),
        }
    }
}
